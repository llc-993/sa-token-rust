// Author: 金书记
//
// 中文 | English
// Warp 框架集成 | Warp Framework Integration
//
//! # sa-token-plugin-warp
//! 
//! Warp框架集成插件 - 一站式认证授权解决方案
//! Warp framework integration plugin - One-stop authentication and authorization solution
//! 
//! ## 快速开始 | Quick Start
//! 
//! 只需要导入这一个包，即可使用所有功能：
//! Just import this package to use all features:
//! 
//! ```toml
//! [dependencies]
//! sa-token-plugin-warp = "0.1.7"  # 默认使用内存存储 | Default using memory storage
//! # 或者使用 Redis 存储 | Or use Redis storage
//! sa-token-plugin-warp = { version = "0.1.7", features = ["redis"] }
//! ```
//! 
//! ## 使用示例 | Usage Example
//! 
//! ```rust,ignore
//! use warp::Filter;
//! use std::sync::Arc;
//! use sa_token_plugin_warp::*;  // 一次性导入所有功能 | Import all features at once
//! 
//! #[tokio::main]
//! async fn main() {
//!     // 1. 初始化（使用内存存储） | Initialize (using memory storage)
//!     let state = SaTokenState::builder()
//!         .storage(Arc::new(MemoryStorage::new()))
//!         .timeout(7200)
//!         .build();
//!     
//!     // 2. 创建路由 | Create routes
//!     
//!     // 公共路由 | Public routes
//!     let login_route = warp::path!("login")
//!         .and(warp::post())
//!         .and_then(login_handler);
//!     
//!     // 需要登录的路由 | Routes requiring login
//!     let user_route = warp::path!("api" / "user" / "info")
//!         .and(sa_token_layer(state.clone()))
//!         .and(with_auth(state.clone()))
//!         .and_then(user_info_handler)
//!         .with(sa_token_cleanup());
//!     
//!     // 需要特定权限的路由 | Routes requiring specific permission
//!     let admin_route = warp::path!("api" / "admin")
//!         .and(sa_token_layer(state.clone()))
//!         .and(with_permission(state.clone(), "admin:access"))
//!         .and_then(admin_handler)
//!         .with(sa_token_cleanup());
//!     
//!     // 组合路由 | Combine routes
//!     let routes = login_route
//!         .or(user_route)
//!         .or(admin_route)
//!         .recover(handle_rejection);
//!     
//!     warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
//! }
//! ```

pub mod adapter;
pub mod extractor;
pub mod layer;
pub mod middleware;
pub mod state;
pub mod filter;

// ============================================================================
// Warp 框架集成（本插件特有） | Warp framework integration (plugin specific)
// ============================================================================
pub use filter::{sa_token_filter, sa_check_login_filter};
pub use layer::{sa_token_layer, sa_token_cleanup, sa_check_login, sa_check_permission, sa_check_role, extract_token_from_request};
pub use middleware::{with_auth, with_permission, with_role, require_auth, require_permission, require_role};
pub use extractor::{SaTokenExtractor, OptionalSaTokenExtractor, LoginIdExtractor, AuthError, PermissionError, RoleError, handle_rejection};
pub use adapter::{WarpRequestAdapter, WarpResponseAdapter};
pub use state::{SaTokenState, SaTokenStateBuilder};

// ============================================================================
// 重新导出核心功能（sa-token-core） | Re-export core functionalities (sa-token-core)
// ============================================================================
pub use sa_token_core::{self,
    // 核心管理器 | Core managers
    SaTokenManager, StpUtil,
    
    // 配置 | Configuration
    SaTokenConfig,
    config::TokenStyle,
    
    // Token 相关 | Token related
    TokenValue, TokenInfo,
    
    // 会话管理 | Session management
    SaSession,
    
    // 权限 | Permissions
    PermissionChecker,
    
    // 错误处理 | Error handling
    SaTokenError,
    
    // 事件系统 | Event system
    SaTokenEvent, SaTokenListener, SaTokenEventBus, LoggingListener,
    
    // JWT 支持 | JWT support
    JwtManager, JwtClaims, JwtAlgorithm,
    
    // OAuth2 支持 | OAuth2 support
    OAuth2Manager, OAuth2Client, AuthorizationCode, AccessToken, OAuth2TokenInfo,
    
    // 安全特性 | Security features
    NonceManager, RefreshTokenManager,
    
    // WebSocket 认证 | WebSocket authentication
    WsAuthManager, WsAuthInfo, WsTokenExtractor, DefaultWsTokenExtractor,
    
    // 在线用户管理 | Online user management
    OnlineManager, OnlineUser, PushMessage, MessageType, MessagePusher, InMemoryPusher,
    
    // 分布式会话 | Distributed session
    DistributedSessionManager, DistributedSession, DistributedSessionStorage, 
    ServiceCredential, InMemoryDistributedStorage,
    
    // 模块 | Modules
    token, error
};

// ============================================================================
// 重新导出适配器接口（sa-token-adapter） | Re-export adapter interfaces (sa-token-adapter)
// ============================================================================
pub use sa_token_adapter::{self,
    storage::SaStorage,
    framework::FrameworkAdapter,
};

// ============================================================================
// 重新导出宏（sa-token-macro） | Re-export macros (sa-token-macro)
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
// 重新导出存储实现（根据 feature 条件编译） | Re-export storage implementations (feature-gated)
// ============================================================================

/// 内存存储（默认启用） | Memory storage (enabled by default)
#[cfg(feature = "memory")]
pub use sa_token_storage_memory::MemoryStorage;

/// Redis 存储 | Redis storage
#[cfg(feature = "redis")]
pub use sa_token_storage_redis::RedisStorage;

/// 数据库存储 | Database storage
#[cfg(feature = "database")]
pub use sa_token_storage_database::DatabaseStorage;