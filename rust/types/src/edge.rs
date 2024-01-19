use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FromToEdge<Meta> {
    pub(crate) from_id: usize,
    pub(crate) to_id: usize,
    #[serde(default)]
    #[serde(skip_deserializing)]
    pub(crate) metadata: Meta,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FromToUncompressedEdge<Meta> {
    pub(crate) to: String,
    pub(crate) from: String,
    pub(crate) metadata: Meta,
}
