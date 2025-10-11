//! Rocket Request Guards (提取器)

use rocket::request::{FromRequest, Request, Outcome};
use rocket::http::Status;
use sa_token_core::token::TokenValue;

/// Token 守卫 - 必须存在，否则返回错误
pub struct SaTokenGuard(pub TokenValue);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SaTokenGuard {
    type Error = ();
    
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(token) = request.local_cache(|| None::<TokenValue>) {
            if let Some(token) = token {
                return Outcome::Success(SaTokenGuard(token.clone()));
            }
        }
        
        Outcome::Error((Status::Unauthorized, ()))
    }
}

/// 可选 Token 守卫 - 不存在也不报错
pub struct OptionalSaTokenGuard(pub Option<TokenValue>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for OptionalSaTokenGuard {
    type Error = ();
    
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.local_cache(|| None::<TokenValue>).clone();
        Outcome::Success(OptionalSaTokenGuard(token))
    }
}

/// LoginId 守卫 - 直接获取登录用户的 ID
pub struct LoginIdGuard(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoginIdGuard {
    type Error = ();
    
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(login_id) = request.local_cache(|| None::<String>) {
            if let Some(login_id) = login_id {
                return Outcome::Success(LoginIdGuard(login_id.clone()));
            }
        }
        
        Outcome::Error((Status::Unauthorized, ()))
    }
}

