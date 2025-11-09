// Author: 金书记
//
//! Axum中间件层

use std::task::{Context, Poll};
use tower::{Layer, Service};
use http::{Request, Response};
use sa_token_adapter::context::SaRequest;
use crate::{SaTokenState, adapter::AxumRequestAdapter};
use sa_token_core::SaTokenContext;
use std::sync::Arc;

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
    
    fn call(&mut self, mut request: Request<ReqBody>) -> Self::Future {
        let mut inner = self.inner.clone();
        let state = self.state.clone();
        
        Box::pin(async move {
            let mut ctx = SaTokenContext::new();
            
            // 从请求中提取 token
            if let Some(token_str) = extract_token_from_request(&request, &state) {
                tracing::debug!("Sa-Token: extracted token from request: {}", token_str);
                let token = sa_token_core::token::TokenValue::new(token_str);
                
                // 验证 token 是否有效
                if state.manager.is_valid(&token).await {
                    // 将 token 存储到请求扩展中
                    request.extensions_mut().insert(token.clone());
                    
                    // 尝试获取 token 信息并存储 login_id
                    // 注意：get_token_info 内部已经处理了自动续签（如果配置开启）
                    if let Ok(token_info) = state.manager.get_token_info(&token).await {
                        let login_id = token_info.login_id.clone();
                        request.extensions_mut().insert(login_id.clone());
                        
                        // 设置上下文
                        ctx.token = Some(token.clone());
                        ctx.token_info = Some(Arc::new(token_info));
                        ctx.login_id = Some(login_id);
                    }
                }
            }
            
            // 设置当前请求的上下文
            SaTokenContext::set_current(ctx);
            
            // 继续处理请求
            let response = inner.call(request).await;
            
            // 清除上下文
            SaTokenContext::clear();
            
            response
        })
    }
}

/// 从请求中提取 Token
/// 
/// 按优先级顺序查找 Token：
/// 1. HTTP Header - `<token_name>: <token>` 或 `<token_name>: Bearer <token>`
/// 2. Cookie - `<token_name>=<token>`
/// 3. Query Parameter - `?<token_name>=<token>`
/// 
/// # 参数
/// - `request` - HTTP 请求
/// - `state` - SaToken 状态（从配置中获取 token_name）
/// 
/// # 返回
/// - `Some(token)` - 找到有效的 token
/// - `None` - 未找到 token
fn extract_token_from_request<T>(request: &Request<T>, state: &SaTokenState) -> Option<String> {
    let adapter = AxumRequestAdapter::new(request);
    // 从配置中获取 token_name
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
    if let Some(query) = request.uri().query() {
        if let Some(token) = parse_query_param(query, token_name) {
            return Some(token);
        }
    }
    
    None
}

/// 提取 Bearer Token
/// 
/// 支持两种格式：
/// - `Bearer <token>` - 标准 Bearer Token 格式
/// - `<token>` - 直接的 Token 字符串
fn extract_bearer_token(header_value: &str) -> String {
    const BEARER_PREFIX: &str = "Bearer ";
    
    if header_value.starts_with(BEARER_PREFIX) {
        // 去除 "Bearer " 前缀
        header_value[BEARER_PREFIX.len()..].trim().to_string()
    } else {
        // 直接返回 token
        header_value.trim().to_string()
    }
}

/// 从 Query 字符串中解析参数
fn parse_query_param(query: &str, param_name: &str) -> Option<String> {
    for pair in query.split('&') {
        let parts: Vec<&str> = pair.splitn(2, '=').collect();
        if parts.len() == 2 && parts[0] == param_name {
            // URL 解码
            return urlencoding::decode(parts[1])
                .ok()
                .map(|s| s.into_owned());
        }
    }
    None
}
