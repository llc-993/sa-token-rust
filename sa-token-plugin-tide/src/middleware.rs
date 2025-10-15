// Author: 金书记
//
// 中文 | English
// Tide 认证中间件 | Tide authentication middleware

use tide::{Middleware, Request, Response, Next, StatusCode};
use sa_token_core::StpUtil;
use async_trait::async_trait;

/// 中文 | English
/// 认证中间件 - 验证用户登录状态 | Authentication middleware - verify user login status
///
/// # 示例 | Example
/// ```rust,ignore
/// use tide::prelude::*;
/// use sa_token_plugin_tide::AuthMiddleware;
///
/// let mut app = tide::new();
/// app.with(AuthMiddleware);
/// app.at("/user").get(user_handler);
/// ```
#[derive(Clone)]
pub struct AuthMiddleware;

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for AuthMiddleware {
    async fn handle(&self, mut req: Request<State>, next: Next<'_, State>) -> tide::Result {
        // 中文 | English
        // 从请求头中获取 token | Get token from request headers
        let token = req
            .header("Authorization")
            .and_then(|v| v.as_str().strip_prefix("Bearer "))
            .map(|s| s.to_string());
        
        if let Some(token_str) = token {
            // 中文 | English
            // 验证 token 是否有效 | Verify if token is valid
            use sa_token_core::TokenValue;
            let token_value = TokenValue::from(token_str.clone());
            if StpUtil::is_login(&token_value).await {
                // 中文 | English
                // Token 有效，将 login_id 存入扩展数据 | Token valid, store login_id in extensions
                if let Ok(login_id) = StpUtil::get_login_id(&token_value).await {
                    req.set_ext(login_id);
                    return Ok(next.run(req).await);
                }
            }
        }
        
        // 中文 | English
        // Token 无效，返回 401 | Token invalid, return 401
        let mut res = Response::new(StatusCode::Unauthorized);
        res.set_body(r#"{"error":"Unauthorized"}"#);
        res.set_content_type("application/json");
        Ok(res)
    }
}

/// 中文 | English
/// 权限验证中间件 - 验证用户是否拥有指定权限 | Permission middleware - verify if user has specified permissions
///
/// # 示例 | Example
/// ```rust,ignore
/// let mut app = tide::new();
/// app.with(PermissionMiddleware::new("user:read"));
/// ```
#[derive(Clone)]
pub struct PermissionMiddleware {
    permission: String,
}

impl PermissionMiddleware {
    /// 中文 | English
    /// 创建权限验证中间件 | Create permission middleware
    pub fn new(permission: impl Into<String>) -> Self {
        Self {
            permission: permission.into(),
        }
    }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for PermissionMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> tide::Result {
        // 中文 | English
        // 从扩展数据获取 login_id | Get login_id from extensions
        if let Some(login_id) = req.ext::<String>() {
            // 中文 | English
            // 验证权限 | Verify permission
            if StpUtil::has_permission(login_id, &self.permission).await {
                return Ok(next.run(req).await);
            }
        }
        
        // 中文 | English
        // 无权限，返回 403 | No permission, return 403
        let mut res = Response::new(StatusCode::Forbidden);
        res.set_body(r#"{"error":"Forbidden"}"#);
        res.set_content_type("application/json");
        Ok(res)
    }
}

