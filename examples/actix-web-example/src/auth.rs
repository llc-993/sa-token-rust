// Author: 金书记
//
//! 认证相关代码
//! Authentication related code

use actix_web::{
    web,
    HttpResponse,
    ResponseError,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sa_token_plugin_actix_web::SaTokenState;

// ==================== 请求/响应类型 ====================
// ==================== Request/Response Types ====================

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub nickname: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_info: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteUserRequest {
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ManageUserRequest {
    pub user_id: String,
    pub action: String,
}

#[derive(Debug, Deserialize)]
pub struct AddPermissionRequest {
    pub user_id: String,
    pub permission: String,
}

#[derive(Debug, Deserialize)]
pub struct RemovePermissionRequest {
    pub user_id: String,
    pub permission: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }
    
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            code: -1,
            message: message.into(),
            data: None,
        }
    }
}

// ==================== 错误处理 ====================
// ==================== Error Handling ====================

#[derive(Debug)]
pub enum ApiError {
    Unauthorized(String),
    Forbidden(String),
    BadRequest(String),
    InternalError(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    
    fn error_response(&self) -> HttpResponse {
        let (status, code, message) = match self {
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, 401, msg),
            ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, 403, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, 400, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, 500, msg),
        };
        
        HttpResponse::build(status)
            .json(serde_json::json!({
                "code": code,
                "message": message,
                "data": serde_json::Value::Null,
            }))
    }
}

// ==================== 登录接口 ====================
// ==================== Login Endpoint ====================

pub async fn login(
    state: web::Data<SaTokenState>,
    req: web::Json<LoginRequest>,
) -> Result<web::Json<ApiResponse<LoginResponse>>, ApiError> {
    // 验证用户名密码（实际应该查询数据库）
    // Validate username and password (should query database in real application)
    let user_id = match req.username.as_str() {
        "admin" if req.password == "admin123" => "admin",
        "user" if req.password == "user123" => "user",
        "guest" if req.password == "guest123" => "guest",
        _ => {
            return Err(ApiError::Unauthorized("用户名或密码错误 / Invalid username or password".to_string()));
        }
    };
    
    // 生成token - 使用注入的 sa_token 状态
    // Generate token - using injected sa_token state
    let token = state.manager
        .login(user_id)
        .await
        .map_err(|e| ApiError::InternalError(format!("登录失败 / Login failed: {}", e)))?;
    
    // 获取用户权限和角色（使用 StpUtil）
    // Get user permissions and roles (using StpUtil)
    let permissions = sa_token_core::StpUtil::get_permissions(user_id).await;
    let roles = sa_token_core::StpUtil::get_roles(user_id).await;
    
    tracing::info!(
        "✅ 用户 {} 登录成功，权限: {:?}, 角色: {:?}", 
        user_id, permissions, roles
    );
    tracing::info!(
        "✅ User {} logged in successfully, permissions: {:?}, roles: {:?}", 
        user_id, permissions, roles
    );
    
    let user_info = UserInfo {
        id: user_id.to_string(),
        username: req.username.clone(),
        nickname: match user_id {
            "admin" => "管理员",
            "user" => "普通用户",
            "guest" => "访客",
            _ => "未知",
        }.to_string(),
        email: Some(format!("{}@example.com", req.username)),
    };
    
    let response = LoginResponse {
        token: token.to_string(),
        user_info,
    };
    
    Ok(web::Json(ApiResponse::success(response)))
}
