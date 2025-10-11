//! Rocket Fairing (中间件)

use rocket::{Request, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use crate::SaTokenState;
use crate::adapter::RocketRequestAdapter;
use sa_token_adapter::context::SaRequest;
use sa_token_core::token::TokenValue;

/// sa-token Fairing - 提取并验证 token
pub struct SaTokenFairing {
    state: SaTokenState,
}

impl SaTokenFairing {
    pub fn new(state: SaTokenState) -> Self {
        Self { state }
    }
}

#[rocket::async_trait]
impl Fairing for SaTokenFairing {
    fn info(&self) -> Info {
        Info {
            name: "SaToken Authentication",
            kind: Kind::Request,
        }
    }
    
    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        // 提取 token
        if let Some(token_str) = extract_token_from_request(request, &self.state) {
            let token = TokenValue::new(token_str);
            
            // 验证 token
            if self.state.manager.is_valid(&token).await {
                // 存储 token 到本地缓存
                request.local_cache(|| Some(token.clone()));
                
                // 获取并存储 login_id
                if let Ok(token_info) = self.state.manager.get_token_info(&token).await {
                    request.local_cache(|| Some(token_info.login_id.clone()));
                }
            }
        }
    }
}

/// sa-token 登录检查 Fairing - 强制要求登录
pub struct SaCheckLoginFairing {
    state: SaTokenState,
}

impl SaCheckLoginFairing {
    pub fn new(state: SaTokenState) -> Self {
        Self { state }
    }
}

#[rocket::async_trait]
impl Fairing for SaCheckLoginFairing {
    fn info(&self) -> Info {
        Info {
            name: "SaToken Check Login",
            kind: Kind::Request | Kind::Response,
        }
    }
    
    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        // 提取 token
        if let Some(token_str) = extract_token_from_request(request, &self.state) {
            let token = TokenValue::new(token_str);
            
            // 验证 token
            if self.state.manager.is_valid(&token).await {
                // 存储 token
                request.local_cache(|| Some(token.clone()));
                
                // 获取并存储 login_id
                if let Ok(token_info) = self.state.manager.get_token_info(&token).await {
                    request.local_cache(|| Some(token_info.login_id.clone()));
                }
                return;
            }
        }
        
        // 未登录，标记为未授权
        request.local_cache(|| Some("unauthorized"));
    }
    
    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // 检查是否标记为未授权
        if let Some(_) = request.local_cache(|| None::<&str>) {
            if *request.local_cache(|| None::<&str>) == Some("unauthorized") {
                response.set_status(Status::Unauthorized);
                response.set_sized_body(None, std::io::Cursor::new(
                    serde_json::json!({
                        "code": 401,
                        "message": "未登录"
                    }).to_string()
                ));
            }
        }
    }
}

/// 从请求中提取 token
fn extract_token_from_request(request: &Request<'_>, state: &SaTokenState) -> Option<String> {
    let adapter = RocketRequestAdapter::new(request);
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
    if let Some(token) = adapter.get_param(token_name) {
        return Some(token);
    }
    
    None
}

/// 提取 Bearer token
fn extract_bearer_token(token: &str) -> String {
    if token.starts_with("Bearer ") {
        token[7..].to_string()
    } else {
        token.to_string()
    }
}

