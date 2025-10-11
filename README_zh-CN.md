# sa-token-rust

中文文档 | [English](README.md)

一个轻量级、高性能的 Rust 认证授权框架，灵感来自 [sa-token](https://github.com/dromara/sa-token)。

## ✨ 特性

- 🚀 **多框架支持**: Axum, Actix-web, Poem, Rocket, Warp
- 🔐 **完整的认证**: 登录、登出、Token 验证、Session 管理
- 🛡️ **细粒度授权**: 基于权限和角色的访问控制
- 💾 **灵活存储**: 内存、Redis 和数据库存储后端
- 🎯 **易于使用**: 过程宏和工具类简化集成
- ⚡ **高性能**: 零拷贝设计，支持 async/await
- 🔧 **高度可配置**: Token 超时、Cookie 选项、自定义 Token 名称

## 📦 架构

```
sa-token-rust/
├── sa-token-core/              # 核心库（Token、Session、Manager）
├── sa-token-adapter/           # 适配器接口（Storage、Request/Response）
├── sa-token-macro/             # 过程宏（#[sa_check_login] 等）
├── sa-token-storage-memory/    # 内存存储实现
├── sa-token-storage-redis/     # Redis 存储实现
├── sa-token-storage-database/  # 数据库存储实现（占位符）
├── sa-token-plugin-axum/       # Axum 框架集成
├── sa-token-plugin-actix-web/  # Actix-web 框架集成
├── sa-token-plugin-poem/       # Poem 框架集成
├── sa-token-plugin-rocket/     # Rocket 框架集成
├── sa-token-plugin-warp/       # Warp 框架集成
└── examples/                   # 示例项目
    ├── axum-full-example/      # 完整 Axum 示例
    └── poem-full-example/      # 完整 Poem 示例
```

## 🎯 核心组件

### 1. **sa-token-core**
核心认证授权逻辑：
- `SaTokenManager`: Token 和 Session 操作的主管理器
- `StpUtil`: 提供简化 API 的工具类 ([文档](docs/StpUtil_zh-CN.md))
- Token 生成、验证和刷新
- Session 管理
- 权限和角色检查

### 2. **sa-token-adapter**
框架集成的抽象层：
- `SaStorage`: Token 和 Session 的存储接口
- `SaRequest` / `SaResponse`: 请求/响应抽象

### 3. **sa-token-macro**
用于注解式认证的过程宏：
- `#[sa_check_login]`: 要求登录
- `#[sa_check_permission("user:list")]`: 检查权限
- `#[sa_check_role("admin")]`: 检查角色
- `#[sa_check_permissions_and(...)]`: 检查多个权限（AND）
- `#[sa_check_permissions_or(...)]`: 检查多个权限（OR）
- `#[sa_ignore]`: 跳过认证

### 4. **Web 框架插件**
所有插件都提供：
- 使用 Builder 模式的状态管理
- 双重中间件（基础 + 强制登录）
- 三种提取器（必须、可选、LoginId）
- 请求/响应适配器
- 从 Header/Cookie/Query 提取 Token
- Bearer Token 支持

## 🚀 快速开始

### 1. 添加依赖

```toml
[dependencies]
sa-token-core = "0.1"
sa-token-storage-memory = "0.1"
sa-token-plugin-axum = "0.1"  # 选择你的框架
tokio = { version = "1", features = ["full"] }
axum = "0.7"
```

### 2. 初始化 sa-token

```rust
use sa_token_core::StpUtil;
use sa_token_plugin_axum::SaTokenState;
use sa_token_storage_memory::MemoryStorage;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // 创建状态
    let state = SaTokenState::builder()
        .storage(Arc::new(MemoryStorage::new()))
        .token_name("Authorization")
        .timeout(86400)  // 24 小时
        .build();
    
    // 初始化 StpUtil（全局单例）
    StpUtil::init_manager((*state.manager).clone());
    
    // 你的应用代码...
}
```

### 3. 用户登录

```rust
use sa_token_core::StpUtil;

// 用户登录
let token = StpUtil::login("user_id_10001").await?;
println!("Token: {}", token.value());

// 设置权限和角色
StpUtil::set_permissions(
    "user_id_10001",
    vec!["user:list".to_string(), "user:add".to_string()]
).await?;

StpUtil::set_roles(
    "user_id_10001",
    vec!["admin".to_string()]
).await?;
```

### 4. 检查认证（Axum 示例）

```rust
use axum::{Router, routing::get};
use sa_token_plugin_axum::{SaTokenMiddleware, LoginIdExtractor};

async fn user_info(LoginIdExtractor(login_id): LoginIdExtractor) -> String {
    format!("当前用户: {}", login_id)
}

async fn admin_panel(login_id: LoginIdExtractor) -> String {
    // 检查权限
    if !StpUtil::has_permission(&login_id.0, "admin:panel").await {
        return "无权限".to_string();
    }
    format!("欢迎管理员: {}", login_id.0)
}

let app = Router::new()
    .route("/user/info", get(user_info))
    .route("/admin/panel", get(admin_panel))
    .layer(SaTokenMiddleware::new(state));
```

### 5. 使用过程宏

```rust
use sa_token_macro::*;

#[sa_check_login]
async fn protected_route() -> &'static str {
    "此路由需要登录"
}

#[sa_check_permission("user:delete")]
async fn delete_user(user_id: String) -> &'static str {
    "用户已删除"
}

#[sa_check_role("admin")]
async fn admin_only() -> &'static str {
    "仅管理员可见内容"
}
```

## 📚 框架集成示例

### Axum

```rust
use axum::{Router, routing::{get, post}};
use sa_token_plugin_axum::{SaTokenState, SaTokenMiddleware, LoginIdExtractor};

let state = SaTokenState::builder()
    .storage(Arc::new(MemoryStorage::new()))
    .build();

let app = Router::new()
    .route("/user/info", get(user_info))
    .layer(SaTokenMiddleware::new(state));
```

### Actix-web

```rust
use actix_web::{App, HttpServer, web};
use sa_token_plugin_actix_web::{SaTokenState, SaTokenMiddleware, LoginIdExtractor};

let state = SaTokenState::builder()
    .storage(Arc::new(MemoryStorage::new()))
    .build();

HttpServer::new(move || {
    App::new()
        .app_data(state.clone())
        .wrap(SaTokenMiddleware::new((*state).clone()))
        .route("/user/info", web::get().to(user_info))
})
.bind("127.0.0.1:8080")?
.run()
.await
```

### Poem

```rust
use poem::{Route, Server};
use sa_token_plugin_poem::{SaTokenState, SaTokenMiddleware, LoginIdExtractor};

let state = SaTokenState::builder()
    .storage(Arc::new(MemoryStorage::new()))
    .build();

let app = Route::new()
    .at("/user/info", poem::get(user_info))
    .with(SaTokenMiddleware::new(state));

Server::new(TcpListener::bind("127.0.0.1:8080"))
    .run(app)
    .await
```

### Rocket

```rust
use rocket::{launch, get, routes};
use sa_token_plugin_rocket::{SaTokenState, SaTokenFairing, LoginIdGuard};

#[get("/user/info")]
fn user_info(login_id: LoginIdGuard) -> String {
    format!("用户: {}", login_id.0)
}

#[launch]
fn rocket() -> _ {
    let state = SaTokenState::builder()
        .storage(Arc::new(MemoryStorage::new()))
        .build();
    
    rocket::build()
        .attach(SaTokenFairing::new(state))
        .mount("/", routes![user_info])
}
```

### Warp

```rust
use warp::Filter;
use sa_token_plugin_warp::{SaTokenState, sa_token_filter};

let state = SaTokenState::builder()
    .storage(Arc::new(MemoryStorage::new()))
    .build();

let routes = warp::path("user")
    .and(warp::path("info"))
    .and(sa_token_filter(state))
    .map(|token_data| {
        format!("用户信息")
    });

warp::serve(routes)
    .run(([127, 0, 0, 1], 8080))
    .await;
```

## 📖 文档

- [StpUtil API 参考](docs/StpUtil_zh-CN.md) - StpUtil 工具类完整指南
- [示例](examples/) - 所有支持框架的工作示例

## 🔧 高级用法

### 自定义存储

实现 `SaStorage` trait 来使用自己的存储后端：

```rust
use sa_token_adapter::storage::SaStorage;
use async_trait::async_trait;

pub struct CustomStorage;

#[async_trait]
impl SaStorage for CustomStorage {
    async fn get(&self, key: &str) -> Option<String> {
        // 你的实现
    }
    
    async fn set(&self, key: &str, value: String, timeout: Option<i64>) {
        // 你的实现
    }
    
    // ... 其他方法
}
```

### Token 配置

```rust
let state = SaTokenState::builder()
    .storage(Arc::new(MemoryStorage::new()))
    .token_name("X-Token")           // 自定义 Token 名称
    .timeout(7200)                    // Token 超时（秒）
    .active_timeout(1800)             // 活动超时（秒）
    .build();
```

## 🤝 贡献

欢迎贡献！请随时提交 issues 和 pull requests。

## 📄 许可证

本项目采用以下任一许可证：

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

由你选择。

## 🙏 致谢

本项目受 [sa-token](https://github.com/dromara/sa-token) Java 框架启发。

