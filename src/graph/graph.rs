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

use crate::{
    Connector, Node,
    audio_context::AudioContext,
    error::track::{TrackError, TrackErrorKind},
    graph::{
        EdgeBuffer, NodeID, TypeRegistry, built_in::OutputNode, connector::ConnectorID,
        graph_process_data::GraphProcessData,
    },
};
use std::collections::{HashMap, VecDeque};

/// Represents a graph of audio nodes.
/// Processes the input audio by applying a series of audio nodes.
///
/// # What is `Graph`?
///
/// In Knodiq Audio Engine, `Graph` is a fundamental component that represents a network of audio processing nodes.
/// It allows the creation of complex audio processing chains by connecting various nodes together.
#[derive(Default)]
pub struct Graph {
    /// Stores the type information for each registered type.
    type_registry: TypeRegistry,

    /// Vector of node instances in the graph.
    nodes: HashMap<NodeID, Box<dyn Node>>,
    /// Represents the connections between nodes in the graph.
    connections: HashMap<ConnectorID, Connector>,
    /// Cache of graph process data.
    process_data: GraphProcessData,

    /// NodeID of the input node.
    input_id: NodeID,
    /// NodeID of the output node.
    output_id: NodeID,

    /// The next available NodeID for new nodes.
    next_node_id: usize,
    /// The next available ConnectorID for new connections.
    next_connector_id: usize,
}

impl Graph {
    /// Creates a new instance of `Graph`.
    pub fn new(input_node: Box<dyn Node>) -> Self {
        let mut graph = Graph::default();

        // Create input and output nodes
        let output_node = Box::new(OutputNode::default());
        let input_id = graph.add_node(input_node);
        let output_id = graph.add_node(output_node);

        graph.input_id = input_id;
        graph.output_id = output_id;

        graph
    }

    pub fn generate_node_id(&mut self) -> NodeID {
        let id = NodeID::new(self.next_node_id);
        self.next_node_id += 1;
        id
    }

    pub fn generate_connector_id(&mut self) -> ConnectorID {
        let id = ConnectorID::new(self.next_connector_id);
        self.next_connector_id += 1;
        id
    }

    /// Returns the NodeID of the input node of the graph.
    pub fn get_input_node_id(&self) -> NodeID {
        self.input_id
    }

    /// Returns the NodeID of the output node of the graph.
    pub fn get_output_node_id(&self) -> NodeID {
        self.output_id
    }

    /// Returns the node with the given id.
    pub fn get_node(&self, id: &NodeID) -> Option<&Box<dyn Node>> {
        self.nodes.get(id)
    }

    /// Returns the mutable node with the given id.
    pub fn get_node_mut(&mut self, id: &NodeID) -> Option<&mut Box<dyn Node>> {
        self.nodes.get_mut(id)
    }

    /// Adds a new node to the graph and returns the id.
    pub fn add_node(&mut self, node: Box<dyn Node>) -> NodeID {
        let id = self.generate_node_id();
        self.nodes.insert(id, node);
        id
    }

    /// Removes a node from the graph.
    pub fn remove_node(&mut self, id: NodeID) {
        // Check if the node is not the input or output node
        if id == self.input_id || id == self.output_id {
            return;
        }
        // Remove the node from the HashMap
        self.nodes.remove(&id);
        // Remove all connections from or to the node
        self.connections
            .retain(|_, connector| connector.from != id && connector.to != id);
    }

    /// Connect the node to another.
    /// Doesn't add a connection to the node if it already exists.
    pub fn connect(
        &mut self,
        from: NodeID,
        from_param: usize,
        to: NodeID,
        to_param: usize,
    ) -> Option<ConnectorID> {
        // Check if the exact connection already exists
        if self.connections.iter().any(|(_, connector)| {
            connector.to == to
                && connector.from == from
                && connector.to_param == to_param
                && connector.from_param == from_param
        }) {
            return None;
        }

        let from_node = self.get_node(&from)?;
        let to_node = self.get_node(&to)?;

        let from_type_info = from_node.get_output_type(from_param)?;
        let to_type_info = to_node.get_input_type(to_param)?;
        if from_type_info != to_type_info {
            return None;
        }

        let value_type = self.type_registry.register_or_get(from_type_info)?;

        let connector_id = self.generate_connector_id();
        self.connections.insert(
            connector_id,
            Connector {
                from,
                from_param,
                to,
                to_param,
                value_type,
            },
        );

        Some(connector_id)
    }

    /// Disconnect the node from one another.
    pub fn disconnect(&mut self, connector_id: ConnectorID) {
        self.connections.remove(&connector_id);
    }

    /// Sort the node using tolopogical sort
    pub fn topological_sort(&mut self) -> Result<(), TrackError> {
        let mut in_degree = HashMap::new();
        let mut adj_list = HashMap::new();

        // Initialize all the nodes with 0 degree
        for node_id in self.nodes.keys() {
            in_degree.insert(*node_id, 0);
            adj_list.insert(*node_id, vec![]);
        }

        // Register connections
        for connection in self.connections.values() {
            *in_degree.entry(connection.to).or_insert(0) += 1;
            adj_list
                .entry(connection.from)
                .or_insert(vec![])
                .push(connection.to);
        }

        // Add the node with 0 degree to the queue
        let mut queue = VecDeque::new();
        for (&node, &deg) in &in_degree {
            if deg == 0 {
                queue.push_back(node);
            }
        }

        let mut sorted = vec![];

        while let Some(node) = queue.pop_front() {
            sorted.push(node);

            // Reduce the degree of neighbors and add to the queue when it reaches 0
            if let Some(neighbors) = adj_list.get(&node) {
                for &neighbor in neighbors {
                    if let Some(deg) = in_degree.get_mut(&neighbor) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }

        // Return the error when the cycle is detected
        if sorted.len() != self.nodes.len() {
            return Err(TrackError::NodeCycle);
        }

        self.process_data.sorted_nodes = sorted;
        Ok(())
    }

    pub fn prepare(&mut self, audio_ctx: &AudioContext) -> Result<(), TrackError> {
        // Prepare the graph for processing
        // Call prepare() on each node
        for node in self.nodes.values_mut() {
            node.prepare(audio_ctx)?;
        }

        // Create EdgeBuffers for each connector
        for connector_id in self.connections.keys() {
            let connector = self.connections.get(connector_id).unwrap();
            let type_info = self.type_registry.get_info(&connector.value_type).unwrap();
            let buffer_size = audio_ctx.buffer_samples * type_info.size;
            self.process_data.edge_buffers.insert(
                *connector_id,
                EdgeBuffer::new(connector.value_type, buffer_size),
            );
        }

        self.topological_sort()?;

        self.process_data.buffer_samples = audio_ctx.buffer_samples;

        Ok(())
    }

    /// Processes the input audio source and returns the output.
    ///
    /// # Arguments
    /// - `audio_ctx`: Current audio context.
    /// - `input`: The value to pass to the input node.
    pub fn process(
        &mut self,
        audio_ctx: &AudioContext,
        input: &[u8],
    ) -> Result<*const u8, TrackError> {
        // 1. Process nodes in the sorted order
        let sorted_nodes = self.process_data.sorted_nodes.clone();
        for node_id in sorted_nodes {
            // Get the incoming connectors
            let in_connectors: Vec<ConnectorID> = self
                .connections
                .iter()
                .filter(|(_, connector)| connector.to == node_id)
                .map(|(id, _)| *id)
                .collect();

            // Collect the input buffers before mutably borrowing self.nodes
            let input_buffers: Vec<Option<*const u8>> = self.process_data.input_indices[&node_id]
                .iter()
                .map(|connector_id| {
                    connector_id
                        .and_then(|id| self.process_data.edge_buffers.get(&id))
                        .map(|buf| buf.data.as_ptr())
                })
                .collect();

            // Collect output buffers
            let output_buffers: Vec<Option<*mut u8>> = self.process_data.output_indices[&node_id]
                .iter()
                .map(|connector_id| {
                    connector_id
                        .and_then(|id| self.process_data.edge_buffers.get_mut(&id))
                        .map(|buf| buf.data.as_mut_ptr())
                })
                .collect();

            let buffer_samples = self.process_data.buffer_samples;

            if let Some(node) = self.get_node_mut(&node_id) {
                // Pass the input and process
                node.process(&input_buffers, &output_buffers, audio_ctx)?;
            }
        }

        // 3. Get the output of the output node and return it
        let output_connector_id = self
            .connections
            .iter()
            .find(|(_, c)| c.to == self.output_node)
            .map(|(id, _)| *id)
            .ok_or_else(|| TrackError::new(TrackErrorKind::NodeNotFoundError(self.output_id)))?;

        let edge_buffer = self
            .process_data
            .edge_buffers
            .get(&output_connector_id)
            .map(|buf| buf.data.as_ptr())
            .ok_or_else(|| TrackError::new(TrackErrorKind::NodeNotFoundError(self.output_id)))?;

        Ok(edge_buffer)
    }
}
