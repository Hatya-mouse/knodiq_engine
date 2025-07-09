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

use crate::{Sample, Value};
use std::{fmt::Display, ops::Add, ops::Div, ops::Mul, ops::Rem, ops::Sub};

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Float(sample) => write!(f, "{}", sample),
            Value::Array(vec) => {
                let inner = vec
                    .iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "[{}]", inner)
            }
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Self) -> Self::Output {
        Value::apply_op(&[&self, &other], |v| v[0] + v[1]).unwrap_or(Value::Float(0.0))
    }
}

impl Add<Sample> for Value {
    type Output = Value;

    fn add(self, other: f32) -> Self::Output {
        Value::apply_op(&[&self, &Value::Float(other)], |v| v[0] + v[1])
            .unwrap_or(Value::Float(0.0))
    }
}

impl Add<Value> for Sample {
    type Output = Value;

    fn add(self, other: Value) -> Self::Output {
        Value::apply_op(&[&Value::Float(self), &other], |v| v[0] + v[1])
            .unwrap_or(Value::Float(0.0))
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Self) -> Self::Output {
        Value::apply_op(&[&self, &other], |v| v[0] - v[1]).unwrap_or(Value::Float(0.0))
    }
}

impl Sub<Sample> for Value {
    type Output = Value;

    fn sub(self, other: f32) -> Self::Output {
        Value::apply_op(&[&self, &Value::Float(other)], |v| v[0] - v[1])
            .unwrap_or(Value::Float(0.0))
    }
}

impl Sub<Value> for Sample {
    type Output = Value;

    fn sub(self, other: Value) -> Self::Output {
        Value::apply_op(&[&Value::Float(self), &other], |v| v[0] - v[1])
            .unwrap_or(Value::Float(0.0))
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Self) -> Self::Output {
        Value::apply_op(&[&self, &other], |v| v[0] * v[1]).unwrap_or(Value::Float(0.0))
    }
}

impl Mul<Sample> for Value {
    type Output = Value;

    fn mul(self, other: f32) -> Self::Output {
        Value::apply_op(&[&self, &Value::Float(other)], |v| v[0] * v[1])
            .unwrap_or(Value::Float(0.0))
    }
}

impl Mul<Value> for Sample {
    type Output = Value;

    fn mul(self, other: Value) -> Self::Output {
        Value::apply_op(&[&Value::Float(self), &other], |v| v[0] * v[1])
            .unwrap_or(Value::Float(0.0))
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Self) -> Self::Output {
        Value::apply_op(&[&self, &other], |v| {
            if v[1] == 0.0 {
                0.0 // Handle division by zero
            } else {
                v[0] / v[1]
            }
        })
        .unwrap_or(Value::Float(0.0))
    }
}

impl Div<Sample> for Value {
    type Output = Value;

    fn div(self, other: f32) -> Self::Output {
        if other == 0.0 {
            return Value::Float(0.0); // Handle division by zero
        }
        Value::apply_op(&[&self, &Value::Float(other)], |v| v[0] / v[1])
            .unwrap_or(Value::Float(0.0))
    }
}

impl Div<Value> for Sample {
    type Output = Value;

    fn div(self, other: Value) -> Self::Output {
        if let Value::Float(b) = other {
            if b == 0.0 {
                return Value::Float(0.0); // Handle division by zero
            }
        }
        Value::apply_op(&[&Value::Float(self), &other], |v| v[0] / v[1])
            .unwrap_or(Value::Float(0.0))
    }
}

impl Rem for Value {
    type Output = Value;

    fn rem(self, other: Self) -> Self::Output {
        Value::apply_op(&[&self, &other], |v| {
            if v[1] == 0.0 {
                0.0 // Handle division by zero
            } else {
                v[0] % v[1]
            }
        })
        .unwrap_or(Value::Float(0.0))
    }
}

impl Rem<Sample> for Value {
    type Output = Value;

    fn rem(self, other: f32) -> Self::Output {
        if other == 0.0 {
            return Value::Float(0.0); // Handle division by zero
        }
        Value::apply_op(&[&self, &Value::Float(other)], |v| v[0] % v[1])
            .unwrap_or(Value::Float(0.0))
    }
}

impl Rem<Value> for Sample {
    type Output = Value;

    fn rem(self, other: Value) -> Self::Output {
        if let Value::Float(b) = other {
            if b == 0.0 {
                return Value::Float(0.0); // Handle division by zero
            }
        }
        Value::apply_op(&[&Value::Float(self), &other], |v| v[0] % v[1])
            .unwrap_or(Value::Float(0.0))
    }
}
