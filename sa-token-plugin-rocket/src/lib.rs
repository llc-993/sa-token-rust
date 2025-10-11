// Author: 金书记
//
//! # sa-token-plugin-rocket
//! 
//! Rocket框架集成插件

pub mod middleware;
pub mod extractor;
pub mod adapter;

pub use middleware::SaTokenFairing;
pub use extractor::{SaTokenGuard, OptionalSaTokenGuard, LoginIdGuard};
pub use adapter::{RocketRequestAdapter, RocketResponseAdapter};

use std::sync::Arc;
use sa_token_core::{SaTokenManager, SaTokenConfig};
use sa_token_adapter::storage::SaStorage;

/// Rocket 应用状态
#[derive(Clone)]
pub struct SaTokenState {
    pub manager: Arc<SaTokenManager>,
}

impl SaTokenState {
    /// 创建状态构建器
    pub fn builder() -> SaTokenStateBuilder {
        SaTokenStateBuilder::new()
    }
}

/// 状态构建器
pub struct SaTokenStateBuilder {
    storage: Option<Arc<dyn SaStorage>>,
    token_name: Option<String>,
    timeout: Option<i64>,
    active_timeout: Option<i64>,
}

impl SaTokenStateBuilder {
    pub fn new() -> Self {
        Self {
            storage: None,
            token_name: None,
            timeout: None,
            active_timeout: None,
        }
    }
    
    pub fn storage(mut self, storage: Arc<dyn SaStorage>) -> Self {
        self.storage = Some(storage);
        self
    }
    
    pub fn token_name(mut self, name: impl Into<String>) -> Self {
        self.token_name = Some(name.into());
        self
    }
    
    pub fn timeout(mut self, timeout: i64) -> Self {
        self.timeout = Some(timeout);
        self
    }
    
    pub fn active_timeout(mut self, timeout: i64) -> Self {
        self.active_timeout = Some(timeout);
        self
    }
    
    pub fn build(self) -> SaTokenState {
        let storage = self.storage
            .expect("Storage is required");
        
        let mut config = SaTokenConfig::default();
        if let Some(name) = self.token_name {
            config.token_name = name;
        }
        if let Some(timeout) = self.timeout {
            config.timeout = timeout;
        }
        if let Some(active_timeout) = self.active_timeout {
            config.active_timeout = active_timeout;
        }
        
        let manager = SaTokenManager::new(storage, config);
        
        // 自动初始化全局 StpUtil
        sa_token_core::StpUtil::init_manager(manager.clone());
        
        SaTokenState {
            manager: Arc::new(manager),
        }
    }
}

impl Default for SaTokenStateBuilder {
    fn default() -> Self {
        Self::new()
    }
}
