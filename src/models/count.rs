use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CountJoinInput {
    pub values: Vec<usize>
}
