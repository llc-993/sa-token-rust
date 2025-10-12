// Author: 金书记
//
//! # sa-token-plugin-poem
//! 
//! Poem 框架集成插件
//! 
//! ## 功能特性
//! 
//! - 🔐 完整的认证和授权支持
//! - 🚀 高性能异步中间件
//! - 🎯 灵活的 Token 提取器
//! - 🛠 易于集成和使用
//! 
//! ## 使用示例
//! 
//! ### 基础使用
//! 
//! ```rust,ignore
//! use std::sync::Arc;
//! use poem::{Route, Server, listener::TcpListener, handler, web::Data};
//! use sa_token_plugin_poem::{SaTokenState, SaTokenMiddleware, SaTokenExtractor};
//! use sa_token_storage_memory::MemoryStorage;
//! 
//! #[handler]
//! async fn user_info(token: SaTokenExtractor) -> String {
//!     format!("User ID: {}", token.login_id())
//! }
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), std::io::Error> {
//!     // 创建 sa-token 状态
//!     let sa_token_state = SaTokenState::builder()
//!         .storage(Arc::new(MemoryStorage::new()))
//!         .token_name("Authorization")
//!         .timeout(7200)
//!         .build();
//!     
//!     // 初始化全局 StpUtil
//!     sa_token_core::StpUtil::init_manager((*sa_token_state.manager).clone());
//!     
//!     // 创建路由
//!     let app = Route::new()
//!         .at("/api/user/info", poem::get(user_info))
//!         .with(SaTokenMiddleware::new(sa_token_state.manager.clone()))
//!         .data(sa_token_state);
//!     
//!     // 启动服务器
//!     Server::new(TcpListener::bind("127.0.0.1:3000"))
//!         .run(app)
//!         .await
//! }
//! ```

pub mod adapter;
pub mod middleware;
pub mod extractor;

pub use middleware::{SaTokenMiddleware, SaCheckLoginMiddleware};
pub use extractor::{SaTokenExtractor, OptionalSaTokenExtractor, LoginIdExtractor};
pub use adapter::{PoemRequestAdapter, PoemResponseAdapter};

use std::sync::Arc;
use sa_token_core::{SaTokenManager, SaTokenConfig};
use sa_token_adapter::storage::SaStorage;

/// Poem 应用状态
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
    /// use sa_token_plugin_poem::SaTokenState;
    /// use sa_token_storage_memory::MemoryStorage;
    /// 
    /// let state = SaTokenState::builder()
    ///     .storage(Arc::new(MemoryStorage::new()))
    ///     .token_name("Authorization")
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
    
    /// 设置是否开启自动续签
    pub fn auto_renew(mut self, enabled: bool) -> Self {
        self.config_builder = self.config_builder.auto_renew(enabled);
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
