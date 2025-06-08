// buffer_track.rs
// A type of buffer that stores buffer as audio data.
// © 2025 Shuntaro Kasatani

use crate::{
    AudioResampler, AudioSource, Graph, Region, Track,
    audio_utils::{self, Beats},
    mixing::region::BufferRegion,
};
use std::any::Any;

pub struct BufferTrack {
    /// Unique identifier for the track.
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
}

impl BufferTrack {
    pub fn new(id: u32, name: &str, channels: usize) -> Self {
        Self {
            id,
            name: name.to_string(),
            volume: 1.0,
            graph: Graph::new(),
            channels,
            regions: Vec::new(),
            rendered_data: None,
            resamplers: Vec::new(),
            residual_samples: 0.0,
        }
    }

    pub fn add_region(&mut self, mut region: BufferRegion, at: Beats, duration: Beats) {
        region.set_start_time(at);
        region.set_duration(duration);
        self.regions.push(region);
    }
}

impl Track for BufferTrack {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn graph(&mut self) -> &mut Graph {
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
            .find(|r| *r.id() == id)
            .map(|r| r as &dyn Region)
    }

    fn get_region_mut(&mut self, id: u32) -> Option<&mut dyn Region> {
        self.regions
            .iter_mut()
            .find(|r| *r.id() == id)
            .map(|r| r as &mut dyn Region)
    }

    fn remove_region(&mut self, id: u32) {
        self.regions.retain(|r| *r.id() != id);
    }

    fn duration(&self) -> Beats {
        self.regions
            .iter()
            .map(|r| r.end_time())
            .fold(0.0, |max, end| max.max(end))
    }

    fn prepare(&mut self, _chunk_size: f32, sample_rate: usize) {
        self.graph.prepare(1024);
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
    }

    fn render_chunk_at(&mut self, playhead: f32, chunk_size: f32, sample_rate: usize) -> bool {
        // Clear the rendered data
        self.rendered_data = Some(AudioSource::new(sample_rate, self.channels));

        // Whether the rendering has finished
        let mut completed = true;

        for (region_index, region) in self
            .regions
            .iter_mut()
            .filter(|r| r.is_active_at(playhead, playhead + chunk_size))
            .enumerate()
        {
            if playhead < region.end_time() {
                completed = false;
            }

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
            let resampled = match self.resamplers[region_index].process(chunk, sample_rate) {
                Ok(chunk) => chunk,
                Err(err) => {
                    eprintln!("Error resampling chunk: {:?}", err);
                    continue;
                }
            };

            // Process the chunk through the graph
            let processed = match self.graph.process(
                resampled,
                sample_rate,
                self.channels,
                region_playhead,
                region_playhead + region_chunk_size,
            ) {
                Ok(chunk) => chunk,
                Err(err) => {
                    eprintln!("Error processing chunk: {:?}", err);
                    continue;
                }
            };

            if let Some(ref mut data) = self.rendered_data {
                // Calculate the chunk start position (in chunk-based position)
                let region_start_in_chunk = (region.start_time() - playhead).max(0.0);
                // Convert the chunk start position to the sample index
                let chunk_start =
                    audio_utils::beats_as_samples(region.samples_per_beat, region_start_in_chunk);
                // Add the processed chunk to the rendered data at the chunk start position
                data.mix_at(&processed, chunk_start);
            }
        }

        // Return whether the rendering has ended
        completed
    }

    fn rendered_data(&self) -> Result<&AudioSource, Box<dyn std::error::Error>> {
        match self.rendered_data {
            Some(ref data) => Ok(data),
            None => Err("No rendered data available".into()),
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
        }
    }
}
