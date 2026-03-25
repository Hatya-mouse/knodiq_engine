use crate::data_types::Beats;

pub enum AudioCommand {
    Play,
    Pause,
    Seek(Beats),
}
