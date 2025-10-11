//! Poem 中间件实现

use std::sync::Arc;
use poem::{
    Endpoint, IntoResponse, Middleware, Request, Response, Result as PoemResult,
    http::StatusCode,
};
use sa_token_core::SaTokenManager;
use crate::adapter::PoemRequestAdapter;

/// sa-token 中间件
pub struct SaTokenMiddleware {
    manager: Arc<SaTokenManager>,
}

impl SaTokenMiddleware {
    /// 创建新的中间件实例
    pub fn new(manager: Arc<SaTokenManager>) -> Self {
        Self { manager }
    }
}

impl<E: Endpoint> Middleware<E> for SaTokenMiddleware {
    type Output = SaTokenMiddlewareImpl<E>;
    
    fn transform(&self, ep: E) -> Self::Output {
        SaTokenMiddlewareImpl {
            ep,
            manager: self.manager.clone(),
        }
    }
}

/// 中间件实现
pub struct SaTokenMiddlewareImpl<E> {
    ep: E,
    manager: Arc<SaTokenManager>,
}

impl<E: Endpoint> Endpoint for SaTokenMiddlewareImpl<E> {
    type Output = Response;
    
    async fn call(&self, mut req: Request) -> PoemResult<Self::Output> {
        // 从请求中提取 token
        let adapter = PoemRequestAdapter::new(&req);
        let token_name = &self.manager.config.token_name;
        
        // 按优先级查找 token：Header > Cookie > Query
        let token_value = adapter.get_header(token_name)
            .or_else(|| adapter.get_cookie(token_name))
            .or_else(|| adapter.get_param(token_name));
        
        // 如果找到 token，验证并存储到请求扩展中
        if let Some(token_str) = token_value {
            let token = sa_token_core::token::TokenValue::new(token_str);
            
            // 验证 token 是否有效
            if self.manager.is_valid(&token).await {
                // 将 token 存储到请求扩展中，供后续处理器使用
                req.extensions_mut().insert(token.clone());
                
                // 尝试获取 token 信息并存储 login_id
                if let Ok(token_info) = self.manager.get_token_info(&token).await {
                    req.extensions_mut().insert(token_info.login_id.clone());
                }
            }
        }
        
        // 调用下一个处理器
        let response = self.ep.call(req).await;
        
        // 将响应转换为 Response 类型
        match response {
            Ok(resp) => Ok(resp.into_response()),
            Err(e) => Err(e),
        }
    }
}

/// 创建需要登录检查的中间件
pub struct SaCheckLoginMiddleware {
    manager: Arc<SaTokenManager>,
}

impl SaCheckLoginMiddleware {
    pub fn new(manager: Arc<SaTokenManager>) -> Self {
        Self { manager }
    }
}

impl<E: Endpoint> Middleware<E> for SaCheckLoginMiddleware {
    type Output = SaCheckLoginMiddlewareImpl<E>;
    
    fn transform(&self, ep: E) -> Self::Output {
        SaCheckLoginMiddlewareImpl {
            ep,
            manager: self.manager.clone(),
        }
    }
}

pub struct SaCheckLoginMiddlewareImpl<E> {
    ep: E,
    manager: Arc<SaTokenManager>,
}

impl<E: Endpoint> Endpoint for SaCheckLoginMiddlewareImpl<E> {
    type Output = Response;
    
    async fn call(&self, req: Request) -> PoemResult<Self::Output> {
        // 从请求扩展中获取 token
        if let Some(token) = req.extensions().get::<sa_token_core::token::TokenValue>() {
            // 验证 token
            if self.manager.is_valid(token).await {
                // Token 有效，继续处理
                return match self.ep.call(req).await {
                    Ok(resp) => Ok(resp.into_response()),
                    Err(e) => Err(e),
                };
            }
        }
        
        // 未登录，返回 401
        Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized: Please login first"))
    }
}

