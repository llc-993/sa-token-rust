use tide::{Middleware, Request, Result, Next};
use sa_token_core::{token::TokenValue, SaTokenContext};
use std::sync::Arc;
use crate::state::SaTokenState;

#[derive(Clone)]
pub struct SaTokenLayer {
    state: SaTokenState,
}

impl SaTokenLayer {
    pub fn new(state: SaTokenState) -> Self {
        Self { state }
    }
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for SaTokenLayer {
    async fn handle(&self, mut req: Request<State>, next: Next<'_, State>) -> Result {
        let mut ctx = SaTokenContext::new();
        
        if let Some(token_str) = extract_token_from_request(&req, &self.state) {
            tracing::debug!("Sa-Token: extracted token from request: {}", token_str);
            let token = TokenValue::new(token_str);
            
            if self.state.manager.is_valid(&token).await {
                req.set_ext(token.clone());
                
                if let Ok(token_info) = self.state.manager.get_token_info(&token).await {
                    let login_id = token_info.login_id.clone();
                    req.set_ext(login_id.clone());
                    
                    ctx.token = Some(token.clone());
                    ctx.token_info = Some(Arc::new(token_info));
                    ctx.login_id = Some(login_id);
                }
            }
        }
        
        SaTokenContext::set_current(ctx);
        let result = next.run(req).await;
        SaTokenContext::clear();
        Ok(result)
    }
}

fn extract_token_from_request<State>(req: &Request<State>, token_state: &SaTokenState) -> Option<String> {
    let token_name = &token_state.manager.config.token_name;
    
    if let Some(header_value) = req.header(token_name.as_str()) {
        if let Some(value_str) = header_value.get(0) {
            return Some(extract_bearer_token(value_str.as_str()));
        }
    }
    
    if let Some(auth_header) = req.header("authorization") {
        if let Some(auth_str) = auth_header.get(0) {
            return Some(extract_bearer_token(auth_str.as_str()));
        }
    }
    
    if let Some(cookie_value) = req.cookie(token_name) {
        return Some(cookie_value.value().to_string());
    }
    
    if let Some(query) = req.url().query() {
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
