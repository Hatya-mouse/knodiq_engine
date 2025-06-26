// node.rs
// Traits for audio processing graph nodes.
//
// Copyright 2025 Shuntaro Kasatani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use crate::{Beats, NodeId, Value};
use std::any::Any;

/// Represents a audio processing node.
/// In Knodiq we process audio data using "Node", instead of "Effects".
pub trait Node: Send + Sync + Any + NodeClone {
    /// Process the audio source.
    /// This method is called when the node is ready to process audio data.
    ///
    /// # Arguments
    /// - `sample_rate`: The sample rate of the audio data
    /// - `channels`: The number of audio channels
    /// - `chunk_start`: The start sample index of the chunk to process
    /// - `chunk_end`: The end sample index of the chunk to process
    fn process(
        &mut self,
        sample_rate: usize,
        channels: usize,
        chunk_start: usize,
        chunk_end: usize,
    ) -> Result<(), Box<dyn crate::error::GraphError>>;

    /// Prepare the node for processing. Called before processing.
    /// Chunk size is passed to the node to prepare for processing.
    ///
    /// This method is called after the all input properties are set.
    ///
    /// # Arguments
    /// - `chunk_size`: The size of the chunk to process, in beats.
    /// - `sample_rate`: The sample rate.
    fn prepare(
        &mut self,
        _chunk_size: Beats,
        _sample_rate: usize,
    ) -> Result<(), Box<dyn crate::error::GraphError>> {
        Ok(())
    }

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

    /// Get the node type.
    /// This is used to identify the type of the node, such as "BufferInputNode", "BufferOutputNode", etc.
    fn get_type(&self) -> String;

    /// Set the node id.
    fn set_id(&mut self, id: NodeId);

    /// Get the node id.
    fn get_id(&self) -> NodeId;

    /// Set the node name.
    /// This is used to label the node in the UI.
    fn set_name(&mut self, name: String);

    /// Get the node name.
    /// This is used to label the node in the UI.
    fn get_name(&self) -> String;

    /// Get whether the node is an input node.
    fn is_input(&self) -> bool;

    /// Get whether the node is an output node.
    fn is_output(&self) -> bool;

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
