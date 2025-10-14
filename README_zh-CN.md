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
- 🎧 **事件监听**: 监听登录、登出、踢出下线等认证事件
- 🔑 **JWT 支持**: 完整的 JWT (JSON Web Token) 实现，支持多种算法
- 🔒 **安全特性**: Nonce 防重放攻击、Refresh Token 刷新机制
- 🌐 **OAuth2 支持**: 完整的 OAuth2 授权码模式实现

## 📦 架构

```
sa-token-rust/
├── sa-token-core/              # 核心库（Token、Session、Manager）
│   ├── token/                  # Token 管理
│   │   ├── generator.rs        # Token 生成（UUID、Random、JWT、Hash、Timestamp、Tik）
│   │   ├── validator.rs        # Token 验证
│   │   ├── jwt.rs              # JWT 实现（HS256/384/512、RS256/384/512、ES256/384）
│   │   └── mod.rs              # Token 类型（TokenValue、TokenInfo）
│   ├── session/                # Session 管理
│   ├── permission/             # 权限和角色检查
│   ├── event/                  # 事件监听系统
│   │   └── mod.rs              # 事件总线、监听器（Login、Logout、KickOut等）
│   ├── nonce.rs                # Nonce 管理器（防重放攻击）
│   ├── refresh.rs              # Refresh Token 管理器
│   ├── oauth2.rs               # OAuth2 授权码模式
│   ├── manager.rs              # SaTokenManager（核心管理器）
│   ├── config.rs               # 配置和构建器
│   └── util.rs                 # StpUtil（工具类）
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
├── examples/                   # 示例项目
│   ├── event_listener_example.rs      # 事件监听演示
│   ├── jwt_example.rs                 # JWT 完整演示
│   ├── token_styles_example.rs        # Token 风格演示
│   ├── security_features_example.rs   # Nonce & Refresh Token 演示
│   └── oauth2_example.rs              # OAuth2 授权流程演示
└── docs/                       # 文档
    ├── JWT_GUIDE.md / JWT_GUIDE_zh-CN.md
    ├── OAUTH2_GUIDE.md / OAUTH2_GUIDE_zh-CN.md
    ├── EVENT_LISTENER.md / EVENT_LISTENER_zh-CN.md
    └── StpUtil.md / StpUtil_zh-CN.md
```

## 🎯 核心组件

### 1. **sa-token-core**
核心认证授权逻辑：
- `SaTokenManager`: Token 和 Session 操作的主管理器
- `StpUtil`: 提供简化 API 的工具类 ([文档](docs/StpUtil_zh-CN.md))
- Token 生成、验证和刷新
- 多种 Token 风格（UUID、Random、JWT、Hash、Timestamp、Tik）
- Session 管理
- 权限和角色检查
- 事件监听系统 ([文档](docs/EVENT_LISTENER_zh-CN.md))
- JWT 支持，多种算法 ([JWT 指南](docs/JWT_GUIDE_zh-CN.md))
- 安全特性：Nonce 防重放攻击、Refresh Token 刷新机制
- OAuth2 授权码模式 ([OAuth2 指南](docs/OAUTH2_GUIDE_zh-CN.md))

### 2. **sa-token-adapter**
框架集成的抽象层：
- `SaStorage`: Token 和 Session 的存储接口
- `SaRequest` / `SaResponse`: 请求/响应抽象

### 3. **sa-token-macro**
用于注解式认证的过程宏：
- `#[sa_check_login]`: 要求登录
- `#[sa_check_permission("user:list")]`: 检查权限 ([匹配规则](docs/PermissionMatching.md#中文))
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

#### 方式 A: 使用内存存储（开发环境）

```rust
use sa_token_core::StpUtil;
use sa_token_plugin_axum::SaTokenState;
use sa_token_storage_memory::MemoryStorage;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // 创建状态（StpUtil 会自动初始化）
    let state = SaTokenState::builder()
        .storage(Arc::new(MemoryStorage::new()))
        .token_name("Authorization")
        .timeout(86400)  // 24 小时
        .build();
    
    // StpUtil 已就绪，可以直接使用！
    // 你的应用代码...
}
```

#### 方式 B: 使用 Redis 存储（生产环境）

**方法 1: Redis URL（推荐简单场景）**

```rust
use sa_token_storage_redis::RedisStorage;
use sa_token_plugin_axum::SaTokenState;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 连接 Redis（带密码）
    let storage = RedisStorage::new(
        "redis://:Aq23-hjPwFB3mBDNFp3W1@localhost:6379/0",
        "sa-token:"
    ).await?;
    
    let state = SaTokenState::builder()
        .storage(Arc::new(storage))
        .timeout(86400)
        .build();
    
    Ok(())
}
```

**方法 2: RedisConfig 结构体（推荐配置文件读取）**

```rust
use sa_token_storage_redis::{RedisStorage, RedisConfig};
use sa_token_plugin_axum::SaTokenState;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = RedisConfig {
        host: "localhost".to_string(),
        port: 6379,
        password: Some("Aq23-hjPwFB3mBDNFp3W1".to_string()),
        database: 0,
        pool_size: 10,
    };
    
    let storage = RedisStorage::from_config(config, "sa-token:").await?;
    
    let state = SaTokenState::builder()
        .storage(Arc::new(storage))
        .timeout(86400)
        .build();
    
    Ok(())
}
```

**方法 3: Builder 构建器（最灵活）**

```rust
use sa_token_storage_redis::RedisStorage;
use sa_token_plugin_axum::SaTokenState;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let storage = RedisStorage::builder()
        .host("localhost")
        .port(6379)
        .password("Aq23-hjPwFB3mBDNFp3W1")
        .database(0)
        .key_prefix("sa-token:")
        .build()
        .await?;
    
    let state = SaTokenState::builder()
        .storage(Arc::new(storage))
        .timeout(86400)
        .build();
    
    Ok(())
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

### 6. 事件监听

监听登录、登出、踢出下线等认证事件：

```rust
use async_trait::async_trait;
use sa_token_core::SaTokenListener;
use std::sync::Arc;

// 创建自定义监听器
struct MyListener;

#[async_trait]
impl SaTokenListener for MyListener {
    async fn on_login(&self, login_id: &str, token: &str, login_type: &str) {
        println!("用户 {} 登录了", login_id);
        // 在这里添加你的业务逻辑：
        // - 记录到数据库
        // - 发送通知
        // - 更新统计数据
    }

    async fn on_logout(&self, login_id: &str, token: &str, login_type: &str) {
        println!("用户 {} 登出了", login_id);
    }

    async fn on_kick_out(&self, login_id: &str, token: &str, login_type: &str) {
        println!("用户 {} 被踢出下线", login_id);
    }
}

// 注册监听器
StpUtil::register_listener(Arc::new(MyListener)).await;

// 或使用内置的日志监听器
use sa_token_core::LoggingListener;
StpUtil::register_listener(Arc::new(LoggingListener)).await;

// 事件会自动触发
let token = StpUtil::login("user_123").await?; // 触发登录事件
StpUtil::logout(&token).await?;                 // 触发登出事件
StpUtil::kick_out("user_123").await?;          // 触发踢出下线事件
```

📖 **[完整事件监听文档](docs/EVENT_LISTENER_zh-CN.md)**

### 7. Token 风格

sa-token-rust 支持多种 Token 生成风格，满足不同场景需求：

```rust
use sa_token_core::SaTokenConfig;
use sa_token_core::config::TokenStyle;

let config = SaTokenConfig::builder()
    .token_style(TokenStyle::Tik)  // 选择你喜欢的风格
    .build_config();
```

#### 可用的 Token 风格

| 风格 | 长度 | 示例 | 使用场景 |
|------|------|------|----------|
| **Uuid** | 36 字符 | `550e8400-e29b-41d4-a716-446655440000` | 标准 UUID 格式，通用性强 |
| **SimpleUuid** | 32 字符 | `550e8400e29b41d4a716446655440000` | 无横杠的 UUID，更紧凑 |
| **Random32** | 32 字符 | `a3f5c9d8e2b7f4a6c1e8d3b9f2a7c5e1` | 随机十六进制字符串，安全性好 |
| **Random64** | 64 字符 | `a3f5c9d8...` | 更长的随机字符串，安全性更高 |
| **Random128** | 128 字符 | `a3f5c9d8...` | 最长随机字符串，超高安全性 |
| **Jwt** | 可变长度 | `eyJhbGc...` | 自包含令牌，带有声明信息 ([JWT指南](docs/JWT_GUIDE.md)) |
| **Hash** ⭐ | 64 字符 | `472c7dce...` | SHA256 哈希，包含用户信息，可追溯 |
| **Timestamp** ⭐ | ~30 字符 | `1760404107094_a8f4f17d88fcddb8` | 包含时间戳，易于追踪 |
| **Tik** ⭐ | 8 字符 | `GIxYHHD5` | 短小精悍，适合分享 |

⭐ = 本版本新增

#### Token 风格示例

```rust
// Uuid 风格（默认）
.token_style(TokenStyle::Uuid)
// 输出: 550e8400-e29b-41d4-a716-446655440000

// Hash 风格 - 哈希中包含用户信息
.token_style(TokenStyle::Hash)
// 输出: 472c7dceee2b3079a1ae70746f43ba99b91636292ba7811b3bc8985a1148836f

// Timestamp 风格 - 包含毫秒级时间戳
.token_style(TokenStyle::Timestamp)
// 输出: 1760404107094_a8f4f17d88fcddb8

// Tik 风格 - 短小的8位字符 token
.token_style(TokenStyle::Tik)
// 输出: GIxYHHD5

// JWT 风格 - 自包含令牌
.token_style(TokenStyle::Jwt)
.jwt_secret_key("your-secret-key")
// 输出: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

#### 如何选择 Token 风格

- **Uuid/SimpleUuid**: 标准选择，兼容性广
- **Random32/64/128**: 需要特定长度的随机 token 时
- **JWT**: 需要自包含令牌，内嵌信息时
- **Hash**: 需要可追溯到用户信息的 token 时
- **Timestamp**: 需要知道 token 创建时间时
- **Tik**: 需要短小 token 用于分享（URL、二维码等）时

运行示例查看所有 Token 风格效果：
```bash
cargo run --example token_styles_example
```

### 8. 安全特性

#### Nonce 防重放攻击

```rust
use sa_token_core::NonceManager;

let nonce_manager = NonceManager::new(storage, 300); // 5 分钟有效期

// 生成 nonce
let nonce = nonce_manager.generate();

// 验证并消费（单次使用）
nonce_manager.validate_and_consume(&nonce, "user_123").await?;

// 第二次使用将失败（检测到重放攻击）
match nonce_manager.validate_and_consume(&nonce, "user_123").await {
    Err(_) => println!("重放攻击已阻止！"),
    _ => {}
}
```

#### Refresh Token 刷新机制

```rust
use sa_token_core::RefreshTokenManager;

let refresh_manager = RefreshTokenManager::new(storage, config);

// 生成 refresh token
let refresh_token = refresh_manager.generate("user_123");
refresh_manager.store(&refresh_token, &access_token, "user_123").await?;

// 访问令牌过期时刷新
let (new_access_token, user_id) = refresh_manager
    .refresh_access_token(&refresh_token)
    .await?;
```

运行安全特性示例：
```bash
cargo run --example security_features_example
```

### 9. OAuth2 授权

完整的 OAuth2 授权码模式实现：

```rust
use sa_token_core::{OAuth2Manager, OAuth2Client};

let oauth2 = OAuth2Manager::new(storage);

// 注册 OAuth2 客户端
let client = OAuth2Client {
    client_id: "web_app_001".to_string(),
    client_secret: "secret_abc123xyz".to_string(),
    redirect_uris: vec!["http://localhost:3000/callback".to_string()],
    grant_types: vec!["authorization_code".to_string()],
    scope: vec!["read".to_string(), "write".to_string()],
};

oauth2.register_client(&client).await?;

// 生成授权码
let auth_code = oauth2.generate_authorization_code(
    "web_app_001".to_string(),
    "user_123".to_string(),
    "http://localhost:3000/callback".to_string(),
    vec!["read".to_string()],
);

oauth2.store_authorization_code(&auth_code).await?;

// 授权码换取令牌
let token = oauth2.exchange_code_for_token(
    &auth_code.code,
    "web_app_001",
    "secret_abc123xyz",
    "http://localhost:3000/callback",
).await?;

// 验证访问令牌
let token_info = oauth2.verify_access_token(&token.access_token).await?;

// 刷新令牌
let new_token = oauth2.refresh_access_token(
    token.refresh_token.as_ref().unwrap(),
    "web_app_001",
    "secret_abc123xyz",
).await?;
```

📖 **[OAuth2 完整指南](docs/OAUTH2_GUIDE_zh-CN.md)**

运行 OAuth2 示例：
```bash
cargo run --example oauth2_example
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
- [事件监听系统](docs/EVENT_LISTENER.md) - 监听登录、登出等认证事件
- [权限匹配规则](docs/PermissionMatching.md#中文) - 权限检查工作原理
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

## 👨‍💻 作者

**金书记**

## 🙏 致谢

本项目受 [sa-token](https://github.com/dromara/sa-token) Java 框架启发。

