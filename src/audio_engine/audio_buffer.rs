use std::f32;
use std::fs::File;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;

pub type Sample = f32;

/// A simple class representing an audio buffer.
pub struct SAudioBuffer {
    /// Sample rate of the audio buffer.
    pub sample_rate: u32,
    /// Buffer data.
    /// Outer vector represents channels, inner vector represents samples.
    /// ```
    /// Vec<Vec<                                 Sample>>
    ///     ^^^^ This represents each channel... ^^^^^^ ...and this represents each sample.
    /// ```
    pub data: Vec<Vec<Sample>>,
}

impl SAudioBuffer {
    /// Get the audio buffer from the audio file, in specified path.
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

        // Get the sample rate from the track's codec parameters
        let sample_rate = match tracks[track_number].codec_params.sample_rate {
            Some(sample_rate) => sample_rate,
            None => return Err("Codec parameters invalid. 🎛️"),
        };

        // Make a decoder from the codec registry and the track's codec parameters
        let mut decoder =
            match codec_registry.make(&tracks[track_number].codec_params, &decoder_options) {
                Ok(decoder) => decoder,
                Err(_) => return Err("The decoder could not be initialized. 😹"),
            };

        // Capacity of the AudioBuffer
        let mut output_buffer: Vec<Vec<Sample>> = vec![];

        // Create a packet from the probe result
        while let Ok(packet) = probe_result.format.next_packet() {
            // Decode the packet using the decoder
            match decoder.decode(&packet) {
                Ok(decoded) => merge_buffer(&mut output_buffer, decoded),
                Err(_) => return Err("Decode error. 😿"),
            }
        }

        println!("{} decoding finished.", path);

        Ok(Self {
            sample_rate,
            data: output_buffer,
        })
    }

    /// Normalize the audio buffer to maximize sample value.
    pub fn normalize(&mut self) {
        let max_sample = self
            .data
            .iter()
            .flatten()
            .fold(0.0, |max: Sample, &sample: &Sample| max.max(sample.abs()));
        if max_sample > 0.0 {
            self.data.iter_mut().for_each(|channel| {
                channel.iter_mut().for_each(|sample| *sample /= max_sample);
            });
        }
    }

    /// Get the number of channels in the audio buffer.
    pub fn channels(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of samples in the audio buffer.
    pub fn samples(&self) -> usize {
        self.data[0].len()
    }
}

/// Merge the output buffer with the decoded audio buffer ref.
/// ```
/// | ** Output Buffer ** | <-Merge-- | ** Decoded AudioBufferRef ** |
/// ```
fn merge_buffer(output_buffer: &mut Vec<Vec<Sample>>, decoded: AudioBufferRef) {
    let channel_count = decoded.spec().channels.count();
    while output_buffer.len() < channel_count {
        output_buffer.push(vec![]);
    }

    let mut add_sample = |channel_index: usize, sample: Sample| {
        output_buffer[channel_index].push(sample as Sample);
    };

    match decoded {
        AudioBufferRef::U8(buf) => {
            for channel in 0..channel_count {
                // Vector of samples in the specified channel
                let channel_samples = buf.chan(channel);
                for sample in channel_samples {
                    add_sample(channel, *sample as Sample);
                }
            }
        }
        AudioBufferRef::U16(buf) => {
            for channel in 0..channel_count {
                // Vector of samples in the specified channel
                let channel_samples = buf.chan(channel);
                for sample in channel_samples {
                    add_sample(channel, *sample as Sample);
                }
            }
        }
        AudioBufferRef::S8(buf) => {
            for channel in 0..channel_count {
                // Vector of samples in the specified channel
                let channel_samples = buf.chan(channel);
                for sample in channel_samples {
                    add_sample(channel, *sample as Sample);
                }
            }
        }
        AudioBufferRef::S16(buf) => {
            for channel in 0..channel_count {
                // Vector of samples in the specified channel
                let channel_samples = buf.chan(channel);
                for sample in channel_samples {
                    add_sample(channel, *sample as Sample);
                }
            }
        }
        AudioBufferRef::S32(buf) => {
            for channel in 0..channel_count {
                // Vector of samples in the specified channel
                let channel_samples = buf.chan(channel);
                for sample in channel_samples {
                    add_sample(channel, *sample as Sample);
                }
            }
        }
        AudioBufferRef::F32(buf) => {
            for channel in 0..channel_count {
                // Vector of samples in the specified channel
                let channel_samples = buf.chan(channel);
                for sample in channel_samples {
                    add_sample(channel, *sample as Sample);
                }
            }
        }
        AudioBufferRef::F64(buf) => {
            for channel in 0..channel_count {
                // Vector of samples in the specified channel
                let channel_samples = buf.chan(channel);
                for sample in channel_samples {
                    add_sample(channel, *sample as Sample);
                }
            }
        }
        _ => {}
    }
}
