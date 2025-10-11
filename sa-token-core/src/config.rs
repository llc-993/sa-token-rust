//! 配置模块

use std::time::Duration;
use serde::{Deserialize, Serialize};

/// sa-token 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaTokenConfig {
    /// Token 名称（例如在 header 或 cookie 中的键名）
    pub token_name: String,
    
    /// Token 有效期（秒），-1 表示永久有效
    pub timeout: i64,
    
    /// Token 最低活跃频率（秒），-1 表示不限制
    pub active_timeout: i64,
    
    /// 是否允许同一账号并发登录
    pub is_concurrent: bool,
    
    /// 在多人登录同一账号时，是否共享一个 token
    pub is_share: bool,
    
    /// Token 风格（uuid、simple-uuid、random-32、random-64、random-128）
    pub token_style: TokenStyle,
    
    /// 是否输出操作日志
    pub is_log: bool,
    
    /// 是否从 cookie 中读取 token
    pub is_read_cookie: bool,
    
    /// 是否从 header 中读取 token
    pub is_read_header: bool,
    
    /// 是否从请求体中读取 token
    pub is_read_body: bool,
    
    /// token 前缀（例如 "Bearer "）
    pub token_prefix: Option<String>,
    
    /// JWT 密钥（如果使用 JWT）
    pub jwt_secret_key: Option<String>,
}

impl Default for SaTokenConfig {
    fn default() -> Self {
        Self {
            token_name: "sa-token".to_string(),
            timeout: 2592000, // 30天
            active_timeout: -1,
            is_concurrent: true,
            is_share: true,
            token_style: TokenStyle::Uuid,
            is_log: false,
            is_read_cookie: true,
            is_read_header: true,
            is_read_body: false,
            token_prefix: None,
            jwt_secret_key: None,
        }
    }
}

impl SaTokenConfig {
    pub fn builder() -> SaTokenConfigBuilder {
        SaTokenConfigBuilder::default()
    }
    
    pub fn timeout_duration(&self) -> Option<Duration> {
        if self.timeout < 0 {
            None
        } else {
            Some(Duration::from_secs(self.timeout as u64))
        }
    }
}

/// Token 风格
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TokenStyle {
    /// UUID 风格
    Uuid,
    /// 简化的 UUID（去掉横杠）
    SimpleUuid,
    /// 32位随机字符串
    Random32,
    /// 64位随机字符串
    Random64,
    /// 128位随机字符串
    Random128,
}

/// 配置构建器
#[derive(Debug, Default)]
pub struct SaTokenConfigBuilder {
    config: SaTokenConfig,
}

impl SaTokenConfigBuilder {
    pub fn token_name(mut self, name: impl Into<String>) -> Self {
        self.config.token_name = name.into();
        self
    }
    
    pub fn timeout(mut self, timeout: i64) -> Self {
        self.config.timeout = timeout;
        self
    }
    
    pub fn active_timeout(mut self, timeout: i64) -> Self {
        self.config.active_timeout = timeout;
        self
    }
    
    pub fn is_concurrent(mut self, concurrent: bool) -> Self {
        self.config.is_concurrent = concurrent;
        self
    }
    
    pub fn is_share(mut self, share: bool) -> Self {
        self.config.is_share = share;
        self
    }
    
    pub fn token_style(mut self, style: TokenStyle) -> Self {
        self.config.token_style = style;
        self
    }
    
    pub fn token_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.config.token_prefix = Some(prefix.into());
        self
    }
    
    pub fn jwt_secret_key(mut self, key: impl Into<String>) -> Self {
        self.config.jwt_secret_key = Some(key.into());
        self
    }
    
    pub fn build(self) -> SaTokenConfig {
        self.config
    }
}

