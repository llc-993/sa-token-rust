//! sa-token-rust Poem 完整示例
//! 
//! 展示如何：
//! 1. 配置 sa-token
//! 2. 加载用户权限和角色
//! 3. 使用中间件和提取器
//! 4. 实现完整的认证流程

use std::sync::Arc;
use poem::{
    Route, Server, listener::TcpListener, 
    handler, web::Json, web::Data,
    Result as PoemResult, Response, IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sa_token_core::{SaTokenConfig, StpUtil};
use sa_token_storage_memory::MemoryStorage;
use sa_token_plugin_poem::{
    SaTokenState, SaTokenMiddleware, SaTokenExtractor, 
    OptionalSaTokenExtractor, LoginIdExtractor,
};
use sa_token_core::config::TokenStyle;

/// API 响应结构
#[derive(Debug, Serialize)]
struct ApiResponse<T: Serialize> {
    code: i32,
    message: String,
    data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }
    
    fn error(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }
}

/// 登录请求
#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

/// 登录响应
#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
    user_id: String,
    permissions: Vec<String>,
    roles: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    tracing::info!("🚀 启动 sa-token-rust Poem 完整示例");
    
    // 1. 使用构建器模式创建 sa-token 状态
    let sa_token_state = SaTokenState::builder()
        .storage(Arc::new(MemoryStorage::new()))
        .token_name("Authorization")
        .timeout(86400)  // 24小时
        .token_style(TokenStyle::Random64)
        .build();
    
    // 2. 初始化全局 StpUtil
    StpUtil::init_manager((*sa_token_state.manager).clone());
    
    // 3. 初始化测试用户的权限和角色
    init_test_permissions().await;
    
    // 4. 创建路由
    let app = Route::new()
        // 公开接口（不需要登录）
        .at("/", handler(index))
        .at("/api/health", handler(health_check))
        .at("/api/auth/login", poem::post(login))
        
        // 需要登录的接口
        .at("/api/user/info", poem::get(user_info))
        .at("/api/user/permissions", poem::get(list_permissions))
        .at("/api/user/roles", poem::get(list_roles))
        
        // 需要权限的接口
        .at("/api/admin/users", poem::get(list_users))
        .at("/api/admin/config", poem::get(admin_config))
        
        // 应用中间件
        .with(SaTokenMiddleware::new(sa_token_state.manager.clone()))
        .data(sa_token_state);
    
    tracing::info!("📡 服务器运行在 http://127.0.0.1:3000");
    tracing::info!("   试试访问: http://127.0.0.1:3000/api/health");
    tracing::info!("   登录接口: POST http://127.0.0.1:3000/api/auth/login");
    tracing::info!("");
    tracing::info!("💡 测试用户:");
    tracing::info!("   - admin/admin123  (管理员)");
    tracing::info!("   - user/user123    (普通用户)");
    tracing::info!("   - guest/guest123  (访客)");
    
    // 5. 启动服务器
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}

/// 初始化测试用户的权限和角色
async fn init_test_permissions() {
    tracing::info!("🔐 初始化测试用户权限...");
    
    // 管理员用户
    StpUtil::set_permissions(
        "admin",
        vec![
            "user:list".to_string(),
            "user:create".to_string(),
            "user:update".to_string(),
            "user:delete".to_string(),
            "system:config".to_string(),
            "admin:*".to_string(),
        ],
    ).await.unwrap();
    
    StpUtil::set_roles(
        "admin",
        vec!["admin".to_string(), "user".to_string()],
    ).await.unwrap();
    
    tracing::info!("  ✓ 管理员 (admin) 权限已初始化");
    
    // 普通用户
    StpUtil::set_permissions(
        "user",
        vec![
            "user:list".to_string(),
            "user:view".to_string(),
        ],
    ).await.unwrap();
    
    StpUtil::set_roles(
        "user",
        vec!["user".to_string()],
    ).await.unwrap();
    
    tracing::info!("  ✓ 普通用户 (user) 权限已初始化");
    
    // 访客用户
    StpUtil::set_permissions(
        "guest",
        vec!["user:view".to_string()],
    ).await.unwrap();
    
    StpUtil::set_roles(
        "guest",
        vec!["guest".to_string()],
    ).await.unwrap();
    
    tracing::info!("  ✓ 访客 (guest) 权限已初始化");
    tracing::info!("✅ 所有测试用户权限初始化完成！\n");
}

// ==================== 公开接口 ====================

#[handler]
async fn index() -> &'static str {
    "Welcome to sa-token-rust Poem example! Visit /api/health to check health."
}

#[handler]
async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("OK".to_string()))
}

/// 登录接口
#[handler]
async fn login(
    Data(state): Data<&SaTokenState>,
    Json(req): Json<LoginRequest>,
) -> PoemResult<Json<ApiResponse<LoginResponse>>> {
    tracing::info!("🔑 用户登录请求: username={}", req.username);
    
    // 验证用户名密码（这里简化处理）
    let (user_id, valid) = match req.username.as_str() {
        "admin" if req.password == "admin123" => ("admin", true),
        "user" if req.password == "user123" => ("user", true),
        "guest" if req.password == "guest123" => ("guest", true),
        _ => ("", false),
    };
    
    if !valid {
        return Ok(Json(ApiResponse::error(401, "用户名或密码错误".to_string())));
    }
    
    // 执行登录
    let token = state.manager
        .login(user_id)
        .await
        .map_err(|e| poem::Error::from_string(
            format!("登录失败: {}", e),
            StatusCode::INTERNAL_SERVER_ERROR
        ))?;
    
    // 获取用户权限和角色
    let permissions = StpUtil::get_permissions(user_id).await;
    let roles = StpUtil::get_roles(user_id).await;
    
    tracing::info!(
        "✅ 用户 {} 登录成功，权限: {:?}, 角色: {:?}", 
        user_id, permissions, roles
    );
    
    Ok(Json(ApiResponse::success(LoginResponse {
        token: token.as_str().to_string(),
        user_id: user_id.to_string(),
        permissions,
        roles,
    })))
}

// ==================== 需要登录的接口 ====================

/// 获取用户信息
#[handler]
async fn user_info(token: SaTokenExtractor) -> Json<ApiResponse<serde_json::Value>> {
    tracing::info!("📋 获取用户信息: user_id={}", token.login_id());
    
    let permissions = StpUtil::get_permissions(token.login_id()).await;
    let roles = StpUtil::get_roles(token.login_id()).await;
    
    Json(ApiResponse::success(serde_json::json!({
        "user_id": token.login_id(),
        "permissions": permissions,
        "roles": roles,
    })))
}

/// 查询用户权限列表
#[handler]
async fn list_permissions(
    LoginIdExtractor(user_id): LoginIdExtractor
) -> Json<ApiResponse<Vec<String>>> {
    tracing::info!("📋 查询用户权限: user_id={}", user_id);
    
    let permissions = StpUtil::get_permissions(&user_id).await;
    
    Json(ApiResponse::success(permissions))
}

/// 查询用户角色列表
#[handler]
async fn list_roles(
    LoginIdExtractor(user_id): LoginIdExtractor
) -> Json<ApiResponse<Vec<String>>> {
    tracing::info!("📋 查询用户角色: user_id={}", user_id);
    
    let roles = StpUtil::get_roles(&user_id).await;
    
    Json(ApiResponse::success(roles))
}

// ==================== 需要权限的接口 ====================

/// 获取用户列表（需要 user:list 权限）
#[handler]
async fn list_users(
    LoginIdExtractor(user_id): LoginIdExtractor
) -> PoemResult<Json<ApiResponse<Vec<String>>>> {
    tracing::info!("📋 获取用户列表: user_id={}", user_id);
    
    // 检查权限
    if !StpUtil::has_permission(&user_id, "user:list").await {
        return Ok(Json(ApiResponse::error(403, "权限不足：需要 user:list 权限".to_string())));
    }
    
    let users = vec![
        "admin".to_string(),
        "user1".to_string(),
        "user2".to_string(),
    ];
    
    Ok(Json(ApiResponse::success(users)))
}

/// 管理员配置（需要 admin 角色）
#[handler]
async fn admin_config(
    LoginIdExtractor(user_id): LoginIdExtractor
) -> PoemResult<Json<ApiResponse<String>>> {
    tracing::info!("⚙️  获取管理员配置: user_id={}", user_id);
    
    // 检查角色
    if !StpUtil::has_role(&user_id, "admin").await {
        return Ok(Json(ApiResponse::error(403, "权限不足：需要 admin 角色".to_string())));
    }
    
    Ok(Json(ApiResponse::success("Admin configuration data".to_string())))
}

