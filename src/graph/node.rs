// node.rs
// Traits for audio processing graph nodes.
// © 2025 Shuntaro Kasatani

use crate::AudioSource;
use std::any::Any;

/// Represents a audio processing node.
/// In Segment we process audio data using "Node", instead of "Effects".
pub trait Node: Send + Sync + Any {
    /// Process the audio source and return the output audio source.
    fn process(&mut self) -> Result<AudioSource, Box<dyn std::error::Error>>;

    /// Prepare the node for processing. Called before processing.
    /// Chunk size is passed to the node to prepare for processing.
    ///
    /// This method is called after the all input properties are set.
    fn prepare(&mut self, chunk_size: usize);

    /// Get the list of properties that can be set on this node.
    fn get_property_list(&self) -> Vec<String>;

    /// Get the node property. Panics if the property does not exist.
    fn get_property(&self, property: &str) -> Box<dyn Any>;

    /// Set the node property.
    fn set_property(&mut self, property: &str, value: Box<dyn Any>);

    fn as_any(&self) -> &dyn Any;
}
