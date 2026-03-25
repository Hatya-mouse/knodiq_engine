use crate::{
    data_types::AudioContext,
    mixer::{TempoMap, track_id::TrackID},
    track::Track,
};
use std::collections::HashMap;

pub struct Project {
    // --- TRACKS ---
    pub tracks: HashMap<TrackID, Box<dyn Track>>,

    // --- TEMPO MAP ---
    pub tempo_map: TempoMap,

    // --- AUDIO CONTEXT ---
    pub audio_ctx: AudioContext,

    // --- MISCS ---
    next_track_id: usize,
}

impl Project {
    // --- NEW ---

    /// Creates a new project with the given tempo map.
    pub fn new(audio_ctx: AudioContext, bpm: f64) -> Self {
        Self {
            tracks: HashMap::new(),
            tempo_map: TempoMap::new(audio_ctx.clone(), bpm),
            audio_ctx,
            next_track_id: 0,
        }
    }

    // --- TRACK ID GENERATION ---

    /// Generates a new unique track ID.
    fn generate_track_id(&mut self) -> TrackID {
        let id = TrackID(self.next_track_id);
        self.next_track_id += 1;
        id
    }

    // --- TRACK MANAGEMENT ---

    /// Adds a new track to the mixer, setting the audio context to the one in the mixer.
    pub fn add_track(&mut self, mut track: Box<dyn Track>) -> TrackID {
        let id = self.generate_track_id();
        track.set_audio_ctx(&self.audio_ctx);
        self.tracks.insert(id, track);
        id
    }

    /// Removes the track from the mixer.
    pub fn remove_track(&mut self, id: &TrackID) {
        self.tracks.remove(id);
    }

    /// Returns a mutable reference to the track.
    pub fn get_track_mut(&mut self, id: &TrackID) -> Option<&mut Box<dyn Track>> {
        self.tracks.get_mut(id)
    }
}
