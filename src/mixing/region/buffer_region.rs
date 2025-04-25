// buffer_region.rs
// Type of region that stores buffer data as a data.
// © 2025 Shuntaro Kasatani

use crate::{AudioSource, Duration, Region};

pub struct BufferRegion {
    /// Start time of the region in frames.
    pub start_time: f32,
    /// Number of samples in every single beat in the region.
    pub samples_per_beat: f32,
    /// Audio source of the region.
    pub source: AudioSource,
}

impl BufferRegion {
    /// Creates a new buffer region with the given audio source.
    pub fn new(source: AudioSource) -> Self {
        Self {
            start_time: 0.0,
            samples_per_beat: 0.0,
            source,
        }
    }

    /// Returns the audio source of the region.
    pub fn audio_source(&self) -> &AudioSource {
        &self.source
    }

    /// Sets the audio source of the region.
    pub fn set_audio_source(&mut self, source: AudioSource) {
        self.source = source;
    }
}

impl Region for BufferRegion {
    fn start_time(&self) -> f32 {
        self.start_time
    }

    fn set_start_time(&mut self, start_time: f32) {
        self.start_time = start_time;
    }

    fn end_time(&self) -> f32 {
        self.start_time + self.duration()
    }

    fn duration(&self) -> f32 {
        // Convert the number of samples to beats using the samples per beat.
        self.source.samples() as f32 / self.samples_per_beat as f32
    }
}
