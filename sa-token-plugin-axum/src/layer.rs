//! Axum中间件层

use std::task::{Context, Poll};
use tower::{Layer, Service};
use http::{Request, Response};
use crate::SaTokenState;

/// sa-token中间件层
#[derive(Clone)]
pub struct SaTokenLayer {
    state: SaTokenState,
}

impl SaTokenLayer {
    pub fn new(state: SaTokenState) -> Self {
        Self { state }
    }
}

impl<S> Layer<S> for SaTokenLayer {
    type Service = SaTokenMiddleware<S>;
    
    fn layer(&self, inner: S) -> Self::Service {
        SaTokenMiddleware {
            inner,
            state: self.state.clone(),
        }
    }
}

/// sa-token中间件服务
#[derive(Clone)]
pub struct SaTokenMiddleware<S> {
    pub(crate) inner: S,
    pub(crate) state: SaTokenState,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for SaTokenMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
    
    fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
        let mut inner = self.inner.clone();
        let state = self.state.clone();
        
        Box::pin(async move {
            // 从请求中提取token
            let token = extract_token_from_request(&request, &state);
            
            // TODO: 将token信息存入请求扩展中
            // request.extensions_mut().insert(token);
            
            // 继续处理请求
            inner.call(request).await
        })
    }
}

fn extract_token_from_request<T>(_request: &Request<T>, _state: &SaTokenState) -> Option<String> {
    // TODO: 从header、cookie等位置提取token
    None
}

