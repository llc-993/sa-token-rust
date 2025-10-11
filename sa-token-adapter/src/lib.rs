// Author: 金书记
//
//! # sa-token-adapter
//! 
//! 适配器trait定义，用于实现框架无关的抽象层
//! 
//! 这个crate定义了所有需要适配的接口，包括：
//! - 存储适配器
//! - 请求/响应上下文适配器
//! - 框架集成适配器

pub mod storage;
pub mod context;
pub mod framework;

pub use storage::SaStorage;
pub use context::{SaRequest, SaResponse, CookieOptions};
pub use framework::FrameworkAdapter;
