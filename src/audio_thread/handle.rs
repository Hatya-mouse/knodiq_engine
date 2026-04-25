use crate::audio_thread::{AudioCommand, AudioError, AudioResult};
use std::sync::{Arc, atomic::AtomicUsize, mpsc};

/// A struct to communicate with the audio thread.
pub struct AudioThreadHandle {
    pub command_tx: mpsc::Sender<AudioCommand>,
    pub result_rx: mpsc::Receiver<Result<AudioResult, AudioError>>,
    pub playhead: Arc<AtomicUsize>,
}
