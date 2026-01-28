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
    /// Error that occurred during rendering.
    render_error: Option<Box<dyn TrackError>>,
    /// Original mixed data before processing.
    original_mixed_data: Option<AudioSource>,
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
            render_error: None,
            original_mixed_data: None,
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

        self.resamplers.clear();
        for region in &self.regions {
            self.resamplers
                .push(AudioResampler::new(audio_utils::beats_as_samples(
                    region.samples_per_beat as f32,
                    chunk_size,
                )));
        }

        Ok(())
    }

    fn render_chunk_at(
        &mut self,
        playhead: f32,
        chunk_size: f32,
        sample_rate: usize,
        samples_per_beat: f32,
    ) {
        self.render_error = None;

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

            // Calculate the relative start and end beats of the region
            let region_rel_start = playhead - region.start_time;
            let region_rel_start_clipped = region_rel_start.max(0.0);
            let region_rel_end = playhead - region.start_time + chunk_size;

            // Calculate the start and end samples for the region
            let start_sample =
                audio_utils::beats_as_samples(region.samples_per_beat, region_rel_start_clipped);
            let end_sample = audio_utils::beats_as_samples(region.samples_per_beat, region_rel_end)
                .min(region_source.samples());

            // Calculate the gap between the playhead and the region start
            let playhead_offset = (-region_rel_start).max(0.0);
            let playhead_offset_samples =
                audio_utils::beats_as_samples(region.samples_per_beat, playhead_offset);

            // Slice the region to get the chunk
            // To fill the gap, we create a chunk of zeros
            // and then fill the rest with the region data
            let mut chunk = AudioSource::zeros(
                region_source.sample_rate,
                self.channels,
                playhead_offset_samples,
            );
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

        match self.original_mixed_data.as_mut() {
            Some(original) => original.mix_at(&mixed, playhead_samples),
            None => self.original_mixed_data = Some(mixed),
        }

        let original_mixed_data = self.original_mixed_data.as_ref().expect("IMPOSSIBLE");

        // Pass the resampled chunk to the graph input node
        if let Some(input_node) = self.graph.get_input_node_mut() {
            input_node.set_input(
                "audio",
                Value::from_buffer(original_mixed_data.data.clone()),
            );
        }

        // Process the chunk through the graph
        let mut processed = match self.graph.process(
            sample_rate,
            samples_per_beat,
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

        if processed.samples() > chunk_size_samples {
            processed.slice(0, chunk_size_samples);
        } else if processed.samples() < chunk_size_samples {
            // If the mixed data is shorter than the chunk size, pad it with zeros
            processed.pad(chunk_size_samples - processed.samples());
        }

        self.rendered_data = Some(processed);
    }

    fn rendered_data(&self) -> Result<&AudioSource, Box<dyn TrackError>> {
        match self.render_error {
            Some(ref error) => Err(error.clone()),
            None => Ok(self.rendered_data.as_ref().ok_or_else(|| {
                Box::new(UnknownTrackError { track_id: self.id }) as Box<dyn TrackError>
            })?),
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
            render_error: self.render_error.clone(),
            original_mixed_data: self.original_mixed_data.clone(),
        }
    }
}
