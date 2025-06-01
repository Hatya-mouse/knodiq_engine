// mixer.rs
// Mixer mixes multiple audio tracks into AudioSource.
// © 2025 Shuntaro Kasatani

use crate::{AudioSource, Track, audio_utils};
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

    /// Currently rendering position in beat.
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
        self.tracks.push(track);
    }

    /// Prepares the mixer for rendering.
    pub fn prepare(&mut self) {
        for track in &mut self.tracks {
            track.prepare(CHUNK_BEATS, self.sample_rate);
        }
    }

    pub fn samples_per_beat(&self) -> f32 {
        (self.sample_rate as f32) / (self.tempo as f32 / 60.0)
    }

    pub fn set_tempo(&mut self, tempo: Beats) {
        self.tempo = tempo;
    }

    pub fn set_sample_rate(&mut self, sample_rate: usize) {
        self.sample_rate = sample_rate;
    }

    pub fn set_channels(&mut self, channels: usize) {
        self.channels = channels;
    }

    pub fn get_track_by_id(&self, id: u32) -> Option<&Box<dyn Track>> {
        self.tracks.iter().filter(|t| t.id() == id).next()
    }

    pub fn get_track_by_id_mut(&mut self, id: u32) -> Option<&mut Box<dyn Track>> {
        self.tracks.iter_mut().filter(|t| t.id() == id).next()
    }

    pub fn remove_track(&mut self, id: u32) {
        self.tracks.retain(|t| t.id() != id);
    }

    /// Mixes all the tracks into a single audio source.
    ///
    /// # Arguments
    /// - `callback` - Called when the chunk has rendered. Rendered sample is passed. Sample will be passed in this way:
    /// `Sample 0` from `Channel 0`, `Sample 0` from `Channel 1`, `Sample 1` from `Channel 0`, `Sample 1` from `Channel 1`...
    pub fn mix(&mut self, mut callback: Box<dyn FnMut(f32) + Send>) -> AudioSource {
        self.playhead_beats = 0.0;

        // Create a new AudioSource instance to return
        let mut output = AudioSource::new(self.sample_rate, self.channels);

        // Chunk size in output sample rate
        let chunk_size = audio_utils::beats_as_samples(self.samples_per_beat(), CHUNK_BEATS);

        loop {
            // Process the chunk and get whether the rendering has completed
            if self.process_chunk(&mut output, CHUNK_BEATS) {
                break;
            }

            // Call the callback function for only the newly rendered chunk
            let start_sample =
                audio_utils::beats_as_samples(self.samples_per_beat(), self.playhead_beats);
            let end_sample = (start_sample + chunk_size).min(output.data[0].len());

            for sample in start_sample..end_sample {
                for channel in 0..self.channels {
                    callback(output.data[channel][sample]);
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
    ///
    /// # Returns
    /// - `true` - The rendering has completed.
    /// - `false` - The rendering hasn't completed yet.
    pub fn process_chunk(&mut self, output: &mut AudioSource, chunk_duration: Beats) -> bool {
        // Whether the processing has finished
        let mut completed = true;

        // Precompute samples per beat to avoid immutable borrow during the loop
        let samples_per_beat = self.samples_per_beat();

        // Loop through tracks
        for track in &mut self.tracks {
            // Render the track and get the rendered audio source from the track
            if !track.render_chunk_at(self.playhead_beats, chunk_duration, self.sample_rate) {
                completed = false;
            };
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

        // Return whether the rendering has completed
        completed
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
