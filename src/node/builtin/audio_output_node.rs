use crate::{audio_context::AudioContext, node::Node, type_registry::TypeInfo};
use std::ptr::copy_nonoverlapping;

/// An empty node that just writes the `process` input to the node output.
#[derive(Default)]
pub struct AudioOutputNode {
    data_type: TypeInfo,
}

impl Node for AudioOutputNode {
    fn get_input_names(&self) -> Vec<String> {
        vec!["input".to_string()]
    }

    fn get_output_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn get_input_len(&self) -> usize {
        1
    }

    fn get_output_len(&self) -> usize {
        0
    }

    fn get_input_type(&self, index: usize) -> Option<&TypeInfo> {
        if index == 0 {
            Some(&self.data_type)
        } else {
            None
        }
    }

    fn get_output_type(&self, _index: usize) -> Option<&TypeInfo> {
        None
    }

    fn update(&mut self, audio_ctx: &AudioContext) {
        self.data_type = TypeInfo::new(
            4 * audio_ctx.channels as usize * audio_ctx.buffer_size as usize,
            4,
        );
    }

    fn prepare(&mut self, _audio_ctx: &AudioContext) {}

    fn process(&mut self, inputs: &[*const u8], outputs: &[*mut u8], _audio_ctx: &AudioContext) {
        for (input, output) in inputs.iter().zip(outputs.iter()) {
            unsafe {
                // Copy the entire input to the output
                copy_nonoverlapping(*input, *output, self.data_type.size);
            }
        }
    }
}
