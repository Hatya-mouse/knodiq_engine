// node_input_type_error.rs
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

use crate::{NodeId, Type, error::TrackError};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct NodeInputTypeError {
    pub track_id: u32,
    pub node_id: NodeId,
    pub input_name: String,
    pub expected_type: Type,
    pub received_type: Type,
}

impl TrackError for NodeInputTypeError {
    fn clone_box(&self) -> Box<dyn TrackError> {
        Box::new(self.clone())
    }
}

impl std::error::Error for NodeInputTypeError {}

impl Display for NodeInputTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node {}: Input '{}' expected type '{}', but received type '{}'",
            self.node_id, self.input_name, self.expected_type, self.received_type
        )
    }
}
