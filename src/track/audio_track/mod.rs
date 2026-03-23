mod audio_region;

pub use audio_region::AudioRegion;

use crate::{
    data_types::{AudioContext, Beats},
    resampler::resample_channels,
    track::{RegionID, Track},
};
use std::collections::HashMap;

pub struct AudioTrack {
    regions: HashMap<RegionID, AudioRegion>,
    processed: Vec<f32>,

    audio_ctx: AudioContext,
}

impl Track for AudioTrack {
    fn set_audio_ctx(&mut self, audio_ctx: &AudioContext) {}

    fn prepare(&mut self) {
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

            // Calculate the start sample of the buffer
            let start_sample =
                (region.start.0 * self.audio_ctx.tempo as f64 * self.audio_ctx.sample_rate as f64
                    / 60.0) as usize
                    * self.audio_ctx.channels as usize;

            // Extend the mixed buffer
            if self.processed.len() < start_sample + resampled.len() {
                self.processed.resize(start_sample + resampled.len(), 0.0);
            }

            // Add the resampled samples
            for (i, sample) in resampled.iter().enumerate() {
                self.processed[start_sample + i] += sample;
            }
        }
    }

    fn process(&mut self, playhead: Beats, output: *mut u8, ctx: &AudioContext) {}
}
