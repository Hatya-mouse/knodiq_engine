use crate::{AudioBuffer, Sample};

pub enum Value {
    /// Represents a floating-point value.
    Float(Sample),
    /// Represents a multiple audio samples.
    Buffer(AudioBuffer),
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Float(f) => Value::Float(*f),
            Value::Buffer(b) => Value::Buffer(b.clone()),
        }
    }
}
