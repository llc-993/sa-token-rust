# 快速开始

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
# 核心库
sa-token-core = "0.1"

# 选择一个存储实现
sa-token-storage-memory = "0.1"  # 内存存储（开发环境）
# 或
sa-token-storage-redis = "0.1"   # Redis存储（生产环境）

# 选择一个Web框架插件
sa-token-plugin-axum = "0.1"          # Axum
# 或
sa-token-plugin-actix-web = "0.1"     # Actix-web

# 必需的依赖
tokio = { version = "1", features = ["full"] }
```

## 5分钟快速开始（Axum）

### 1. 创建项目

```bash
cargo new my-auth-app
cd my-auth-app
```

### 2. 添加依赖

```toml
[dependencies]
sa-token-core = "0.1"
sa-token-storage-memory = "0.1"
sa-token-plugin-axum = "0.1"
tokio = { version = "1", features = ["full"] }
axum = "0.7"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 3. 编写代码

```rust
use std::sync::Arc;
use axum::{
    Router,
    routing::{get, post},
    Json,
    extract::State,
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sa_token_core::{SaTokenConfig, SaTokenManager};
use sa_token_storage_memory::MemoryStorage;
use sa_token_plugin_axum::SaTokenState;

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[tokio::main]
async fn main() {
    // 1. 创建存储
    let storage = Arc::new(MemoryStorage::new());
    
    // 2. 创建配置
    let config = SaTokenConfig::builder()
        .token_name("Authorization")
        .timeout(86400)  // 24小时
        .build();
    
    // 3. 创建状态
    let state = SaTokenState::new(storage, config);
    
    // 4. 创建路由
    let app = Router::new()
        .route("/api/login", post(login))
        .route("/api/logout", post(logout))
        .route("/api/user/info", get(user_info))
        .with_state(state);
    
    // 5. 启动服务器
    println!("Server running on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// 登录接口
async fn login(
    State(state): State<SaTokenState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // 这里应该验证用户名密码
    if req.username == "admin" && req.password == "123456" {
        // 登录成功，生成token
        let token = state.manager
            .login(&req.username)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok(Json(LoginResponse {
            token: token.to_string(),
        }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

// 登出接口
async fn logout(
    State(state): State<SaTokenState>,
) -> impl IntoResponse {
    // 实际应用中应该从请求中获取token
    StatusCode::OK
}

// 获取用户信息（受保护的接口）
async fn user_info() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "username": "admin",
        "nickname": "管理员",
        "roles": ["admin"]
    }))
}
```

### 4. 运行测试

```bash
# 启动服务
cargo run

# 在另一个终端测试登录
curl -X POST http://localhost:3000/api/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"123456"}'

# 返回：{"token":"xxxxx-xxxx-xxxx"}

# 使用token访问受保护接口
curl http://localhost:3000/api/user/info \
  -H "Authorization: Bearer xxxxx-xxxx-xxxx"
```

## 5分钟快速开始（Actix-web）

### 1. 添加依赖

```toml
[dependencies]
sa-token-core = "0.1"
sa-token-storage-memory = "0.1"
sa-token-plugin-actix-web = "0.1"
tokio = { version = "1", features = ["full"] }
actix-web = "4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 2. 编写代码

```rust
use std::sync::Arc;
use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};
use sa_token_core::SaTokenConfig;
use sa_token_storage_memory::MemoryStorage;
use sa_token_plugin_actix_web::{SaTokenMiddleware, SaTokenAppState};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. 创建存储和配置
    let storage = Arc::new(MemoryStorage::new());
    let config = SaTokenConfig::default();
    let state = SaTokenAppState::new(storage, config);
    
    // 2. 启动服务器
    println!("Server running on http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(SaTokenMiddleware)
            .route("/api/login", web::post().to(login))
            .route("/api/user/info", web::get().to(user_info))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn login(
    state: web::Data<SaTokenAppState>,
    req: web::Json<LoginRequest>,
) -> HttpResponse {
    if req.username == "admin" && req.password == "123456" {
        let token = state.manager.login(&req.username).await.unwrap();
        HttpResponse::Ok().json(LoginResponse {
            token: token.to_string(),
        })
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

async fn user_info() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "username": "admin",
        "nickname": "管理员"
    }))
}
```

## 使用Redis存储

```rust
use sa_token_storage_redis::RedisStorage;

#[tokio::main]
async fn main() {
    // 连接到Redis
    let storage = Arc::new(
        RedisStorage::new("redis://127.0.0.1:6379", "sa-token:")
            .await
            .expect("Failed to connect to Redis")
    );
    
    let config = SaTokenConfig::default();
    let state = SaTokenState::new(storage, config);
    
    // 后续代码与内存存储相同...
}
```

## 下一步

- 查看[核心概念](CONCEPTS.md)了解更多功能
- 查看[API文档](https://docs.rs/sa-token-rust)
- 查看[示例项目](../examples/)
- 学习如何[自定义存储](CUSTOM_STORAGE.md)
- 学习如何[集成到其他框架](FRAMEWORK_INTEGRATION.md)

