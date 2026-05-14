use serde::de::Unexpected::Str;

#[macro_export]
macro_rules! validate_jwt {
    ($req:expr, $db:expr, $hash:expr, $auth:expr) => {
        match $auth.validate_token(&$req.header.jwt, &$db, &$hash, &JwtClaim::None).await {
            Ok(auth) => auth,
            Err(e) => return HttpResponse::Ok().json(Response::new_error(e, String::new())),
        }
    };
    ($req:expr, $db:expr, $hash:expr, $auth:expr, $claim:expr) => {
        match $auth.validate_token(&$req.header.jwt, &$db, &$hash, $claim).await {
            Ok(auth) => auth,
            Err(e) => return HttpResponse::Ok().json(Response::new_error(e, String::new())),
        }
    };
}