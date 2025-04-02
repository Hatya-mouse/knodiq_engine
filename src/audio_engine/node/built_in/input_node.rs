use crate::audio_engine::node::traits::node::Node;
use crate::audio_engine::source::AudioSource;
use std::panic::panic_any;

pub struct InputNode {}

impl InputNode {
    pub fn new() -> Self {
        InputNode {}
    }
}

impl Node for InputNode {
    fn process(&mut self, input: AudioSource) -> AudioSource {
        input
    }

    fn get_property_list(&self) -> Vec<String> {
        Vec::new()
    }

    fn get_property(&self, _property: String) -> f64 {
        panic_any("InputNode has no property");
    }

    fn set_property(&mut self, _property: String, _value: f64) {
        panic_any("InputNode has no property");
    }
}
