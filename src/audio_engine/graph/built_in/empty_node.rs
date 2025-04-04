// empty_node.rs
// A graph node that just pass the audio source.
// © 2025 Shuntaro Kasatani

use crate::audio_engine::graph::Node;
use crate::audio_engine::AudioSource;
use std::any::Any;
use std::panic::panic_any;

/// A node that does nothing.
pub struct EmptyNode {
    input: Option<AudioSource>,
}

impl EmptyNode {
    /// Creates a new instance of the EmptyNode.
    pub fn new() -> Self {
        EmptyNode { input: None }
    }
}

impl Node for EmptyNode {
    fn process(&mut self) -> Result<AudioSource, Box<dyn std::error::Error>> {
        Ok(self.input.as_ref().ok_or("Input not provided")?.clone())
    }

    fn prepare(&mut self, _: usize) {}

    fn get_property_list(&self) -> Vec<String> {
        Vec::new()
    }

    fn get_property(&self, property: &str) -> Box<dyn Any> {
        match property {
            "input" => Box::new(match self.input {
                Some(ref input) => Some(input.clone()),
                None => None,
            }),
            _ => panic_any("Invalid property"),
        }
    }

    fn set_property(&mut self, property: &str, value: Box<dyn Any>) {
        match property {
            "input" => {
                if let Some(input) = value.downcast_ref::<AudioSource>() {
                    self.input = Some(input.clone());
                }
            }
            _ => panic_any("Invalid property"),
        }
    }
}
