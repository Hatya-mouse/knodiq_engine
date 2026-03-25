use crate::{audio_thread::AudioCommand, graph::error::GraphError};

pub enum AudioError {
    GraphError(GraphError),
    PlayStreamError(cpal::PlayStreamError),
    PauseStreamError(cpal::PauseStreamError),
    CommandFailed(AudioCommand),
}
