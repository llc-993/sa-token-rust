// Author: 金书记
//
//! Token 管理器 - sa-token 的核心入口

use std::sync::Arc;
use std::collections::HashMap;
use chrono::{Duration, Utc};
use tokio::sync::RwLock;
use sa_token_adapter::storage::SaStorage;
use crate::config::SaTokenConfig;
use crate::error::{SaTokenError, SaTokenResult};
use crate::token::{TokenInfo, TokenValue, TokenGenerator};
use crate::session::SaSession;
use crate::event::{SaTokenEventBus, SaTokenEvent};

/// sa-token 管理器
#[derive(Clone)]
pub struct SaTokenManager {
    pub(crate) storage: Arc<dyn SaStorage>,
    pub config: SaTokenConfig,
    /// 用户权限映射 user_id -> permissions
    pub(crate) user_permissions: Arc<RwLock<HashMap<String, Vec<String>>>>,
    /// 用户角色映射 user_id -> roles
    pub(crate) user_roles: Arc<RwLock<HashMap<String, Vec<String>>>>,
    /// 事件总线
    pub(crate) event_bus: SaTokenEventBus,
}

impl SaTokenManager {
    /// 创建新的管理器实例
    pub fn new(storage: Arc<dyn SaStorage>, config: SaTokenConfig) -> Self {
        Self { 
            storage, 
            config,
            user_permissions: Arc::new(RwLock::new(HashMap::new())),
            user_roles: Arc::new(RwLock::new(HashMap::new())),
            event_bus: SaTokenEventBus::new(),
        }
    }
    
    /// 获取事件总线的引用
    pub fn event_bus(&self) -> &SaTokenEventBus {
        &self.event_bus
    }
    
    /// 登录：为指定账号创建 token
    pub async fn login(&self, login_id: impl Into<String>) -> SaTokenResult<TokenValue> {
        let login_id = login_id.into();
        
        // 生成 token（支持 JWT）
        let token = TokenGenerator::generate_with_login_id(&self.config, &login_id);
        
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
        
        // 保存 login_id 到 token 的映射（用于根据 login_id 查找 token）
        let login_token_key = format!("sa:login:token:{}", login_id);
        self.storage.set(&login_token_key, token.as_str(), self.config.timeout_duration()).await
            .map_err(|e| SaTokenError::StorageError(e.to_string()))?;
        
        // 如果不允许并发登录，踢掉之前的 token
        if !self.config.is_concurrent {
            self.logout_by_login_id(&login_id).await?;
        }
        
        // 触发登录事件
        let event = SaTokenEvent::login(login_id.clone(), token.as_str())
            .with_login_type(&token_info.login_type);
        self.event_bus.publish(event).await;
        
        Ok(token)
    }
    
    /// 登出：删除指定 token
    pub async fn logout(&self, token: &TokenValue) -> SaTokenResult<()> {
        // 先从存储获取 token 信息，用于触发事件（不调用 get_token_info 避免递归）
        let key = format!("sa:token:{}", token.as_str());
        let token_info_str = self.storage.get(&key).await
            .map_err(|e| SaTokenError::StorageError(e.to_string()))?;
        
        let token_info = if let Some(value) = token_info_str {
            serde_json::from_str::<TokenInfo>(&value).ok()
        } else {
            None
        };
        
        // 删除 token
        self.storage.delete(&key).await
            .map_err(|e| SaTokenError::StorageError(e.to_string()))?;
        
        // 触发登出事件
        if let Some(info) = token_info {
            let event = SaTokenEvent::logout(info.login_id, token.as_str())
                .with_login_type(&info.login_type);
            self.event_bus.publish(event).await;
        }
        
        Ok(())
    }
    
    /// 根据登录 ID 登出所有 token
    pub async fn logout_by_login_id(&self, _login_id: &str) -> SaTokenResult<()> {
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
        
        // 如果开启了自动续签，则自动续签
        // 注意：为了避免递归调用 get_token_info，这里直接更新过期时间
        if self.config.auto_renew {
            let renew_timeout = if self.config.active_timeout > 0 {
                self.config.active_timeout
            } else {
                self.config.timeout
            };
            
            // 直接续签（不递归调用 get_token_info）
            let _ = self.renew_timeout_internal(token, renew_timeout, &token_info).await;
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
    
    /// 续期 token（重置过期时间）
    pub async fn renew_timeout(
        &self,
        token: &TokenValue,
        timeout_seconds: i64,
    ) -> SaTokenResult<()> {
        let token_info = self.get_token_info(token).await?;
        self.renew_timeout_internal(token, timeout_seconds, &token_info).await
    }
    
    /// 内部续期方法（避免递归调用 get_token_info）
    async fn renew_timeout_internal(
        &self,
        token: &TokenValue,
        timeout_seconds: i64,
        token_info: &TokenInfo,
    ) -> SaTokenResult<()> {
        let mut new_token_info = token_info.clone();
        
        // 设置新的过期时间
        use chrono::{Utc, Duration};
        let new_expire_time = Utc::now() + Duration::seconds(timeout_seconds);
        new_token_info.expire_time = Some(new_expire_time);
        
        // 保存更新后的 token 信息
        let key = format!("sa:token:{}", token.as_str());
        let value = serde_json::to_string(&new_token_info)
            .map_err(|e| SaTokenError::SerializationError(e))?;
        
        let timeout = std::time::Duration::from_secs(timeout_seconds as u64);
        self.storage.set(&key, &value, Some(timeout)).await
            .map_err(|e| SaTokenError::StorageError(e.to_string()))?;
        
        Ok(())
    }
    
    /// 踢人下线
    pub async fn kick_out(&self, login_id: &str) -> SaTokenResult<()> {
        // 先获取 token，用于触发事件
        let token_result = self.storage.get(&format!("sa:login:token:{}", login_id)).await;
        
        self.logout_by_login_id(login_id).await?;
        self.delete_session(login_id).await?;
        
        // 触发踢出下线事件
        if let Ok(Some(token_str)) = token_result {
            let event = SaTokenEvent::kick_out(login_id, token_str);
            self.event_bus.publish(event).await;
        }
        
        Ok(())
    }
}
