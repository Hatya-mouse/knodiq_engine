use crate::{data_types::Beats, graph::error::GraphError, mixer::Project};

#[derive(Clone)]
pub enum AudioCommand {
    Play,
    Pause,
    Seek(Beats),
    UpdateProject(Project),
    ExportAudio(Project),
}

#[derive(Clone)]
pub enum AudioResult {
    ExportedAudio(Vec<f32>),
}

pub enum AudioError {
    GraphError(GraphError),
    PlayStreamError(cpal::PlayStreamError),
    CommandFailed(AudioCommand),
}

unsafe impl Sync for AudioError {}
