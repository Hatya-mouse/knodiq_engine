// resample.rs
// Resample the audio source to the desired sample rate.
// © 2025 Shuntaro Kasatani

use crate::audio_engine::{node::traits::node::Node, source::AudioSource};
use rubato::{FftFixedIn, Resampler};

/// AudioResampler is a struct that resamples audio sources to a desired sample rate.
pub struct AudioResampler {
    output_sample_rate: usize,
}

impl AudioResampler {
    pub fn new(output_sample_rate: usize) -> Self {
        AudioResampler { output_sample_rate }
    }
}

impl Node for AudioResampler {
    fn process(&mut self, input: AudioSource) -> AudioSource {
        // Chunk size of the resampler
        let chunk_size = 1024;

        // Get the data from the audio source
        let source_channels = input.channels;
        let original_length = input.samples();
        let input_sample_rate = input.sample_rate;

        // If the source sample rate is the same as the output sample rate, return the source as is
        if input_sample_rate == self.output_sample_rate {
            return input;
        }

        println!(
            "Resampling the source. Input: {}, Output: {}",
            input_sample_rate, self.output_sample_rate
        );

        // Create a resampler from the data
        let mut resampler = match FftFixedIn::<f32>::new(
            input_sample_rate,
            self.output_sample_rate,
            chunk_size,
            chunk_size,
            source_channels,
        ) {
            Ok(resampler) => resampler,
            Err(err) => {
                println!("Error creating a new resampler: {}", err);
                return AudioSource::new(self.output_sample_rate, source_channels);
            }
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
            let _new_length = original_length * self.output_sample_rate / input_sample_rate;
            // Calculate how many frames resampler needs
            let needed_frames = <FftFixedIn<f32> as Resampler<f32>>::input_frames_next(&resampler);

            // If the remaining frames are less than needed, break the loop
            if original_length - frame_index < needed_frames {
                break;
            }

            // Get the next chunk of data from the iterator
            let (input_buffer, next_index) =
                read_frames(input.clone_buffer(), frame_index, chunk_size);
            frame_index = next_index;

            // Resample the data
            let output_buffer = match <FftFixedIn<f32> as Resampler<f32>>::process(
                &mut resampler,
                &input_buffer,
                None,
            ) {
                Ok(buffer) => buffer,
                Err(err) => {
                    println!("Resampling error: {}", err);
                    return AudioSource::new(self.output_sample_rate, source_channels);
                }
            };

            // Append the data to the temporary buffer
            for (i, channel) in output_buffer.iter().enumerate() {
                temp_buffer[i].extend(channel);
            }
        }

        // Resample the left samples
        let (input_buffer, _) = read_frames(input.clone_buffer(), frame_index, chunk_size);
        let output_buffer = match <FftFixedIn<f32> as Resampler<f32>>::process_partial(
            &mut resampler,
            Some(&input_buffer),
            None,
        ) {
            Ok(buffer) => buffer,
            Err(err) => {
                println!("Resampling error: {}", err);
                return AudioSource::new(self.output_sample_rate, source_channels);
            }
        };

        // Append the data to the temporary buffer
        for (i, channel) in output_buffer.iter().enumerate() {
            temp_buffer[i].extend(channel);
        }

        // Return the resampled data
        AudioSource {
            data: temp_buffer,
            channels: source_channels,
            sample_rate: self.output_sample_rate,
        }
    }

    fn get_property_list(&self) -> Vec<String> {
        vec!["output_sample_rate".to_string()]
    }

    fn get_property(&self, property: String) -> f64 {
        match property.as_str() {
            "output_sample_rate" => self.output_sample_rate as f64,
            _ => panic!("Unknown property"),
        }
    }

    fn set_property(&mut self, property: String, value: f64) {
        match property.as_str() {
            "output_sample_rate" => self.output_sample_rate = value as usize,
            _ => panic!("Unknown property"),
        }
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
