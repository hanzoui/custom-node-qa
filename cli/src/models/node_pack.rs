use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePack {
    pub name: String,
    pub node_count: usize,
    pub tested: bool,
}

impl NodePack {
    pub fn new(name: String, node_count: usize, tested: bool) -> Self {
        Self {
            name,
            node_count,
            tested,
        }
    }
}
