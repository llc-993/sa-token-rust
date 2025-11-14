use salvo::{Depot, Request, Response, Handler, FlowCtrl};
use sa_token_core::{token::TokenValue, SaTokenContext};
use crate::state::SaTokenState;
use std::sync::Arc;
use sa_token_adapter::utils::{parse_cookies, parse_query_string, extract_bearer_token as utils_extract_bearer_token};

#[derive(Clone)]
pub struct SaTokenLayer {
    state: SaTokenState,
}

impl SaTokenLayer {
    pub fn new(state: SaTokenState) -> Self {
        Self { state }
    }
}

#[salvo::async_trait]
impl Handler for SaTokenLayer {
    async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
        let mut ctx = SaTokenContext::new();
        
        if let Some(token_str) = extract_token_from_request(req, &self.state) {
            tracing::debug!("Sa-Token: extracted token from request: {}", token_str);
            let token = TokenValue::new(token_str);
            
            if self.state.manager.is_valid(&token).await {
                depot.insert("sa_token", token.clone());
                
                if let Ok(token_info) = self.state.manager.get_token_info(&token).await {
                    let login_id = token_info.login_id.clone();
                    depot.insert("sa_login_id", login_id.clone());
                    
                    ctx.token = Some(token.clone());
                    ctx.token_info = Some(Arc::new(token_info));
                    ctx.login_id = Some(login_id);
                }
            }
        }
        
        SaTokenContext::set_current(ctx);
        ctrl.call_next(req, depot, res).await;
        SaTokenContext::clear();
    }
}

/// 中文 | English
/// 从请求中提取 token | Extract token from request
///
/// 按以下顺序尝试提取 token: | Try to extract token in the following order:
/// 1. 从指定名称的请求头 | From specified header name
/// 2. 从 Authorization 请求头 | From Authorization header
/// 3. 从 Cookie | From cookie
/// 4. 从查询参数 | From query parameter
pub fn extract_token_from_request(req: &Request, state: &SaTokenState) -> Option<String> {
    let token_name = &state.manager.config.token_name;
    
    // 1. 从指定名称的请求头提取 | Extract from specified header name
    if let Some(header_value) = req.headers().get(token_name) {
        if let Ok(value_str) = header_value.to_str() {
            if !value_str.is_empty() {
                if let Some(token) = utils_extract_bearer_token(value_str) {
                    return Some(token);
                }
            }
        }
    }
    
    // 2. 从 Authorization 请求头提取 | Extract from Authorization header
    if let Some(auth_header) = req.headers().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if !auth_str.is_empty() {
                if let Some(token) = utils_extract_bearer_token(auth_str) {
                    return Some(token);
                }
            }
        }
    }
    
    // 3. 从 Cookie 提取 | Extract from cookie
    if let Some(cookie_header) = req.headers().get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            let cookies = parse_cookies(cookie_str);
            if let Some(token) = cookies.get(token_name) {
                if !token.is_empty() {
                    return Some(token.to_string());
                }
            }
        }
    }
    
    // 4. 从查询参数提取 | Extract from query parameter
    if let Some(query) = req.uri().query() {
        let params = parse_query_string(query);
        if let Some(token) = params.get(token_name) {
            if !token.is_empty() {
                return Some(token.to_string());
            }
        }
    }
    
    None
}
