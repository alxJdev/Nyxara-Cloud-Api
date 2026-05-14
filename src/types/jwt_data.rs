use ormlite::types::JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtData {
    pub sub: String,
    pub name: String,
    pub exp: i64,
    pub iat: i64,
    pub claims: JwtClaims,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct JwtClaims {
    pub claims: Vec<JwtClaim>,
}

impl JwtClaims {
    pub fn new() -> JwtClaims {
        JwtClaims {
            claims: Vec::new(),
        }
    }

    pub fn add(&mut self, claim: &JwtClaim) -> Self {
        if !self.claims.contains(&claim) {
            self.claims.push(claim.clone());
        }
        self.clone()
    }

    pub fn contains(&self, claim: &JwtClaim) -> bool {
        if claim == &JwtClaim::None {
            return true;
        }
        if self.claims.contains(&JwtClaim::Admin) {
            return true;
        }
        self.claims.contains(&claim)
    }

    pub fn resolve_claim(claim: &JwtClaim) -> JwtClaim {
        match claim {
            JwtClaim::UserRegister => JwtClaim::Admin,
            JwtClaim::UserLogin => JwtClaim::Anonymous,
            _ => claim.clone(),
        }
    }
}

pub struct JwtResult {
    refresh: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub enum JwtClaim {
    Anonymous,
    None,
    Admin,
    UserRegister,
    UserLogin,
    UserData,
}