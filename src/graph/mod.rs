pub mod error;
pub mod node_id;
pub mod topological_sort;

use crate::{
    data_types::AudioContext,
    graph::{error::GraphError, node_id::NodeID},
    node::Node,
};
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Graph {
    // --- GRAPH STRUCTURE ---
    nodes: HashMap<NodeID, Box<dyn Node>>,
    edges: Vec<(NodeID, usize, NodeID, usize)>,
    adjacency: HashMap<NodeID, Vec<NodeID>>,
    input_id: NodeID,
    output_id: NodeID,

    // --- PROCESSING DATA ---
    sorted_nodes: Vec<NodeID>,
    output_buffers: HashMap<(NodeID, usize), Vec<u8>>,
    // Pointers to the edge buffer in the input order
    node_inputs: HashMap<NodeID, Vec<*const u8>>,
    node_outputs: HashMap<NodeID, Vec<*mut u8>>,
    zero_buffer: Vec<u8>,

    // --- CONFIGURATIONS ---
    /// The current audio context.
    audio_ctx: AudioContext,

    // --- MISC ---
    next_node_id: usize,
}

impl Graph {
    // --- INITIALIZATION ---

    /// Creates a new Graph instance with the given input and output node..
    pub fn new(
        input_node: Box<dyn Node>,
        output_node: Box<dyn Node>,
        audio_ctx: AudioContext,
    ) -> Self {
        let mut graph = Graph {
            audio_ctx,
            ..Default::default()
        };
        // Register the input and output nodes
        let input_id = graph.add_node(input_node);
        let output_id = graph.add_node(output_node);
        graph.input_id = input_id;
        graph.output_id = output_id;
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

    // --- EDGE GETTING ---

    pub fn get_edges(&self) -> &Vec<(NodeID, usize, NodeID, usize)> {
        &self.edges
    }

    // --- EDGE MANIPULATION ---

    pub fn add_edge(&mut self, from: NodeID, out_idx: usize, to: NodeID, in_idx: usize) {
        self.edges.push((from, out_idx, to, in_idx));
    }

    // --- NODE GETTING ---

    pub fn get_input_id(&self) -> NodeID {
        self.input_id
    }

    pub fn get_output_id(&self) -> NodeID {
        self.output_id
    }

    pub fn get_node_map(&self) -> &HashMap<NodeID, Box<dyn Node>> {
        &self.nodes
    }

    pub fn get_node_map_mut(&mut self) -> &mut HashMap<NodeID, Box<dyn Node>> {
        &mut self.nodes
    }

    // --- NODE MANIPULATION ---

    pub fn set_input_id(&mut self, id: NodeID) {
        self.input_id = id;
    }

    pub fn set_output_id(&mut self, id: NodeID) {
        self.output_id = id;
    }

    /// Adds a new node to the graph, and returns the newly generated node ID.
    pub fn add_node(&mut self, mut node: Box<dyn Node>) -> NodeID {
        let id = self.generate_node_id();
        // Update the node
        node.update(&self.audio_ctx);
        // Insert the node to the map
        self.nodes.insert(id, node);
        id
    }

    /// Adds a new node to the graph with the given ID.
    pub fn add_node_with_id(&mut self, id: NodeID, mut node: Box<dyn Node>) {
        // Update the node
        node.update(&self.audio_ctx);
        // Insert the node to the map
        self.nodes.insert(id, node);
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

    // --- AUDIO CONTEXT UPDATING ---

    /// Sets the audio context to the new one.
    pub fn set_audio_ctx(&mut self, audio_ctx: &AudioContext) {
        self.audio_ctx = audio_ctx.clone();

        // Call update functions for every nodes
        for node in self.nodes.values_mut() {
            node.update(audio_ctx);
        }
    }

    // --- GRAPH PROCESSING ---

    fn allocate_output_buffer(
        node_id: &NodeID,
        node: &dyn Node,
        output_buffers: &mut HashMap<(NodeID, usize), Vec<u8>>,
        node_outputs: &mut HashMap<NodeID, Vec<*mut u8>>,
        audio_ctx: &AudioContext,
    ) -> Result<(), GraphError> {
        // Create a buffer for all outputs
        for output_index in 0..node.get_output_len() {
            let output_type = node
                .get_output_type(output_index)
                .ok_or(GraphError::OutputTypeUnavailable(*node_id, output_index))?;
            let buffer = vec![0u8; output_type.size * audio_ctx.buffer_size];

            // Insert the output buffer to the output_buffers
            output_buffers.insert((*node_id, output_index), buffer);

            // Register the pointer to the buffer in the node_outputs map
            let ptr = output_buffers
                .get_mut(&(*node_id, output_index))
                .unwrap()
                .as_mut_ptr();
            node_outputs.entry(*node_id).or_default().push(ptr);
        }

        Ok(())
    }

    /// Prepares the graph for processing. The host must call this function before start processing, or it may lead to undefined behavior.
    pub fn prepare(&mut self) -> Result<(), GraphError> {
        // First sort the graph
        self.sort_graph()?;

        // Allocate output buffer for the input node
        if let Some(input_node) = self.nodes.get_mut(&self.input_id) {
            Self::allocate_output_buffer(
                &self.input_id,
                input_node.as_ref(),
                &mut self.output_buffers,
                &mut self.node_outputs,
                &self.audio_ctx,
            )?;
        }

        for node_id in &self.sorted_nodes {
            if let Some(node) = self.nodes.get_mut(node_id) {
                // Call prepare function for every nodes
                node.prepare().map_err(GraphError::NodeError)?;

                Self::allocate_output_buffer(
                    node_id,
                    node.as_ref(),
                    &mut self.output_buffers,
                    &mut self.node_outputs,
                    &self.audio_ctx,
                )?;
            }
        }

        // Calculate the max buffer size and create a zero buffer
        let mut max_size = 4usize;
        for (node_id, node) in &self.nodes {
            for i in 0..node.get_input_len() {
                let type_info = node
                    .get_input_type(i)
                    .ok_or(GraphError::InputTypeUnavailable(*node_id, i))?;
                max_size = max_size.max(type_info.size);
            }
        }
        self.zero_buffer = vec![0u8; max_size * self.audio_ctx.buffer_size];

        // Build node_inputs from edges
        for edge in &self.edges {
            let ptr = self.output_buffers.get(&(edge.0, edge.1)).unwrap().as_ptr();
            self.node_inputs.entry(edge.2).or_insert_with(|| {
                vec![self.zero_buffer.as_ptr(); self.nodes[&edge.2].get_input_len()]
            })[edge.3] = ptr;
        }

        Ok(())
    }

    /// Processes the graph in the sorted order and writes the result in the output pointer.
    /// The host must pass the audio context which is as the same as the one given in the `set_audio_ctx` function.
    pub fn process(&mut self, inputs: &[*const u8], outputs: &[*mut u8]) {
        // Get the pointer to the output buffer of the input node
        let Some(output_buffers) = self.get_output_ptr(&self.input_id) else {
            return;
        };
        let input_node = self.nodes.get_mut(&self.input_id).unwrap();
        // Process the input node
        input_node.process(inputs, &output_buffers, &self.audio_ctx);

        for node_id in self.sorted_nodes.clone() {
            // Get the pointer to the input buffer of the node
            let Some(input_buffers) = self.get_input_ptr(&node_id) else {
                return;
            };
            // Get the pointer to the output buffer of the node
            let Some(output_buffers) = self.get_output_ptr(&node_id) else {
                return;
            };

            // Pass the pointers and process
            if let Some(node) = self.nodes.get_mut(&node_id) {
                node.process(&input_buffers, &output_buffers, &self.audio_ctx);
            }
        }

        // Get the pointer to the input buffer of the output node
        let Some(input_buffers) = self.get_input_ptr(&self.output_id) else {
            return;
        };
        let output_node = self.nodes.get_mut(&self.output_id).unwrap();
        // Process the output node
        // Output data will be written to the output pointer
        output_node.process(&input_buffers, outputs, &self.audio_ctx);
    }

    fn get_output_ptr(&self, from: &NodeID) -> Option<Vec<*mut u8>> {
        self.node_outputs.get(from).cloned()
    }

    fn get_input_ptr(&self, to: &NodeID) -> Option<Vec<*const u8>> {
        self.node_inputs.get(to).cloned()
    }
}

unsafe impl Send for Graph {}
