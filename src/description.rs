use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Description {
    pub id: u64,
    pub time_slice_id: u64,
    pub description: String,
}

impl Description {
    pub fn new(id: u64, time_slice_id: u64, desc: &str) -> Self {
        Self {
            id,
            time_slice_id,
            description: desc.to_string(),
        }
    }
}
