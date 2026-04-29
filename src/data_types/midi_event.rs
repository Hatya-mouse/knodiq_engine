#[derive(Clone, Debug)]
pub enum MidiEvent {
    NoteOn { pitch: u8, velocity: u8 },
    NoteOff { pitch: u8 },
}
