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

use crate::{AudioBuffer, Sample, Type, error::TypeError};
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
        F: Fn(Sample) -> Sample + Clone,
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
                                .map(|val| val.apply_fn(f.clone()))
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
    pub fn apply_op<F>(args: &[&Value], f: F) -> Option<Value>
    where
        F: Fn(&[Sample]) -> Sample + Clone,
    {
        let mut shapes = vec![];
        for arg in args {
            shapes.push(arg.get_shape());
        }

        // Calculate the target shape by taking the maximum dimensions from all shapes
        let mut target_shape = vec![];
        for shape in shapes {
            if shape.len() > target_shape.len() {
                target_shape = shape;
            } else if shape.len() == target_shape.len() {
                for (i, dim) in shape.iter().enumerate() {
                    if *dim > target_shape[i] {
                        target_shape[i] = *dim;
                    }
                }
            }
        }

        // Reshape all values to the target shape
        let reshaped_args: Vec<Value> = args
            .iter()
            .map(|arg| Value::reshape(arg, target_shape.clone()))
            .filter_map(|opt| opt) // Filter out None values
            .collect();

        // Apply operation on reshaped values recursively
        Value::recurse_op(&reshaped_args.iter().collect::<Vec<_>>(), f)
    }

    pub fn recurse_op<F>(args: &[&Value], f: F) -> Option<Value>
    where
        F: Fn(&[Sample]) -> Sample + Clone,
    {
        // Check if all values are of the same type
        if args.is_empty() {
            return None;
        }

        let first_type = args[0].get_type();
        if !args.iter().all(|arg| arg.get_type() == first_type) {
            return None;
        }

        // Check if all values are of the same shape
        let first_shape = args[0].get_shape();
        if !args.iter().all(|arg| arg.get_shape() == first_shape) {
            return None;
        }

        // If all values are of the same type and shape, we can apply the operation
        match first_type {
            Type::Float => {
                // If the type is Float, we can apply the operation directly
                let samples: Vec<Sample> = args
                    .iter()
                    .filter_map(|arg| match arg {
                        Value::Float(sample) => Some(*sample),
                        _ => None,
                    })
                    .collect();
                if samples.is_empty() {
                    return None;
                }
                Some(Value::Float(f(&samples)))
            }
            Type::Array(_) => {
                // If the type is Array, we need to apply the operation using recurse_op recursively
                let mut inner_arrays = vec![];
                for arg in args {
                    match arg {
                        Value::Array(vec) => inner_arrays.push(vec),
                        _ => return None,
                    }
                }

                let mut iterators = inner_arrays
                    .iter()
                    .map(|vec| vec.iter())
                    .collect::<Vec<_>>();
                let mut result = vec![];

                loop {
                    let mut inner_args = vec![];

                    for iter in iterators.iter_mut() {
                        match iter.next() {
                            Some(value) => inner_args.push(value),
                            None => return Some(Value::Array(result)),
                        }
                    }

                    let operated = match Value::recurse_op(inner_args.as_slice(), f.clone()) {
                        Some(val) => val,
                        None => break,
                    };
                    result.push(operated);
                }

                Some(Value::Array(result))
            }
            Type::None => None,
        }
    }

    pub fn reshape(v: &Value, shape: Vec<usize>) -> Option<Value> {
        match v {
            Value::Float(sample) => {
                // If the value is a single float, we create an array of the specified shape filled with that float
                if !shape.is_empty() {
                    let mut shape_iter = shape.iter();
                    let mut array = vec![Value::Float(*sample); *shape_iter.next_back().unwrap()];
                    // If the shape is not empty, we wrap the array in another array to match the shape
                    while let Some(dim) = shape_iter.next_back() {
                        array = vec![Value::Array(array); *dim];
                    }
                    Some(Value::Array(array))
                } else {
                    Some(Value::Float(*sample))
                }
            }
            Value::Array(vec) => {
                // If the value is an array, we resize it to match the specified shape
                let mut new_vec;
                let vec_shape = v.get_shape();

                if vec_shape.len() > shape.len() {
                    return None;
                }

                new_vec = vec.clone();
                if vec_shape.len() < shape.len() {
                    for _ in 0..shape.len() - vec_shape.len() {
                        new_vec = vec![Value::Array(new_vec)];
                    }
                }

                let resized = Value::Array(new_vec).resize_val(shape);

                return resized;
            }
        }
    }

    pub fn resize_val(&self, shape: Vec<usize>) -> Option<Value> {
        Some(match self {
            Value::Float(_) => self.clone(),
            Value::Array(vec) => {
                let mut result = vec![];
                let mut resized = vec![];

                if vec.len() == shape[0] {
                    resized.extend(vec.clone());
                } else if vec.len() == 1 {
                    resized.resize(shape[0], vec[0].clone());
                } else {
                    return None;
                }

                let rest = shape[1..].to_vec();

                for value in resized {
                    match value.resize_val(rest.clone()) {
                        Some(resized_inner) => result.push(resized_inner),
                        None => return None,
                    }
                }

                Value::Array(result)
            }
        })
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
    pub fn as_buffer(&self) -> Result<AudioBuffer, TypeError> {
        match self {
            Value::Float(_) => Err(TypeError {
                expected_type: Type::Array(Box::new(Type::Array(Box::new(Type::Float)))),
                received_type: Type::Float,
            }),
            Value::Array(vector) => {
                let mut buffer = AudioBuffer::new();
                for value in vector {
                    match value {
                        Value::Float(_) => {
                            return Err(TypeError {
                                expected_type: Type::Array(Box::new(Type::Array(Box::new(
                                    Type::Float,
                                )))),
                                received_type: Type::Array(Box::new(Type::Float)),
                            });
                        }
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
                Ok(buffer)
            }
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Value::Float(_) => Type::Float,
            Value::Array(vec) => Type::Array(Box::new(match vec.first() {
                Some(val) => val.get_type(),
                None => Type::Float,
            })),
        }
    }

    pub fn get_shape(&self) -> Vec<usize> {
        match self {
            Value::Float(_) => vec![],
            Value::Array(vec) => {
                let mut shape = vec![vec.len()];
                if let Some(first) = vec.first() {
                    shape.extend(first.get_shape());
                }
                shape
            }
        }
    }
}
