// Author: 金书记
//
// 中文 | English
// Ntex 认证中间件 | Ntex authentication middleware

use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::web::{Error, ErrorRenderer, WebRequest, WebResponse};
use sa_token_core::StpUtil;
/// 中文 | English
/// 认证中间件 - 验证用户登录状态 | Authentication middleware - verify user login status
///
/// # 示例 | Example
/// ```rust,ignore
/// use ntex::web;
/// use sa_token_plugin_ntex::AuthMiddleware;
///
/// let app = web::App::new()
///     .wrap(AuthMiddleware)
///     .route("/user", web::get().to(user_handler));
/// ```
pub struct AuthMiddleware;

impl<S> Middleware<S> for AuthMiddleware {
    type Service = AuthMiddlewareService<S>;

    fn create(&self, service: S) -> Self::Service {
        AuthMiddlewareService { service }
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, Err> Service<WebRequest<Err>> for AuthMiddlewareService<S>
where
    S: Service<WebRequest<Err>, Response = WebResponse, Error = Error>,
    Err: ErrorRenderer,
{
    type Response = WebResponse;
    type Error = Error;

    async fn call(&self, req: WebRequest<Err>, ctx: ServiceCtx<'_, Self>) -> Result<Self::Response, Self::Error> {
        // 中文 | English
        // 从请求头中获取 token | Get token from request headers
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .map(|s| s.to_string());
        
        if let Some(token_str) = token {
            // 中文 | English
            // 验证 token 是否有效 | Verify if token is valid
            use sa_token_core::TokenValue;
            let token_value = TokenValue::from(token_str.clone());
            if StpUtil::is_login(&token_value).await {
                // 中文 | English
                // Token 有效，继续处理请求 | Token valid, continue processing
                if let Ok(login_id) = StpUtil::get_login_id(&token_value).await {
                    req.extensions_mut().insert(login_id);
                    return ctx.call(&self.service, req).await;
                }
            }
        }
        
        // 中文 | English
        // Token 无效，返回 401 | Token invalid, return 401
        use ntex::web::error::InternalError;
        use ntex::web::Error as WebError;
        Err(WebError::from(InternalError::new(
            "Unauthorized",
            ntex::http::StatusCode::UNAUTHORIZED,
        )))
    }
}

/// 中文 | English
/// 权限验证中间件 - 验证用户是否拥有指定权限 | Permission middleware - verify if user has specified permissions
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

impl<S> Middleware<S> for PermissionMiddleware {
    type Service = PermissionMiddlewareService<S>;

    fn create(&self, service: S) -> Self::Service {
        PermissionMiddlewareService {
            service,
            permission: self.permission.clone(),
        }
    }
}

pub struct PermissionMiddlewareService<S> {
    service: S,
    permission: String,
}

impl<S, Err> Service<WebRequest<Err>> for PermissionMiddlewareService<S>
where
    S: Service<WebRequest<Err>, Response = WebResponse, Error = Error>,
    Err: ErrorRenderer,
{
    type Response = WebResponse;
    type Error = Error;

    async fn call(&self, req: WebRequest<Err>, ctx: ServiceCtx<'_, Self>) -> Result<Self::Response, Self::Error> {
        // 中文 | English
        // 从扩展数据获取 login_id | Get login_id from extensions
        let has_permission = if let Some(login_id) = req.extensions().get::<String>() {
            // 中文 | English
            // 验证权限 | Verify permission
            StpUtil::has_permission(login_id, &self.permission).await
        } else {
            false
        };
        
        if has_permission {
            return ctx.call(&self.service, req).await;
        }
        
        // 中文 | English
        // 无权限，返回 403 | No permission, return 403
        use ntex::web::error::InternalError;
        use ntex::web::Error as WebError;
        Err(WebError::from(InternalError::new(
            "Forbidden",
            ntex::http::StatusCode::FORBIDDEN,
        )))
    }
}

