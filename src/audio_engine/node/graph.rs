use crate::audio_engine::node::built_in::{input_node::InputNode, output_node::OutputNode};
use crate::audio_engine::node::traits::node::Node;
use crate::audio_engine::source::AudioSource;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

type NodeId = Uuid;

/// Represents a graph of audio nodes.
/// Processes the input audio by applying a series of audio nodes.
///
/// # What is `Graph`?
///
/// In Segment DAW, `Graph` is a fundamental component that represents a network of audio processing nodes.
/// It allows the creation of complex audio processing chains by connecting various nodes together!
pub struct Graph {
    /// Vector of node instances in the graph.
    nodes: HashMap<NodeId, Box<dyn Node>>,
    /// Represents the connections between nodes in the graph.
    connections: HashMap<NodeId, Vec<NodeId>>,

    /// UUID of the input node.
    input_node: NodeId,
    /// UUID of the output node.
    output_node: NodeId,
}

impl Graph {
    /// Creates a new instance of `Graph`.
    pub fn new() -> Self {
        // Create UUID for input and output nodes
        let input_id = Uuid::new_v4();
        let output_id = Uuid::new_v4();

        // Create input and output nodes
        let mut nodes = HashMap::new();
        nodes.insert(input_id, Box::new(InputNode::new()) as Box<dyn Node>);
        nodes.insert(output_id, Box::new(OutputNode::new()) as Box<dyn Node>);

        Graph {
            nodes,
            connections: HashMap::new(),
            input_node: input_id,
            output_node: output_id,
        }
    }

    /// Adds a new node to the graph and return the id.
    fn add_node(&mut self, node: Box<dyn Node>) -> NodeId {
        let id = Uuid::new_v4();
        self.nodes.insert(id, node);
        id
    }

    /// Removes a node from the graph.
    fn remove_node(&mut self, id: NodeId) {
        // Remove the node from the HashMap
        self.nodes.remove(&id);
        // Remove all connections from the node
        self.connections.remove(&id);
        // Remove all connections to the node
        for connected_nods in self.connections.values_mut() {
            connected_nods.retain(|&node_id| node_id != id);
        }
    }

    /// Connect the node to one another.
    fn connect(&mut self, from: NodeId, to: NodeId) {
        self.connections.entry(from).or_insert(Vec::new()).push(to);
    }

    /// Disconnect the node from one another.
    fn disconnect(&mut self, from: NodeId, to: NodeId) {
        if let Some(connected_nodes) = self.connections.get_mut(&from) {
            connected_nodes.retain(|&node_id| node_id != to);
        }
    }

    /// Processes the input audio source and returns the output.
    pub fn process(&mut self, input: AudioSource) -> AudioSource {
        let mut buffer_map: HashMap<NodeId, AudioSource> = HashMap::new();

        // First process the input data
        buffer_map.insert(Uuid::nil(), input.duplicated());

        // A hash set to save visited nodes
        let mut visited = HashSet::new();
        // [0]: Graph's input node
        let mut stack = vec![Uuid::nil()];

        while let Some(id) = stack.pop() {
            if visited.contains(&id) {
                continue;
            }
            visited.insert(id);

            // Process the node
            let node = self.nodes.get_mut(&id).unwrap();
            let input_buffer = buffer_map.get(&id).unwrap_or(&input).duplicated();
            let output_buffer = node.process(input_buffer);
            buffer_map.insert(id, output_buffer.duplicated());

            // Pass the output buffer to the next node
            if let Some(outputs) = self.connections.get(&id) {
                for &next_id in outputs {
                    stack.push(next_id);
                    buffer_map.insert(next_id, output_buffer.duplicated());
                }
            }
        }

        let last_node_id = self.nodes.keys().last().unwrap();
        buffer_map.get(last_node_id).unwrap_or(&input).duplicated()
    }
}
