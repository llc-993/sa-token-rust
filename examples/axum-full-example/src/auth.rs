//! 认证相关代码

use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::AppState;

// ==================== 请求/响应类型 ====================

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

pub enum ApiError {
    Unauthorized(String),
    Forbidden(String),
    BadRequest(String),
    InternalError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, 401, msg),
            ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, 403, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, 400, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, 500, msg),
        };
        
        let body = Json(serde_json::json!({
            "code": code,
            "message": message,
            "data": null::<()>,
        }));
        
        (status, body).into_response()
    }
}

// ==================== 登录接口 ====================

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, ApiError> {
    // 验证用户名密码（实际应该查询数据库）
    let user_id = match req.username.as_str() {
        "admin" if req.password == "admin123" => "admin",
        "user" if req.password == "user123" => "user",
        "guest" if req.password == "guest123" => "guest",
        _ => {
            return Err(ApiError::Unauthorized("用户名或密码错误".to_string()));
        }
    };
    
    // 生成token
    let token = state.sa_token.manager
        .login(user_id)
        .await
        .map_err(|e| ApiError::InternalError(format!("登录失败: {}", e)))?;
    
    // 获取用户信息
    let permissions = state.permission_service.get_user_permissions(user_id).await;
    let roles = state.permission_service.get_user_roles(user_id).await;
    
    tracing::info!(
        "✅ 用户 {} 登录成功，权限: {:?}, 角色: {:?}", 
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
    
    Ok(Json(ApiResponse::success(response)))
}

