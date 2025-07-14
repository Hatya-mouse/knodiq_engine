// value_type.rs
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

use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Float,
    Array(Box<Type>),
    None,
}

impl Type {
    pub fn get_depth(&self) -> usize {
        match self {
            Type::Int | Type::Float | Type::None => 0,
            Type::Array(t) => 1 + t.get_depth(),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Type::Int => "Int".to_string(),
                Type::Float => "Float".to_string(),
                Type::Array(t) => format!("[{}]", t),
                Type::None => "None".to_string(),
            }
        )
    }
}

pub fn type_of(left: &Type, right: &Type) -> Type {
    if left == right {
        return left.clone();
    }

    match (left, right) {
        (Type::Int, Type::Float) | (Type::Float, Type::Int) => Type::Float,
        (Type::Array(l), Type::Array(r)) => Type::Array(Box::new(type_of(l, r))),
        _ => {
            if left.get_depth() > right.get_depth() {
                left.clone()
            } else if right.get_depth() > left.get_depth() {
                right.clone()
            } else {
                Type::None
            }
        }
    }
}
