// mixer.rs
// Mixer mixes multiple audio tracks into AudioSource.
// © 2025 Shuntaro Kasatani

use crate::audio_engine::{AudioSource, Track};

pub struct Mixer {
    /// Tracks to be mixed.
    tracks: Vec<Box<dyn Track>>,

    /// Number of channels in the output audio source.
    channels: usize,

    /// Sample rate of the output audio source.
    sample_rate: usize,
}

impl Mixer {
    /// Creates a new mixer instance.
    pub fn new(sample_rate: usize, channels: usize) -> Self {
        Mixer {
            tracks: Vec::new(),
            channels,
            sample_rate,
        }
    }

    /// Adds a new track to the mixer.
    pub fn add_track(&mut self, track: Box<dyn Track>) {
        self.tracks.push(track);
    }

    /// Mixes all the tracks into a single audio source.
    pub fn mix(&mut self) -> AudioSource {
        // Create a new AudioSource instance to return
        let mut output = AudioSource::new(self.sample_rate, self.channels);

        for track in &mut self.tracks {
            // Render the track and get the rendered audio source from the track
            track.render();
            let rendered_track = match track.rendered_data() {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("Error rendering track: {}", err);
                    continue;
                }
            };

            // Mix the rendered track into the output audio source
            output.mix(rendered_track);
        }

        // Return the mixed output.
        output
    }
}
