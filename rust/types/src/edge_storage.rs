use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdjacencyLists<Edge> {
    /// Vector of N (number of vertexes) elements contains indices of adjacent edges stored in `edges`.
    pub(crate) adj_lists: Vec<Vec<usize>>,
    pub(crate) edges: Vec<Edge>,
}
