use crate::thread::{AudioCommand, AudioError, AudioResult, audio_command::MidiCommand};
use std::sync::{Arc, atomic::AtomicUsize, mpsc};

/// A struct to communicate with the audio thread.
pub struct AudioThreadHandle {
    pub audio_command_tx: mpsc::Sender<AudioCommand>,
    pub midi_command_tx: mpsc::Sender<MidiCommand>,
    pub result_rx: mpsc::Receiver<Result<AudioResult, AudioError>>,
    pub playhead: Arc<AtomicUsize>,
}
