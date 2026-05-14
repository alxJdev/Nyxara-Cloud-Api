use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestHeader {
    pub jwt: String,
}