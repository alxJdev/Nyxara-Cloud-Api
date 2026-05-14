use std::ops::DerefMut;
use actix_web::{post, web, HttpResponse, Responder};
use ormlite::Model;
use serde::{Deserialize, Serialize};
use crate::models::user::User;
use crate::services::auth_service::AuthService;
use crate::services::db_service::DbService;
use crate::services::hash_service::HashService;
use crate::types::errors::errors::{AppError, DbSelectError};
use crate::types::jwt_data::JwtClaim;
use crate::types::request_header::RequestHeader;
use crate::types::response_header::ResponseHeader;
use crate::validate_jwt;

#[post("/user/data")]
pub async fn user_data(req: web::Json<Request>,
                       db_service: web::Data<DbService>,
                       hash_service: web::Data<HashService>,
                       auth_service: web::Data<AuthService>) -> impl Responder {
    let refresh_jwt = validate_jwt!(req, db_service, hash_service, auth_service, &JwtClaim::UserData);
    let user_id = match auth_service.get_user_id(&req.header.jwt).await {
        Ok(user_id) => user_id,
        Err(e) => return HttpResponse::Ok().json(Response::new_error(e, refresh_jwt)),
    };
    let mut conn = match db_service.provide_connection().await {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::Ok().json(Response::new_error(e, refresh_jwt)),
    };
    let mut user = match User::select().where_("id = ?")
        .bind(user_id)
        .fetch_one(conn.deref_mut()).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Ok().json(Response::new_error(DbSelectError::new(), refresh_jwt))
    };
    user.password_hash = String::new();
    let res = Response {
        header: ResponseHeader {
            error: false,
            error_msg: "".to_string(),
            refresh_jwt,
        },
        user,
    };
    HttpResponse::Ok().json(res)
}

#[derive(Deserialize)]
struct Request {
    header: RequestHeader,
}

#[derive(Serialize)]
struct Response {
    header: ResponseHeader,
    user: User,
}

impl Response {
    fn new_error(err: Box<dyn AppError>, jwt: String) -> Self {
        Response {
            header: ResponseHeader::new_error(err, jwt),
            user: User {
                id: "".to_string(),
                username: "".to_string(),
                password_hash: "".to_string(),
                claims: "".to_string(),
            },
        }
    }
}