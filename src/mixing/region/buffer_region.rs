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

#[derive(Clone)]
pub struct BufferRegion {
    /// Start time of the region in beats.
    pub start_time: Beats,
    /// Duration of the region in beats.
    pub duration: Beats,
    /// Sample rate of the region.
    pub sample_rate: usize,
    /// Samples per beat of the region.
    pub samples_per_beat: f32,
    /// Audio source of the region.
    pub source: Option<AudioSource>,
}

impl BufferRegion {
    /// Creates a new buffer region with the given audio source.
    pub fn new(name: String, source: AudioSource, tempo: Beats) -> Self {
        let samples_per_beat = source.sample_rate as f32 / (tempo / 60.0);
        let duration = samples_as_beats(samples_per_beat, source.samples());

        Self {
            start_time: Beats::default(),
            duration,
            sample_rate: source.sample_rate,
            samples_per_beat,
            source: Some(source),
        }
    }

    /// Creates a new empty buffer region.
    pub fn empty(name: String, sample_rate: usize) -> Self {
        Self {
            start_time: Beats::default(),
            duration: Beats::default(),
            sample_rate,
            samples_per_beat: 0.0,
            source: None,
        }
    }

    /// Returns the audio source of the region.
    pub fn audio_source(&self) -> Option<&AudioSource> {
        self.source.as_ref()
    }

    /// Sets the audio source of the region.
    pub fn set_audio_source(&mut self, source: Option<AudioSource>, tempo: Beats) {
        self.source = source;
        self.samples_per_beat = match &self.source {
            Some(src) => src.sample_rate as f32 / (tempo / 60.0),
            None => 0.0,
        };
        self.duration = match &self.source {
            Some(src) => samples_as_beats(self.samples_per_beat, src.samples()),
            None => Beats::default(),
        };
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

    fn scale(&mut self, duration: f32) {
        self.duration = duration;
        if let Some(source) = &self.source {
            self.samples_per_beat = source.samples() as f32 / duration;
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
