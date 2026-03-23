use crate::data_types::Beats;

pub struct Note {
    /// Relative start position in the region in beats.
    pub start: Beats,
    /// Duration of the note in beats.
    pub duration: Beats,
    /// Frequency of the note.
    pub frequency: f32,

    pub velocity: f32,
}

pub struct NoteRegion {
    pub start: Beats,
    pub duration: Beats,
    pub notes: Vec<Note>,
}
