use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Student {
    pub _id: u32,
    pub name: String,
    pub gpa: f64,
}

impl Student {
    pub fn new(_id: u32, name: String, gpa: f64) -> Self {
        Self { _id, name, gpa }
    }
}