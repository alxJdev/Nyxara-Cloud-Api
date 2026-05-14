use serde::Serialize;
use crate::types::errors::errors::AppError;

#[derive(Serialize)]
pub struct ResponseHeader {
    pub error: bool,
    pub error_msg: String,
    pub refresh_jwt: String,
}

impl ResponseHeader {
    pub fn new() -> Self {
        ResponseHeader {
            error: false,
            error_msg: String::new(),
            refresh_jwt: String::new(),
        }
    }
    pub fn new_error(err: Box<dyn AppError>, jwt: String) -> Self {
        ResponseHeader {
            error: true,
            error_msg: err.get_message(),
            refresh_jwt: jwt,
        }
    }
}