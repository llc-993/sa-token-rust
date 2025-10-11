//! StpUtil - sa-token 便捷工具类
//! 
//! 提供类似 Java 版 StpUtil 的静态方法，方便进行认证和权限操作
//! 
//! ## 使用示例
//! 
//! ```rust,ignore
//! use sa_token_core::StpUtil;
//! 
//! // 登录
//! let token = StpUtil::login(&manager, "user_123").await?;
//! 
//! // 检查登录
//! let is_login = StpUtil::is_login(&manager, &token).await;
//! 
//! // 获取登录ID
//! let login_id = StpUtil::get_login_id(&manager, &token).await?;
//! ```

use crate::{SaTokenManager, SaTokenResult, SaTokenError};
use crate::token::{TokenValue, TokenInfo};
use crate::session::SaSession;

/// StpUtil - 权限认证工具类
/// 
/// 提供便捷的认证和授权操作方法，类似于 Java 版 sa-token 的 StpUtil
pub struct StpUtil;

impl StpUtil {
    // ==================== 登录相关 ====================
    
    /// 会话登录
    /// 
    /// # 参数
    /// - `manager` - SaTokenManager 实例
    /// - `login_id` - 登录ID（通常是用户ID）
    /// 
    /// # 返回
    /// 返回生成的 token
    /// 
    /// # 示例
    /// ```rust,ignore
    /// let token = StpUtil::login(&manager, "user_123").await?;
    /// ```
    pub async fn login(
        manager: &SaTokenManager,
        login_id: impl Into<String>,
    ) -> SaTokenResult<TokenValue> {
        manager.login(login_id).await
    }
    
    /// 会话登出
    /// 
    /// # 参数
    /// - `manager` - SaTokenManager 实例
    /// - `token` - 要登出的 token
    pub async fn logout(
        manager: &SaTokenManager,
        token: &TokenValue,
    ) -> SaTokenResult<()> {
        manager.logout(token).await
    }
    
    /// 踢人下线（根据登录ID）
    /// 
    /// # 参数
    /// - `manager` - SaTokenManager 实例
    /// - `login_id` - 登录ID
    pub async fn kick_out(
        manager: &SaTokenManager,
        login_id: &str,
    ) -> SaTokenResult<()> {
        manager.kick_out(login_id).await
    }
    
    /// 强制登出（根据登录ID）
    pub async fn logout_by_login_id(
        manager: &SaTokenManager,
        login_id: &str,
    ) -> SaTokenResult<()> {
        manager.logout_by_login_id(login_id).await
    }
    
    // ==================== Token 验证 ====================
    
    /// 检查当前 token 是否已登录
    /// 
    /// # 返回
    /// - `true` - 已登录
    /// - `false` - 未登录或 token 已过期
    pub async fn is_login(
        manager: &SaTokenManager,
        token: &TokenValue,
    ) -> bool {
        manager.is_valid(token).await
    }
    
    /// 检查当前 token 是否已登录，如果未登录则抛出异常
    /// 
    /// # 错误
    /// 如果未登录，返回 `SaTokenError::NotLogin` 错误
    pub async fn check_login(
        manager: &SaTokenManager,
        token: &TokenValue,
    ) -> SaTokenResult<()> {
        if !Self::is_login(manager, token).await {
            return Err(SaTokenError::NotLogin);
        }
        Ok(())
    }
    
    /// 获取 token 信息
    pub async fn get_token_info(
        manager: &SaTokenManager,
        token: &TokenValue,
    ) -> SaTokenResult<TokenInfo> {
        manager.get_token_info(token).await
    }
    
    /// 获取当前 token 的登录ID
    /// 
    /// # 返回
    /// 返回登录ID，如果未登录则返回错误
    pub async fn get_login_id(
        manager: &SaTokenManager,
        token: &TokenValue,
    ) -> SaTokenResult<String> {
        let token_info = manager.get_token_info(token).await?;
        Ok(token_info.login_id)
    }
    
    /// 获取当前 token 的登录ID，如果未登录则返回默认值
    pub async fn get_login_id_or_default(
        manager: &SaTokenManager,
        token: &TokenValue,
        default: impl Into<String>,
    ) -> String {
        Self::get_login_id(manager, token)
            .await
            .unwrap_or_else(|_| default.into())
    }
    
    // ==================== Session 会话 ====================
    
    /// 获取当前登录账号的 Session
    pub async fn get_session(
        manager: &SaTokenManager,
        login_id: &str,
    ) -> SaTokenResult<SaSession> {
        manager.get_session(login_id).await
    }
    
    /// 保存 Session
    pub async fn save_session(
        manager: &SaTokenManager,
        session: &SaSession,
    ) -> SaTokenResult<()> {
        manager.save_session(session).await
    }
    
    /// 删除 Session
    pub async fn delete_session(
        manager: &SaTokenManager,
        login_id: &str,
    ) -> SaTokenResult<()> {
        manager.delete_session(login_id).await
    }
    
    /// 在 Session 中设置值
    pub async fn set_session_value<T: serde::Serialize>(
        manager: &SaTokenManager,
        login_id: &str,
        key: &str,
        value: T,
    ) -> SaTokenResult<()> {
        let mut session = manager.get_session(login_id).await?;
        session.set(key, value)?;
        manager.save_session(&session).await
    }
    
    /// 从 Session 中获取值
    pub async fn get_session_value<T: serde::de::DeserializeOwned>(
        manager: &SaTokenManager,
        login_id: &str,
        key: &str,
    ) -> SaTokenResult<Option<T>> {
        let session = manager.get_session(login_id).await?;
        session.get::<T>(key)
    }
    
    // ==================== Token 相关 ====================
    
    /// 创建一个新的 token（但不登录）
    pub fn create_token(token_value: impl Into<String>) -> TokenValue {
        TokenValue::new(token_value.into())
    }
    
    /// 检查 token 格式是否有效（仅检查格式，不检查是否存在于存储中）
    pub fn is_valid_token_format(token: &str) -> bool {
        !token.is_empty() && token.len() >= 16
    }
}

// ==================== 扩展工具方法 ====================

impl StpUtil {
    /// 批量踢人下线
    pub async fn kick_out_batch(
        manager: &SaTokenManager,
        login_ids: &[&str],
    ) -> SaTokenResult<Vec<Result<(), SaTokenError>>> {
        let mut results = Vec::new();
        for login_id in login_ids {
            results.push(manager.kick_out(login_id).await);
        }
        Ok(results)
    }
    
    /// 获取 token 剩余有效时间（秒）
    pub async fn get_token_timeout(
        manager: &SaTokenManager,
        token: &TokenValue,
    ) -> SaTokenResult<Option<i64>> {
        let token_info = manager.get_token_info(token).await?;
        
        if let Some(expire_time) = token_info.expire_time {
            let now = chrono::Utc::now();
            let duration = expire_time.signed_duration_since(now);
            Ok(Some(duration.num_seconds()))
        } else {
            Ok(None) // 永久有效
        }
    }
    
    /// 续期 token（重置过期时间）
    pub async fn renew_timeout(
        manager: &SaTokenManager,
        token: &TokenValue,
        timeout_seconds: i64,
    ) -> SaTokenResult<()> {
        let mut token_info = manager.get_token_info(token).await?;
        
        // 设置新的过期时间
        let new_expire_time = chrono::Utc::now() + chrono::Duration::seconds(timeout_seconds);
        token_info.expire_time = Some(new_expire_time);
        
        // 保存更新后的 token 信息
        let key = format!("sa:token:{}", token.as_str());
        let value = serde_json::to_string(&token_info)
            .map_err(|e| SaTokenError::SerializationError(e))?;
        
        let timeout = std::time::Duration::from_secs(timeout_seconds as u64);
        manager.storage.set(&key, &value, Some(timeout)).await
            .map_err(|e| SaTokenError::StorageError(e.to_string()))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_token_format_validation() {
        assert!(StpUtil::is_valid_token_format("1234567890abcdef"));
        assert!(!StpUtil::is_valid_token_format(""));
        assert!(!StpUtil::is_valid_token_format("short"));
    }
    
    #[test]
    fn test_create_token() {
        let token = StpUtil::create_token("test-token-123");
        assert_eq!(token.as_str(), "test-token-123");
    }
}

