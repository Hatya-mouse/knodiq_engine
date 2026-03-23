pub(super) struct VoiceEvent {
    pub sample_index: usize,
    pub frequency: f32,
    pub velocity: f32,
    pub is_note_on: bool,
}

impl VoiceEvent {
    pub fn new(sample_index: usize, frequency: f32, velocity: f32, is_note_on: bool) -> Self {
        Self {
            sample_index,
            frequency,
            velocity,
            is_note_on,
        }
    }
}
