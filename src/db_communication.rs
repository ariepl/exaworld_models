use serde::{Deserialize, Serialize};
use crate::ModelType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditEntry {
    pub id: u64,
    pub model_type: ModelType,
}
