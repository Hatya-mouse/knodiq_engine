use crate::{
    data_types::{AudioContext, TypeInfo},
    node::Node,
};

/// An empty node that just writes the `process` input to the node output.
#[derive(Default)]
pub struct AudioOutputNode {
    data_type: TypeInfo,
}

impl Node for AudioOutputNode {
    fn get_input_names(&self) -> Vec<String> {
        vec!["audio".to_string()]
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
        self.data_type = TypeInfo::new(4 * audio_ctx.channels * audio_ctx.buffer_size, 4);
    }

    fn prepare(&mut self) {}

    fn process(&self, inputs: &[*const u8], outputs: &[*mut u8], _audio_ctx: &AudioContext) {
        for (input, output) in inputs.iter().zip(outputs.iter()) {
            unsafe {
                // Add the input data to the output buffer
                let len = self.data_type.size / 4;
                let src = std::slice::from_raw_parts(*input as *const f32, len);
                let dst = std::slice::from_raw_parts_mut(*output as *mut f32, len);
                for (d, s) in dst.iter_mut().zip(src.iter()) {
                    *d += *s;
                }
            }
        }
    }
}
