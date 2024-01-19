use serde::{Deserialize, Serialize};

use crate::Node;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DirectedGraph<GraphMeta, NodeMeta, Edges> {
    pub(crate) metadata: GraphMeta,
    pub(crate) nodes: Vec<Node<NodeMeta>>,
    pub(crate) edges: Edges,
}
