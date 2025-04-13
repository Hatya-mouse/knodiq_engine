// resample.rs
// Resample the audio source to the desired sample rate.
// © 2025 Shuntaro Kasatani

use crate::audio_engine::AudioSource;
use rubato::{FftFixedIn, Resampler};

/// AudioResampler is a struct that resamples audio sources to a desired sample rate.
pub struct AudioResampler {
    // Resampler to resample the audio region
    resampler: Option<FftFixedIn<f32>>,
    // Processing chunk size.
    chunk_size: usize,
}

impl AudioResampler {
    /// Create a new AudioResampler with the given output sample rate.
    pub fn new(chunk_size: usize) -> Self {
        AudioResampler {
            resampler: None,
            chunk_size,
        }
    }

    pub fn prepare(
        &mut self,
        input_channels: usize,
        input_sample_rate: usize,
        output_sample_rate: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.resampler = match FftFixedIn::<f32>::new(
            input_sample_rate,
            output_sample_rate,
            self.chunk_size,
            self.chunk_size,
            input_channels,
        ) {
            Ok(resampler) => Some(resampler),
            Err(err) => return Err(Box::new(err)),
        };
        Ok(())
    }

    pub fn process(
        &mut self,
        input: AudioSource,
        output_sample_rate: usize,
    ) -> Result<AudioSource, Box<dyn std::error::Error>> {
        // Get the data from the audio source
        let source_channels = input.channels;
        let original_length = input.samples();
        let input_sample_rate = input.sample_rate;

        // If the source sample rate is the same as the output sample rate, return the source as is
        if input_sample_rate == output_sample_rate {
            return Ok(input.clone());
        }

        // Create a resampler from the data
        if self.resampler.is_none() {
            self.prepare(source_channels, input_sample_rate, output_sample_rate)?;
        }
        let mut resampler = match self.resampler {
            Some(ref mut resampler) => resampler,
            None => return Err("Resampler not initialized".into()),
        };

        // Create a temporary buffer to hold the resampled data
        let mut temp_buffer: Vec<Vec<f32>> = vec![Vec::new(); source_channels];

        // Current processing frame index
        let mut frame_index = 0;

        // Resample each chunks
        loop {
            // Calculate how many frames resampler needs
            let needed_frames = <FftFixedIn<f32> as Resampler<f32>>::input_frames_next(&resampler);

            // If the remaining frames are less than needed, break the loop
            if original_length - frame_index < needed_frames {
                break;
            }

            // Get the next chunk of data from the iterator
            let (input_buffer, next_index) =
                read_frames(input.clone_buffer(), frame_index, self.chunk_size);
            frame_index = next_index;

            // Resample the data
            let output_buffer = match <FftFixedIn<f32> as Resampler<f32>>::process(
                &mut resampler,
                &input_buffer,
                None,
            ) {
                Ok(buffer) => buffer,
                Err(err) => return Err(Box::new(err)),
            };

            // Append the data to the temporary buffer
            for (i, channel) in output_buffer.iter().enumerate() {
                temp_buffer[i].extend(channel);
            }
        }

        // Check if any samples are left to resample
        if frame_index < original_length {
            // Then reasample the remaining samples
            let (input_buffer, _) = read_frames(input.clone_buffer(), frame_index, self.chunk_size);
            let output_buffer = match <FftFixedIn<f32> as Resampler<f32>>::process_partial(
                &mut resampler,
                Some(&input_buffer),
                None,
            ) {
                Ok(buffer) => buffer,
                Err(err) => return Err(Box::new(err)),
            };

            // Append the data to the temporary buffer
            for (i, channel) in output_buffer.iter().enumerate() {
                temp_buffer[i].extend(channel);
            }
        }

        // Return the resampled data
        Ok(AudioSource {
            data: temp_buffer,
            channels: source_channels,
            sample_rate: output_sample_rate,
        })
    }
}

fn read_frames(
    from: Vec<Vec<f32>>,
    frame_index: usize,
    chunk_size: usize,
) -> (Vec<Vec<f32>>, usize) {
    // Number of channels in the input data
    let channels = from.len();
    // Calculate the end index for the next chunk
    let end_index = frame_index + chunk_size;

    // Output buffer
    let mut output_buffer: Vec<Vec<f32>> = vec![];

    // Append vector which represents channel and contains chunk_size_per_channel elements
    for _ in 0..channels {
        output_buffer.push(vec![]);
    }

    // Add samples to the output
    for channel in 0..channels {
        for sample_index in frame_index..end_index {
            if sample_index < from[channel].len() {
                output_buffer[channel].push(from[channel][sample_index]);
            }
        }
    }

    (output_buffer, end_index)
}
