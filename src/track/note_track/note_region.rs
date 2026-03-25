use crate::data_types::Beats;

#[derive(Clone)]
pub struct Note {
    /// Relative start position in the region in beats.
    pub start: Beats,
    /// Duration of the note in beats.
    pub duration: Beats,
    /// Frequency of the note.
    pub frequency: f32,
    /// Velocity of the note.
    pub velocity: f32,
}

impl Note {
    pub fn new(start: Beats, duration: Beats, frequency: f32, velocity: f32) -> Self {
        Self {
            start,
            duration,
            frequency,
            velocity,
        }
    }
}

#[derive(Clone)]
pub struct NoteRegion {
    pub start: Beats,
    pub duration: Beats,
    pub notes: Vec<Note>,
}

impl NoteRegion {
    pub fn new(start: Beats, duration: Beats, notes: Vec<Note>) -> Self {
        Self {
            start,
            duration,
            notes,
        }
    }

    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }
}
