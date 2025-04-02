use crate::mixer::track::buffer_track::BufferTrack;

pub enum TrackType {
    Buffer(BufferTrack),
}

pub struct Mixer {}

impl Mixer {
    /// Create a new mixer instance.
    pub fn new() -> Self {
        Self {}
    }
}
