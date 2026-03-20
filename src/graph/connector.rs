// connector.rs
// Represents the connection between nodes.
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

use crate::graph::{NodeID, TypeID};

#[derive(Clone, Debug)]
/// Represents the connection between nodes in the graph.
pub struct Connector {
    /// The ID of the node that provides the input signal.
    pub from: NodeID,
    /// The name of the parameter on the source node that provides the input signal.
    pub from_param: usize,
    /// The ID of the node that receives the input signal.
    pub to: NodeID,
    /// The name of the parameter on the destination node that receives the input signal.
    pub to_param: usize,
    /// The type of the signal being passed through this connector.
    pub value_type: TypeID,
}

/// An ID used to identify a node in the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, serde::Serialize)]
pub struct ConnectorID(usize);

impl ConnectorID {
    pub fn new(val: usize) -> Self {
        Self(val)
    }
}

impl Display for ConnectorID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
