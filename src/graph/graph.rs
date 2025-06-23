// graph.rs
// Represents a graph of audio nodes that includes nodes, and connections between them.
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

use crate::{AudioSource, Connector, Node, Value, graph::built_in::BufferOutputNode};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

pub type NodeId = Uuid;

/// Represents a graph of audio nodes.
/// Processes the input audio by applying a series of audio nodes.
///
/// # What is `Graph`?
///
/// In Knodiq Audio Engine, `Graph` is a fundamental component that represents a network of audio processing nodes.
/// It allows the creation of complex audio processing chains by connecting various nodes together!
pub struct Graph {
    /// Vector of node instances in the graph.
    nodes: Vec<Box<dyn Node>>,
    /// Represents the connections between nodes in the graph.
    connections: Vec<Connector>,

    /// UUID of the input node.
    input_node: NodeId,
    /// UUID of the output node.
    output_node: NodeId,
}

impl Graph {
    /// Creates a new instance of `Graph`.
    pub fn new(input_node: Box<dyn Node>) -> Self {
        // Create input and output nodes
        let input_id = input_node.get_id();
        let output_node = Box::new(BufferOutputNode::new());
        let output_id = output_node.get_id();

        println!("Output Node ID: {:?}", output_id);

        Graph {
            nodes: vec![input_node, output_node],
            connections: Vec::new(),
            input_node: input_id,
            output_node: output_id,
        }
    }

    /// Gets the uuid of the input node of the graph.
    pub fn get_input_node_id(&self) -> NodeId {
        self.input_node
    }

    /// Gets the input node of the graph.
    pub fn get_input_node(&self) -> Option<&Box<dyn Node>> {
        self.nodes
            .iter()
            .find(|node| node.get_id() == self.input_node)
    }

    /// Gets the mutable input node of the graph.
    pub fn get_input_node_mut(&mut self) -> Option<&mut Box<dyn Node>> {
        self.nodes
            .iter_mut()
            .find(|node| node.get_id() == self.input_node)
    }

    /// Gets the uuid of the output node of the graph.
    pub fn get_output_node_id(&self) -> NodeId {
        self.output_node
    }

    /// Gets the output node of the graph.
    pub fn get_output_node(&self) -> Option<&Box<dyn Node>> {
        println!(
            "{:?}",
            self.nodes
                .iter()
                .map(|node| node.get_id())
                .collect::<Vec<NodeId>>()
        );
        println!("{:?}", self.output_node);
        println!(
            "output: {}",
            if self
                .nodes
                .iter()
                .find(|node| node.get_id() == self.output_node)
                .is_some()
            {
                "found"
            } else {
                "not found"
            }
        );

        self.nodes
            .iter()
            .find(|node| node.get_id() == self.output_node)
    }

    /// Gets the mutable output node of the graph.
    pub fn get_output_node_mut(&mut self) -> Option<&mut Box<dyn Node>> {
        self.nodes
            .iter_mut()
            .find(|node| node.get_id() == self.output_node)
    }

    /// Returns all nodes in the graph.
    pub fn get_nodes(&self) -> &Vec<Box<dyn Node>> {
        &self.nodes
    }

    /// Returns all mutable nodes in the graph.
    pub fn get_nodes_mut(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.nodes
    }

    /// Returns all connections in the graph.
    pub fn get_connections(&self) -> &Vec<Connector> {
        &self.connections
    }

    /// Returns all mutable connections in the graph.
    pub fn get_connections_mut(&mut self) -> &mut Vec<Connector> {
        &mut self.connections
    }

    /// Returns the node with the given id.
    pub fn get_node(&self, id: NodeId) -> Option<&Box<dyn Node>> {
        self.nodes.iter().find(|node| node.get_id() == id)
    }

    /// Returns the mutable node with the given id.
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Box<dyn Node>> {
        self.nodes.iter_mut().find(|node| node.get_id() == id)
    }

    /// Adds a new node to the graph and return the id.
    pub fn add_node(&mut self, node: Box<dyn Node>) -> NodeId {
        let id = node.get_id();
        self.nodes.push(node);
        id
    }

    /// Removes a node from the graph.
    pub fn remove_node(&mut self, id: NodeId) {
        // Check if the node is not the input or output node
        if id == self.input_node || id == self.output_node {
            return;
        }
        // Remove the node from the Vec
        self.nodes.retain(|node| node.get_id() != id);
        // Remove all connections from or to the node
        self.connections
            .retain(|connector| connector.from != id && connector.to != id);
    }

    /// Connect the node to one another.
    /// Doesn't add a connection to the node if it already exists.
    pub fn connect(&mut self, from: NodeId, from_param: String, to: NodeId, to_param: String) {
        // Check if the exact connection already exists
        if self.connections.iter().any(|c| {
            c.to == to && c.from == from && c.to_param == to_param && c.from_param == from_param
        }) {
            return;
        }

        // Check if the node is valid
        if self.get_node(from).is_none() || self.get_node(to).is_none() {
            return;
        }

        // Check if the parameters are valid
        if !self
            .get_node(from)
            .unwrap()
            .get_output_list()
            .contains(&from_param)
            && !self
                .get_node(to)
                .unwrap()
                .get_input_list()
                .contains(&to_param)
        {
            return;
        }

        self.connections.push(Connector {
            from,
            from_param,
            to,
            to_param,
        });
    }

    /// Disconnect the node from one another.
    pub fn disconnect(&mut self, from: NodeId, from_param: String, to: NodeId, to_param: String) {
        self.connections.retain(|connector| {
            connector.from != from
                || connector.from_param != from_param
                || connector.to != to
                || connector.to_param != to_param
        });
    }

    /// Sort the node using tolopogical sort
    pub fn topological_sort(&self) -> Result<Vec<NodeId>, &'static str> {
        let mut in_degree = HashMap::new();
        let mut adj_list = HashMap::new();

        // Initialize all the nodes with 0 degree
        for node_id in self.nodes.iter().map(|node| node.get_id()) {
            in_degree.insert(node_id, 0);
            adj_list.insert(node_id, vec![]);
        }

        // Register connections
        for connection in &self.connections {
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
            return Err("Cycle detected");
        }

        Ok(sorted)
    }

    pub fn prepare(&mut self, chunk_size: usize) {
        // Prepare the graph for processing
        // Call prepare() on each node
        for node in self.nodes.iter_mut() {
            node.prepare(chunk_size);
        }
    }

    /// Processes the input audio source and returns the output.
    ///
    /// # Arguments
    /// - `sample_rate`: The sample rate of the audio source.
    /// - `channels`: The number of channels in the audio source.
    /// - `chunk_start`: The sample index of the first sample in the processing chunk.
    /// - `chunk_end`: The sample index of the last sample in the processing chunk.
    pub fn process(
        &mut self,
        sample_rate: usize,
        channels: usize,
        chunk_start: usize,
        chunk_end: usize,
    ) -> Result<AudioSource, Box<dyn std::error::Error>> {
        // 1. Decide the process order using topological sort
        let sorted_nodes = self.topological_sort()?;

        // 2. Process nodes in order calculated
        for node_id in sorted_nodes {
            // Collect the connectors first to avoid borrowing issues
            let connected_connectors: Vec<Connector> = self
                .connections
                .iter()
                .filter(|connector| connector.to == node_id)
                .cloned()
                .collect();

            // Collect the input values before mutably borrowing self.nodes
            let input_values: Vec<(String, Value)> = connected_connectors
                .into_iter()
                .filter_map(|connector| {
                    // Get the output from the origin node
                    self.get_node_mut(connector.from).and_then(|origin_node| {
                        origin_node
                            .get_output(&connector.from_param)
                            .map(|value| (connector.to_param.clone(), value))
                    })
                })
                .collect();

            if let Some(node) = self.get_node_mut(node_id) {
                // Pass each input
                for (to_param, value) in input_values {
                    node.set_input(&to_param, value);
                }

                node.process(sample_rate, channels, chunk_start, chunk_end)?;
            }
        }

        // 3. Get the output of the output node and return it
        match self.get_output_node() {
            Some(node) => match node.get_output("buffer") {
                Some(value) => match value {
                    Value::Buffer(buffer) => {
                        Ok(AudioSource::from_buffer(buffer, sample_rate, channels))
                    }
                    _ => Err("Output wasn't a buffer".into()),
                },
                None => Err("Output not found".into()),
            },
            None => Err("Output node not found".into()),
        }
    }
}

impl Clone for Graph {
    fn clone(&self) -> Self {
        Graph {
            nodes: self.nodes.clone(),
            connections: self.connections.clone(),
            input_node: self.input_node,
            output_node: self.output_node,
        }
    }
}
