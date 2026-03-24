mod tempo_map;
mod track_id;

pub use tempo_map::TempoMap;

use crate::{
    data_types::{AudioContext, Beats},
    graph::error::GraphError,
    mixer::track_id::TrackID,
    track::Track,
};
use std::collections::HashMap;

#[derive(Default)]
pub struct Mixer {
    // --- TRACKS ---
    tracks: HashMap<TrackID, Box<dyn Track>>,

    // --- TEMPO MAP ---
    tempo_map: TempoMap,

    // --- AUDIO CONTEXT ---
    audio_ctx: AudioContext,

    // --- MISCS ---
    next_track_id: usize,
}

impl Mixer {
    // --- NEW ---

    /// Creates a new mixer with the given tempo.
    pub fn new(tempo: f64) -> Self {
        Self {
            tempo_map: TempoMap::new(tempo),
            ..Default::default()
        }
    }

    // --- TRACK ID GENERATION ---

    fn generate_track_id(&mut self) -> TrackID {
        let id = TrackID(self.next_track_id);
        self.next_track_id += 1;
        id
    }

    // --- TRACK ADDITION ---

    pub fn add_track(&mut self, track: Box<dyn Track>) -> TrackID {
        let id = self.generate_track_id();
        self.tracks.insert(id, track);
        id
    }

    // --- MIXING PROCESS ---

    /// Prepares the tracks in the mixer for the playback starting at the given beats.
    pub fn prepare(&mut self, start: Beats, duration: Beats) -> Result<(), GraphError> {
        // Convert the start and duration beats to samples
        let start_samples = self
            .tempo_map
            .beats_to_samples(start, self.audio_ctx.sample_rate);
        let duration_samples = self
            .tempo_map
            .beats_to_samples(duration, self.audio_ctx.sample_rate);

        // Prepare the tracks one by one
        for track in self.tracks.values_mut() {
            track.prepare(start_samples, duration_samples, &self.tempo_map)?;
        }

        Ok(())
    }

    /// Processes the tracks in the mixer a the specified playhead.
    pub fn process(&mut self, playhead: Beats, output: *mut u8) {
        // Fill the output buffer with zeros before processing
        unsafe {
            let len = self.audio_ctx.buffer_size * self.audio_ctx.channels;
            let dst = std::slice::from_raw_parts_mut(output as *mut f32, len);
            dst.fill(0.0);
        }

        // Convert the playhead beats to samples
        let playhead_samples = self
            .tempo_map
            .beats_to_samples(playhead, self.audio_ctx.sample_rate);

        // Call process function for every tracks
        for track in self.tracks.values_mut() {
            println!("{:?}", playhead);
            track.process(playhead_samples, output);
        }
    }
}
