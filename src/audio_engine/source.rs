use std::f32;
use std::fs::File;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;

pub type Sample = f32;

/// A simple class representing an source.
pub struct AudioSource {
    /// Sample rate of the audio buffer.
    pub sample_rate: u32,
    /// Number of channels in the audio buffer.
    pub channels: usize,
    /// Buffer data.
    pub data: Vec<Sample>,
    /// Current iteration index.
    index: usize,
}

impl AudioSource {
    /// Create a new audio source instance from the audio file in the specified path.
    /// Uses symphonia crate to decode the audio file.
    pub fn new(path: &str, track_number: usize) -> Result<Self, &'static str> {
        // Open the audio file
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err("Failed to open the audio file. 😿 File seems to not exist."),
        };

        println!("Initializing audio buffer from file: {}", path);

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
            Err(_) => return Err(
                "Failed to probe the audio format. 🔈 Maybe the file is corrupted or not supported? 😿",
            ),
        };

        // Get the tracks from the probe result
        let tracks = probe_result.format.tracks();
        // And get the track at the specified index
        let track = &tracks[track_number];

        // Get the sample rate from the track's codec parameters
        let sample_rate = match track.codec_params.sample_rate {
            Some(sample_rate) => sample_rate,
            None => return Err("Codec parameters invalid. 🎛️"),
        };

        let channels = match track.codec_params.channels {
            Some(channels) => channels,
            None => return Err("Codec parameters invalid. 🎛️"),
        }
        .count();

        // Make a decoder from the codec registry and the track's codec parameters
        let mut decoder = match codec_registry.make(&track.codec_params, &decoder_options) {
            Ok(decoder) => decoder,
            Err(_) => return Err("The decoder could not be initialized. 😹"),
        };

        // Create a vector to store the decoded samples
        let mut output_buffer: Vec<Sample> = vec![];

        // Decode packets until there are no more packets
        while let Ok(packet) = probe_result.format.next_packet() {
            // Decode the packet using the decoder
            match decoder.decode(&packet) {
                Ok(decoded) => merge_buffer(&mut output_buffer, decoded, channels),
                Err(_) => return Err("Decode error. 😿"),
            }
        }

        println!("{} decoding finished.", path);

        Ok(Self {
            sample_rate,
            channels,
            data: output_buffer,
            index: 0,
        })
    }

    /// Normalize the audio buffer to maximize sample value.
    pub fn normalize(&mut self) {
        let max_sample = self
            .data
            .iter()
            .fold(0.0, |max: f32, &sample| max.max(sample.abs()));
        if max_sample > 0.0 {
            self.data
                .iter_mut()
                .for_each(|sample| *sample /= max_sample);
        }
    }

    /// Returns the number of samples in the audio buffer.
    pub fn samples(&self) -> usize {
        self.data.len() / self.channels
    }

    pub(crate) fn clone(&self) -> Self {
        Self {
            sample_rate: self.sample_rate.clone(),
            channels: self.channels.clone(),
            data: self.data.clone(),
            index: 0,
        }
    }
}

/// Merge the output buffer with the decoded audio buffer ref.
/// ```
/// | ** Output Buffer ** | <-Merge-- | ** Decoded AudioBufferRef ** |
/// ```
fn merge_buffer(output_buffer: &mut Vec<Sample>, decoded: AudioBufferRef, channel_count: usize) {
    let mut add_sample = |channel_index: usize, index: usize, sample: Sample| {
        let position = index * channel_count + channel_index;
        if position >= output_buffer.len() {
            output_buffer.resize(position + 1, 0.0);
        }
        output_buffer[position] = sample as Sample;
    };

    match decoded {
        AudioBufferRef::U8(buf) => {
            for channel in 0..channel_count {
                // Vector of samples in the specified channel
                let channel_samples = buf.chan(channel);
                for sample_index in 0..channel_samples.len() {
                    add_sample(
                        channel,
                        sample_index,
                        channel_samples[sample_index] as Sample,
                    );
                }
            }
        }
        AudioBufferRef::U16(buf) => {
            for channel in 0..channel_count {
                // Vector of samples in the specified channel
                let channel_samples = buf.chan(channel);
                for sample_index in 0..channel_samples.len() {
                    add_sample(
                        channel,
                        sample_index,
                        channel_samples[sample_index] as Sample,
                    );
                }
            }
        }
        AudioBufferRef::S8(buf) => {
            for channel in 0..channel_count {
                // Vector of samples in the specified channel
                let channel_samples = buf.chan(channel);
                for sample_index in 0..channel_samples.len() {
                    add_sample(
                        channel,
                        sample_index,
                        channel_samples[sample_index] as Sample,
                    );
                }
            }
        }
        AudioBufferRef::S16(buf) => {
            for channel in 0..channel_count {
                // Vector of samples in the specified channel
                let channel_samples = buf.chan(channel);
                for sample_index in 0..channel_samples.len() {
                    add_sample(
                        channel,
                        sample_index,
                        channel_samples[sample_index] as Sample,
                    );
                }
            }
        }
        AudioBufferRef::S32(buf) => {
            for channel in 0..channel_count {
                // Vector of samples in the specified channel
                let channel_samples = buf.chan(channel);
                for sample_index in 0..channel_samples.len() {
                    add_sample(
                        channel,
                        sample_index,
                        channel_samples[sample_index] as Sample,
                    );
                }
            }
        }
        AudioBufferRef::F32(buf) => {
            for channel in 0..channel_count {
                // Vector of samples in the specified channel
                let channel_samples = buf.chan(channel);
                for sample_index in 0..channel_samples.len() {
                    add_sample(
                        channel,
                        sample_index,
                        channel_samples[sample_index] as Sample,
                    );
                }
            }
        }
        AudioBufferRef::F64(buf) => {
            for channel in 0..channel_count {
                // Vector of samples in the specified channel
                let channel_samples = buf.chan(channel);
                for sample_index in 0..channel_samples.len() {
                    add_sample(
                        channel,
                        sample_index,
                        channel_samples[sample_index] as Sample,
                    );
                }
            }
        }
        _ => {}
    }
}

impl rodio::Source for AudioSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.data.len())
    }

    fn channels(&self) -> u16 {
        self.channels as u16
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        Some(std::time::Duration::from_millis(
            (self.data.len() as f64 / self.sample_rate as f64 * 1000.0) as u64,
        ))
    }
}

impl Iterator for AudioSource {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        // self.data.iter().next().copied()
        let sample = self.data[self.index];
        if self.index >= self.data.len() {
            self.index = 0;
            return None;
        }
        self.index += 1;
        Some(sample)
    }
}
