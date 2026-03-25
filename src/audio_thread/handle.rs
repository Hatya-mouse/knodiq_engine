use crate::audio_thread::{AudioCommand, error::AudioError};
use std::sync::{Arc, atomic::AtomicUsize, mpsc};

/// A struct to communicate with the audio thread.
pub struct AudioThreadHandle {
    pub command_tx: mpsc::Sender<AudioCommand>,
    pub error_rx: mpsc::Receiver<AudioError>,
    pub playhead: Arc<AtomicUsize>,
}
