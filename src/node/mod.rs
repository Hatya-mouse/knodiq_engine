pub mod builtin;

use crate::{audio_context::AudioContext, type_registry::TypeInfo};

pub trait Node: Send {
    /// Returns a vector of the names of all inputs.
    fn get_input_names(&self) -> Vec<String>;

    /// Returns a vector of the names of all outputs.
    fn get_output_names(&self) -> Vec<String>;

    /// Returns the number of outputs.
    fn get_output_len(&self) -> usize;

    /// Returns the number of inputs.
    fn get_input_len(&self) -> usize;

    /// Returns the value type information of the specified input.
    fn get_input_type(&self, index: usize) -> Option<&TypeInfo>;

    /// Returns the value type information of the specified output.
    fn get_output_type(&self, index: usize) -> Option<&TypeInfo>;

    /// Updates the node with the given audio context.
    fn update(&mut self, audio_ctx: &AudioContext);

    /// Prepares the node for processing.
    fn prepare(&mut self, audio_ctx: &AudioContext);

    /// Processes the given input pointer and writes the output to the output pointer.
    fn process(&mut self, inputs: &[*const u8], outputs: &[*mut u8], audio_ctx: &AudioContext);
}
