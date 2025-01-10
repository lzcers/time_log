use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: u128,
    pub name: String,
    pub color: Option<String>,
}

impl Tag {
    pub fn new(name: &str, color: Option<&str>) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
            color: color.map(|c| c.to_string()),
        }
    }
}
