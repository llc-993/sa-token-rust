// Author: 金书记
//
//! # sa-token-plugin-axum
//! 
//! Axum框架集成插件
//! 
//! ## 使用示例
//! 
//! ```rust,ignore
//! use axum::{Router, routing::get};
//! use sa_token_plugin_axum::{SaTokenLayer, SaTokenState};
//! use sa_token_storage_memory::MemoryStorage;
//! use sa_token_core::SaTokenConfig;
//! 
//! #[tokio::main]
//! async fn main() {
//!     let storage = Arc::new(MemoryStorage::new());
//!     let config = SaTokenConfig::default();
//!     let state = SaTokenState::new(storage, config);
//!     
//!     let app = Router::new()
//!         .route("/api/user", get(user_info))
//!         .layer(SaTokenLayer::new(state.clone()))
//!         .with_state(state);
//!     
//!     // 启动服务器...
//! }
//! ```

pub mod layer;
pub mod extractor;
pub mod middleware;
pub mod adapter;

pub use layer::SaTokenLayer;
pub use extractor::SaTokenExtractor;
pub use middleware::SaTokenMiddleware;

use std::sync::Arc;
use sa_token_core::{SaTokenManager, SaTokenConfig};
use sa_token_adapter::storage::SaStorage;

/// Axum应用状态
#[derive(Clone)]
pub struct SaTokenState {
    pub manager: Arc<SaTokenManager>,
}

impl SaTokenState {
    /// 从存储和配置创建状态
    pub fn new(storage: Arc<dyn SaStorage>, config: SaTokenConfig) -> Self {
        Self {
            manager: Arc::new(SaTokenManager::new(storage, config)),
        }
    }
    
    /// 从 SaTokenManager 创建状态
    pub fn from_manager(manager: SaTokenManager) -> Self {
        // 自动初始化全局 StpUtil
        sa_token_core::StpUtil::init_manager(manager.clone());
        
        Self {
            manager: Arc::new(manager),
        }
    }
    
    /// 使用构建器模式创建状态
    /// 
    /// # 示例
    /// ```rust,ignore
    /// use std::sync::Arc;
    /// use sa_token_plugin_axum::SaTokenState;
    /// use sa_token_storage_memory::MemoryStorage;
    /// use sa_token_core::SaTokenConfig;
    /// 
    /// let state = SaTokenState::builder()
    ///     .storage(Arc::new(MemoryStorage::new()))
    ///     .timeout(7200)
    ///     .build();
    /// ```
    pub fn builder() -> SaTokenStateBuilder {
        SaTokenStateBuilder::default()
    }
}

/// SaTokenState 构建器
#[derive(Default)]
pub struct SaTokenStateBuilder {
    config_builder: sa_token_core::config::SaTokenConfigBuilder,
}

impl SaTokenStateBuilder {
    pub fn token_name(mut self, name: impl Into<String>) -> Self {
        self.config_builder = self.config_builder.token_name(name);
        self
    }
    
    pub fn timeout(mut self, timeout: i64) -> Self {
        self.config_builder = self.config_builder.timeout(timeout);
        self
    }
    
    pub fn active_timeout(mut self, timeout: i64) -> Self {
        self.config_builder = self.config_builder.active_timeout(timeout);
        self
    }
    
    pub fn is_concurrent(mut self, concurrent: bool) -> Self {
        self.config_builder = self.config_builder.is_concurrent(concurrent);
        self
    }
    
    pub fn is_share(mut self, share: bool) -> Self {
        self.config_builder = self.config_builder.is_share(share);
        self
    }
    
    pub fn token_style(mut self, style: sa_token_core::config::TokenStyle) -> Self {
        self.config_builder = self.config_builder.token_style(style);
        self
    }
    
    pub fn token_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.config_builder = self.config_builder.token_prefix(prefix);
        self
    }
    
    pub fn jwt_secret_key(mut self, key: impl Into<String>) -> Self {
        self.config_builder = self.config_builder.jwt_secret_key(key);
        self
    }
    
    pub fn storage(mut self, storage: Arc<dyn SaStorage>) -> Self {
        self.config_builder = self.config_builder.storage(storage);
        self
    }
    
    pub fn build(self) -> SaTokenState {
        let manager = self.config_builder.build();
        SaTokenState::from_manager(manager)
    }
}
