// mixer.rs
// Mixer mixes multiple audio tracks into AudioSource.
// © 2025 Shuntaro Kasatani

use crate::audio_engine::{AudioSource, Track};
use crate::utils::ansi;

pub struct Mixer {
    /// Tracks to be mixed.
    tracks: Vec<Box<dyn Track>>,

    /// Number of channels in the output audio source.
    channels: usize,

    /// Sample rate of the output audio source.
    sample_rate: usize,

    /// Currently rendering position.
    playhead_position: usize,
}

impl Mixer {
    /// Creates a new mixer instance.
    pub fn new(sample_rate: usize, channels: usize) -> Self {
        Mixer {
            tracks: Vec::new(),
            channels,
            sample_rate,
            playhead_position: 0,
        }
    }

    /// Adds a new track to the mixer.
    pub fn add_track(&mut self, track: Box<dyn Track>) {
        self.tracks.push(track);
    }

    /// Prepares the mixer for rendering.
    pub fn prepare(&mut self) {
        for track in &mut self.tracks {
            track.prepare(self.sample_rate);
        }
    }

    /// Mixes all the tracks into a single audio source.
    ///
    /// # Arguments
    /// - `callback` - Called when the chunk has rendered. Rendered sample is passed. Sample will be passed in this way:
    /// `Sample 0` from `Channel 0`, `Sample 0` from `Channel 1`, `Sample 1` from `Channel 0`, `Sample 1` from `Channel 1`...
    pub fn mix(&mut self, mut callback: Box<dyn FnMut(f32)>) -> AudioSource {
        self.playhead_position = 0;

        // Create a new AudioSource instance to return
        let mut output = AudioSource::new(self.sample_rate, self.channels);

        // Define the chunk size
        let chunk_size: usize = 1024;

        loop {
            // Process the chunk and get whether the rendering has completed
            if self.process_chunk(&mut output, chunk_size) {
                break;
            }

            // Call the callback function for only the newly rendered chunk
            let start_sample = self.playhead_position;
            let end_sample = (start_sample + chunk_size).min(output.samples());

            for sample in start_sample..end_sample {
                for channel in 0..self.channels {
                    callback(output.data[channel][sample]);
                }
            }

            // Increment the playhead duration
            self.playhead_position += chunk_size;
        }

        println!(
            "{}{}Rendering finished.{}",
            ansi::BOLD,
            ansi::BRIGHT_MAGENTA,
            ansi::RESET
        );

        // Return the mixed output.
        output
    }

    /// Processes the chunk of each track.
    ///
    /// # Arguments
    /// - `output` - The output audio source to save the rendered data. Must be an mutable reference.
    /// - `chunk_size` - Processing chunk size.
    /// - `callback` - Called when the chunk has rendered.
    ///
    /// # Returns
    /// - `true` - The rendering has completed.
    /// - `false` - The rendering hasn't completed yet.
    pub fn process_chunk(&mut self, output: &mut AudioSource, chunk_size: usize) -> bool {
        // Whether the processing has finished
        let mut completed = true;

        // Loop through tracks
        for track in &mut self.tracks {
            // Render the track and get the rendered audio source from the track
            if !track.render_chunk_at(self.playhead_position, chunk_size, self.sample_rate) {
                completed = false;
            };
            let rendered_track = match track.rendered_data() {
                Ok(data) => data,
                Err(err) => {
                    eprintln!(
                        "{}{}Error rendering track{}: {}",
                        ansi::BOLD,
                        ansi::RED,
                        ansi::RESET,
                        err,
                    );
                    continue;
                }
            };

            // Mix the rendered track into the output audio source
            output.mix_at(rendered_track, self.playhead_position);
        }

        // Return whether the rendering has completed
        completed
    }
}
