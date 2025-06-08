// connector.rs
// Represents the connection between nodes.
// © 2025 Shuntaro Kasatani

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
