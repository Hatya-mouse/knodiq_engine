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

use crate::graph::TypeID;

pub type Value = Vec<u8>;

pub struct EdgeBuffer {
    pub data: Vec<u8>,
    pub value_type: TypeID,
    pub size: usize,
}

impl EdgeBuffer {
    pub fn new(value_type: TypeID, size: usize) -> Self {
        Self {
            data: vec![0; size],
            value_type,
            size,
        }
    }
}
