# sa-token-rust

[![Crates.io](https://img.shields.io/crates/v/sa-token-rust.svg)](https://crates.io/crates/sa-token-rust)
[![Documentation](https://docs.rs/sa-token-rust/badge.svg)](https://docs.rs/sa-token-rust)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

🦀 一个强大的Rust认证授权框架，灵感来自Java的sa-token。

## ✨ 特性

- 🔐 **登录认证** - Token生成、验证、刷新
- 🛡️ **权限验证** - 基于角色/权限的访问控制
- 📦 **Session管理** - 灵活的会话存储与管理
- 🚀 **框架无关** - 核心逻辑与Web框架解耦
- 🔌 **多框架支持** - Axum、Actix-web、Rocket、Warp、Poem
- 💾 **多存储后端** - 内存、Redis、数据库
- ⚡ **高性能** - 基于Tokio异步运行时
- 🎯 **类型安全** - 充分利用Rust的类型系统
- 🔧 **易于扩展** - 基于trait的适配器模式

## 📦 项目结构

```
sa-token-rust/
├── sa-token-core/              # 核心库（框架无关）
├── sa-token-adapter/           # 适配器trait定义
├── sa-token-macro/             # 过程宏支持
├── sa-token-storage-memory/    # 内存存储
├── sa-token-storage-redis/     # Redis存储
├── sa-token-storage-database/  # 数据库存储（占位符）
├── sa-token-plugin-axum/       # Axum集成
├── sa-token-plugin-actix-web/  # Actix-web集成
├── sa-token-plugin-rocket/     # Rocket集成（占位符）
├── sa-token-plugin-warp/       # Warp集成（占位符）
└── sa-token-plugin-poem/       # Poem集成（占位符）
```

## 🚀 快速开始

### 使用Axum

```toml
[dependencies]
sa-token-core = "0.1"
sa-token-storage-memory = "0.1"
sa-token-plugin-axum = "0.1"
tokio = { version = "1", features = ["full"] }
axum = "0.7"
```

```rust
use std::sync::Arc;
use axum::{Router, routing::{get, post}, Json};
use sa_token_core::{SaTokenConfig, SaTokenManager};
use sa_token_storage_memory::MemoryStorage;
use sa_token_plugin_axum::{SaTokenLayer, SaTokenState};

#[tokio::main]
async fn main() {
    // 创建存储
    let storage = Arc::new(MemoryStorage::new());
    
    // 创建配置
    let config = SaTokenConfig::default();
    
    // 创建状态
    let state = SaTokenState::new(storage, config);
    
    // 创建路由
    let app = Router::new()
        .route("/login", post(login))
        .route("/user/info", get(user_info))
        .layer(SaTokenLayer::new(state.clone()))
        .with_state(state);
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn login(state: axum::extract::State<SaTokenState>) -> Json<String> {
    let token = state.manager.login("user_123").await.unwrap();
    Json(token.to_string())
}

async fn user_info() -> &'static str {
    "User info"
}
```

### 使用Actix-web

```toml
[dependencies]
sa-token-core = "0.1"
sa-token-storage-memory = "0.1"
sa-token-plugin-actix-web = "0.1"
tokio = { version = "1", features = ["full"] }
actix-web = "4"
```

```rust
use std::sync::Arc;
use actix_web::{web, App, HttpServer, HttpResponse};
use sa_token_core::SaTokenConfig;
use sa_token_storage_memory::MemoryStorage;
use sa_token_plugin_actix_web::{SaTokenMiddleware, SaTokenAppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let storage = Arc::new(MemoryStorage::new());
    let config = SaTokenConfig::default();
    let state = SaTokenAppState::new(storage, config);
    
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(SaTokenMiddleware)
            .route("/login", web::post().to(login))
            .route("/user/info", web::get().to(user_info))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn login(state: web::Data<SaTokenAppState>) -> HttpResponse {
    let token = state.manager.login("user_123").await.unwrap();
    HttpResponse::Ok().json(token.to_string())
}

async fn user_info() -> HttpResponse {
    HttpResponse::Ok().body("User info")
}
```

## 📚 核心概念

### Token管理

```rust
// 登录
let token = manager.login("user_123").await?;

// 登出
manager.logout(&token).await?;

// 验证token
let is_valid = manager.is_valid(&token).await;

// 获取token信息
let token_info = manager.get_token_info(&token).await?;

// 踢人下线
manager.kick_out("user_123").await?;
```

### Session管理

```rust
// 获取session
let mut session = manager.get_session("user_123").await?;

// 设置值
session.set("nickname", "张三")?;
session.set("age", 25)?;

// 获取值
let nickname: String = session.get("nickname").unwrap();
let age: i32 = session.get("age").unwrap();

// 保存session
manager.save_session(&session).await?;
```

### 配置

```rust
use sa_token_core::{SaTokenConfig, TokenStyle};

let config = SaTokenConfig::builder()
    .token_name("Authorization")
    .timeout(86400)  // 24小时
    .token_style(TokenStyle::Uuid)
    .token_prefix("Bearer")
    .jwt_secret_key("your-secret-key")
    .build();
```

### 存储适配

```rust
// 内存存储（开发环境）
let storage = Arc::new(MemoryStorage::new());

// Redis存储（生产环境）
let storage = Arc::new(
    RedisStorage::new("redis://127.0.0.1:6379", "sa-token:").await?
);
```

## 🔧 扩展开发

### 实现自定义存储

```rust
use async_trait::async_trait;
use sa_token_adapter::storage::{SaStorage, StorageResult};

pub struct MyStorage;

#[async_trait]
impl SaStorage for MyStorage {
    async fn get(&self, key: &str) -> StorageResult<Option<String>> {
        // 实现获取逻辑
    }
    
    async fn set(&self, key: &str, value: &str, ttl: Option<Duration>) -> StorageResult<()> {
        // 实现设置逻辑
    }
    
    // 实现其他必需方法...
}
```

### 为新框架添加支持

1. 创建新的插件crate
2. 实现`SaRequest`和`SaResponse` trait
3. 实现框架特定的中间件/拦截器
4. 提供文档和示例

## 🎯 路线图

- [x] 核心Token管理功能
- [x] Session管理
- [x] 内存存储实现
- [x] Redis存储实现
- [x] Axum框架集成
- [x] Actix-web框架集成
- [ ] 数据库存储实现
- [ ] 权限验证系统完整实现
- [ ] 过程宏完整实现
- [ ] Rocket框架集成
- [ ] Warp框架集成
- [ ] Poem框架集成
- [ ] JWT支持
- [ ] SSO单点登录
- [ ] OAuth2集成
- [ ] 完整文档和示例

## 🤝 贡献

欢迎贡献代码、报告问题或提出建议！

1. Fork本仓库
2. 创建你的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交你的改动 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启一个Pull Request

## 📄 许可证

本项目采用 MIT 或 Apache-2.0 双重许可。

## 🙏 致谢

- 灵感来自 [sa-token](https://github.com/dromara/sa-token) (Java)
- 感谢Rust社区的所有贡献者

## 📮 联系方式

- Issue: [GitHub Issues](https://github.com/your-username/sa-token-rust/issues)
- 讨论: [GitHub Discussions](https://github.com/your-username/sa-token-rust/discussions)

---

**注意**: 本项目目前处于早期开发阶段，API可能会发生变化。不建议在生产环境中使用。

