//
// © 2025-2026 Shuntaro Kasatani
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

use crate::{Node, audio_context::AudioContext, graph::TypeInfo};

/// An empty node that has single sample input.
#[derive(Default)]
pub struct OutputNode {}

impl Node for OutputNode {
    fn process(
        &mut self,
        _inputs: &[Option<*const u8>],
        _outputs: &[Option<*mut u8>],
        _audio_ctx: &AudioContext,
    ) {
    }

    fn prepare(&mut self, _audio_ctx: &AudioContext) {}

    fn get_input_list(&self) -> Vec<String> {
        vec!["Output".to_string()]
    }

    fn get_output_list(&self) -> Vec<String> {
        vec![]
    }

    fn get_input_type(&self, index: usize) -> Option<TypeInfo> {
        if index == 0 {
            Some(TypeInfo::new("Float".to_string(), 4, 4))
        } else {
            None
        }
    }

    fn get_output_type(&self, index: usize) -> Option<TypeInfo> {
        None
    }

    fn get_node_type(&self) -> String {
        "OutputNode".to_string()
    }
}
