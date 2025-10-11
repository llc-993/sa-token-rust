//! Token 管理模块

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod generator;
pub mod validator;

pub use generator::TokenGenerator;
pub use validator::TokenValidator;

/// Token 值
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenValue(String);

impl TokenValue {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for TokenValue {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<TokenValue> for String {
    fn from(v: TokenValue) -> Self {
        v.0
    }
}

impl std::fmt::Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Token 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    /// Token 值
    pub token: TokenValue,
    
    /// 登录 ID
    pub login_id: String,
    
    /// 登录类型（user、admin 等）
    pub login_type: String,
    
    /// Token 创建时间
    pub create_time: DateTime<Utc>,
    
    /// Token 最后活跃时间
    pub last_active_time: DateTime<Utc>,
    
    /// Token 过期时间（None 表示永不过期）
    pub expire_time: Option<DateTime<Utc>>,
    
    /// 设备标识
    pub device: Option<String>,
    
    /// 额外数据
    pub extra_data: Option<serde_json::Value>,
}

impl TokenInfo {
    pub fn new(token: TokenValue, login_id: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            token,
            login_id: login_id.into(),
            login_type: "default".to_string(),
            create_time: now,
            last_active_time: now,
            expire_time: None,
            device: None,
            extra_data: None,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        if let Some(expire_time) = self.expire_time {
            Utc::now() > expire_time
        } else {
            false
        }
    }
    
    pub fn update_active_time(&mut self) {
        self.last_active_time = Utc::now();
    }
}

/// Token 签名
#[derive(Debug, Clone)]
pub struct TokenSign {
    pub value: String,
    pub device: Option<String>,
}

