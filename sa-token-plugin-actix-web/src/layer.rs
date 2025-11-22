use std::future::{ready, Ready, Future};
use std::pin::Pin;
use std::rc::Rc;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use crate::SaTokenState;
use crate::adapter::ActixRequestAdapter;
use sa_token_adapter::context::SaRequest;
use sa_token_core::{token::TokenValue, SaTokenContext};
use std::sync::Arc;

#[derive(Clone)]
pub struct SaTokenLayer {
    state: SaTokenState,
}

impl SaTokenLayer {
    pub fn new(state: SaTokenState) -> Self {
        Self { state }
    }
}

impl<S, B> Transform<S, ServiceRequest> for SaTokenLayer
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SaTokenLayerService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SaTokenLayerService {
            service: Rc::new(service),
            state: self.state.clone(),
        }))
    }
}

pub struct SaTokenLayerService<S> {
    service: Rc<S>,
    state: SaTokenState,
}

impl<S, B> Service<ServiceRequest> for SaTokenLayerService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    
    fn poll_ready(&self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }
    
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let state = self.state.clone();
        
        Box::pin(async move {
            let mut ctx = SaTokenContext::new();
            
            if let Some(token_str) = extract_token_from_request(&req, &state) {
                tracing::debug!("Sa-Token: extracted token from request: {}", token_str);
                let token = TokenValue::new(token_str);
                
                if state.manager.is_valid(&token).await {
                    req.extensions_mut().insert(token.clone());
                    
                    if let Ok(token_info) = state.manager.get_token_info(&token).await {
                        let login_id = token_info.login_id.clone();
                        req.extensions_mut().insert(login_id.clone());
                        
                        ctx.token = Some(token.clone());
                        ctx.token_info = Some(Arc::new(token_info));
                        ctx.login_id = Some(login_id);
                    }
                }
            }
            
            SaTokenContext::set_current(ctx);
            let result = service.call(req).await;
            SaTokenContext::clear();
            result
        })
    }
}

fn extract_token_from_request(req: &ServiceRequest, state: &SaTokenState) -> Option<String> {
    let adapter = ActixRequestAdapter::new(req.request());
    let token_name = &state.manager.config.token_name;
    
    // 1. 优先从 Header 中获取（检查 token_name 配置的头）
    if let Some(token) = adapter.get_header(token_name) {
        return Some(extract_bearer_token(&token));
    }
    
    // 2. 如果 token_name 不是 "Authorization"，也尝试从 "Authorization" 头获取
    if token_name != "Authorization" {
        if let Some(token) = adapter.get_header("Authorization") {
            return Some(extract_bearer_token(&token));
        }
    }
    
    // 3. 从 Cookie 中获取
    if let Some(token) = adapter.get_cookie(token_name) {
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
        return Some(query);
    }
    
    None
}

fn extract_bearer_token(token: &str) -> String {
    if token.starts_with("Bearer ") {
        token[7..].to_string()
    } else {
        token.to_string()
    }
}
