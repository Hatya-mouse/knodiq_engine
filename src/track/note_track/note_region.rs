use crate::{
    data_types::Beats,
    track::note_track::{Note, NoteID},
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct NoteRegion {
    pub start: Beats,
    pub duration: Beats,
    pub notes: HashMap<NoteID, Note>,

    next_note_id: usize,
}

impl NoteRegion {
    // --- NEW ---

    /// Creates a new note region.
    pub fn new(start: Beats, duration: Beats) -> Self {
        Self {
            start,
            duration,
            notes: HashMap::new(),
            next_note_id: 0,
        }
    }

    // --- NOTE ID GENERATION ---

    /// Generates a new note ID.
    fn generate_note_id(&mut self) -> NoteID {
        let id = NoteID(self.next_note_id);
        self.next_note_id += 1;
        id
    }

    // --- NOTE MANAGEMENT ---

    /// Adds a given note to the region.
    pub fn add_note(&mut self, note: Note) {
        let id = self.generate_note_id();
        self.notes.insert(id, note);
    }

    /// Removes the note from the region.
    pub fn remove_note(&mut self, id: &NoteID) {
        self.notes.remove(id);
    }

    // --- NOTE GETTING ---

    /// Returns a mutable reference to the note.
    pub fn get_note_mut(&mut self, id: &NoteID) -> Option<&mut Note> {
        self.notes.get_mut(id)
    }

    // --- NOTE MODIFICATION ---

    /// Changes the note's start to the given start.
    pub fn set_start(&mut self, id: &NoteID, start: Beats) {
        if let Some(note) = self.get_note_mut(id) {
            note.start = start;
        }
    }

    /// Sets the note's duration to the given duration.
    pub fn set_duration(&mut self, id: &NoteID, duration: Beats) {
        if let Some(note) = self.get_note_mut(id) {
            note.duration = duration;
        }
    }

    /// Sets the note's pitch to the given pitch.
    pub fn set_pitch(&mut self, id: &NoteID, pitch: f32) {
        if let Some(note) = self.get_note_mut(id) {
            note.pitch = pitch;
        }
    }

    /// Sets the note's velocity to the given velocity.
    pub fn set_velocity(&mut self, id: &NoteID, velocity: f32) {
        if let Some(note) = self.get_note_mut(id) {
            note.velocity = velocity;
        }
    }
}
