pub mod error;
pub mod node_id;
pub mod topological_sort;

use crate::{
    audio_context::AudioContext,
    graph::{error::GraphError, node_id::NodeID},
    node::Node,
};
use std::collections::HashMap;

#[derive(Default)]
pub struct Graph {
    nodes: HashMap<NodeID, Box<dyn Node>>,
    edges: Vec<(NodeID, usize, NodeID, usize)>,
    adjacency: HashMap<NodeID, Vec<NodeID>>,
    sorted_nodes: Vec<NodeID>,
    edge_buffers: HashMap<(NodeID, usize), Vec<u8>>,

    input_node: NodeID,
    output_node: NodeID,

    next_node_id: usize,
}

impl Graph {
    // --- INITIALIZATION ---

    /// Creates a new Graph instance with the given input and output node..
    pub fn new(input_node: Box<dyn Node>, output_node: Box<dyn Node>) -> Self {
        let mut graph = Graph::default();
        // Register the input and output nodes
        let input_id = graph.add_node(input_node);
        let output_id = graph.add_node(output_node);
        graph.input_node = input_id;
        graph.output_node = output_id;
        // Return the newly created graph
        graph
    }

    // --- ID GENERATION ---

    /// Generates a new NodeID which is unique inside the graph.
    fn generate_node_id(&mut self) -> NodeID {
        let id = NodeID(self.next_node_id);
        self.next_node_id += 1;
        id
    }

    // --- NODE MANIPULATION ---

    /// Adds a new node to the graph, and returns the newly generated node ID.
    pub fn add_node(&mut self, node: Box<dyn Node>) -> NodeID {
        let id = self.generate_node_id();
        self.nodes.insert(id, node);
        id
    }

    /// Connects the node's output to another node's input.
    pub fn connect(
        &mut self,
        from: &NodeID,
        output_name: &str,
        to: &NodeID,
        input_name: &str,
    ) -> Result<(), GraphError> {
        let output_idx = self.nodes[from]
            .get_output_names()
            .iter()
            .position(|n| n == output_name)
            .ok_or_else(|| GraphError::OutputNotFound(*from, output_name.to_string()))?;
        let input_idx = self.nodes[to]
            .get_input_names()
            .iter()
            .position(|n| n == input_name)
            .ok_or_else(|| GraphError::InputNotFound(*from, input_name.to_string()))?;
        self.edges.push((*from, output_idx, *to, input_idx));
        Ok(())
    }

    // --- GRAPH PROCESSING ---

    fn prepare(&mut self, audio_ctx: &AudioContext) {}

    fn process(&mut self, inputs: &[*const u8], outputs: &[*mut u8], ctx: &AudioContext) {}
}
