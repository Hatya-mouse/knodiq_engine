use crate::{AudioBuffer, Sample};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Represents a floating-point value.
    Float(Sample),
    /// Represents a multiple audio samples.
    Buffer(AudioBuffer),
}
