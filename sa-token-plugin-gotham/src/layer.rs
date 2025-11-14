use gotham::state::State;
use gotham::middleware::Middleware;
use gotham::handler::HandlerFuture;
use std::pin::Pin;
use sa_token_core::{token::TokenValue, SaTokenContext};
use crate::state::SaTokenState;
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

impl Middleware for SaTokenLayer {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Pin<Box<HandlerFuture>>
    where
        Chain: FnOnce(State) -> Pin<Box<HandlerFuture>> + Send + 'static,
    {
        Box::pin(async move {
            let mut ctx = SaTokenContext::new();
            
            if let Some(token_str) = extract_token_from_state(&state, &self.state) {
                tracing::debug!("Sa-Token: extracted token from request: {}", token_str);
                let token = TokenValue::new(token_str);
                
                if self.state.manager.is_valid(&token).await {
                    if let Ok(token_info) = self.state.manager.get_token_info(&token).await {
                        let login_id = token_info.login_id.clone();
                        
                        ctx.token = Some(token.clone());
                        ctx.token_info = Some(Arc::new(token_info));
                        ctx.login_id = Some(login_id.clone());
                        
                        state.put(crate::wrapper::TokenValueWrapper(token));
                        state.put(crate::wrapper::LoginIdWrapper(login_id));
                    }
                }
            }
            
            SaTokenContext::set_current(ctx);
            let result = chain(state).await;
            SaTokenContext::clear();
            result
        })
    }
}

/// 从 Gotham State 中提取 Token
/// 
/// 按优先级顺序查找 Token：
/// 1. HTTP Header - `<token_name>: <token>` 或 `<token_name>: Bearer <token>`
/// 2. Authorization Header - `Authorization: Bearer <token>`
/// 3. Cookie - `<token_name>=<token>`
/// 4. Query Parameter - `?<token_name>=<token>`
fn extract_token_from_state(state: &State, token_state: &SaTokenState) -> Option<String> {
    use gotham::hyper::{HeaderMap, Uri};
    use sa_token_adapter::utils::{parse_cookies, parse_query_string};
    
    // 从配置中获取 token_name
    let token_name = &token_state.manager.config.token_name;
    
    // 1. 从 Header 中获取
    if let Some(headers) = state.try_borrow::<HeaderMap>() {
        // 1.1 尝试从指定名称的 header 获取
        if let Some(header_value) = headers.get(token_name) {
            if let Ok(value_str) = header_value.to_str() {
                return Some(extract_bearer_token(value_str));
            }
        }
        
        // 1.2 尝试从 Authorization header 获取
        if let Some(auth_header) = headers.get("authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                return Some(extract_bearer_token(auth_str));
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
