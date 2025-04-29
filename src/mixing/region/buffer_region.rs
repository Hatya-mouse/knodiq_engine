// buffer_region.rs
// Type of region that stores buffer data as a data.
// © 2025 Shuntaro Kasatani

use crate::audio_utils::Beats;
use crate::{AudioSource, Region};
use std::any::Any;

pub struct BufferRegion {
    /// ID of the region.
    pub id: u32,
    /// Name of the region.
    pub name: String,
    /// Start time of the region in frames.
    pub start_time: Beats,
    /// Duration of the region in frames.
    pub duration: Beats,
    /// Number of samples per beat.
    pub samples_per_beat: f32,
    /// Audio source of the region.
    pub source: AudioSource,
}

impl BufferRegion {
    /// Creates a new buffer region with the given audio source.
    pub fn new(id: u32, name: String, source: AudioSource, samples_per_beat: f32) -> Self {
        Self {
            id,
            name,
            start_time: Beats::default(),
            duration: (source.samples() as f32 / samples_per_beat as f32) as Beats,
            samples_per_beat,
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
    fn set_duration(&mut self, duration: Beats) {
        self.duration = duration;
    }

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

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn id(&self) -> &u32 {
        &self.id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Clone for BufferRegion {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            start_time: self.start_time,
            duration: self.duration,
            samples_per_beat: self.samples_per_beat,
            source: self.source.clone(),
        }
    }
}
