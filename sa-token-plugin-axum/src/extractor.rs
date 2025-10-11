//! Axum提取器

use axum::{
    async_trait,
    extract::{FromRequestParts, rejection::TypedHeaderRejection},
    http::request::Parts,
};
use sa_token_core::token::TokenValue;

/// Token提取器
pub struct SaTokenExtractor(pub Option<TokenValue>);

#[async_trait]
impl<S> FromRequestParts<S> for SaTokenExtractor
where
    S: Send + Sync,
{
    type Rejection = TypedHeaderRejection;
    
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 从header中提取token
        let token = parts.headers.get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| {
                if v.starts_with("Bearer ") {
                    Some(TokenValue::new(v[7..].to_string()))
                } else {
                    Some(TokenValue::new(v.to_string()))
                }
            });
        
        Ok(SaTokenExtractor(token))
    }
}

