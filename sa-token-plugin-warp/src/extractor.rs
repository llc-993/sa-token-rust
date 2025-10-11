//! Warp 提取器

use crate::filter::TokenData;
use sa_token_core::token::TokenValue;

/// Token 提取器 - 从 TokenData 中提取
pub struct SaTokenExtractor(pub TokenValue);

impl From<TokenData> for Result<SaTokenExtractor, warp::Rejection> {
    fn from(data: TokenData) -> Self {
        if let Some(token) = data.token {
            Ok(SaTokenExtractor(token))
        } else {
            Err(warp::reject::custom(crate::filter::UnauthorizedError))
        }
    }
}

/// 可选 Token 提取器
pub struct OptionalSaTokenExtractor(pub Option<TokenValue>);

impl From<TokenData> for OptionalSaTokenExtractor {
    fn from(data: TokenData) -> Self {
        OptionalSaTokenExtractor(data.token)
    }
}

/// LoginId 提取器
pub struct LoginIdExtractor(pub String);

impl From<TokenData> for Result<LoginIdExtractor, warp::Rejection> {
    fn from(data: TokenData) -> Self {
        if let Some(login_id) = data.login_id {
            Ok(LoginIdExtractor(login_id))
        } else {
            Err(warp::reject::custom(crate::filter::UnauthorizedError))
        }
    }
}

