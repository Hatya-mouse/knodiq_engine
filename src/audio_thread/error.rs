use crate::{audio_thread::AudioCommand, graph::error::GraphError};

pub enum AudioError {
    GraphError(GraphError),
    CommandFailed(AudioCommand),
}
