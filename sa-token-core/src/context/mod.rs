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

/// sa-token 上下文
#[derive(Debug, Clone)]
pub struct SaTokenContext {
    /// 当前请求的 token
    pub token: Option<TokenValue>,
    
    /// 当前请求的 token 信息
    pub token_info: Option<Arc<TokenInfo>>,
    
    /// 登录 ID
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
    
    /// 设置当前上下文
    pub fn set_current(ctx: SaTokenContext) {
        CONTEXT.with(|c| {
            *c.borrow_mut() = Some(ctx);
        });
    }
    
    /// 获取当前上下文
    pub fn get_current() -> Option<SaTokenContext> {
        CONTEXT.with(|c| {
            c.borrow().clone()
        })
    }
    
    /// 清除当前上下文
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
