pub mod audio_track;
pub mod note_track;
mod region_id;

pub use region_id::RegionID;

use crate::{
    data_types::AudioContext,
    graph::{Graph, error::GraphError},
    mixer::TempoMap,
};

pub trait Track: Send {
    /// Returns a mutable pointer to the Graph.
    fn get_graph_mut(&mut self) -> &mut Graph;

    /// Sets the audio context to the new one.
    fn set_audio_ctx(&mut self, audio_ctx: &AudioContext);

    /// Prepares for the seeking.
    fn seek(&mut self);

    /// Prepares the track for processing.
    fn prepare(
        &mut self,
        start: usize,
        duration: usize,
        tempo_map: &TempoMap,
    ) -> Result<(), GraphError>;

    /// Processes the track with the given input and output pointer.
    fn process(&mut self, playhead: usize, output: *mut u8);
}
