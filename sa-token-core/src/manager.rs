//! Token 管理器 - sa-token 的核心入口

use std::sync::Arc;
use chrono::{Duration, Utc};
use sa_token_adapter::storage::SaStorage;
use crate::config::SaTokenConfig;
use crate::error::{SaTokenError, SaTokenResult};
use crate::token::{TokenInfo, TokenValue, TokenGenerator};
use crate::session::SaSession;

/// sa-token 管理器
pub struct SaTokenManager {
    storage: Arc<dyn SaStorage>,
    config: SaTokenConfig,
}

impl SaTokenManager {
    /// 创建新的管理器实例
    pub fn new(storage: Arc<dyn SaStorage>, config: SaTokenConfig) -> Self {
        Self { storage, config }
    }
    
    /// 登录：为指定账号创建 token
    pub async fn login(&self, login_id: impl Into<String>) -> SaTokenResult<TokenValue> {
        let login_id = login_id.into();
        
        // 生成 token
        let token = TokenGenerator::generate(&self.config);
        
        // 创建 token 信息
        let mut token_info = TokenInfo::new(token.clone(), login_id.clone());
        token_info.login_type = "default".to_string();
        
        // 设置过期时间
        if let Some(timeout) = self.config.timeout_duration() {
            token_info.expire_time = Some(Utc::now() + Duration::from_std(timeout).unwrap());
        }
        
        // 存储 token 信息
        let key = format!("sa:token:{}", token.as_str());
        let value = serde_json::to_string(&token_info)
            .map_err(|e| SaTokenError::SerializationError(e))?;
        
        self.storage.set(&key, &value, self.config.timeout_duration()).await
            .map_err(|e| SaTokenError::StorageError(e.to_string()))?;
        
        // 如果不允许并发登录，踢掉之前的 token
        if !self.config.is_concurrent {
            self.logout_by_login_id(&login_id).await?;
        }
        
        Ok(token)
    }
    
    /// 登出：删除指定 token
    pub async fn logout(&self, token: &TokenValue) -> SaTokenResult<()> {
        let key = format!("sa:token:{}", token.as_str());
        self.storage.delete(&key).await
            .map_err(|e| SaTokenError::StorageError(e.to_string()))?;
        Ok(())
    }
    
    /// 根据登录 ID 登出所有 token
    pub async fn logout_by_login_id(&self, login_id: &str) -> SaTokenResult<()> {
        // TODO: 实现根据登录 ID 查找所有 token 并删除
        // 这需要维护 login_id -> tokens 的映射
        Ok(())
    }
    
    /// 获取 token 信息
    pub async fn get_token_info(&self, token: &TokenValue) -> SaTokenResult<TokenInfo> {
        let key = format!("sa:token:{}", token.as_str());
        let value = self.storage.get(&key).await
            .map_err(|e| SaTokenError::StorageError(e.to_string()))?
            .ok_or(SaTokenError::TokenNotFound)?;
        
        let token_info: TokenInfo = serde_json::from_str(&value)
            .map_err(|e| SaTokenError::SerializationError(e))?;
        
        // 检查是否过期
        if token_info.is_expired() {
            // 删除过期的 token
            self.logout(token).await?;
            return Err(SaTokenError::TokenExpired);
        }
        
        Ok(token_info)
    }
    
    /// 检查 token 是否有效
    pub async fn is_valid(&self, token: &TokenValue) -> bool {
        self.get_token_info(token).await.is_ok()
    }
    
    /// 获取 session
    pub async fn get_session(&self, login_id: &str) -> SaTokenResult<SaSession> {
        let key = format!("sa:session:{}", login_id);
        let value = self.storage.get(&key).await
            .map_err(|e| SaTokenError::StorageError(e.to_string()))?;
        
        if let Some(value) = value {
            let session: SaSession = serde_json::from_str(&value)
                .map_err(|e| SaTokenError::SerializationError(e))?;
            Ok(session)
        } else {
            Ok(SaSession::new(login_id))
        }
    }
    
    /// 保存 session
    pub async fn save_session(&self, session: &SaSession) -> SaTokenResult<()> {
        let key = format!("sa:session:{}", session.id);
        let value = serde_json::to_string(session)
            .map_err(|e| SaTokenError::SerializationError(e))?;
        
        self.storage.set(&key, &value, None).await
            .map_err(|e| SaTokenError::StorageError(e.to_string()))?;
        
        Ok(())
    }
    
    /// 删除 session
    pub async fn delete_session(&self, login_id: &str) -> SaTokenResult<()> {
        let key = format!("sa:session:{}", login_id);
        self.storage.delete(&key).await
            .map_err(|e| SaTokenError::StorageError(e.to_string()))?;
        Ok(())
    }
    
    /// 踢人下线
    pub async fn kick_out(&self, login_id: &str) -> SaTokenResult<()> {
        self.logout_by_login_id(login_id).await?;
        self.delete_session(login_id).await?;
        Ok(())
    }
}

