// Author: 金书记
//
//! 上下文模块 - 用于在请求处理过程中传递 token 信息
//! 
//! 注意：在实际应用中，建议通过框架的请求扩展（如 Axum 的 Extension）
//! 来传递上下文，而不是使用 thread_local。这里提供的是一个简单的实现。

use std::sync::Arc;
use std::cell::RefCell;
use crate::token::{TokenInfo, TokenValue};

thread_local! {
    static CONTEXT: RefCell<Option<SaTokenContext>> = RefCell::new(None);
}

/// sa-token 上下文 | sa-token Context
/// 
/// 用于在请求处理过程中传递 Token 相关信息
/// Used to pass token-related information during request processing
/// 
/// # 字段说明 | Field Description
/// - `token`: 当前请求的 token | Current request's token
/// - `token_info`: Token 详细信息 | Token detailed information
/// - `login_id`: 登录用户 ID | Logged-in user ID
#[derive(Debug, Clone)]
pub struct SaTokenContext {
    /// 当前请求的 token | Current request's token
    pub token: Option<TokenValue>,
    
    /// 当前请求的 token 信息 | Current request's token info
    pub token_info: Option<Arc<TokenInfo>>,
    
    /// 登录 ID | Login ID
    pub login_id: Option<String>,
}

impl SaTokenContext {
    pub fn new() -> Self {
        Self {
            token: None,
            token_info: None,
            login_id: None,
        }
    }
    
    /// 设置当前上下文 | Set Current Context
    /// 
    /// # 参数 | Parameters
    /// - `ctx`: 要设置的上下文 | Context to set
    pub fn set_current(ctx: SaTokenContext) {
        CONTEXT.with(|c| {
            *c.borrow_mut() = Some(ctx);
        });
    }
    
    /// 获取当前上下文 | Get Current Context
    /// 
    /// # 返回 | Returns
    /// 当前线程的上下文，如果不存在则返回 None
    /// Current thread's context, or None if not exists
    pub fn get_current() -> Option<SaTokenContext> {
        CONTEXT.with(|c| {
            c.borrow().clone()
        })
    }
    
    /// 清除当前上下文 | Clear Current Context
    /// 
    /// 清除当前线程的上下文信息
    /// Clear current thread's context information
    pub fn clear() {
        CONTEXT.with(|c| {
            *c.borrow_mut() = None;
        });
    }
}

impl Default for SaTokenContext {
    fn default() -> Self {
        Self::new()
    }
}
