// Author: 金书记
//
// 中文 | English
// Salvo 认证中间件 | Salvo authentication middleware

use salvo::prelude::*;
use sa_token_core::StpUtil;

/// 中文 | English
/// 认证中间件 - 验证用户登录状态 | Authentication middleware - verify user login status
///
/// # 示例 | Example
/// ```rust,ignore
/// use salvo::prelude::*;
/// use sa_token_plugin_salvo::auth_middleware;
///
/// let router = Router::new()
///     .hoop(auth_middleware())
///     .push(Router::with_path("user").get(user_handler));
/// ```
pub fn auth_middleware() -> impl Handler {
    auth_middleware_handler
}

#[handler]
async fn auth_middleware_handler(req: &mut Request, res: &mut Response, depot: &mut Depot, ctrl: &mut FlowCtrl) {
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
            // Token 有效，将 login_id 存入 depot | Token valid, store login_id in depot
            if let Ok(login_id) = StpUtil::get_login_id(&token_value).await {
                depot.insert("login_id", login_id);
                ctrl.call_next(req, depot, res).await;
                return;
            }
        }
    }
    
    // 中文 | English
    // Token 无效，返回 401 | Token invalid, return 401
    res.status_code(StatusCode::UNAUTHORIZED);
    res.render(Text::Json(r#"{"error":"Unauthorized"}"#));
    ctrl.skip_rest();
}

/// 中文 | English
/// 权限验证中间件 - 验证用户是否拥有指定权限 | Permission middleware - verify if user has specified permissions
///
/// # 参数 | Parameters
/// - `permission`: 需要的权限 | Required permission
///
/// # 示例 | Example
/// ```rust,ignore
/// let router = Router::new()
///     .hoop(permission_middleware("user:read"))
///     .push(Router::with_path("user").get(user_handler));
/// ```
pub fn permission_middleware(permission: &'static str) -> impl Handler {
    PermissionMiddleware { permission }
}

struct PermissionMiddleware {
    permission: &'static str,
}

#[handler]
impl PermissionMiddleware {
    async fn handle(&self, req: &mut Request, res: &mut Response, depot: &mut Depot, ctrl: &mut FlowCtrl) {
        // 中文 | English
        // 从 depot 获取 login_id | Get login_id from depot
        if let Ok(login_id) = depot.get::<String>("login_id") {
            // 中文 | English
            // 验证权限 | Verify permission
            if StpUtil::has_permission(login_id, self.permission).await {
                ctrl.call_next(req, depot, res).await;
                return;
            }
        }
        
        // 中文 | English
        // 无权限，返回 403 | No permission, return 403
        res.status_code(StatusCode::FORBIDDEN);
        res.render(Text::Json(r#"{"error":"Forbidden"}"#));
        ctrl.skip_rest();
    }
}

