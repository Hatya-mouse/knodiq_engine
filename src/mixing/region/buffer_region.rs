// buffer_region.rs
// Type of region that stores buffer data as a data.
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

use crate::audio_utils::{Beats, samples_as_beats};
use crate::{AudioSource, Region};
use std::any::Any;

pub struct BufferRegion {
    /// ID of the region.
    /// This will be set by adding the region to a track, so you don't need to set it manually.
    pub id: u32,
    /// Name of the region.
    pub name: String,
    /// Start time of the region in frames.
    pub start_time: Beats,
    /// Duration of the region in frames.
    pub duration: Beats,
    /// Samples per beat of the region.
    pub samples_per_beat: f32,
    /// Audio source of the region.
    pub source: Option<AudioSource>,
}

impl BufferRegion {
    /// Creates a new buffer region with the given audio source.
    pub fn new(name: String, source: Option<AudioSource>, tempo: Beats) -> Self {
        let samples_per_beat = match &source {
            Some(src) => src.sample_rate as f32 / (tempo as f32 / 60.0),
            None => 0.0,
        };
        let duration = match &source {
            Some(src) => samples_as_beats(samples_per_beat, src.samples()),
            None => Beats::default(),
        };
        Self {
            id: 0,
            name,
            start_time: Beats::default(),
            duration,
            samples_per_beat,
            source,
        }
    }

    /// Creates a new empty buffer region.
    pub fn empty(name: String) -> Self {
        Self {
            id: 0,
            name,
            start_time: Beats::default(),
            duration: Beats::default(),
            samples_per_beat: 0.0,
            source: None,
        }
    }

    /// Returns the audio source of the region.
    pub fn audio_source(&self) -> &Option<AudioSource> {
        &self.source
    }

    /// Sets the audio source of the region.
    pub fn set_audio_source(&mut self, source: Option<AudioSource>, tempo: Beats) {
        self.source = source;
        self.samples_per_beat = match &self.source {
            Some(src) => src.sample_rate as f32 / (tempo as f32 / 60.0),
            None => 0.0,
        };
        self.duration = match &self.source {
            Some(src) => samples_as_beats(self.samples_per_beat, src.samples()),
            None => Beats::default(),
        };
    }

    /// Scales the region so the samples per beat will be changed.
    pub fn scale(&mut self, duration: Beats) {
        self.duration = duration;
        if let Some(source) = &self.source {
            self.samples_per_beat = source.samples() as f32 / duration;
        }
    }
}

impl Region for BufferRegion {
    fn get_id(&self) -> &u32 {
        &self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

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

    fn set_samples_per_beat(&mut self, samples_per_beat: u32) {
        self.samples_per_beat = samples_per_beat as f32;
        if let Some(source) = &self.source {
            self.duration = samples_as_beats(self.samples_per_beat, source.samples());
        }
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
