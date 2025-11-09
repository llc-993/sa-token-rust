use ntex::service::{Service, ServiceCtx, Middleware};
use ntex::web::{Error, ErrorRenderer, WebRequest, WebResponse};
use crate::state::SaTokenState;
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

impl<S> Middleware<S> for SaTokenLayer {
    type Service = SaTokenMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        SaTokenMiddleware {
            service,
            state: self.state.clone(),
        }
    }
}

pub struct SaTokenMiddleware<S> {
    service: S,
    state: SaTokenState,
}

impl<S, Err> Service<WebRequest<Err>> for SaTokenMiddleware<S>
where
    S: Service<WebRequest<Err>, Response = WebResponse, Error = Error>,
    Err: ErrorRenderer,
{
    type Response = WebResponse;
    type Error = Error;

    async fn call(&self, req: WebRequest<Err>, ctx: ServiceCtx<'_, Self>) -> Result<Self::Response, Self::Error> {
        let mut sa_ctx = SaTokenContext::new();
        
        if let Some(token_str) = extract_token_from_request(&req, &self.state) {
            tracing::debug!("Sa-Token: extracted token from request: {}", token_str);
            let token = TokenValue::new(token_str);
            
            if self.state.manager.is_valid(&token).await {
                req.extensions_mut().insert(token.clone());
                
                if let Ok(token_info) = self.state.manager.get_token_info(&token).await {
                    let login_id = token_info.login_id.clone();
                    req.extensions_mut().insert(login_id.clone());
                    
                    sa_ctx.token = Some(token.clone());
                    sa_ctx.token_info = Some(Arc::new(token_info));
                    sa_ctx.login_id = Some(login_id);
                }
            }
        }
        
        SaTokenContext::set_current(sa_ctx);
        let result = ctx.call(&self.service, req).await;
        SaTokenContext::clear();
        result
    }
}

fn extract_token_from_request<Err>(req: &WebRequest<Err>, state: &SaTokenState) -> Option<String> 
where
    Err: ErrorRenderer,
{
    let token_name = &state.manager.config.token_name;
    let headers = req.headers();
    
    // 1. 从 token_name 指定的 header 获取
    if let Some(header_value) = headers.get(token_name) {
        if let Ok(value_str) = header_value.to_str() {
            return Some(extract_bearer_token(value_str));
        }
    }
    
    // 2. 从标准 Authorization 头获取
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            return Some(extract_bearer_token(auth_str));
        }
    }
    
    // 3. 从 Cookie 获取
    if let Some(cookie_header) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            if let Some(token) = parse_cookie(cookie_str, token_name) {
                return Some(token);
            }
        }
    }
    
    // 4. 从查询参数获取
    if let Some(query) = req.uri().query() {
        if let Some(token) = parse_query_param(query, token_name) {
            return Some(token);
        }
    }
    
    None
}

fn extract_bearer_token(header_value: &str) -> String {
    if header_value.starts_with("Bearer ") {
        header_value[7..].trim().to_string()
    } else {
        header_value.trim().to_string()
    }
}

fn parse_cookie(cookie_str: &str, token_name: &str) -> Option<String> {
    for part in cookie_str.split(';') {
        let part = part.trim();
        if let Some(eq_pos) = part.find('=') {
            let (name, value) = part.split_at(eq_pos);
            if name.trim() == token_name {
                return Some(value[1..].trim().to_string());
            }
        }
    }
    None
}

fn parse_query_param(query: &str, param_name: &str) -> Option<String> {
    for pair in query.split('&') {
        let parts: Vec<&str> = pair.splitn(2, '=').collect();
        if parts.len() == 2 && parts[0] == param_name {
            return urlencoding::decode(parts[1])
                .ok()
                .map(|s| s.into_owned());
        }
    }
    None
}