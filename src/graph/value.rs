// value.rs
// An enum for audio processing graph values.
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

use crate::{AudioBuffer, Sample};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    /// Represents a floating-point value.
    Float(Sample),
    /// Represents an array of floating-point values.
    Array(Vec<Value>),
}

impl Value {
    /// Applies a function to the contained value. Returns None if the value is not compatible.
    ///
    /// ## Arguments
    /// - `f`: A function that takes a `Sample` and returns a value of type `T`.
    ///
    /// ## Discussion
    /// This method will apply the provided function `f` to the contained `Sample` if the `self` is of type `Float`.
    /// If the `self` is of type `Array`, it will apply the function to each sample in the array and return a new array with the results.
    ///
    /// The provided function should not rely on the order of samples.
    pub fn apply_fn<F>(&self, f: F) -> Option<Value>
    where
        F: Fn(Sample) -> Sample,
    {
        match self {
            Value::Float(sample) => Some(Value::Float(f(*sample))),
            Value::Array(vector) => {
                // let processed_samples: Vec<Vec<Sample>> = buffer
                //     .iter()
                //     .map(|channel| channel.iter().map(|&sample| f(sample)).collect())
                //     .collect();
                // Some(Value::Buffer(processed_samples))
                let processed_array: Vec<Value> = vector
                    .iter()
                    .map(|value| match value {
                        Value::Float(sample) => Value::Float(f(*sample)),
                        Value::Array(vec) => {
                            // Recursively call apply_fn to inner arrays
                            // If apply_fn returns None, also return None in this level
                            let processed_inner = vec
                                .iter()
                                .map(|val| val.apply_fn(&f))
                                .filter_map(|opt| opt) // Filter out None values
                                .collect();
                            Value::Array(processed_inner)
                        }
                    })
                    .collect();
                Some(Value::Array(processed_array))
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
            (Value::Array(a), Value::Array(b)) => {
                if a.len() != b.len() {
                    return None; // Buffers must have the same number of channels
                }
                let processed_array: Vec<Value> = a
                    .iter()
                    .zip(b.iter())
                    .map(|(v_a, v_b)| v_a.apply_op(v_b, &f))
                    .filter_map(|opt| opt) // Filter out None values
                    .collect();
                Some(Value::Array(processed_array))
            }
            (Value::Array(a), Value::Float(v)) | (Value::Float(v), Value::Array(a)) => {
                let processed_array: Vec<Value> = a
                    .iter()
                    .map(|value| match value {
                        Value::Float(sample) => Value::Float(f(*sample, *v)),
                        Value::Array(vec) => {
                            // Recursively call apply_op to inner arrays
                            // If apply_op returns None, also return None in this level
                            let processed_inner = vec
                                .iter()
                                .map(|val| val.apply_op(&Value::Float(*v), &f))
                                .filter_map(|opt| opt) // Filter out None values
                                .collect();
                            Value::Array(processed_inner)
                        }
                    })
                    .collect();
                Some(Value::Array(processed_array))
            }
        }
    }

    /// Creates a new `Value` from a `AudioBuffer`.
    pub fn from_buffer(buffer: AudioBuffer) -> Self {
        let array = buffer
            .into_iter()
            .map(|channel| Value::Array(channel.into_iter().map(Value::Float).collect()))
            .collect();
        Value::Array(array)
    }

    /// Converts the value into an audio buffer, if possible.
    pub fn as_buffer(&self) -> Option<AudioBuffer> {
        match self {
            Value::Float(_) => None,
            Value::Array(vector) => {
                let mut buffer = AudioBuffer::new();
                for value in vector {
                    match value {
                        Value::Float(_) => return None,
                        Value::Array(inner_vector) => {
                            // Recursively convert inner arrays to buffers
                            let inner_buffer = inner_vector
                                .iter()
                                .map(|v| match v {
                                    Value::Float(sample) => *sample,
                                    _ => 0.0,
                                })
                                .collect();
                            buffer.push(inner_buffer);
                        }
                    }
                }
                Some(buffer)
            }
        }
    }
}
