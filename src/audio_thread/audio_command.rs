use crate::{data_types::Beats, mixer::Project};

pub enum AudioCommand {
    Play,
    Pause,
    Seek(Beats),
    UpdateProject(Project),
}
