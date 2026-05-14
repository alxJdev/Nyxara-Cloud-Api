use crate::models::user::User;
use crate::models::user_token::UserToken;
use crate::services::db_service::DbService;
use crate::services::hash_service::HashService;
use crate::types::errors::errors::{AppError, DbInsertError, DbSelectError, DbTransactionError, DbUpdateError, JsonError, JwtError};
use crate::types::jwt_data::{JwtClaim, JwtClaims, JwtData};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use ormlite::model::ModelBuilder;
use ormlite::{Connection, Model};
use std::ops::{Add, DerefMut, Sub};

pub struct AuthService {
    jwt_secret: String,
}

impl AuthService {
    pub fn new() -> AuthService {
        AuthService {
            jwt_secret: "test".to_string(),
        }
    }

    pub fn create_token(&self, user: &User) -> Result<String, Box<dyn AppError>> {
        let now = Utc::now().timestamp();
        let exp = Utc::now().add(chrono::Duration::days(5)).timestamp();
        let claims: JwtClaims = match serde_json::from_str(&user.claims.as_str()) {
            Ok(claims) => claims,
            Err(_) => {return Err(JsonError::new())}
        };
        let jwt_data = JwtData {
            sub: user.id.clone(),
            name: user.username.clone(),
            exp,
            iat: now,
            claims,
        };
        let header = Header::default();
        let token = match encode(&header, &jwt_data, &EncodingKey::from_secret(self.jwt_secret.as_ref())) {
            Ok(token) => token,
            Err(_) => return Err(JwtError::new()),
        };
        Ok(token)
    }

    pub async fn validate_token(&self, jwt: &String,
                                db_service: &DbService,
                                hash_service: &HashService,
                                required_claim: &JwtClaim) -> Result<String, Box<dyn AppError>> {
        let required_claim = JwtClaims::resolve_claim(required_claim);
        if &required_claim == &JwtClaim::Anonymous {
            return Ok(String::new());
        }
        let mut conn = match db_service.provide_connection().await {
            Ok(conn) => conn,
            Err(e) => return Err(e),
        };
        let user_token = match UserToken::select()
            .where_("token_hash = ?").bind(hash_service.create_fingerprint(jwt))
            .fetch_one(conn.deref_mut()).await {
            Ok(user_token) => user_token,
            Err(_) => return Err(DbSelectError::new())
        };
        if user_token.locked {
            return Err(JwtError::new());
        }
        let jwt_data: JwtData = match decode(jwt, &DecodingKey::from_secret(self.jwt_secret.as_ref()), &Validation::default()) {
            Ok(data) => data.claims,
            Err(_) => return Err(JwtError::new()),
        };
        if !jwt_data.claims.contains(&required_claim) {
            return Err(JwtError::new());
        }
        let refresh_jwt = match self.refresh_token(&jwt_data, &user_token, &db_service, &hash_service).await {
            Ok(refresh_jwt) => refresh_jwt,
            Err(e) => return Err(e),
        };
        Ok(refresh_jwt)
    }

    async fn refresh_token(&self,
                           jwt_data: &JwtData,
                           user_token: &UserToken,
                           db_service: &DbService,
                           hash_service: &HashService) -> Result<String, Box<dyn AppError>> {
        let mut conn = match db_service.provide_connection().await {
            Ok(conn) => conn,
            Err(e) => return Err(e),
        };
        let refresh_date = Utc::now().sub(chrono::Duration::days(2)).timestamp();
        if jwt_data.iat > refresh_date {
            return Ok(String::new())
        }
        let user = match User::select()
            .where_("id = ?").bind(&jwt_data.sub)
            .fetch_one(conn.deref_mut()).await {
            Ok(user) => user,
            Err(_) => return Err(DbSelectError::new())
        };
        let refresh_jwt = match self.create_token(&user) {
            Ok(refresh_jwt) => refresh_jwt,
            Err(e) => return Err(e),
        };
        let mut trans = match conn.begin().await {
            Ok(trans) => trans,
            Err(_) => return Err(DbTransactionError::new())
        };
        match user_token.update_partial()
            .locked(true)
            .update(trans.deref_mut()).await {
            Ok(_) => {}
            Err(_) => return Err(DbUpdateError::new())
        };
        let user_token = UserToken {
            id: db_service.create_uuid(),
            token_hash: hash_service.create_fingerprint(&refresh_jwt),
            user_id: user.id.clone(),
            locked: false,
        };
        match user_token.insert(&mut trans).await {
            Ok(_) => {}
            Err(_) => return Err(DbInsertError::new()),
        };
        match trans.commit().await {
            Ok(_) => {}
            Err(_) => return Err(DbTransactionError::new())
        };
        Ok(refresh_jwt)
    }

    pub async fn get_user_id(&self, jwt: &String) -> Result<String, Box<dyn AppError>> {
        let jwt_data: JwtData = match decode(jwt, &DecodingKey::from_secret(self.jwt_secret.as_ref()), &Validation::default()) {
            Ok(data) => data.claims,
            Err(_) => return Err(JwtError::new()),
        };
        let user_id = jwt_data.sub.clone();
        Ok(user_id)
    }
}