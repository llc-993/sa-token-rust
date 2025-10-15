// Author: 金书记
//
// 中文 | English
// Gotham 框架集成 | Gotham Framework Integration
//
//! # sa-token-plugin-gotham
//! 
//! 为 Gotham 框架提供 sa-token 认证和授权支持
//! Provides sa-token authentication and authorization support for Gotham framework
//! 
//! ## 特性 | Features
//! 
//! - ✨ 一行导入所有功能 | One-line import for all functionalities
//! - 🔧 支持多种存储后端 | Support for multiple storage backends
//! - 🚀 简化的中间件集成 | Simplified middleware integration
//! - 📦 包含核心、宏、存储 | Includes core, macros, and storage
//! 
//! ## 快速开始 | Quick Start
//! 
//! ```toml
//! [dependencies]
//! sa-token-plugin-gotham = "0.1.4"
//! ```
//! 
//! ```rust,ignore
//! use sa_token_plugin_gotham::*;
//! 
//! #[tokio::main]
//! async fn main() {
//!     let storage = Arc::new(MemoryStorage::new());
//!     
//!     SaTokenConfig::builder()
//!         .token_name("Authorization")
//!         .timeout(7200)
//!         .storage(storage)
//!         .build();
//!     
//!     let addr = "127.0.0.1:8080";
//!     gotham::start(addr, || Ok(router()));
//! }
//! ```

pub mod middleware;

// 重新导出核心功能 | Re-export core functionalities
pub use sa_token_core::*;
pub use sa_token_adapter::*;
pub use sa_token_macro::*;

// 重新导出存储实现（通过 feature 控制）
// Re-export storage implementations (controlled by features)
#[cfg(feature = "memory")]
pub use sa_token_storage_memory::*;

#[cfg(feature = "redis")]
pub use sa_token_storage_redis::*;

#[cfg(feature = "database")]
pub use sa_token_storage_database::*;

// 重新导出本模块的适配器 | Re-export adapters from this module
pub use middleware::*;

