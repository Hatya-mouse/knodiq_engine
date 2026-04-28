use crate::graph::node_id::NodeID;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum GraphError {
    NodeError(Box<dyn NodeError>),
    OutputBufferNotFound(NodeID, usize),
    NodeCycle(NodeID),
    OutputTypeUnavailable(NodeID, usize),
    InputTypeUnavailable(NodeID, usize),
    NodeTypeMismatch((NodeID, usize, NodeID, usize)),
    EdgeNotFound((NodeID, usize, NodeID, usize)),
}

pub trait NodeError: Send + Debug + Display {}
