// empty_node.rs
// A graph node that just pass the audio source.
// © 2025 Shuntaro Kasatani

use crate::{Node, Value};
use std::any::Any;
use std::collections::HashMap;

/// A node that does nothing.
pub struct EmptyNode {
    input: Option<Value>,
    output: Option<Value>,
}

impl EmptyNode {
    /// Creates a new instance of the EmptyNode.
    pub fn new() -> Self {
        EmptyNode {
            input: None,
            output: None,
        }
    }
}

impl Node for EmptyNode {
    fn process(
        &mut self,
        _sample_rate: usize,
        channels: usize,
        chunk_start: usize,
        chunk_end: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let buffer = match self.input.as_ref() {
            Some(Value::Buffer(buffer)) => Value::Buffer(buffer.clone()),
            Some(value) => value.clone(),
            None => Value::Buffer(vec![vec![0.0; chunk_end - chunk_start]; channels]),
        };

        let mut result = HashMap::new();
        result.insert("output".to_string(), buffer.clone());
        self.output = Some(buffer);

        println!(
            "{:?}",
            if self.input.is_some() {
                "EmptyNode: Processed input and produced output."
            } else {
                "EmptyNode: No input provided, produced empty output."
            }
        );

        Ok(())
    }

    fn prepare(&mut self, _: usize) {}

    fn get_input_list(&self) -> Vec<String> {
        vec!["input".to_string()]
    }

    fn get_output_list(&self) -> Vec<String> {
        vec!["output".to_string()]
    }

    fn get_input(&self, property: &str) -> Option<Value> {
        match property {
            "input" => match self.input {
                Some(ref input) => Some(input.clone()),
                None => None,
            },
            _ => None,
        }
    }

    fn set_input(&mut self, property: &str, value: Value) {
        match property {
            "input" => self.input = Some(value),
            _ => (),
        }
    }

    fn get_output(&self, output: &str) -> Option<Value> {
        match output {
            "output" => self.output.clone(),
            _ => None,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Clone for EmptyNode {
    fn clone(&self) -> Self {
        EmptyNode {
            input: self.input.clone(),
            output: self.output.clone(),
        }
    }
}
