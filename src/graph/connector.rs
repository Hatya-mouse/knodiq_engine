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

use crate::NodeId;

#[derive(Clone, Debug)]
/// Represents the connection between nodes in the graph.
pub struct Connector {
    /// The ID of the node that provides the input signal.
    pub from: NodeId,
    /// The name of the parameter on the source node that provides the input signal.
    pub from_param: String,
    /// The ID of the node that receives the input signal.
    pub to: NodeId,
    /// The name of the parameter on the destination node that receives the input signal.
    pub to_param: String,
}
