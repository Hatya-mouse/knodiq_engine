mod audio_region;

pub use audio_region::AudioRegion;

use crate::{
    data_types::{AudioContext, Beats},
    graph::{Graph, error::GraphError},
    resampler::resample_channels,
    track::{RegionID, Track},
};
use std::collections::HashMap;

pub struct AudioTrack {
    // --- GRAPH ---
    graph: Graph,

    // --- RAW AUDIO DATA ---
    regions: HashMap<RegionID, AudioRegion>,
    processed: Vec<f32>,

    // --- AUDIO CONTEXT ---
    audio_ctx: AudioContext,

    // --- MISC ---
    next_region_id: usize,
}

impl AudioTrack {
    // --- NUMBER CONVERSION ---

    fn beats_to_index(&self, beats: Beats) -> usize {
        (beats.0 / self.audio_ctx.tempo as f64 * 60.0 * self.audio_ctx.sample_rate as f64) as usize
            * self.audio_ctx.channels as usize
    }

    // --- REGION ADDITION ---

    fn generate_region_id(&mut self) -> RegionID {
        let id = RegionID(self.next_region_id);
        self.next_region_id += 1;
        id
    }

    fn add_region(&mut self, region: AudioRegion) {
        let id = self.generate_region_id();
        self.regions.insert(id, region);
    }
}

impl Track for AudioTrack {
    // --- AUDIO CONTEXT UPDARING ---

    fn set_audio_ctx(&mut self, audio_ctx: &AudioContext) {
        self.audio_ctx = audio_ctx.clone();
        self.graph.set_audio_ctx(audio_ctx);
    }

    // --- TRACK PROCESSING ---

    fn prepare(&mut self, total_duration: Beats) -> Result<(), GraphError> {
        // Calculate the total sample number
        // Ceil to a multiple of the buffer size
        let total_frames = self
            .beats_to_index(total_duration)
            .div_ceil(self.audio_ctx.buffer_size as usize)
            * self.audio_ctx.buffer_size as usize;
        // Initialize the processed vector with zeros
        self.processed = vec![0.0; total_frames * self.audio_ctx.channels as usize];

        // Resample the each regions
        for region in self.regions.values() {
            let resampled = resample_channels(
                &region.data,
                region.frames,
                region.sample_rate as usize,
                region.channels as usize,
                self.audio_ctx.sample_rate as usize,
                self.audio_ctx.channels as usize,
            );

            // Calculate the start sample index of the buffer
            let region_start_index = self.beats_to_index(region.start);

            // Add the resampled samples
            let available = self.processed.len().saturating_sub(region_start_index);
            let copy_len = resampled.len().min(available);
            for (i, sample) in resampled[..copy_len].iter().enumerate() {
                self.processed[region_start_index + i] += sample;
            }
        }

        // Then prepare the graph
        self.graph.prepare()
    }

    fn process(&mut self, playhead: Beats, output: *mut u8, audio_ctx: &AudioContext) {
        let buffer_start = self.beats_to_index(playhead);
        let buffer_end =
            buffer_start + audio_ctx.buffer_size as usize * audio_ctx.channels as usize;
        // Get the slice pointer from the processed audio data
        let input_ptr = if buffer_end <= self.processed.len() {
            self.processed[buffer_start..buffer_end].as_ptr() as *const u8
        } else {
            std::ptr::null()
        };
        // Process the graph
        self.graph.process(&[input_ptr], &[output], audio_ctx);
    }
}
