//! Actix-web提取器

use actix_web::{FromRequest, HttpRequest, dev::Payload};
use std::future::{ready, Ready};
use sa_token_core::token::TokenValue;

/// Token提取器
pub struct SaTokenExtractor(pub Option<TokenValue>);

impl FromRequest for SaTokenExtractor {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let token = req.headers().get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| {
                if v.starts_with("Bearer ") {
                    Some(TokenValue::new(v[7..].to_string()))
                } else {
                    Some(TokenValue::new(v.to_string()))
                }
            });
        
        ready(Ok(SaTokenExtractor(token)))
    }
}

