// buffer_region.rs
// Type of region that stores buffer data as a data.
// © 2025 Shuntaro Kasatani

use crate::audio_utils::{Beats, samples_as_beats};
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
    pub source: Option<AudioSource>,
}

impl BufferRegion {
    /// Creates a new buffer region with the given audio source.
    pub fn new(id: u32, name: String, source: Option<AudioSource>, samples_per_beat: f32) -> Self {
        let duration = match &source {
            Some(src) => samples_as_beats(samples_per_beat, src.samples()),
            None => Beats::default(),
        };
        Self {
            id,
            name,
            start_time: Beats::default(),
            duration,
            samples_per_beat,
            source,
        }
    }

    /// Creates a new empty buffer region.
    /// This is useful for representing a region that is not yet filled with audio data.
    pub fn empty(id: u32, name: String, samples_per_beat: f32, expected_duration: Beats) -> Self {
        Self {
            id,
            name,
            start_time: Beats::default(),
            duration: expected_duration,
            samples_per_beat,
            source: None,
        }
    }

    /// Returns the audio source of the region.
    pub fn audio_source(&self) -> &Option<AudioSource> {
        &self.source
    }

    /// Sets the audio source of the region.
    pub fn set_audio_source(&mut self, source: Option<AudioSource>) {
        self.source = source;
        self.duration = match &self.source {
            Some(src) => samples_as_beats(self.samples_per_beat, src.samples()),
            None => Beats::default(),
        };
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
        self.duration
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

    fn region_type(&self) -> String {
        "BufferRegion".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
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
