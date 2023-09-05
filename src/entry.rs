use exaworld_dependencies::MAIN_SEPARATOR;
use serde::{Deserialize, Serialize};

use crate::ModelType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: u64,
    pub timestamp_added: u128,
    pub timestamp_changed: u128,
    pub model_type: ModelType,
}

impl Entry {
    pub fn to_data_row_string(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}",
            self.id,
            MAIN_SEPARATOR,
            self.timestamp_added,
            MAIN_SEPARATOR,
            self.timestamp_changed,
            MAIN_SEPARATOR,
            self.model_type.to_model_string()
        )
    }
}
