// track.rs
// Trait that represents a track.
// © 2025 Shuntaro Kasatani

use crate::audio_utils::Beats;
use crate::{AudioSource, Graph};

pub trait Track {
    /// Returns the unique identifier of the track.
    fn id(&self) -> u32;

    /// Returns the name of the track.
    fn name(&self) -> &str;

    /// Sets the name of the track.
    fn set_name(&mut self, name: &str);

    /// Get the graph of the track.
    fn graph(&mut self) -> &mut Graph;

    /// Returns the current volume of the track.
    fn volume(&self) -> f32;

    /// Sets the volume of the track.
    fn set_volume(&mut self, volume: f32);

    /// Prepare the track for rendering.
    fn prepare(&mut self, chunk_size: Beats, sample_rate: usize);

    /// Renders the specified area of the track.
    ///
    /// # Arguments
    /// - `playhead` - The currently rendering duration of the audio track in beats.
    /// - `chunk_size` - The size of the chunk to render.
    /// - `sample_rate` - The sample rate of the audio track.
    ///
    /// # Returns
    /// - `true` The track has finished rendering.
    /// - `false` The track still has regions or the graph to render.
    fn render_chunk_at(&mut self, playhead: Beats, chunk_size: Beats, sample_rate: usize) -> bool;

    /// Returns the rendered audio source.
    fn rendered_data(&self) -> Result<&AudioSource, Box<dyn std::error::Error>>;
}
