use crate::graph::node_id::NodeID;

pub enum GraphError {
    InputNotFound(NodeID, String),
    OutputNotFound(NodeID, String),
    NodeCycle(NodeID),
}
