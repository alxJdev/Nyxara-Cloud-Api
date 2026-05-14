use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CoreConfig {
    pub appdata_path: String,
}

impl CoreConfig {
    pub fn new() -> Self {
        CoreConfig {
            appdata_path: "".to_string(),
        }
    }
}