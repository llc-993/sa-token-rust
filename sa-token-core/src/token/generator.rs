// Author: 金书记
//
//! Token 生成器

use uuid::Uuid;
use crate::config::{TokenStyle, SaTokenConfig};
use crate::token::TokenValue;

pub struct TokenGenerator;

impl TokenGenerator {
    /// 根据配置生成 token
    pub fn generate(config: &SaTokenConfig) -> TokenValue {
        match config.token_style {
            TokenStyle::Uuid => Self::generate_uuid(),
            TokenStyle::SimpleUuid => Self::generate_simple_uuid(),
            TokenStyle::Random32 => Self::generate_random(32),
            TokenStyle::Random64 => Self::generate_random(64),
            TokenStyle::Random128 => Self::generate_random(128),
        }
    }
    
    /// 生成 UUID 风格的 token
    pub fn generate_uuid() -> TokenValue {
        TokenValue::new(Uuid::new_v4().to_string())
    }
    
    /// 生成简化的 UUID（去掉横杠）
    pub fn generate_simple_uuid() -> TokenValue {
        TokenValue::new(Uuid::new_v4().simple().to_string())
    }
    
    /// 生成随机字符串
    pub fn generate_random(length: usize) -> TokenValue {
        use sha2::{Sha256, Digest};
        let uuid = Uuid::new_v4();
        let random_bytes = uuid.as_bytes();
        let hash = Sha256::digest(random_bytes);
        let hex_string = hex::encode(hash);
        TokenValue::new(hex_string[..length.min(hex_string.len())].to_string())
    }
}

// 添加 hex 依赖
