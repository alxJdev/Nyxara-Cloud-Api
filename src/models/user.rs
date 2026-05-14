use ormlite::Model;
use serde::Serialize;
use crate::types::jwt_data::JwtClaims;

#[derive(Model, Debug, Serialize)]
#[ormlite(table = "User")]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub claims: String,
}