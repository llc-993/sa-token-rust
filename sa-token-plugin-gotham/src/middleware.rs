// Author: 金书记
//
//! 中间件实现
//!
//! 提供多种中间件：
//! - `SaTokenMiddleware`：基础 token 提取和验证中间件
//! - `SaCheckLoginMiddleware`：检查登录中间件，未登录时返回401错误
//! - `SaCheckPermissionMiddleware`：检查权限中间件，无权限时返回403错误
//! - `SaCheckRoleMiddleware`：检查角色中间件，无角色时返回403错误
//! - `AuthMiddleware`：已废弃，建议使用上述中间件

use gotham::state::{State, StateData};
use gotham::middleware::Middleware;
use gotham::handler::HandlerFuture;
use gotham::hyper::{Response, StatusCode};
use gotham::hyper::body::Body;
use std::pin::Pin;
use std::sync::Arc;
use serde_json::json;
use sa_token_core::{
    error::messages, 
    token::TokenValue, 
    SaTokenContext
};
use sa_token_adapter::utils::{parse_cookies, parse_query_string, extract_bearer_token};
use crate::{SaTokenState, wrapper::{TokenValueWrapper, LoginIdWrapper}};

/// 中文 | English
/// 登录 ID 状态数据 | Login ID state data
#[derive(Clone, StateData)]
pub struct LoginId(pub String);

/// sa-token 基础中间件 - 提取并验证 token
/// 
/// 此中间件会从请求中提取 token，验证其有效性，并将相关信息存储到 State 中
#[derive(Clone)]
pub struct SaTokenMiddleware {
    pub state: SaTokenState,
}

impl SaTokenMiddleware {
    pub fn new(state: SaTokenState) -> Self {
        Self { state }
    }
}

impl Middleware for SaTokenMiddleware {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Pin<Box<HandlerFuture>>
    where
        Chain: FnOnce(State) -> Pin<Box<HandlerFuture>> + Send + 'static,
    {
        let token_state = self.state.clone();
        
        Box::pin(async move {
            let mut ctx = SaTokenContext::new();
            
            // 提取 token
            if let Some(token_str) = extract_token_from_state(&state, &token_state) {
                tracing::debug!("Sa-Token: extracted token from request: {}", token_str);
                let token = TokenValue::new(token_str);
                
                // 验证 token
                if token_state.manager.is_valid(&token).await {
                    // 存储 token 到 State
                    state.put(TokenValueWrapper(token.clone()));
                    
                    // 获取并存储 login_id
                    if let Ok(token_info) = token_state.manager.get_token_info(&token).await {
                        let login_id = token_info.login_id.clone();
                        state.put(LoginIdWrapper(login_id.clone()));
                        
                        // 设置上下文
                        ctx.token = Some(token.clone());
                        ctx.token_info = Some(Arc::new(token_info));
                        ctx.login_id = Some(login_id);
                    }
                }
            }
            
            // 设置当前上下文
            SaTokenContext::set_current(ctx);
            
            // 继续处理请求
            let result = chain(state).await;
            
            // 清除上下文
            SaTokenContext::clear();
            
            result
        })
    }
}

/// 中文 | English
/// 认证中间件 - 验证用户登录状态 | Authentication middleware - verify user login status
/// 
/// 注意：此中间件已废弃，建议使用 SaTokenMiddleware + SaCheckLoginMiddleware
#[deprecated(note = "Use SaTokenMiddleware + SaCheckLoginMiddleware instead")]
#[derive(Clone)]
pub struct AuthMiddleware;

impl AuthMiddleware {
    /// 中文 | English
    /// 创建新的认证中间件 | Create a new authentication middleware
    pub fn new() -> Self {
        Self
    }
}

impl Middleware for AuthMiddleware {
    fn call<Chain>(self, state: State, chain: Chain) -> Pin<Box<HandlerFuture>>
    where
        Chain: FnOnce(State) -> Pin<Box<HandlerFuture>> + Send + 'static,
    {
        // 注意：Gotham 的 State 系统较为复杂
        // 这里提供一个简化实现，用户可以根据需要扩展
        // 建议在 handler 中手动验证 token
        Box::pin(chain(state))
    }
}

impl Default for AuthMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

/// sa-token 登录检查中间件 - 强制要求登录
/// 
/// 此中间件会检查用户是否已登录，如果未登录则返回401错误
/// 建议与 SaTokenMiddleware 一起使用
#[derive(Clone)]
pub struct SaCheckLoginMiddleware {
    pub state: SaTokenState,
}

impl SaCheckLoginMiddleware {
    pub fn new(state: SaTokenState) -> Self {
        Self { state }
    }
}

impl Middleware for SaCheckLoginMiddleware {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Pin<Box<HandlerFuture>>
    where
        Chain: FnOnce(State) -> Pin<Box<HandlerFuture>> + Send + 'static,
    {
        let token_state = self.state.clone();
        
        Box::pin(async move {
            let mut ctx = SaTokenContext::new();
            
            // 提取 token
            if let Some(token_str) = extract_token_from_state(&state, &token_state) {
                tracing::debug!("Sa-Token(login-check): extracted token from request: {}", token_str);
                let token = TokenValue::new(token_str);
                
                // 验证 token
                if token_state.manager.is_valid(&token).await {
                    // 存储 token 和 login_id
                    state.put(TokenValueWrapper(token.clone()));
                    
                    if let Ok(token_info) = token_state.manager.get_token_info(&token).await {
                        let login_id = token_info.login_id.clone();
                        state.put(LoginIdWrapper(login_id.clone()));
                        
                        // 设置上下文
                        ctx.token = Some(token.clone());
                        ctx.token_info = Some(Arc::new(token_info));
                        ctx.login_id = Some(login_id);
                        
                        SaTokenContext::set_current(ctx);
                        let result = chain(state).await;
                        SaTokenContext::clear();
                        return result;
                    }
                }
            }
            
            // 未登录，返回401错误
            let error_json = json!({
                "code": 401,
                "message": messages::AUTH_ERROR
            });
            
            let response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("Content-Type", "application/json")
                .body(Body::from(error_json.to_string()))
                .expect("Unable to create response");
            
            Ok((state, response))
        })
    }
}

/// sa-token 权限检查中间件 - 强制要求特定权限
/// 
/// 此中间件会检查用户是否拥有指定权限，如果没有则返回403错误
#[derive(Clone)]
pub struct SaCheckPermissionMiddleware {
    pub state: SaTokenState,
    permission: String,
}

impl SaCheckPermissionMiddleware {
    pub fn new(state: SaTokenState, permission: impl Into<String>) -> Self {
        Self {
            state,
            permission: permission.into(),
        }
    }
}

impl Middleware for SaCheckPermissionMiddleware {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Pin<Box<HandlerFuture>>
    where
        Chain: FnOnce(State) -> Pin<Box<HandlerFuture>> + Send + 'static,
    {
        let token_state = self.state.clone();
        let permission = self.permission.clone();
        
        Box::pin(async move {
            let mut ctx = SaTokenContext::new();
            
            // 提取 token
            if let Some(token_str) = extract_token_from_state(&state, &token_state) {
                tracing::debug!("Sa-Token(permission-check): extracted token from request: {}", token_str);
                let token = TokenValue::new(token_str);
                
                // 验证 token
                if token_state.manager.is_valid(&token).await {
                    if let Ok(token_info) = token_state.manager.get_token_info(&token).await {
                        let login_id = token_info.login_id.clone();
                        
                        // 检查权限
                        if sa_token_core::StpUtil::has_permission(&login_id, &permission).await {
                            // 存储信息到 State
                            state.put(TokenValueWrapper(token.clone()));
                            state.put(LoginIdWrapper(login_id.clone()));
                            
                            // 设置上下文
                            ctx.token = Some(token.clone());
                            ctx.token_info = Some(Arc::new(token_info));
                            ctx.login_id = Some(login_id);
                            
                            SaTokenContext::set_current(ctx);
                            let result = chain(state).await;
                            SaTokenContext::clear();
                            return result;
                        }
                    }
                }
            }
            
            // 无权限或未登录，返回403错误
            let error_json = json!({
                "code": 403,
                "message": messages::PERMISSION_REQUIRED
            });
            
            let response = Response::builder()
                .status(StatusCode::FORBIDDEN)
                .header("Content-Type", "application/json")
                .body(Body::from(error_json.to_string()))
                .expect("Unable to create response");
            
            Ok((state, response))
        })
    }
}

/// sa-token 角色检查中间件 - 强制要求特定角色
/// 
/// 此中间件会检查用户是否拥有指定角色，如果没有则返回403错误
#[derive(Clone)]
pub struct SaCheckRoleMiddleware {
    pub state: SaTokenState,
    role: String,
}

impl SaCheckRoleMiddleware {
    pub fn new(state: SaTokenState, role: impl Into<String>) -> Self {
        Self {
            state,
            role: role.into(),
        }
    }
}

impl Middleware for SaCheckRoleMiddleware {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Pin<Box<HandlerFuture>>
    where
        Chain: FnOnce(State) -> Pin<Box<HandlerFuture>> + Send + 'static,
    {
        let token_state = self.state.clone();
        let role = self.role.clone();
        
        Box::pin(async move {
            let mut ctx = SaTokenContext::new();
            
            // 提取 token
            if let Some(token_str) = extract_token_from_state(&state, &token_state) {
                tracing::debug!("Sa-Token(role-check): extracted token from request: {}", token_str);
                let token = TokenValue::new(token_str);
                
                // 验证 token
                if token_state.manager.is_valid(&token).await {
                    if let Ok(token_info) = token_state.manager.get_token_info(&token).await {
                        let login_id = token_info.login_id.clone();
                        
                        // 检查角色
                        if sa_token_core::StpUtil::has_role(&login_id, &role).await {
                            // 存储信息到 State
                            state.put(TokenValueWrapper(token.clone()));
                            state.put(LoginIdWrapper(login_id.clone()));
                            
                            // 设置上下文
                            ctx.token = Some(token.clone());
                            ctx.token_info = Some(Arc::new(token_info));
                            ctx.login_id = Some(login_id);
                            
                            SaTokenContext::set_current(ctx);
                            let result = chain(state).await;
                            SaTokenContext::clear();
                            return result;
                        }
                    }
                }
            }
            
            // 无角色或未登录，返回403错误
            let error_json = json!({
                "code": 403,
                "message": messages::ROLE_REQUIRED
            });
            
            let response = Response::builder()
                .status(StatusCode::FORBIDDEN)
                .header("Content-Type", "application/json")
                .body(Body::from(error_json.to_string()))
                .expect("Unable to create response");
            
            Ok((state, response))
        })
    }
}

/// 从 State 中提取 token
/// 
/// 参考 Actix-web 实现，支持从 Header、Cookie、Query 参数中提取
fn extract_token_from_state(state: &State, token_state: &SaTokenState) -> Option<String> {
    use gotham::hyper::{HeaderMap, Uri};
    
    let token_name = &token_state.manager.config.token_name;
    
    // 1. 优先从 Header 中获取
    if let Some(headers) = state.try_borrow::<HeaderMap>() {
        if let Some(header_value) = headers.get(token_name) {
            if let Ok(value_str) = header_value.to_str() {
                if let Some(token) = extract_bearer_token(value_str) {
                    return Some(token);
                }
            }
        }
        
        // 检查 Authorization header
        if let Some(auth_header) = headers.get("authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = extract_bearer_token(auth_str) {
                    return Some(token);
                }
            }
        }
        
        // 2. 从 Cookie 中获取
        if let Some(cookie_header) = headers.get("cookie") {
            if let Ok(cookie_str) = cookie_header.to_str() {
                let cookies = parse_cookies(cookie_str);
                if let Some(token) = cookies.get(token_name) {
                    return Some(token.clone());
                }
            }
        }
    }
    
    // 3. 从 Query 参数中获取
    if let Some(uri) = state.try_borrow::<Uri>() {
        if let Some(query) = uri.query() {
            let params = parse_query_string(query);
            if let Some(token) = params.get(token_name) {
                return Some(token.clone());
            }
        }
    }
    
    None
}

