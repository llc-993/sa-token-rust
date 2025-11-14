// Author: 金书记
//
//! Poem middleware layer for Sa-Token
//! Poem 中间件层，用于 Sa-Token

use poem::{Endpoint, Middleware, Request, Result};
use std::sync::Arc;
use sa_token_core::{token::TokenValue, SaTokenContext};
use sa_token_adapter::utils::{parse_cookies, parse_query_string, extract_bearer_token};
use crate::SaTokenState;

/// Sa-Token layer for Poem framework
/// Poem 框架的 Sa-Token 层
pub struct SaTokenLayer {
    state: SaTokenState,
}

impl SaTokenLayer {
    pub fn new(state: SaTokenState) -> Self {
        Self { state }
    }
}

impl<E> Middleware<E> for SaTokenLayer
where
    E: Endpoint,
{
    type Output = SaTokenMiddleware<E>;

    fn transform(&self, ep: E) -> Self::Output {
        SaTokenMiddleware {
            inner: ep,
            state: self.state.clone(),
        }
    }
}

/// Sa-Token middleware for Poem
/// Poem 的 Sa-Token 中间件
pub struct SaTokenMiddleware<E> {
    inner: E,
    state: SaTokenState,
}

impl<E> Endpoint for SaTokenMiddleware<E>
where
    E: Endpoint,
{
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        let mut ctx = SaTokenContext::new();
        
        // Extract token from request | 从请求中提取 token
        if let Some(token_str) = extract_token_from_request(&req, &self.state) {
            tracing::debug!("Sa-Token: extracted token from request: {}", token_str);
            let token = TokenValue::new(token_str);
            
            // Validate token | 验证 token
            if self.state.manager.is_valid(&token).await {
                // Store token in request extensions | 将 token 存储到请求扩展中
                req.extensions_mut().insert(token.clone());
                
                // Get and store login_id | 获取并存储 login_id
                if let Ok(token_info) = self.state.manager.get_token_info(&token).await {
                    let login_id = token_info.login_id.clone();
                    req.extensions_mut().insert(login_id.clone());
                    
                    // Set context | 设置上下文
                    ctx.token = Some(token.clone());
                    ctx.token_info = Some(Arc::new(token_info));
                    ctx.login_id = Some(login_id);
                }
            }
        }
        
        // Set current context | 设置当前上下文
        SaTokenContext::set_current(ctx);
        
        // Continue processing | 继续处理请求
        let result = self.inner.call(req).await;
        
        // Clear context | 清除上下文
        SaTokenContext::clear();
        
        result
    }
}

/// Extract token from Poem request | 从 Poem 请求中提取 token
fn extract_token_from_request(req: &Request, state: &SaTokenState) -> Option<String> {
    let token_name = &state.manager.config.token_name;
    
    // 1. From header | 从 Header 中获取
    if let Some(header_value) = req.headers().get(token_name) {
        if let Ok(value_str) = header_value.to_str() {
            if let Some(token) = extract_bearer_token(value_str) {
                return Some(token);
            }
        }
    }
    
    // Check Authorization header | 检查 Authorization header
    if let Some(auth_header) = req.headers().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = extract_bearer_token(auth_str) {
                return Some(token);
            }
        }
    }
    
    // 2. From cookie | 从 Cookie 中获取
    if let Some(cookie_header) = req.headers().get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            let cookies = parse_cookies(cookie_str);
            if let Some(token) = cookies.get(token_name) {
                return Some(token.clone());
            }
        }
    }
    
    // 3. From query parameters | 从查询参数中获取
    if let Some(query) = req.uri().query() {
        let params = parse_query_string(query);
        if let Some(token) = params.get(token_name) {
            return Some(token.clone());
        }
    }
    
    None
}
