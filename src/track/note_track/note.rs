use crate::data_types::Beats;

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
pub struct NoteID(pub usize);

#[derive(Clone)]
pub struct Note {
    /// Relative start position in the region in beats.
    pub start: Beats,
    /// Duration of the note in beats.
    pub duration: Beats,
    /// Frequency of the note.
    pub pitch: f32,
    /// Velocity of the note.
    pub velocity: f32,
}

impl Note {
    pub fn new(start: Beats, duration: Beats, pitch: f32, velocity: f32) -> Self {
        Self {
            start,
            duration,
            pitch,
            velocity,
        }
    }
}
