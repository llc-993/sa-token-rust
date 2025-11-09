use rocket::{Request, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use sa_token_core::{token::TokenValue, SaTokenContext};
use crate::SaTokenState;
use std::sync::Arc;

pub struct SaTokenLayer {
    state: SaTokenState,
}

impl SaTokenLayer {
    pub fn new(state: SaTokenState) -> Self {
        Self { state }
    }
}

#[rocket::async_trait]
impl Fairing for SaTokenLayer {
    fn info(&self) -> Info {
        Info {
            name: "Sa-Token Authentication",
            kind: Kind::Request | Kind::Response,
        }
    }
    
    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        let mut ctx = SaTokenContext::new();
        
        if let Some(token_str) = extract_token_from_request(req, &self.state) {
            tracing::debug!("Sa-Token: extracted token from request: {}", token_str);
            let token = TokenValue::new(token_str);
            
            if self.state.manager.is_valid(&token).await {
                req.local_cache(|| Some(token.clone()));
                
                if let Ok(token_info) = self.state.manager.get_token_info(&token).await {
                    let login_id = token_info.login_id.clone();
                    req.local_cache(|| Some(login_id.clone()));
                    
                    ctx.token = Some(token.clone());
                    ctx.token_info = Some(Arc::new(token_info));
                    ctx.login_id = Some(login_id);
                }
            }
        }
        
        SaTokenContext::set_current(ctx);
    }
    
    async fn on_response<'r>(&self, _req: &'r Request<'_>, _res: &mut Response<'r>) {
        SaTokenContext::clear();
    }
}

fn extract_token_from_request(req: &Request, state: &SaTokenState) -> Option<String> {
    let token_name = &state.manager.config.token_name;
    
    if let Some(header_value) = req.headers().get_one(token_name) {
        return Some(extract_bearer_token(header_value));
    }
    
    if let Some(auth_header) = req.headers().get_one("authorization") {
        return Some(extract_bearer_token(auth_header));
    }
    
    if let Some(cookie_value) = req.cookies().get(token_name) {
        return Some(cookie_value.value().to_string());
    }
    
    if let Some(query_value) = req.query_value::<String>(token_name) {
        if let Ok(value) = query_value {
            return Some(value);
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
