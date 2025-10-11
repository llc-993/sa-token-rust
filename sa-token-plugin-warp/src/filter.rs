// Author: 金书记
//
//! Warp Filter (中间件)

use warp::{Filter, Rejection, http::HeaderMap};
use crate::SaTokenState;
use sa_token_core::token::TokenValue;

/// Token 数据，存储在请求中
#[derive(Clone)]
pub struct TokenData {
    pub token: Option<TokenValue>,
    pub login_id: Option<String>,
}

/// sa-token 基础过滤器 - 提取并验证 token
pub fn sa_token_filter(
    state: SaTokenState,
) -> impl Filter<Extract = (TokenData,), Error = Rejection> + Clone {
    warp::any()
        .and(warp::header::headers_cloned())
        .and(warp::cookie::optional::<String>("satoken"))
        .and(warp::query::<std::collections::HashMap<String, String>>().or_else(|_| async {
            Ok::<(std::collections::HashMap<String, String>,), Rejection>((std::collections::HashMap::new(),))
        }))
        .and(warp::any().map(move || state.clone()))
        .and_then(extract_and_validate_token)
}

/// sa-token 登录检查过滤器 - 强制要求登录
pub fn sa_check_login_filter(
    state: SaTokenState,
) -> impl Filter<Extract = (TokenData,), Error = Rejection> + Clone {
    sa_token_filter(state)
        .and_then(|token_data: TokenData| async move {
            if token_data.token.is_some() && token_data.login_id.is_some() {
                Ok(token_data)
            } else {
                Err(warp::reject::custom(UnauthorizedError))
            }
        })
}

/// 提取并验证 token
async fn extract_and_validate_token(
    headers: HeaderMap,
    cookie_token: Option<String>,
    query: std::collections::HashMap<String, String>,
    state: SaTokenState,
) -> Result<TokenData, Rejection> {
    let token_name = &state.manager.config.token_name;
    
    // 1. 从 Header 获取
    let token_str = if let Some(header_val) = headers.get(token_name) {
        header_val.to_str().ok().map(|s| extract_bearer_token(s))
    }
    // 2. 从 Cookie 获取
    else if let Some(token) = cookie_token {
        Some(token)
    }
    // 3. 从 Query 参数获取
    else {
        query.get(token_name).cloned()
    };
    
    if let Some(token_str) = token_str {
        let token = TokenValue::new(token_str);
        
        // 验证 token
        if state.manager.is_valid(&token).await {
            // 获取 login_id
            if let Ok(token_info) = state.manager.get_token_info(&token).await {
                return Ok(TokenData {
                    token: Some(token),
                    login_id: Some(token_info.login_id),
                });
            }
        }
    }
    
    Ok(TokenData {
        token: None,
        login_id: None,
    })
}

/// 提取 Bearer token
fn extract_bearer_token(token: &str) -> String {
    if token.starts_with("Bearer ") {
        token[7..].to_string()
    } else {
        token.to_string()
    }
}

/// 未授权错误
#[derive(Debug)]
pub struct UnauthorizedError;

impl warp::reject::Reject for UnauthorizedError {}
