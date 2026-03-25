use crate::{data_types::Beats, mixer::Project};

#[derive(Clone)]
pub enum AudioCommand {
    Play,
    Pause,
    Seek(Beats),
    UpdateProject(Project),
}
