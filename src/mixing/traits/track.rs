// track.rs
// Trait that represents a track.
// © 2025 Shuntaro Kasatani

use crate::audio_utils::Beats;
use crate::{AudioSource, Graph, Region};
use std::any::Any;

pub trait Track: Send + Sync + Any {
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

    /// Returns the number of channels of the track.
    fn channels(&self) -> usize;

    /// Returns the regions of the track.
    fn regions(&self) -> Vec<&dyn Region>;

    /// Returns the regions of the track as mutable references.
    fn regions_mut(&mut self) -> Vec<&mut dyn Region>;

    /// Returns the specific region of the track by its identifier.
    fn get_region(&mut self, id: u32) -> Option<&dyn Region>;

    /// Returns the specific region of the track by its identifier as a mutable reference.
    fn get_region_mut(&mut self, id: u32) -> Option<&mut dyn Region>;

    /// Removes the specified region from the track.
    fn remove_region(&mut self, id: u32);

    /// Returns the type of the track in the form of a string.
    fn track_type(&self) -> String;

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

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
