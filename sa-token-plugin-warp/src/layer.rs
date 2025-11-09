use warp::{Filter, Rejection};
use sa_token_core::{token::TokenValue, SaTokenContext};
use crate::SaTokenState;
use std::sync::Arc;

/// 创建 Sa-Token 认证层
/// 
/// 这个过滤器会从请求中提取 token，验证有效性，并设置上下文
pub fn sa_token_layer(
    state: SaTokenState,
) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::any()
        .and(warp::header::headers_cloned())
        .and_then(move |headers: warp::http::HeaderMap| {
            let state = state.clone();
            async move {
                let mut ctx = SaTokenContext::new();
                
                if let Some(token_str) = extract_token_from_headers(&headers, &state) {
                    tracing::debug!("Sa-Token: extracted token from request: {}", token_str);
                    let token = TokenValue::new(token_str);
                    
                    if state.manager.is_valid(&token).await {
                        if let Ok(token_info) = state.manager.get_token_info(&token).await {
                            let login_id = token_info.login_id.clone();
                            
                            ctx.token = Some(token.clone());
                            ctx.token_info = Some(Arc::new(token_info));
                            ctx.login_id = Some(login_id);
                        }
                    }
                }
                
                SaTokenContext::set_current(ctx);
                Ok::<(), Rejection>(())
            }
        })
        .untuple_one()
}

/// 从 HTTP 头中提取 token
fn extract_token_from_headers(headers: &warp::http::HeaderMap, state: &SaTokenState) -> Option<String> {
    let token_name = &state.manager.config.token_name;
    
    if let Some(header_value) = headers.get(token_name) {
        if let Ok(value_str) = header_value.to_str() {
            return Some(extract_bearer_token(value_str));
        }
    }
    
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            return Some(extract_bearer_token(auth_str));
        }
    }
    
    if let Some(cookie_header) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            if let Some(token) = parse_cookie(cookie_str, token_name) {
                return Some(token);
            }
        }
    }
    
    None
}

/// 提取 Bearer token
fn extract_bearer_token(header_value: &str) -> String {
    if header_value.starts_with("Bearer ") {
        header_value[7..].trim().to_string()
    } else {
        header_value.trim().to_string()
    }
}

/// 从 cookie 字符串中解析指定名称的 cookie
fn parse_cookie(cookie_str: &str, token_name: &str) -> Option<String> {
    for part in cookie_str.split(';') {
        let part = part.trim();
        if let Some(eq_pos) = part.find('=') {
            let (name, value) = part.split_at(eq_pos);
            if name.trim() == token_name {
                return Some(value[1..].trim().to_string());
            }
        }
    }
    None
}