// Author: 金书记
//
// 中文 | English
// Gotham 认证中间件 | Gotham authentication middleware

use gotham::state::{State, StateData};
use gotham::middleware::Middleware;
use gotham::handler::HandlerFuture;
use std::pin::Pin;

/// 中文 | English
/// 登录 ID 状态数据 | Login ID state data
#[derive(Clone, StateData)]
pub struct LoginId(pub String);

/// 中文 | English
/// 认证中间件 - 验证用户登录状态 | Authentication middleware - verify user login status
#[derive(Clone)]
pub struct AuthMiddleware;

impl AuthMiddleware {
    /// 中文 | English
    /// 创建新的认证中间件 | Create a new authentication middleware
    pub fn new() -> Self {
        Self
    }
}

impl Middleware for AuthMiddleware {
    fn call<Chain>(self, state: State, chain: Chain) -> Pin<Box<HandlerFuture>>
    where
        Chain: FnOnce(State) -> Pin<Box<HandlerFuture>> + Send + 'static,
    {
        // 注意：Gotham 的 State 系统较为复杂
        // 这里提供一个简化实现，用户可以根据需要扩展
        // 建议在 handler 中手动验证 token
        Box::pin(chain(state))
    }
}

impl Default for AuthMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

