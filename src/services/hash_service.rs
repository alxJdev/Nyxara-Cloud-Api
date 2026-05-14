use uuid::Uuid;
use crate::types::errors::errors::{AppError, HashError};

pub struct HashService {
    bcrypt_cost: u32,
}

impl HashService {
    pub fn new() -> HashService {
        HashService {
            bcrypt_cost: 12,
        }
    }

    pub fn hash_password(&self, pwd: &String) ->Result<String, Box<dyn AppError>> {
        let hash = match bcrypt::hash(&pwd, self.bcrypt_cost) {
            Ok(hash) => hash,
            Err(_) => return Err(HashError::new()),
        };
        let valid = bcrypt::verify(&pwd, &hash).unwrap();
        Ok(hash)
    }

    pub fn verify_password(&self, pwd: &String, hash: &String) -> Result<bool, Box<dyn AppError>> {
        let valid = match bcrypt::verify(&pwd, &hash) {
            Ok(valid) => valid,
            Err(_) => return Err(HashError::new()),
        };
        Ok(valid)
    }

    pub fn create_fingerprint(&self, str: &String) -> String {
        let hash = blake3::hash(str.as_bytes()).to_hex().to_string();
        hash
    }
}