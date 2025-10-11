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
use sa_token_core::token::TokenValue;

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
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
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
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    
    forward_ready!(service);
    
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let state = self.state.clone();
        
        Box::pin(async move {
            // 提取 token
            if let Some(token_str) = extract_token_from_request(&req, &state) {
                let token = TokenValue::new(token_str);
                
                // 验证 token
                if state.manager.is_valid(&token).await {
                    // 存储 token
                    req.extensions_mut().insert(token.clone());
                    
                    // 获取并存储 login_id
                    if let Ok(token_info) = state.manager.get_token_info(&token).await {
                        req.extensions_mut().insert(token_info.login_id.clone());
                    }
                }
            }
            
            service.call(req).await
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
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
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
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    
    forward_ready!(service);
    
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let state = self.state.clone();
        
        Box::pin(async move {
            // 提取 token
            if let Some(token_str) = extract_token_from_request(&req, &state) {
                let token = TokenValue::new(token_str);
                
                // 验证 token
                if state.manager.is_valid(&token).await {
                    // 存储 token 和 login_id
                    req.extensions_mut().insert(token.clone());
                    
                    if let Ok(token_info) = state.manager.get_token_info(&token).await {
                        req.extensions_mut().insert(token_info.login_id.clone());
                    }
                    
                    return service.call(req).await;
                }
            }
            
            // 未登录，返回 401
            Err(ErrorUnauthorized(serde_json::json!({
                "code": 401,
                "message": "未登录"
            }).to_string()))
        })
    }
}

/// 从请求中提取 token
fn extract_token_from_request(req: &ServiceRequest, state: &SaTokenState) -> Option<String> {
    let adapter = ActixRequestAdapter::new(req.request());
    let token_name = &state.manager.config.token_name;
    
    // 1. 优先从 Header 中获取
    if let Some(token) = adapter.get_header(token_name) {
        return Some(extract_bearer_token(&token));
    }
    
    // 2. 从 Cookie 中获取
    if let Some(token) = adapter.get_cookie(token_name) {
        return Some(token);
    }
    
    // 3. 从 Query 参数中获取
    if let Some(query) = req.query_string().split('&').find_map(|pair| {
        let mut parts = pair.split('=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            if key == token_name {
                return urlencoding::decode(value).ok().map(|s| s.to_string());
            }
        }
        None
    }) {
        return Some(query);
    }
    
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
