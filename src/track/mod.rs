mod audio_track;
mod note_track;
mod region_id;

pub use region_id::RegionID;

use crate::{
    data_types::{AudioContext, Beats},
    graph::error::GraphError,
};

pub trait Track {
    /// Sets the audio context to the new one.
    fn set_audio_ctx(&mut self, audio_ctx: &AudioContext);

    /// Prepares the track for processing.
    fn prepare(&mut self, total_duration: Beats) -> Result<(), GraphError>;

    /// Processes the track with the given input and output pointer.
    fn process(&mut self, playhead: Beats, output: *mut u8, audio_ctx: &AudioContext);
}
