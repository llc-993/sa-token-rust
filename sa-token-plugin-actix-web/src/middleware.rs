//! Actix-web中间件

use std::future::{ready, Ready, Future};
use std::pin::Pin;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};

/// sa-token中间件
pub struct SaTokenMiddleware;

impl<S, B> Transform<S, ServiceRequest> for SaTokenMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SaTokenMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SaTokenMiddlewareService { service }))
    }
}

pub struct SaTokenMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SaTokenMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    
    forward_ready!(service);
    
    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 从请求中提取token
        let token = req.headers().get("Authorization")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        
        // TODO: 验证token并将信息存入请求扩展中
        
        let fut = self.service.call(req);
        
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

