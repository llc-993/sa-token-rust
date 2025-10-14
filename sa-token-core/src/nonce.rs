// Author: 金书记
//
//! Nonce Manager | Nonce 管理器
//!
//! Prevents replay attacks by tracking used nonces
//! 通过跟踪已使用的 nonce 来防止重放攻击

use std::sync::Arc;
use chrono::{DateTime, Utc};
use sa_token_adapter::storage::SaStorage;
use crate::error::{SaTokenError, SaTokenResult};
use uuid::Uuid;

/// Nonce Manager | Nonce 管理器
///
/// Manages nonce generation and validation to prevent replay attacks
/// 管理 nonce 的生成和验证以防止重放攻击
#[derive(Clone)]
pub struct NonceManager {
    storage: Arc<dyn SaStorage>,
    timeout: i64,
}

impl NonceManager {
    /// Create new nonce manager | 创建新的 nonce 管理器
    ///
    /// # Arguments | 参数
    ///
    /// * `storage` - Storage backend | 存储后端
    /// * `timeout` - Nonce validity period in seconds | Nonce 有效期（秒）
    pub fn new(storage: Arc<dyn SaStorage>, timeout: i64) -> Self {
        Self { storage, timeout }
    }

    /// Generate a new nonce | 生成新的 nonce
    ///
    /// # Returns | 返回
    ///
    /// Unique nonce string | 唯一的 nonce 字符串
    pub fn generate(&self) -> String {
        format!("nonce_{}_{}", Utc::now().timestamp_millis(), Uuid::new_v4().simple())
    }

    /// Store and mark nonce as used | 存储并标记 nonce 为已使用
    ///
    /// # Arguments | 参数
    ///
    /// * `nonce` - Nonce to store | 要存储的 nonce
    /// * `login_id` - Associated user ID | 关联的用户ID
    pub async fn store(&self, nonce: &str, login_id: &str) -> SaTokenResult<()> {
        let key = format!("sa:nonce:{}", nonce);
        let value = serde_json::json!({
            "login_id": login_id,
            "created_at": Utc::now().to_rfc3339(),
        }).to_string();

        let ttl = Some(std::time::Duration::from_secs(self.timeout as u64));
        self.storage.set(&key, &value, ttl)
            .await
            .map_err(|e| SaTokenError::StorageError(e.to_string()))?;

        Ok(())
    }

    /// Validate nonce and ensure it hasn't been used | 验证 nonce 并确保未被使用
    ///
    /// # Arguments | 参数
    ///
    /// * `nonce` - Nonce to validate | 要验证的 nonce
    ///
    /// # Returns | 返回
    ///
    /// `Ok(true)` if valid and not used, `Ok(false)` if already used
    /// 如果有效且未使用返回 `Ok(true)`，如果已使用返回 `Ok(false)`
    pub async fn validate(&self, nonce: &str) -> SaTokenResult<bool> {
        let key = format!("sa:nonce:{}", nonce);
        
        // Check if nonce exists (has been used)
        // 检查 nonce 是否存在（已被使用）
        let exists = self.storage.get(&key)
            .await
            .map_err(|e| SaTokenError::StorageError(e.to_string()))?
            .is_some();

        Ok(!exists) // Valid if NOT exists | 不存在则有效
    }

    /// Validate and consume nonce in one operation | 一次操作验证并消费 nonce
    ///
    /// # Arguments | 参数
    ///
    /// * `nonce` - Nonce to validate and consume | 要验证和消费的 nonce
    /// * `login_id` - Associated user ID | 关联的用户ID
    ///
    /// # Returns | 返回
    ///
    /// `Ok(())` if valid, error if already used or invalid
    /// 如果有效返回 `Ok(())`，如果已使用或无效返回错误
    pub async fn validate_and_consume(&self, nonce: &str, login_id: &str) -> SaTokenResult<()> {
        if !self.validate(nonce).await? {
            return Err(SaTokenError::NonceAlreadyUsed);
        }

        self.store(nonce, login_id).await?;
        Ok(())
    }

    /// Extract timestamp from nonce and check if it's within valid time window
    /// 从 nonce 中提取时间戳并检查是否在有效时间窗口内
    ///
    /// # Arguments | 参数
    ///
    /// * `nonce` - Nonce to check | 要检查的 nonce
    /// * `window_seconds` - Time window in seconds | 时间窗口（秒）
    pub fn check_timestamp(&self, nonce: &str, window_seconds: i64) -> SaTokenResult<bool> {
        // Nonce format: nonce_TIMESTAMP_UUID
        let parts: Vec<&str> = nonce.split('_').collect();
        if parts.len() < 2 || parts[0] != "nonce" {
            return Err(SaTokenError::InvalidNonceFormat);
        }

        let timestamp_ms = parts[1].parse::<i64>()
            .map_err(|_| SaTokenError::InvalidNonceTimestamp)?;

        let nonce_time = DateTime::from_timestamp_millis(timestamp_ms)
            .ok_or_else(|| SaTokenError::InvalidNonceTimestamp)?;

        let now = Utc::now();
        let diff = (now - nonce_time).num_seconds().abs();

        Ok(diff <= window_seconds)
    }

    /// Clean up expired nonces (implementation depends on storage)
    /// 清理过期的 nonce（实现依赖于存储）
    ///
    /// Note: Most storage backends automatically expire keys with TTL
    /// 注意：大多数存储后端会自动过期带 TTL 的键
    pub async fn cleanup_expired(&self) -> SaTokenResult<()> {
        // Storage with TTL support will auto-cleanup
        // 支持 TTL 的存储会自动清理
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sa_token_storage_memory::MemoryStorage;

    #[tokio::test]
    async fn test_nonce_generation() {
        let storage = Arc::new(MemoryStorage::new());
        let nonce_mgr = NonceManager::new(storage, 60);

        let nonce1 = nonce_mgr.generate();
        let nonce2 = nonce_mgr.generate();

        assert_ne!(nonce1, nonce2);
        assert!(nonce1.starts_with("nonce_"));
    }

    #[tokio::test]
    async fn test_nonce_validation() {
        let storage = Arc::new(MemoryStorage::new());
        let nonce_mgr = NonceManager::new(storage, 60);

        let nonce = nonce_mgr.generate();

        // First validation should succeed
        assert!(nonce_mgr.validate(&nonce).await.unwrap());

        // Store the nonce
        nonce_mgr.store(&nonce, "user_123").await.unwrap();

        // Second validation should fail (already used)
        assert!(!nonce_mgr.validate(&nonce).await.unwrap());
    }

    #[tokio::test]
    async fn test_nonce_validate_and_consume() {
        let storage = Arc::new(MemoryStorage::new());
        let nonce_mgr = NonceManager::new(storage, 60);

        let nonce = nonce_mgr.generate();

        // First use should succeed
        nonce_mgr.validate_and_consume(&nonce, "user_123").await.unwrap();

        // Second use should fail
        let result = nonce_mgr.validate_and_consume(&nonce, "user_123").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_nonce_timestamp_check() {
        let storage = Arc::new(MemoryStorage::new());
        let nonce_mgr = NonceManager::new(storage, 60);

        let nonce = nonce_mgr.generate();

        // Should be within 60 seconds
        assert!(nonce_mgr.check_timestamp(&nonce, 60).unwrap());

        // Should also be within 1 second
        assert!(nonce_mgr.check_timestamp(&nonce, 1).unwrap());
    }
}

