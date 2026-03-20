//
// © 2025-2026 Shuntaro Kasatani
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

use crate::graph::{EdgeBuffer, NodeID, connector::ConnectorID};
use std::collections::HashMap;

#[derive(Default)]
pub(super) struct GraphProcessData {
    /// Edge buffers for each connector.
    pub edge_buffers: HashMap<ConnectorID, EdgeBuffer>,
    /// Sorted list of node IDs in the graph.
    pub sorted_nodes: Vec<NodeID>,
    /// List of incoming connectors, sorted to match the node inputs.
    pub input_indices: HashMap<NodeID, Vec<Option<ConnectorID>>>,
    /// List of outgoing connectors, sorted to match the node outputs.
    pub output_indices: HashMap<NodeID, Vec<Option<ConnectorID>>>,
    /// Number of samples in the current buffer.
    pub buffer_samples: usize,
}
