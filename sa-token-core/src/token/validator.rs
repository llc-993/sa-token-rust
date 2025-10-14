// Author: 金书记
//
//! Token 验证器

use crate::error::{SaTokenError, SaTokenResult};
use crate::token::TokenInfo;

pub struct TokenValidator;

impl TokenValidator {
    /// 验证 token 是否有效
    pub fn validate(token_info: &TokenInfo) -> SaTokenResult<()> {
        // 检查是否过期
        if token_info.is_expired() {
            return Err(SaTokenError::TokenExpired);
        }
        
        Ok(())
    }
    
    /// 检查 token 格式是否正确
    pub fn check_format(token: &str) -> SaTokenResult<()> {
        if token.is_empty() {
            return Err(SaTokenError::TokenEmpty);
        }
        
        if token.len() < 8 {
            return Err(SaTokenError::TokenTooShort);
        }
        
        Ok(())
    }
}
