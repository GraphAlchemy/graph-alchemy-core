use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node<Meta> {
    pub(crate) id: usize,
    pub(crate) label: String,
    pub(crate) metadata: Meta,
}
