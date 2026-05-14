use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web::http::StatusCode;
use ormlite::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::user::User;
use crate::services::auth_service::AuthService;
use crate::services::db_service::DbService;
use crate::services::hash_service::HashService;
use crate::types::errors::errors::{AppError, DbInsertError, HashError};
use crate::types::jwt_data::{JwtClaim, JwtClaims};
use crate::types::request_header::RequestHeader;
use crate::types::response_header::ResponseHeader;
use crate::validate_jwt;

#[post("/user/register")]
pub async fn user_register(req: web::Json<Request>,
                           db_service: web::Data<DbService>,
                           hash_service: web::Data<HashService>,
                           auth_service: web::Data<AuthService>) -> impl Responder {
    let refresh_jwt = validate_jwt!(req, db_service, hash_service, auth_service, &JwtClaim::UserRegister);
    let mut conn = db_service.pool.acquire().await.unwrap();
    let password_hash = match hash_service.hash_password(&req.password) {
        Ok(password_hash) => password_hash,
        Err(e) => return HttpResponse::Ok().json(Response::new_error(e, refresh_jwt)),
    };
    let claims = JwtClaims::new().add(&JwtClaim::Admin);
    let claims = serde_json::to_string(&claims).unwrap();
    let user = User{
        id: Uuid::new_v4().to_string(),
        username: req.username.clone(),
        password_hash,
        claims,
    };
    let user = match user.insert(&mut conn).await {
        Ok(user) => user,
        Err(e) => { return HttpResponse::Ok().json(Response::new_error(DbInsertError::new(), refresh_jwt))}
    };

    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct Request {
    header: RequestHeader,
    username: String,
    password: String,
}

#[derive(Serialize)]
struct Response {
    header: ResponseHeader,
}

impl Response {
    fn new_error(err: Box<dyn AppError>, jwt: String) -> Self {
        Response {
            header: ResponseHeader::new_error(err, jwt),
        }
    }
}