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

fn extract_token_from_state(state: &State, token_state: &SaTokenState) -> Option<String> {
    use gotham::hyper::HeaderMap;
    
    if let Some(headers) = state.try_borrow::<HeaderMap>() {
        let token_name = &token_state.manager.config.token_name;
        
        if let Some(header_value) = headers.get(token_name) {
            if let Ok(value_str) = header_value.to_str() {
                return Some(extract_bearer_token(value_str));
            }
        }
        
        if let Some(auth_header) = headers.get("authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                return Some(extract_bearer_token(auth_str));
            }
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
