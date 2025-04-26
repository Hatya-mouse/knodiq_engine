// graph.rs
// Represents a graph of audio nodes that includes nodes, and connections between them.
// © 2025 Shuntaro Kasatani

use crate::{AudioSource, Connector, Node, graph::built_in::EmptyNode};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

pub type NodeId = Uuid;

/// Represents a graph of audio nodes.
/// Processes the input audio by applying a series of audio nodes.
///
/// # What is `Graph`?
///
/// In Segment Audio Engine, `Graph` is a fundamental component that represents a network of audio processing nodes.
/// It allows the creation of complex audio processing chains by connecting various nodes together!
pub struct Graph {
    /// Vector of node instances in the graph.
    nodes: HashMap<NodeId, Box<dyn Node>>,
    /// Represents the connections between nodes in the graph.
    connections: Vec<Connector>,

    /// UUID of the input node.
    pub input_nodes: Vec<NodeId>,
    /// UUID of the output node.
    pub output_node: NodeId,
}

impl Graph {
    /// Creates a new instance of `Graph`.
    pub fn new() -> Self {
        // Create UUID for input and output nodes
        let input_id = Uuid::new_v4();
        let output_id = Uuid::new_v4();

        // Create input and output nodes
        let mut nodes = HashMap::new();
        nodes.insert(input_id, Box::new(EmptyNode::new()) as Box<dyn Node>);
        nodes.insert(output_id, Box::new(EmptyNode::new()) as Box<dyn Node>);

        Graph {
            nodes,
            connections: Vec::new(),
            input_nodes: vec![input_id],
            output_node: output_id,
        }
    }

    /// Adds a new node to the graph and return the id.
    pub fn add_node(&mut self, node: Box<dyn Node>) -> NodeId {
        let id = Uuid::new_v4();
        self.nodes.insert(id, node);
        id
    }

    /// Removes a node from the graph.
    pub fn remove_node(&mut self, id: NodeId) {
        // Remove the node from the HashMap
        self.nodes.remove(&id);
        // Remove all connections from or to the node
        self.connections
            .retain(|connector| connector.from != id && connector.to != id);
    }

    /// Connect the node to one another.
    /// Doesn't add a connection to the node if it already exists.
    pub fn connect(&mut self, from: NodeId, from_param: String, to: NodeId, to_param: String) {
        if self.connections.iter().any(|c| c.to == to) {
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
        for &node_id in self.nodes.keys() {
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
        for (_uuid, node) in self.nodes.iter_mut() {
            node.prepare(chunk_size);
        }
    }

    /// Processes the input audio source and returns the output.
    pub fn process(
        &mut self,
        input_audio: AudioSource,
    ) -> Result<AudioSource, Box<dyn std::error::Error>> {
        // 1. Decide the process order using topological sort
        let sorted_nodes = self.topological_sort()?;

        // 2. Create a buffer map
        let mut sources: HashMap<NodeId, AudioSource> = HashMap::new();

        // 3. Set the input data to the input nodes
        for node_id in &self.input_nodes {
            self.nodes.get_mut(&node_id).map(|ref mut node| {
                node.set_property("input", Box::new(input_audio.clone()));
            });
        }

        // 4. Process nodes in order calculated
        for node_id in sorted_nodes {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                // If the node's output is not connected to other nodes, skip it
                // if !self.connections.iter().any(|conn| conn.from == node_id) {
                //     continue;
                // }

                // Get the output of the node and pass the source to connected nodes
                // Filter the connections to the current node, and then get the source from the connected node
                // Create a vector to store the inputs we need to process
                let inputs: Vec<_> = self
                    .connections
                    .iter()
                    .filter(|connector| connector.to == node_id)
                    .map(|connection| (connection.to_param.clone(), connection.from))
                    .collect();

                // Pass each input
                for (to_param, from_node_id) in inputs {
                    if let Some(source) = sources.get(&from_node_id) {
                        // Clone the audio source and set the property
                        let source = source.clone();
                        node.set_property(&to_param, Box::new(source));
                    }
                }

                // Process the audio and get the output
                let output_source = match node.process() {
                    Ok(source) => source,
                    Err(err) => return Err(err),
                };

                // Save the source
                sources.insert(node_id, output_source);
            }
        }

        // 5. Get the output of the output node and return it
        match sources.remove(&self.output_node) {
            Some(source) => Ok(source),
            None => Err("Output node not found".into()),
        }
    }
}
