// node.rs
// Traits for audio processing graph nodes.
// © 2025 Shuntaro Kasatani

use crate::Value;
use std::{any::Any, collections::HashMap};

/// Represents a audio processing node.
/// In Knodiq we process audio data using "Node", instead of "Effects".
pub trait Node: Send + Sync + Any + NodeClone {
    /// Process the audio source.
    fn process(&mut self) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>>;

    /// Prepare the node for processing. Called before processing.
    /// Chunk size is passed to the node to prepare for processing.
    ///
    /// This method is called after the all input properties are set.
    fn prepare(&mut self, chunk_size: usize);

    /// Get the list of properties that can be set on this node.
    fn get_input_list(&self) -> Vec<String>;

    /// Get the list of output properties that can be retrieved from this node.
    fn get_output_list(&self) -> Vec<String>;

    /// Get the node property. Returns `None` if the property does not exist.
    fn get_input(&self, key: &str) -> Option<Value>;

    /// Set the node property.
    fn set_input(&mut self, key: &str, value: Value);

    /// Get the output with given name. Returns `None` if the property does not exist, or has not processed yet.
    fn get_output(&self, key: &str) -> Option<Value>;

    fn as_any(&self) -> &dyn Any;
}

// Helper trait to enable cloning of Box<dyn Node>
pub trait NodeClone {
    fn clone_box(&self) -> Box<dyn Node>;
}

impl<T> NodeClone for T
where
    T: 'static + Node + Clone,
{
    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Box<dyn Node> {
        self.clone_box()
    }
}
