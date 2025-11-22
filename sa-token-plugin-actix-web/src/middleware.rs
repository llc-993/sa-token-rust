// Author: 金书记
//
//! Actix-web中间件

use std::future::{ready, Ready, Future};
use std::pin::Pin;
use std::rc::Rc;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, error::ErrorUnauthorized,
};
use crate::SaTokenState;
use crate::adapter::ActixRequestAdapter;
use sa_token_adapter::context::SaRequest;
use sa_token_core::{token::TokenValue, SaTokenContext, error::messages};
use std::sync::Arc;

/// sa-token 基础中间件 - 提取并验证 token
pub struct SaTokenMiddleware {
    pub state: SaTokenState,
}

impl SaTokenMiddleware {
    pub fn new(state: SaTokenState) -> Self {
        Self { state }
    }
}

impl<S, B> Transform<S, ServiceRequest> for SaTokenMiddleware
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SaTokenMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SaTokenMiddlewareService {
            service: Rc::new(service),
            state: self.state.clone(),
        }))
    }
}

pub struct SaTokenMiddlewareService<S> {
    service: Rc<S>,
    state: SaTokenState,
}

impl<S, B> Service<ServiceRequest> for SaTokenMiddlewareService<S>
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let state = self.state.clone();
        
        Box::pin(async move {
            let mut ctx = SaTokenContext::new();
            
            tracing::debug!("Sa-Token: 开始处理请求 {} {}", req.method(), req.path());
            
            // 提取 token
            if let Some(token_str) = extract_token_from_request(&req, &state) {
                tracing::debug!("Sa-Token: extracted token from request: {}", token_str);
                let token = TokenValue::new(token_str);
                
                // 验证 token
                if state.manager.is_valid(&token).await {
                    tracing::debug!("Sa-Token: token 验证成功");
                    // 存储 token
                    req.extensions_mut().insert(token.clone());
                    
                    // 获取并存储 login_id
                    if let Ok(token_info) = state.manager.get_token_info(&token).await {
                        let login_id = token_info.login_id.clone();
                        tracing::debug!("Sa-Token: login_id = {}", login_id);
                        req.extensions_mut().insert(login_id.clone());
                        ctx.token = Some(token.clone());
                        ctx.token_info = Some(Arc::new(token_info));
                        ctx.login_id = Some(login_id);
                    }
                } else {
                    tracing::debug!("Sa-Token: token 验证失败");
                }
            } else {
                tracing::debug!("Sa-Token: 未提取到 token");
            }
            
            SaTokenContext::set_current(ctx);
            let result = service.call(req).await;
            SaTokenContext::clear();
            result
        })
    }
}

/// sa-token 登录检查中间件 - 强制要求登录
pub struct SaCheckLoginMiddleware {
    pub state: SaTokenState,
}

impl SaCheckLoginMiddleware {
    pub fn new(state: SaTokenState) -> Self {
        Self { state }
    }
}

impl<S, B> Transform<S, ServiceRequest> for SaCheckLoginMiddleware
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SaCheckLoginMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SaCheckLoginMiddlewareService {
            service: Rc::new(service),
            state: self.state.clone(),
        }))
    }
}

pub struct SaCheckLoginMiddlewareService<S> {
    service: Rc<S>,
    state: SaTokenState,
}

impl<S, B> Service<ServiceRequest> for SaCheckLoginMiddlewareService<S>
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let state = self.state.clone();

        Box::pin(async move {
            let mut ctx = SaTokenContext::new();
            // 提取 token
            if let Some(token_str) = extract_token_from_request(&req, &state) {
                tracing::debug!("Sa-Token(login-check): extracted token from request: {}", token_str);
                let token = TokenValue::new(token_str);

                // 验证 token
                if state.manager.is_valid(&token).await {
                    // 存储 token 和 login_id
                    req.extensions_mut().insert(token.clone());

                    if let Ok(token_info) = state.manager.get_token_info(&token).await {
                        let login_id = token_info.login_id.clone();
                        req.extensions_mut().insert(login_id.clone());
                        ctx.token = Some(token.clone());
                        ctx.token_info = Some(Arc::new(token_info));
                        ctx.login_id = Some(login_id);

                        // 设置上下文
                        SaTokenContext::set_current(ctx);
                        let result = service.call(req).await;
                        SaTokenContext::clear();
                        return result;
                    }
                }
            }

            // 未登录，返回 401
            Err(ErrorUnauthorized(serde_json::json!({
                "code": 401,
                "message": messages::AUTH_ERROR
            }).to_string()))
        })
    }
}

/// 从请求中提取 token
fn extract_token_from_request(req: &ServiceRequest, state: &SaTokenState) -> Option<String> {
    let adapter = ActixRequestAdapter::new(req.request());
    let token_name = &state.manager.config.token_name;
    
    tracing::debug!("Sa-Token: 尝试从请求提取 token，token_name: {}", token_name);
    
    // 1. 优先从 Header 中获取（检查 token_name 配置的头）
    if let Some(token) = adapter.get_header(token_name) {
        tracing::debug!("Sa-Token: 从 Header[{}] 获取到 token", token_name);
        return Some(extract_bearer_token(&token));
    }
    
    // 2. 如果 token_name 不是 "Authorization"，也尝试从 "Authorization" 头获取
    if token_name != "Authorization" {
        if let Some(token) = adapter.get_header("Authorization") {
            tracing::debug!("Sa-Token: 从 Header[Authorization] 获取到 token");
            return Some(extract_bearer_token(&token));
        }
    }
    
    // 3. 从 Cookie 中获取
    if let Some(token) = adapter.get_cookie(token_name) {
        tracing::debug!("Sa-Token: 从 Cookie[{}] 获取到 token", token_name);
        return Some(token);
    }
    
    // 4. 从 Query 参数中获取
    if let Some(query) = req.query_string().split('&').find_map(|pair| {
        let mut parts = pair.split('=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            if key == token_name {
                return urlencoding::decode(value).ok().map(|s| s.to_string());
            }
        }
        None
    }) {
        tracing::debug!("Sa-Token: 从 Query[{}] 获取到 token", token_name);
        return Some(query);
    }
    
    tracing::debug!("Sa-Token: 所有位置都未找到 token");
    None
}

/// 提取 Bearer token
fn extract_bearer_token(token: &str) -> String {
    if token.starts_with("Bearer ") {
        token[7..].to_string()
    } else {
        token.to_string()
    }
}
