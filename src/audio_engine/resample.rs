// resample.rs
// Resample the audio source to the desired sample rate.
// © 2025 Shuntaro Kasatani

use crate::audio_engine::source::AudioSource;
use rubato::{FftFixedIn, Resampler};

/// AudioResampler is a struct that resamples audio sources to a desired sample rate.
pub struct AudioResampler {}

impl AudioResampler {
    /// Create a new AudioResampler instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Resample the audio source to the desired sample rate, and return the resampled audio source.
    pub fn resample(
        &self,
        source: &AudioSource,
        output_sample_rate: usize,
    ) -> Result<AudioSource, Box<dyn std::error::Error>> {
        // Chunk size of the resampler
        let chunk_size = 1024;

        // Get the data from the audio source
        let input_sample_rate = source.sample_rate as usize;
        let source_channels = source.channels;
        let original_length = source.samples();

        // Create a resampler from the data
        let mut resampler = match FftFixedIn::<f32>::new(
            input_sample_rate,
            output_sample_rate,
            chunk_size,
            chunk_size,
            source_channels,
        ) {
            Ok(resampler) => resampler,
            Err(err) => return Err(Box::new(err)),
        };

        // Create a temporary buffer to hold the resampled data
        let mut temp_buffer: Vec<Vec<f32>> = vec![Vec::new(); source_channels];

        // Current processing frame index
        let mut frame_index = 0;

        // Resample each chunks
        loop {
            // Calculate how many frames of delay the resampler gives
            let _delay = <FftFixedIn<f32> as Resampler<f32>>::output_delay(&resampler);
            // Calculate the new length of the clip
            let _new_length = original_length * output_sample_rate / input_sample_rate;
            // Calculate how many frames resampler needs
            let needed_frames = <FftFixedIn<f32> as Resampler<f32>>::input_frames_next(&resampler);

            // If the remaining frames are less than needed, break the loop
            if original_length - frame_index < needed_frames {
                break;
            }

            // Get the next chunk of data from the iterator
            let (input_buffer, next_index) =
                read_frames(source.data.clone(), frame_index, chunk_size);
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

        // Resample the left samples
        let (input_buffer, _) = read_frames(source.data.clone(), frame_index, chunk_size);
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

        // Create a new AudioSource with the resampled data
        Ok(AudioSource {
            data: temp_buffer,
            sample_rate: output_sample_rate,
            channels: source_channels,
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
