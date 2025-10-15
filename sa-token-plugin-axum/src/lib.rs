// Author: 金书记
//
//! # sa-token-plugin-axum
//! 
//! Axum框架集成插件 - 一站式认证授权解决方案
//! 
//! ## 快速开始
//! 
//! 只需要导入这一个包，即可使用所有功能：
//! 
//! ```toml
//! [dependencies]
//! sa-token-plugin-axum = "0.1.3"  # 默认使用内存存储
//! # 或者使用 Redis 存储
//! sa-token-plugin-axum = { version = "0.1.3", features = ["redis"] }
//! ```
//! 
//! ## 使用示例
//! 
//! ```rust,ignore
//! use axum::{Router, routing::get};
//! use sa_token_plugin_axum::*;  // 一次性导入所有功能
//! 
//! #[tokio::main]
//! async fn main() {
//!     // 1. 初始化（使用内存存储，已重新导出）
//!     let state = SaTokenState::builder()
//!         .storage(Arc::new(MemoryStorage::new()))
//!         .timeout(7200)
//!         .build();
//!     
//!     // 2. 创建路由
//!     let app = Router::new()
//!         .route("/api/user", get(user_info))
//!         .layer(SaTokenLayer::new(state.clone()))
//!         .with_state(state);
//!     
//!     // 3. 使用宏检查权限
//!     #[sa_check_login]
//!     async fn user_info() -> String {
//!         "User info".to_string()
//!     }
//! }
//! ```

pub mod layer;
pub mod extractor;
pub mod middleware;
pub mod adapter;

// ============================================================================
// Axum 框架集成（本插件特有）
// ============================================================================
pub use layer::SaTokenLayer;
pub use extractor::SaTokenExtractor;
pub use middleware::SaTokenMiddleware;

// ============================================================================
// 重新导出核心功能（sa-token-core）
// ============================================================================
pub use sa_token_core::{
    // 核心管理器
    SaTokenManager, StpUtil,
    
    // 配置
    SaTokenConfig,
    config::TokenStyle,
    
    // Token 相关
    TokenValue, TokenInfo,
    
    // 会话管理
    SaSession,
    
    // 权限
    PermissionChecker,
    
    // 错误处理
    SaTokenError,
    
    // 事件系统
    SaTokenEvent, SaTokenListener, SaTokenEventBus, LoggingListener,
    
    // JWT 支持
    JwtManager, JwtClaims, JwtAlgorithm,
    
    // OAuth2 支持
    OAuth2Manager, OAuth2Client, AuthorizationCode, AccessToken, OAuth2TokenInfo,
    
    // 安全特性
    NonceManager, RefreshTokenManager,
    
    // WebSocket 认证
    WsAuthManager, WsAuthInfo, WsTokenExtractor, DefaultWsTokenExtractor,
    
    // 在线用户管理
    OnlineManager, OnlineUser, PushMessage, MessageType, MessagePusher, InMemoryPusher,
    
    // 分布式会话
    DistributedSessionManager, DistributedSession, DistributedSessionStorage, 
    ServiceCredential, InMemoryDistributedStorage,
};

// ============================================================================
// 重新导出适配器接口（sa-token-adapter）
// ============================================================================
pub use sa_token_adapter::{
    storage::SaStorage,
    framework::FrameworkAdapter,
};

// ============================================================================
// 重新导出宏（sa-token-macro）
// ============================================================================
pub use sa_token_macro::{
    sa_check_login,
    sa_check_permission,
    sa_check_role,
    sa_check_permissions_and,
    sa_check_permissions_or,
    sa_check_roles_and,
    sa_check_roles_or,
    sa_ignore,
};

// ============================================================================
// 重新导出存储实现（根据 feature 条件编译）
// ============================================================================

/// 内存存储（默认启用）
#[cfg(feature = "memory")]
pub use sa_token_storage_memory::MemoryStorage;

/// Redis 存储
#[cfg(feature = "redis")]
pub use sa_token_storage_redis::RedisStorage;

/// 数据库存储
#[cfg(feature = "database")]
pub use sa_token_storage_database::DatabaseStorage;

use std::sync::Arc;

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
