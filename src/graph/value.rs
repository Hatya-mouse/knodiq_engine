use crate::{AudioBuffer, Sample};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Represents a floating-point value.
    Float(Sample),
    /// Represents a multiple audio samples.
    Buffer(AudioBuffer),
}

impl Value {
    /// Applies a function to the contained value. Returns None if the value is not compatible.
    ///
    /// ## Arguments
    /// - `f`: A function that takes a `Sample` and returns a value of type `T`.
    ///
    /// ## Discussion
    /// This method will apply the provided function `f` to the contained `Sample` if the `self` is of type `Float`.
    /// If the `self` is of type `Buffer`, it will apply the function to each sample in the buffer and return a new buffer with the results.
    ///
    /// The provided function should not rely on the order of samples.
    pub fn apply_fn<F>(&self, f: F) -> Option<Value>
    where
        F: Fn(Sample) -> Sample,
    {
        match self {
            Value::Float(sample) => Some(Value::Float(f(*sample))),
            Value::Buffer(buffer) => {
                let processed_samples: Vec<Vec<Sample>> = buffer
                    .iter()
                    .map(|channel| channel.iter().map(|&sample| f(sample)).collect())
                    .collect();
                Some(Value::Buffer(processed_samples))
            }
        }
    }

    /// Applies a operation which takes more than one `Sample`s and returns a processed `Value`.
    ///
    /// ## Arguments
    /// - `other`: Another `Value` to apply the operation with.
    /// - `f`: A function that takes two `Sample`s and returns a processed `Sample`.
    ///
    /// ## Discussion
    /// This method will apply the provided function `f` to the contained `Sample` and the `Sample` from `other` if both are of type `Float`.
    /// If both are of type `Buffer`, it will apply the function to each pair of samples in the buffers and return a new buffer with the results.
    pub fn apply_op<F>(&self, other: &Value, f: F) -> Option<Value>
    where
        F: Fn(Sample, Sample) -> Sample,
    {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(f(*a, *b))),
            (Value::Buffer(a), Value::Buffer(b)) => {
                if a.len() != b.len() {
                    return None; // Buffers must have the same number of channels
                }
                let processed_samples: Vec<Vec<Sample>> = a
                    .iter()
                    .zip(b.iter())
                    .map(|(channel_a, channel_b)| {
                        channel_a
                            .iter()
                            .zip(channel_b.iter())
                            .map(|(&sample_a, &sample_b)| f(sample_a, sample_b))
                            .collect()
                    })
                    .collect();
                Some(Value::Buffer(processed_samples))
            }
            (Value::Buffer(b), Value::Float(s)) => {
                let processed_samples: Vec<Vec<Sample>> = b
                    .iter()
                    .map(|channel| channel.iter().map(|&sample| f(sample, *s)).collect())
                    .collect();
                Some(Value::Buffer(processed_samples))
            }
            (Value::Float(s), Value::Buffer(b)) => {
                let processed_samples: Vec<Vec<Sample>> = b
                    .iter()
                    .map(|channel| channel.iter().map(|&sample| f(*s, sample)).collect())
                    .collect();
                Some(Value::Buffer(processed_samples))
            }
        }
    }
}
