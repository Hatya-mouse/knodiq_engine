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
    /// Creates a new note region.
    pub fn new(start: Beats, duration: Beats) -> Self {
        Self {
            start,
            duration,
            notes: HashMap::new(),
            next_note_id: 0,
        }
    }

    /// Generates a new note ID.
    fn generate_note_id(&mut self) -> NoteID {
        let id = NoteID(self.next_note_id);
        self.next_note_id += 1;
        id
    }

    /// Adds a given note to the region.
    pub fn add_note(&mut self, note: Note) {
        let id = self.generate_note_id();
        self.notes.insert(id, note);
    }

    /// Removes the note from the region.
    pub fn remove_note(&mut self, id: &NoteID) {
        self.notes.remove(id);
    }
}
