// Author: 金书记
//
//! Rocket Request Guards (提取器)

use rocket::request::{FromRequest, Request, Outcome};
use rocket::http::Status;
use rocket::http::ContentType;
use rocket::response::{self, Responder};
use sa_token_core::{token::TokenValue, error::messages};
use serde_json::json;

/// 认证错误响应
#[derive(Debug)]
pub struct AuthError {
    json: String,
}

impl<'r> Responder<'r, 'static> for AuthError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let mut response = rocket::Response::new();
        response.set_header(ContentType::JSON);
        response.set_status(Status::Unauthorized);
        response.set_sized_body(self.json.len(), std::io::Cursor::new(self.json));
        Ok(response)
    }
}

/// Token 守卫 - 必须存在，否则返回错误
pub struct SaTokenGuard(pub TokenValue);

impl SaTokenGuard {
    pub fn token(&self) -> &TokenValue {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SaTokenGuard {
    type Error = AuthError;
    
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.local_cache(|| None::<TokenValue>);
        if let Some(token) = token {
            return Outcome::Success(SaTokenGuard(token.clone()));
        }
        
        let error = json!({
            "code": 401,
            "message": messages::AUTH_ERROR
        }).to_string();
        
        Outcome::Error((Status::Unauthorized, AuthError { json: error }))
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

impl LoginIdGuard {
    pub fn login_id(&self) -> &str {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoginIdGuard {
    type Error = AuthError;
    
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let login_id = request.local_cache(|| None::<String>);
        if let Some(login_id) = login_id {
            return Outcome::Success(LoginIdGuard(login_id.clone()));
        }
        
        let error = json!({
            "code": 401,
            "message": messages::AUTH_ERROR
        }).to_string();
        
        Outcome::Error((Status::Unauthorized, AuthError { json: error }))
    }
}
