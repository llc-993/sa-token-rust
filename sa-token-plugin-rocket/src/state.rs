// Author: 金书记
//
//! Sa-Token 状态管理

use std::sync::Arc;
use sa_token_core::{SaTokenManager, SaTokenListener};
use sa_token_adapter::storage::SaStorage;

/// Rocket 应用状态
#[derive(Clone)]
pub struct SaTokenState {
    pub manager: Arc<SaTokenManager>,
}

impl SaTokenState {
    /// 创建新的 Sa-Token 状态
    pub fn new(manager: Arc<SaTokenManager>) -> Self {
        Self { manager }
    }
    
    /// 从存储和配置创建状态
    pub fn from_storage_and_config(storage: Arc<dyn SaStorage>, config: sa_token_core::SaTokenConfig) -> Self {
        Self {
            manager: Arc::new(SaTokenManager::new(storage, config)),
        }
    }
    
    /// 创建状态构建器
    pub fn builder() -> SaTokenStateBuilder {
        SaTokenStateBuilder::new()
    }
}

/// 状态构建器
#[derive(Default)]
pub struct SaTokenStateBuilder {
    config_builder: sa_token_core::config::SaTokenConfigBuilder,
    listeners: Vec<Arc<dyn SaTokenListener>>,
}

impl SaTokenStateBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 设置存储
    pub fn storage(mut self, storage: Arc<dyn SaStorage>) -> Self {
        self.config_builder = self.config_builder.storage(storage);
        self
    }
    
    /// 设置 token 名称
    pub fn token_name(mut self, name: impl Into<String>) -> Self {
        self.config_builder = self.config_builder.token_name(name);
        self
    }
    
    /// 设置超时时间（秒）
    pub fn timeout(mut self, timeout: i64) -> Self {
        self.config_builder = self.config_builder.timeout(timeout);
        self
    }
    
    /// 设置活动超时时间（秒）
    pub fn active_timeout(mut self, timeout: i64) -> Self {
        self.config_builder = self.config_builder.active_timeout(timeout);
        self
    }
    
    /// 设置是否开启自动续签
    pub fn auto_renew(mut self, enabled: bool) -> Self {
        self.config_builder = self.config_builder.auto_renew(enabled);
        self
    }
    
    /// 设置是否允许并发登录
    pub fn is_concurrent(mut self, concurrent: bool) -> Self {
        self.config_builder = self.config_builder.is_concurrent(concurrent);
        self
    }
    
    /// 设置是否共享 token
    pub fn is_share(mut self, share: bool) -> Self {
        self.config_builder = self.config_builder.is_share(share);
        self
    }
    
    /// 设置 Token 风格
    pub fn token_style(mut self, style: sa_token_core::config::TokenStyle) -> Self {
        self.config_builder = self.config_builder.token_style(style);
        self
    }
    
    /// 设置 token 前缀
    pub fn token_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.config_builder = self.config_builder.token_prefix(prefix);
        self
    }
    
    /// 设置 JWT 密钥
    pub fn jwt_secret_key(mut self, key: impl Into<String>) -> Self {
        self.config_builder = self.config_builder.jwt_secret_key(key);
        self
    }
    
    /// 添加事件监听器
    pub fn listener(mut self, listener: Arc<dyn SaTokenListener>) -> Self {
        self.listeners.push(listener);
        self
    }
    
    /// 添加多个事件监听器
    pub fn listeners(mut self, listeners: Vec<Arc<dyn SaTokenListener>>) -> Self {
        self.listeners.extend(listeners);
        self
    }
    
    /// 构建 Sa-Token 状态
    pub fn build(self) -> SaTokenState {
        // config_builder.build() 已经自动初始化了 StpUtil
        // config_builder.build() already auto-initializes StpUtil
        let manager = self.config_builder.build();
        
        // 注册事件监听器
        for listener in self.listeners {
            manager.event_bus().register(listener);
        }
        
        // 直接创建 SaTokenState，config_builder.build() 已经初始化了 StpUtil
        // Create SaTokenState directly, config_builder.build() already initialized StpUtil
        SaTokenState {
            manager: Arc::new(manager),
        }
    }
}
