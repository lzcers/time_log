use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Description {
    pub time_slice_id: u64,
    pub description: String,
}

impl Description {
    pub fn new(time_slice_id: u64, desc: &str) -> Self {
        Self {
            time_slice_id,
            description: desc.to_string(),
        }
    }
}
