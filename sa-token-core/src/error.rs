// Author: 金书记
//
//! Error type definitions | 错误类型定义

use thiserror::Error;

pub type SaTokenResult<T> = Result<T, SaTokenError>;

#[derive(Debug, Error)]
pub enum SaTokenError {
    // ============ Basic Token Errors | 基础 Token 错误 ============
    #[error("Token not found or expired")]
    TokenNotFound,
    
    #[error("Token is invalid: {0}")]
    InvalidToken(String),
    
    #[error("Token has expired")]
    TokenExpired,
    
    // ============ Authentication Errors | 认证错误 ============
    #[error("User not logged in")]
    NotLogin,
    
    // ============ Authorization Errors | 授权错误 ============
    #[error("Permission denied: missing permission '{0}'")]
    PermissionDenied(String),
    
    #[error("Role denied: missing role '{0}'")]
    RoleDenied(String),
    
    // ============ Account Status Errors | 账户状态错误 ============
    #[error("Account is banned until {0}")]
    AccountBanned(String),
    
    #[error("Account is kicked out")]
    AccountKickedOut,
    
    // ============ Session Errors | Session 错误 ============
    #[error("Session not found")]
    SessionNotFound,
    
    // ============ Nonce Errors | Nonce 错误 ============
    #[error("Nonce has been used, possible replay attack detected")]
    NonceAlreadyUsed,
    
    #[error("Invalid nonce format")]
    InvalidNonceFormat,
    
    #[error("Nonce timestamp is invalid or expired")]
    InvalidNonceTimestamp,
    
    // ============ Refresh Token Errors | 刷新令牌错误 ============
    #[error("Refresh token not found or expired")]
    RefreshTokenNotFound,
    
    #[error("Invalid refresh token data")]
    RefreshTokenInvalidData,
    
    #[error("Missing login_id in refresh token")]
    RefreshTokenMissingLoginId,
    
    #[error("Invalid expire time format in refresh token")]
    RefreshTokenInvalidExpireTime,
    
    // ============ Token Validation Errors | Token 验证错误 ============
    #[error("Token is empty")]
    TokenEmpty,
    
    #[error("Token is too short")]
    TokenTooShort,
    
    #[error("Login ID is not a valid number")]
    LoginIdNotNumber,
    
    // ============ OAuth2 Errors | OAuth2 错误 ============
    #[error("OAuth2 client not found")]
    OAuth2ClientNotFound,
    
    #[error("Invalid client credentials")]
    OAuth2InvalidCredentials,
    
    #[error("Client ID mismatch")]
    OAuth2ClientIdMismatch,
    
    #[error("Redirect URI mismatch")]
    OAuth2RedirectUriMismatch,
    
    #[error("Authorization code not found or expired")]
    OAuth2CodeNotFound,
    
    #[error("Access token not found or expired")]
    OAuth2AccessTokenNotFound,
    
    #[error("Refresh token not found or expired")]
    OAuth2RefreshTokenNotFound,
    
    #[error("Invalid refresh token data")]
    OAuth2InvalidRefreshToken,
    
    #[error("Invalid scope data")]
    OAuth2InvalidScope,
    
    // ============ System Errors | 系统错误 ============
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}
