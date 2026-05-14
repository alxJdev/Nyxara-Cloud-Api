use std::ops::DerefMut;
use actix_web::{post, web, HttpResponse, Responder};
use ormlite::Model;
use serde::{Deserialize, Serialize};
use crate::models::user::User;
use crate::models::user_token::UserToken;
use crate::services::auth_service::AuthService;
use crate::services::db_service::DbService;
use crate::services::hash_service::HashService;
use crate::types::errors::errors::{AppError, DbInsertError, DbSelectError, UsernameOrPasswordError};
use crate::types::jwt_data::JwtClaim;
use crate::types::request_header::RequestHeader;
use crate::types::response_header::ResponseHeader;
use crate::validate_jwt;

#[post("/user/login")]
pub async fn user_login(req: web::Json<Request>,
                        db_service: web::Data<DbService>,
                        hash_service: web::Data<HashService>,
                        auth_service: web::Data<AuthService>) -> impl Responder {
    let refresh_jwt = validate_jwt!(req, db_service, hash_service, auth_service, &JwtClaim::UserLogin);
    let mut conn = db_service.pool.acquire().await.unwrap();
    let user = match User::select()
        .where_("username = ?").bind(&req.username)
        .fetch_one(conn.deref_mut()).await {
        Ok(user) => user,
        Err(e) => {return HttpResponse::Ok().json(Response::new_error(DbSelectError::new(), refresh_jwt))}
    };
    let password_valid = match hash_service.verify_password(&req.password, &user.password_hash) {
        Ok(valid) => valid,
        Err(e) => {return HttpResponse::Ok().json(Response::new_error(e, refresh_jwt))}
    };
    if !password_valid {
        return HttpResponse::Ok().json(Response::new_error(UsernameOrPasswordError::new(), refresh_jwt));
    }
    let jwt = match auth_service.create_token(&user) {
        Ok(jwt) => jwt,
        Err(e) => {return HttpResponse::Ok().json(Response::new_error(e, refresh_jwt))}
    };
    let user_token = UserToken {
        id: db_service.create_uuid(),
        token_hash: hash_service.create_fingerprint(&jwt),
        user_id: user.id.clone(),
        locked: false,
    };
    let mut conn = db_service.pool.acquire().await.unwrap();
    let user_token = match user_token.insert(&mut conn).await {
        Ok(user_token) => user_token,
        Err(e) => {return HttpResponse::Ok().json(Response::new_error(DbInsertError::new(), refresh_jwt))}
    };
    let res = Response{
        header: ResponseHeader::new(),
        jwt,
    };
    HttpResponse::Ok().json(res)
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
    jwt: String,
}

impl Response {
    fn new_error(err: Box<dyn AppError>, jwt: String) -> Self {
        Response {
            header: ResponseHeader::new_error(err, jwt),
            jwt: String::new(),
        }
    }
}