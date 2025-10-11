// Author: 金书记
//
//! 错误类型定义

use thiserror::Error;

pub type SaTokenResult<T> = Result<T, SaTokenError>;

#[derive(Debug, Error)]
pub enum SaTokenError {
    #[error("Token not found or expired")]
    TokenNotFound,
    
    #[error("Token is invalid: {0}")]
    InvalidToken(String),
    
    #[error("Token has expired")]
    TokenExpired,
    
    #[error("User not logged in")]
    NotLogin,
    
    #[error("Permission denied: missing permission '{0}'")]
    PermissionDenied(String),
    
    #[error("Role denied: missing role '{0}'")]
    RoleDenied(String),
    
    #[error("Account is banned until {0}")]
    AccountBanned(String),
    
    #[error("Account is kicked out")]
    AccountKickedOut,
    
    #[error("Session not found")]
    SessionNotFound,
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}
