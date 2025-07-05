// buffer_track.rs
// A type of buffer that stores buffer as audio data.
//
// Copyright 2025 Shuntaro Kasatani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use crate::{
    AudioResampler, AudioSource, Graph, Region, Track, Value,
    audio_utils::{self, Beats},
    error::{
        TrackError,
        track::{UnknownTrackError, region::InvalidRegionTypeError},
    },
    graph::built_in::EmptyNode,
    mixing::region::BufferRegion,
};
use std::any::Any;

pub struct BufferTrack {
    /// Unique identifier for the track.
    /// This will be set by adding the track to a mixer, so you don't need to set it manually.
    pub id: u32,
    /// Name of the track.
    pub name: String,
    /// Volume of the track.
    pub volume: f32,
    /// Audio node graph.
    pub graph: Graph,
    /// Number of channels in the track.
    pub channels: usize,
    /// Vector of regions in the track.
    pub regions: Vec<BufferRegion>,
    /// Rendered audio data.
    pub rendered_data: Option<AudioSource>,
    /// Resamplers for each regions.
    resamplers: Vec<AudioResampler>,
    /// Residual sample numbers while rendering.
    residual_samples: f32,
    /// Error that occurred during rendering.
    render_error: Option<Box<dyn TrackError>>,
}

impl BufferTrack {
    pub fn new(name: &str, channels: usize) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
            volume: 1.0,
            graph: Graph::new(Box::new(EmptyNode::new_input())),
            channels,
            regions: Vec::new(),
            rendered_data: None,
            resamplers: Vec::new(),
            residual_samples: 0.0,
            render_error: None,
        }
    }
}

impl Track for BufferTrack {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn graph(&self) -> &Graph {
        &self.graph
    }

    fn graph_mut(&mut self) -> &mut Graph {
        &mut self.graph
    }

    fn volume(&self) -> f32 {
        self.volume
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    fn channels(&self) -> usize {
        self.channels
    }

    fn track_type(&self) -> String {
        "BufferTrack".to_string()
    }

    fn regions(&self) -> Vec<&dyn Region> {
        self.regions
            .iter()
            .map(|region| region as &dyn Region)
            .collect()
    }

    fn regions_mut(&mut self) -> Vec<&mut dyn Region> {
        self.regions
            .iter_mut()
            .map(|region| region as &mut dyn Region)
            .collect()
    }

    fn get_region(&mut self, id: u32) -> Option<&dyn Region> {
        self.regions
            .iter()
            .find(|r| *r.get_id() == id)
            .map(|r| r as &dyn Region)
    }

    fn get_region_mut(&mut self, id: u32) -> Option<&mut dyn Region> {
        self.regions
            .iter_mut()
            .find(|r| *r.get_id() == id)
            .map(|r| r as &mut dyn Region)
    }

    fn add_region(
        &mut self,
        region: Box<dyn Region>,
        at: Beats,
        duration: Beats,
    ) -> Result<u32, Box<dyn TrackError>> {
        if let Some(buffer_region) = region.as_any().downcast_ref::<BufferRegion>() {
            let mut buffer_region = buffer_region.clone();
            let id = self.generate_region_id();
            buffer_region.set_start_time(at);
            buffer_region.set_duration(duration);
            buffer_region.set_id(id);
            self.regions.push(buffer_region);
            return Ok(id);
        } else {
            return Err(Box::new(InvalidRegionTypeError {
                expected_type: "BufferRegion".to_string(),
                received_type: region.region_type(),
            }));
        }
    }

    fn remove_region(&mut self, id: u32) {
        self.regions.retain(|r| *r.get_id() != id);
    }

    fn generate_region_id(&self) -> u32 {
        let mut id = 0;
        while self.regions.iter().any(|r| *r.get_id() == id) {
            id += 1;
        }
        id
    }

    fn duration(&self) -> Beats {
        self.regions
            .iter()
            .map(|r| r.end_time())
            .fold(0.0, |max, end| max.max(end))
    }

    fn prepare(
        &mut self,
        chunk_size: Beats,
        sample_rate: usize,
        tempo: Beats,
    ) -> Result<(), Box<dyn TrackError>> {
        self.graph
            .prepare(chunk_size, sample_rate, tempo, self.id)?;
        self.resamplers.resize_with(self.regions.len(), || {
            AudioResampler::new(sample_rate / 100)
        });
        for region in &self.regions {
            self.resamplers
                .push(AudioResampler::new(audio_utils::beats_as_samples(
                    region.samples_per_beat as f32,
                    region.start_time,
                )));
        }
        self.residual_samples = 0.0;
        Ok(())
    }

    fn render_chunk_at(
        &mut self,
        playhead: f32,
        chunk_size: f32,
        sample_rate: usize,
        samples_per_beat: f32,
    ) {
        // Mixed audio data for the chunk
        let playhead_samples = audio_utils::beats_as_samples(samples_per_beat, playhead);
        let chunk_size_samples = audio_utils::beats_as_samples(samples_per_beat, chunk_size);
        let mut mixed = AudioSource::zeros(sample_rate, self.channels, chunk_size_samples);

        for (region_index, region) in self
            .regions
            .iter_mut()
            .filter(|r| r.is_active_at(playhead, playhead + chunk_size))
            .enumerate()
        {
            // If the region does not have an audio source, skip it
            if region.audio_source().is_none() {
                continue;
            }

            let region_source = region.audio_source().as_ref().unwrap();
            let clipped_samples = region.duration() as f32 * region.samples_per_beat;

            // Actual chunk size that isn't rounded
            let actual_chunk_size = region.samples_per_beat as f32 * chunk_size;
            // Chunk size (in Region sample rate)
            let mut region_chunk_size =
                audio_utils::beats_as_samples(region.samples_per_beat as f32, chunk_size);
            // Increment the residual samples
            self.residual_samples += actual_chunk_size - region_chunk_size as f32;

            // If the residual samples number is greater than zero, add it to the chunk size
            if self.residual_samples > 0.0 {
                region_chunk_size += self.residual_samples.floor() as usize;
                self.residual_samples -= self.residual_samples.floor();
            }

            // Calculate the area to be sliced
            // ———————————————————————————————
            // Start sample index of the region (in Region samples per beat) (in global position)
            let region_start =
                audio_utils::beats_as_samples(region.samples_per_beat as f32, region.start_time);
            // Playhead position (in Region samples per beat)
            let region_playhead =
                audio_utils::beats_as_samples(region.samples_per_beat as f32, playhead);

            // Calculate the range to slice (in Region samples per beat) (in Region-based position)
            let start_sample = region_playhead.saturating_sub(region_start);
            //  |    |    | [ R>E G |I O |N ] |    |    |    |    |    |    |
            // region_start ^  ^    ^ region_playhead + region_chunk_size
            //                 region_playhead
            //
            // >: Playhead, |: Chunk separation
            let end_sample = (start_sample + region_chunk_size)
                .clamp(0, clipped_samples.round() as usize)
                .min(region_source.data[0].len());

            // Slice the region to get the chunk
            let mut chunk = AudioSource::new(region_source.sample_rate, self.channels);
            for ch in 0..self.channels {
                chunk.data[ch].extend_from_slice(&region_source.data[ch][start_sample..end_sample]);
            }

            // Resample the chunk with the resampler dedicated to the region
            let resampled_region = match self.resamplers[region_index].process(chunk, sample_rate) {
                Ok(chunk) => chunk,
                Err(err) => {
                    eprintln!("Error resampling chunk: {:?}", err);
                    continue;
                }
            };

            // Mix the resampled chunk into the mixed audio data
            mixed.mix_at(&resampled_region, 0);
        }

        // Pass the resampled chunk to the graph input node
        if let Some(input_node) = self.graph.get_input_node_mut() {
            input_node.set_input("audio", Value::from_buffer(mixed.data));
        }

        // Process the chunk through the graph
        let processed = match self.graph.process(
            sample_rate,
            self.channels,
            playhead_samples,
            playhead_samples + chunk_size_samples,
            self.id,
        ) {
            Ok(chunk) => chunk,
            Err(err) => {
                self.render_error = Some(err);
                AudioSource::zeros(
                    sample_rate,
                    self.channels,
                    playhead_samples + chunk_size_samples,
                )
            }
        };

        self.rendered_data = Some(processed);
    }

    fn rendered_data(&self) -> Result<&AudioSource, Box<dyn TrackError>> {
        match self.rendered_data {
            Some(ref data) => Ok(data),
            None => Err(match self.render_error {
                Some(ref err) => err.clone(),
                None => Box::new(UnknownTrackError { track_id: self.id }),
            }),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Clone for BufferTrack {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            volume: self.volume,
            graph: self.graph.clone(),
            channels: self.channels,
            regions: self.regions.clone(),
            rendered_data: None, // Rendered data should be regenerated
            resamplers: Vec::new(),
            residual_samples: self.residual_samples,
            render_error: self.render_error.clone(),
        }
    }
}
