// buffer_output_node.rs
// A graph node that just pass the audio source.
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

use crate::{Node, NodeId, Type, Value};
use std::any::Any;
use std::collections::HashMap;

/// A node that does nothing.
pub struct BufferOutputNode {
    id: NodeId,
    name: String,
    input: Option<Value>,
    output: Option<Value>,
}

impl BufferOutputNode {
    /// Creates a new instance of the BufferOutputNode.
    pub fn new() -> Self {
        BufferOutputNode {
            id: NodeId::new_v4(),
            name: "Buffer Output Node".to_string(),
            input: None,
            output: None,
        }
    }
}

impl Node for BufferOutputNode {
    fn process(
        &mut self,
        _sample_rate: usize,
        channels: usize,
        chunk_start: usize,
        chunk_end: usize,
    ) -> Result<(), Box<dyn crate::error::TrackError>> {
        let mut err = None;

        let buffer = match self.input.as_ref() {
            Some(Value::Array(array)) => Value::Array(array.clone()),
            _ => {
                err = Some(crate::error::NodeInputTypeError {
                    node_id: self.id.clone(),
                    input_name: "input".to_string(),
                    expected_type: Type::Array(Box::new(Type::Array(Box::new(Type::Float)))),
                    received_type: self
                        .input
                        .as_ref()
                        .map_or(Type::None, |input| input.get_type()),
                });
                Value::from_buffer(vec![vec![0.0; chunk_end - chunk_start]; channels])
            }
        };

        let mut result = HashMap::new();
        result.insert("buffer".to_string(), buffer.clone());
        self.output = Some(buffer);

        if let Some(e) = err {
            Err(Box::new(e))
        } else {
            Ok(())
        }
    }

    fn get_input_list(&self) -> Vec<String> {
        vec!["buffer".to_string()]
    }

    fn get_output_list(&self) -> Vec<String> {
        vec!["output".to_string()]
    }

    fn get_input(&self, property: &str) -> Option<Value> {
        match property {
            "buffer" => match self.input {
                Some(ref input) => Some(input.clone()),
                None => None,
            },
            _ => None,
        }
    }

    fn set_input(&mut self, property: &str, value: Value) {
        match property {
            "buffer" => self.input = Some(value),
            _ => (),
        }
    }

    fn get_output(&self, output: &str) -> Option<Value> {
        match output {
            "output" => self.output.clone(),
            _ => None,
        }
    }

    fn get_type(&self) -> String {
        "BufferOutputNode".to_string()
    }

    fn set_id(&mut self, id: crate::NodeId) {
        self.id = id;
    }

    fn get_id(&self) -> NodeId {
        self.id
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn is_input(&self) -> bool {
        false
    }

    fn is_output(&self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Clone for BufferOutputNode {
    fn clone(&self) -> Self {
        BufferOutputNode {
            id: self.id.clone(),
            name: self.name.clone(),
            input: self.input.clone(),
            output: self.output.clone(),
        }
    }
}
