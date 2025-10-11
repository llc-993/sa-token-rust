//! 上下文模块 - 用于在请求处理过程中传递 token 信息

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::token::{TokenInfo, TokenValue};

thread_local! {
    static CONTEXT: RwLock<Option<SaTokenContext>> = RwLock::new(None);
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
    pub async fn set_current(ctx: SaTokenContext) {
        CONTEXT.with(|c| async move {
            let mut context = c.write().await;
            *context = Some(ctx);
        }).await;
    }
    
    /// 获取当前上下文
    pub async fn get_current() -> Option<SaTokenContext> {
        CONTEXT.with(|c| async move {
            let context = c.read().await;
            context.clone()
        }).await
    }
    
    /// 清除当前上下文
    pub async fn clear() {
        CONTEXT.with(|c| async move {
            let mut context = c.write().await;
            *context = None;
        }).await;
    }
}

impl Default for SaTokenContext {
    fn default() -> Self {
        Self::new()
    }
}

