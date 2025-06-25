// source.rs
// Audio source for holding audio data.
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

use crate::{AudioBuffer, Sample, Value};

use serde::{Deserialize, Serialize};
use std::f32;
use std::fs::File;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;

/// A simple class representing an source.
#[derive(Serialize, Deserialize)]
pub struct AudioSource {
    /// Sample rate of the audio buffer.
    pub sample_rate: usize,
    /// Number of channels in the audio buffer.
    pub channels: usize,
    /// Buffer data.
    pub data: AudioBuffer,
}

impl AudioSource {
    /// Create a empty audio source with the specified sample rate and number of channels.
    pub fn new(sample_rate: usize, channels: usize) -> Self {
        Self {
            sample_rate,
            channels,
            data: vec![vec![]; channels],
        }
    }

    /// Create a empty audio source with the specified sample rate and number of channels, filled with zeros.
    pub fn zeros(sample_rate: usize, channels: usize, length: usize) -> Self {
        let data = vec![vec![0.0; length]; channels];
        Self {
            sample_rate,
            channels,
            data,
        }
    }

    /// Create a new audio source instance from the audio buffer.
    pub fn from_buffer(buffer: AudioBuffer, sample_rate: usize, channels: usize) -> Self {
        Self {
            sample_rate,
            channels,
            data: buffer,
        }
    }

    /// Create a new audio source instance from the audio file in the specified path.
    /// Uses symphonia crate to decode the audio file.
    pub fn from_path(path: &str, track_number: usize) -> Result<Self, String> {
        // Open the audio file
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                return Err(format!(
                    "Failed to open the audio file. 😿 File seems to not exist. [{}]",
                    e
                ));
            }
        };

        // Instantiate the decoding options
        let format_options = FormatOptions::default();
        let metadata_options = MetadataOptions::default();
        let decoder_options = DecoderOptions::default();

        // Initialize the codec registry and probe
        let codec_registry = symphonia::default::get_codecs();
        let probe = symphonia::default::get_probe();

        // Initialize the source stream from the file
        let source_stream =
            MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

        // Initialize the probe result
        let mut probe_result = match probe.format(
            &symphonia::core::probe::Hint::new(),
            source_stream,
            &format_options,
            &metadata_options,
        ) {
            Ok(probe_result) => probe_result,
            Err(e) => {
                return Err(format!(
                    "Failed to probe the audio format. 🔈 Maybe the file is corrupted or not supported? 😿 [{}]",
                    e
                ));
            }
        };

        // Get the tracks from the probe result
        let tracks = probe_result.format.tracks();
        // And get the track at the specified index
        let track = &tracks[track_number];

        // Get the sample rate from the track's codec parameters
        let sample_rate = match track.codec_params.sample_rate {
            Some(sample_rate) => sample_rate as usize,
            None => return Err("Codec parameters invalid. 🎛️ (sample_rate missing)".into()),
        };

        let channels = match track.codec_params.channels {
            Some(channels) => channels,
            None => return Err("Codec parameters invalid. 🎛️ (channels missing)".into()),
        }
        .count();

        // Make a decoder from the codec registry and the track's codec parameters
        let mut decoder = match codec_registry.make(&track.codec_params, &decoder_options) {
            Ok(decoder) => decoder,
            Err(e) => return Err(format!("The decoder could not be initialized. 😹 [{}]", e)),
        };

        // Create a vector to store the decoded samples
        let mut output_buffer: Vec<Vec<Sample>> = vec![];

        // Decode packets until there are no more packets
        while let Ok(packet) = probe_result.format.next_packet() {
            // Decode the packet using the decoder
            match decoder.decode(&packet) {
                Ok(decoded) => merge_buffer(&mut output_buffer, decoded, channels),
                Err(e) => return Err(format!("Decode error: {}", e)),
            }
        }

        Ok(Self {
            sample_rate,
            channels,
            data: output_buffer,
        })
    }

    pub fn get_duration_from_path(path: &str, track_number: usize) -> Result<f32, &'static str> {
        // Open the audio file
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err("Failed to open the audio file. 😿 File seems to not exist."),
        };

        // Instantiate the decoding options
        let format_options = FormatOptions::default();
        let metadata_options = MetadataOptions::default();

        // Initialize the codec registry and probe
        let probe = symphonia::default::get_probe();

        // Initialize the source stream from the file
        let source_stream =
            MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

        // Initialize the probe result
        let probe_result = match probe.format(
            &symphonia::core::probe::Hint::new(),
            source_stream,
            &format_options,
            &metadata_options,
        ) {
            Ok(probe_result) => probe_result,
            Err(_) => {
                return Err(
                    "Failed to probe the audio format. 🔈 Maybe the file is corrupted or not supported? 😿",
                );
            }
        };

        // Get the tracks from the probe result
        let tracks = probe_result.format.tracks();
        // And get the track at the specified index
        let track = &tracks[track_number];

        // Get duration in seconds
        let sample_rate = track.codec_params.sample_rate.unwrap_or(44100) as f32;
        let sample_count = track.codec_params.n_frames.unwrap_or(0) as f32;
        let duration = sample_count / sample_rate;

        Ok(duration)
    }

    /// Mix the audio buffer with another buffer at a specific time.
    ///
    /// # Arguments
    /// - `other` - The other audio source to mix with.
    /// - `at` - The duration at which to mix the audio buffers.
    pub fn mix_at(&mut self, other: &AudioSource, offset: usize) {
        // Instead of cloning the entire audio source, we'll mix directly
        for (channel_index, other_channel) in other.data.iter().enumerate() {
            // If the other source has more channels than this one, add a new channel
            if channel_index >= self.channels {
                self.data.push(vec![0.0; offset + other_channel.len()]);
                self.channels += 1;
            } else if self.data[channel_index].len() < offset + other_channel.len() {
                // Extend current channel if needed
                self.data[channel_index].resize(offset + other_channel.len(), 0.0);
            }

            // Mix the samples at the offset position
            for (sample_index, &other_sample) in other_channel.iter().enumerate() {
                self.data[channel_index][offset + sample_index] += other_sample;
            }
        }
    }

    /// Normalize the audio buffer to maximize sample value.
    pub fn normalize(&mut self) {
        // Find the maximum absolute value across all channels
        let mut max_sample: f32 = 0.0;
        for channel in &self.data {
            for &sample in channel {
                max_sample = max_sample.max(sample.abs());
            }
        }

        // Normalize all samples if max is greater than 0
        if max_sample > 0.0 {
            for channel in &mut self.data {
                for sample in channel {
                    *sample /= max_sample;
                }
            }
        }
    }

    /// Returns the number of samples in the audio buffer.
    pub fn samples(&self) -> usize {
        self.data[0].len()
    }

    /// Returns the copy of the buffer.
    pub fn clone_buffer(&self) -> Vec<Vec<Sample>> {
        self.data.clone()
    }

    /// Returns the copy of the buffer, but as Value.
    pub fn clone_buffer_as_value(&self) -> Value {
        Value::from_buffer(self.clone_buffer())
    }
}

impl Clone for AudioSource {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            channels: self.channels,
            sample_rate: self.sample_rate,
        }
    }
}

/// Merge the output buffer with the decoded audio buffer ref.
/// ```
/// | ** Output Buffer ** | <-Merge-- | ** Decoded AudioBufferRef ** |
/// ```
fn merge_buffer(
    output_buffer: &mut Vec<Vec<Sample>>,
    decoded: AudioBufferRef,
    channel_count: usize,
) {
    // Initialize output_buffer with channels if it's empty
    if output_buffer.is_empty() {
        for _ in 0..channel_count {
            output_buffer.push(Vec::new());
        }
    }

    match decoded {
        AudioBufferRef::U8(buf) => {
            let frames = buf.frames();
            for frame in 0..frames {
                for channel in 0..channel_count {
                    output_buffer[channel].push(buf.chan(channel)[frame] as Sample / 128.0 - 1.0);
                }
            }
        }
        AudioBufferRef::U16(buf) => {
            let frames = buf.frames();
            for frame in 0..frames {
                for channel in 0..channel_count {
                    output_buffer[channel].push(buf.chan(channel)[frame] as Sample / 32768.0 - 1.0);
                }
            }
        }
        AudioBufferRef::S8(buf) => {
            let frames = buf.frames();
            for frame in 0..frames {
                for channel in 0..channel_count {
                    output_buffer[channel].push(buf.chan(channel)[frame] as Sample / 128.0);
                }
            }
        }
        AudioBufferRef::S16(buf) => {
            let frames = buf.frames();
            for frame in 0..frames {
                for channel in 0..channel_count {
                    output_buffer[channel].push(buf.chan(channel)[frame] as Sample / 32768.0);
                }
            }
        }
        AudioBufferRef::S32(buf) => {
            let frames = buf.frames();
            for frame in 0..frames {
                for channel in 0..channel_count {
                    output_buffer[channel].push(buf.chan(channel)[frame] as Sample / 2147483648.0);
                }
            }
        }
        AudioBufferRef::F32(buf) => {
            let frames = buf.frames();
            for frame in 0..frames {
                for channel in 0..channel_count {
                    output_buffer[channel].push(buf.chan(channel)[frame]);
                }
            }
        }
        AudioBufferRef::F64(buf) => {
            let frames = buf.frames();
            for frame in 0..frames {
                for channel in 0..channel_count {
                    output_buffer[channel].push(buf.chan(channel)[frame] as Sample);
                }
            }
        }
        _ => {}
    }
}
