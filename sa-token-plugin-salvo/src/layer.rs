use salvo::{Depot, Request, Response, Handler, FlowCtrl};
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

#[salvo::async_trait]
impl Handler for SaTokenLayer {
    async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
        let mut ctx = SaTokenContext::new();
        
        if let Some(token_str) = extract_token_from_request(req, &self.state) {
            tracing::debug!("Sa-Token: extracted token from request: {}", token_str);
            let token = TokenValue::new(token_str);
            
            if self.state.manager.is_valid(&token).await {
                depot.insert("sa_token", token.clone());
                
                if let Ok(token_info) = self.state.manager.get_token_info(&token).await {
                    let login_id = token_info.login_id.clone();
                    depot.insert("sa_login_id", login_id.clone());
                    
                    ctx.token = Some(token.clone());
                    ctx.token_info = Some(Arc::new(token_info));
                    ctx.login_id = Some(login_id);
                }
            }
        }
        
        SaTokenContext::set_current(ctx);
        ctrl.call_next(req, depot, res).await;
        SaTokenContext::clear();
    }
}

fn extract_token_from_request(req: &Request, state: &SaTokenState) -> Option<String> {
    let token_name = &state.manager.config.token_name;
    
    if let Some(header_value) = req.headers().get(token_name) {
        if let Ok(value_str) = header_value.to_str() {
            return Some(extract_bearer_token(value_str));
        }
    }
    
    if let Some(auth_header) = req.headers().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            return Some(extract_bearer_token(auth_str));
        }
    }
    
    if let Some(cookie_value) = req.cookie(token_name) {
        return Some(cookie_value.value().to_string());
    }
    
    if let Some(query_value) = req.query::<String>(token_name) {
        return Some(query_value);
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
