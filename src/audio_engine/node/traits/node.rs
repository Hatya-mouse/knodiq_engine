use crate::audio_engine::source::AudioSource;

/// Represents a audio processing node.
/// In Segment we process audio data using "Node", instead of "Effects".
pub trait Node {
    /// Process the input audio source and return the output audio source.
    fn process(&mut self, input: AudioSource) -> AudioSource;

    /// Get the list of properties that can be set on this node.
    fn get_property_list(&self) -> Vec<String>;

    /// Get the node property. Panics if the property does not exist.
    fn get_property(&self, property: String) -> f64;

    /// Set the node property.
    fn set_property(&mut self, property: String, value: f64);
}
