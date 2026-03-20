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

use crate::{audio_context::AudioContext, graph::TypeInfo};

/// Represents a audio processing node.
pub trait Node {
    /// Process the audio source.
    /// This method is called when the node is ready to process audio data.
    ///
    /// # Arguments
    /// - `inputs`: A map of input buffer pointers by name.
    /// - `outputs`: A map of output buffer pointers by name.
    /// - `audio_ctx`: The current audio context.
    fn process(
        &mut self,
        inputs: &[Option<*const u8>],
        outputs: &[Option<*mut u8>],
        audio_ctx: &AudioContext,
    );

    /// Prepare the node for processing. Called before processing.
    /// Chunk size is passed to the node to prepare for processing.
    ///
    /// This method is called after the all input properties are set.
    ///
    /// # Arguments
    /// `audio_ctx`: The current audio context.
    fn prepare(&mut self, audio_ctx: &AudioContext);

    /// Get the list of properties that can be set on this node.
    fn get_input_list(&self) -> Vec<String>;

    /// Get the list of output properties that can be retrieved from this node.
    fn get_output_list(&self) -> Vec<String>;

    /// Returns the input value type.
    fn get_input_type(&self, index: usize) -> Option<TypeInfo>;

    /// Returns the output value type.
    fn get_output_type(&self, index: usize) -> Option<TypeInfo>;

    /// Get the node type.
    /// This is used to identify the type of the node, such as "BufferInputNode", "BufferOutputNode", etc.
    fn get_node_type(&self) -> String;
}
