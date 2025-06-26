// mixer.rs
// Mixer mixes multiple audio tracks into AudioSource.
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

use std::error::Error;

use crate::{AudioSource, Sample, Track, audio_utils};
use audio_utils::Beats;

const CHUNK_BEATS: Beats = 2.0;

pub struct Mixer {
    /// Tracks to be mixed.
    pub tracks: Vec<Box<dyn Track>>,

    /// The tempo of the mixer.
    pub tempo: Beats,

    /// Number of channels in the output audio source.
    pub channels: usize,

    /// Sample rate of the output audio source.
    pub sample_rate: usize,

    /// Currently rendering position in beats.
    playhead_beats: f32,
}

impl Mixer {
    /// Creates a new mixer instance.
    pub fn new(tempo: Beats, sample_rate: usize, channels: usize) -> Self {
        Mixer {
            tracks: Vec::new(),
            tempo,
            channels,
            sample_rate,
            playhead_beats: 0.0,
        }
    }

    /// Adds a new track to the mixer.
    pub fn add_track(&mut self, track: Box<dyn Track>) {
        let mut new_track = track;
        new_track.set_id(self.generate_track_id());
        self.tracks.push(new_track);
    }

    /// Prepares the mixer for rendering.
    pub fn prepare(&mut self) -> Result<(), Box<dyn Error>> {
        for track in &mut self.tracks {
            track.prepare(CHUNK_BEATS, self.sample_rate)?;
        }
        Ok(())
    }

    /// Returns the current playhead position in beats.
    pub fn samples_per_beat(&self) -> f32 {
        (self.sample_rate as f32) / (self.tempo as f32 / 60.0)
    }

    /// Returns the current playhead position in beats.
    pub fn set_tempo(&mut self, tempo: Beats) {
        self.tempo = tempo;
    }

    /// Sets the sample rate of the mixer.
    pub fn set_sample_rate(&mut self, sample_rate: usize) {
        self.sample_rate = sample_rate;
    }

    /// Sets the number of channels in the mixer.
    pub fn set_channels(&mut self, channels: usize) {
        self.channels = channels;
    }

    /// Returns a reference to a track by its ID.
    pub fn get_track_by_id(&self, id: u32) -> Option<&Box<dyn Track>> {
        self.tracks.iter().filter(|t| t.get_id() == id).next()
    }

    /// Returns a mutable reference to a track by its ID.
    pub fn get_track_by_id_mut(&mut self, id: u32) -> Option<&mut Box<dyn Track>> {
        self.tracks.iter_mut().filter(|t| t.get_id() == id).next()
    }

    /// Removes a track from the mixer by its ID.
    pub fn remove_track(&mut self, id: u32) {
        self.tracks.retain(|t| t.get_id() != id);
    }

    /// Generates a unique ID for a new track.
    pub fn generate_track_id(&self) -> u32 {
        let mut id = 0;
        while self.tracks.iter().any(|t| t.get_id() == id) {
            id += 1;
        }
        id
    }

    /// Returns the duration of the mixer in beats.
    pub fn duration(&self) -> Beats {
        self.tracks
            .iter()
            .map(|track| track.duration())
            .fold(0.0, |acc, duration| acc.max(duration))
    }

    /// Mixes all the tracks into a single audio source.
    ///
    /// # Arguments
    /// - `at` - The position in beats to start mixing from.
    /// - `callback` - Called when the chunk has rendered. Rendered sample and the current playhead time (is Beats) is passed. Sample will be passed in this way:
    /// `Sample 0` from `Channel 0`, `Sample 0` from `Channel 1`, `Sample 1` from `Channel 0`, `Sample 1` from `Channel 1`...
    /// The callback should return `true` to continue rendering, or `false` to stop rendering.
    pub fn mix(
        &mut self,
        at: Beats,
        callback: Box<dyn Fn(Sample, Beats) -> bool + Send>,
    ) -> AudioSource {
        self.playhead_beats = at;

        // Create a new AudioSource instance to return
        let mut output = AudioSource::new(self.sample_rate, self.channels);

        // Chunk size in output sample rate
        let chunk_size = audio_utils::beats_as_samples(self.samples_per_beat(), CHUNK_BEATS);
        let duration = self.duration();

        while self.playhead_beats < duration {
            // Process the chunk and get whether the rendering has completed
            self.process_chunk(&mut output, CHUNK_BEATS);

            // Call the callback function for only the newly rendered chunk
            let start_sample =
                audio_utils::beats_as_samples(self.samples_per_beat(), self.playhead_beats);
            let end_sample = (start_sample + chunk_size).min(output.data[0].len());

            for sample in start_sample..end_sample {
                for channel in 0..self.channels {
                    if !callback(output.data[channel][sample], self.playhead_beats) {
                        return output;
                    }
                }
            }

            // Increment the playhead duration
            self.playhead_beats += CHUNK_BEATS;
        }

        // Return the mixed output.
        output
    }

    /// Processes the chunk of each track.
    ///
    /// # Arguments
    /// - `output` - The output audio source to save the rendered data. Must be an mutable reference.
    /// - `chunk_duration` - Processing chunk duration in beats.
    /// - `callback` - Called when the chunk has rendered.
    pub fn process_chunk(&mut self, output: &mut AudioSource, chunk_duration: Beats) {
        let samples_per_beat = self.samples_per_beat();

        // Loop through tracks
        for track in &mut self.tracks {
            // Render the track and get the rendered audio source from the track
            track.render_chunk_at(
                self.playhead_beats,
                chunk_duration,
                self.sample_rate,
                samples_per_beat,
            );
            let rendered_track = match track.rendered_data() {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("Error rendering track: {}", err);
                    continue;
                }
            };

            // Mix the rendered track into the output audio source
            let playhead_samples =
                audio_utils::beats_as_samples(samples_per_beat, self.playhead_beats);
            output.mix_at(rendered_track, playhead_samples);
        }
    }
}

// To enable cloning of Box<dyn Track>, the Track trait must support clone_box.
impl Clone for Mixer {
    fn clone(&self) -> Self {
        Mixer {
            tracks: self.tracks.iter().map(|t| t.clone_box()).collect(),
            tempo: self.tempo,
            channels: self.channels,
            sample_rate: self.sample_rate,
            playhead_beats: self.playhead_beats,
        }
    }
}

unsafe impl Sync for Mixer {}

unsafe impl Send for Mixer {}
