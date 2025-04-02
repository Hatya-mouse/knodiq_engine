use crate::audio_engine::source::AudioSource;
use std::any::Any;

/// Represents a audio processing node.
/// In Segment we process audio data using "Node", instead of "Effects".
pub trait Node {
    /// Process the audio source and return the output audio source.
    fn process(&self) -> Result<AudioSource, Box<dyn std::error::Error>>;

    /// Get the list of properties that can be set on this node.
    fn get_property_list(&self) -> Vec<String>;

    /// Get the node property. Panics if the property does not exist.
    fn get_property(&self, property: &str) -> Box<dyn Any>;

    /// Set the node property.
    fn set_property(&mut self, property: &str, value: Box<dyn Any>);
}
