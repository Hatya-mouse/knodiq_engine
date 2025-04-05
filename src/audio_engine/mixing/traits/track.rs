// track.rs
// Trait that represents a track.
// © 2025 Shuntaro Kasatani

use crate::audio_engine::{AudioSource, Graph};
use std::time::Duration;

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

    /// Renders the specified area of the track.
    ///
    /// # Arguments
    /// - `sample_rate` - The sample rate of the audio track.
    /// - `callback` - The callback function to receive the rendered audio data.
    fn render(&mut self, sample_rate: usize, callback: &mut Box<dyn FnMut(f32)>);

    /// Returns the rendered audio source.
    fn rendered_data(&self) -> Result<&AudioSource, Box<dyn std::error::Error>>;
}
