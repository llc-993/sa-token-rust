// Author: 金书记
//
//! Axum提取器

use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use sa_token_core::token::TokenValue;
use std::convert::Infallible;

/// Token提取器
pub struct SaTokenExtractor(pub TokenValue);

impl<S> FromRequestParts<S> for SaTokenExtractor
where
    S: Send + Sync,
{
    type Rejection = Infallible;
    
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 从请求扩展中获取 token（由中间件设置）
        let token = parts.extensions.get::<TokenValue>()
            .cloned()
            .unwrap_or_else(|| TokenValue::new(String::new()));
        
        Ok(SaTokenExtractor(token))
    }
}
