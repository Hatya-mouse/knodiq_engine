use crate::graph::node_id::NodeID;

#[derive(Debug)]
pub enum GraphError {
    InputNotFound(NodeID, String),
    OutputNotFound(NodeID, String),
    NodeCycle(NodeID),
    OutputTypeUnavailable(NodeID, usize),
    InputTypeUnavailable(NodeID, usize),
}
