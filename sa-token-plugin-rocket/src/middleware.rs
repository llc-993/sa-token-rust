// Author: 金书记
//
//! Rocket Fairing (中间件)

use rocket::{Request, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Status, ContentType};
use crate::SaTokenState;
use sa_token_core::{token::TokenValue, error::messages};
use serde_json::json;

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
        let token_str = {
            let token_name = &self.state.manager.config.token_name;
            
            // 1. 从 Header 获取
            if let Some(header_val) = request.headers().get_one(token_name) {
                Some(extract_bearer_token(header_val))
            }
            // 2. 从 Cookie 获取
            else if let Some(cookie) = request.cookies().get(token_name) {
                Some(cookie.value().to_string())
            }
            // 3. 从 Query 参数获取
            else if let Some(query) = request.uri().query() {
                parse_query_param(query.as_str(), token_name)
            } else {
                None
            }
        };
        
        if let Some(token_str) = token_str {
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
        let token_str = {
            let token_name = &self.state.manager.config.token_name;
            
            // 1. 从 Header 获取
            if let Some(header_val) = request.headers().get_one(token_name) {
                Some(extract_bearer_token(header_val))
            }
            // 2. 从 Cookie 获取
            else if let Some(cookie) = request.cookies().get(token_name) {
                Some(cookie.value().to_string())
            }
            // 3. 从 Query 参数获取
            else if let Some(query) = request.uri().query() {
                parse_query_param(query.as_str(), token_name)
            } else {
                None
            }
        };
        
        if let Some(token_str) = token_str {
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
                    json!({
                        "code": 401,
                        "message": messages::AUTH_ERROR
                    }).to_string()
                ));
            }
        }
    }
}

/// sa-token 权限检查 Fairing - 强制要求特定权限
pub struct SaCheckPermissionFairing {
    #[allow(dead_code)]
    state: SaTokenState,
    permission: String,
}

impl SaCheckPermissionFairing {
    pub fn new(state: SaTokenState, permission: impl Into<String>) -> Self {
        Self {
            state,
            permission: permission.into(),
        }
    }
}

#[rocket::async_trait]
impl Fairing for SaCheckPermissionFairing {
    fn info(&self) -> Info {
        Info {
            name: "SaToken Check Permission",
            kind: Kind::Request | Kind::Response,
        }
    }
    
    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        // 检查是否有登录ID
        if let Some(login_id) = request.local_cache(|| None::<String>).clone() {
            // 检查权限
            if sa_token_core::StpUtil::has_permission(&login_id, &self.permission).await {
                return;
            }
        }
        
        // 无权限，标记为禁止访问
        request.local_cache(|| Some("forbidden"));
    }
    
    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // 检查是否标记为禁止访问
        if let Some(_) = request.local_cache(|| None::<&str>) {
            if *request.local_cache(|| None::<&str>) == Some("forbidden") {
                response.set_status(Status::Forbidden);
                response.set_header(ContentType::JSON);
                response.set_sized_body(None, std::io::Cursor::new(
                    json!({
                        "code": 403,
                        "message": messages::PERMISSION_REQUIRED
                    }).to_string()
                ));
            }
        }
    }
}

/// sa-token 角色检查 Fairing - 强制要求特定角色
pub struct SaCheckRoleFairing {
    #[allow(dead_code)]
    state: SaTokenState,
    role: String,
}

impl SaCheckRoleFairing {
    pub fn new(state: SaTokenState, role: impl Into<String>) -> Self {
        Self {
            state,
            role: role.into(),
        }
    }
}

#[rocket::async_trait]
impl Fairing for SaCheckRoleFairing {
    fn info(&self) -> Info {
        Info {
            name: "SaToken Check Role",
            kind: Kind::Request | Kind::Response,
        }
    }
    
    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        // 检查是否有登录ID
        if let Some(login_id) = request.local_cache(|| None::<String>).clone() {
            // 检查角色
            if sa_token_core::StpUtil::has_role(&login_id, &self.role).await {
                return;
            }
        }
        
        // 无角色，标记为禁止访问
        request.local_cache(|| Some("forbidden_role"));
    }
    
    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // 检查是否标记为禁止访问
        if let Some(_) = request.local_cache(|| None::<&str>) {
            if *request.local_cache(|| None::<&str>) == Some("forbidden_role") {
                response.set_status(Status::Forbidden);
                response.set_header(ContentType::JSON);
                response.set_sized_body(None, std::io::Cursor::new(
                    json!({
                        "code": 403,
                        "message": messages::ROLE_REQUIRED
                    }).to_string()
                ));
            }
        }
    }
}

/// 提取 Bearer token
fn extract_bearer_token(token: &str) -> String {
    if token.starts_with("Bearer ") {
        token[7..].to_string()
    } else {
        token.to_string()
    }
}

/// 解析查询参数
fn parse_query_param(query: &str, name: &str) -> Option<String> {
    for pair in query.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            if key == name {
                return urlencoding::decode(value).ok().map(|s| s.to_string());
            }
        }
    }
    None
}
