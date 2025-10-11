# sa-token-plugin-poem

[Poem](https://github.com/poem-web/poem) 框架集成插件，为 `sa-token-rust` 提供完整的 Poem 框架支持。

## 功能特性

- 🔐 **完整的认证和授权支持** - 提供登录验证、权限检查、角色验证等功能
- 🚀 **高性能异步中间件** - 基于 Poem 的异步中间件实现
- 🎯 **灵活的提取器 (Extractor)** - 支持多种 Token 提取方式
- 🛠 **易于集成** - 简单的 API 设计，开箱即用
- 📦 **Builder 模式** - 优雅的配置方式

## 快速开始

### 添加依赖

```toml
[dependencies]
sa-token-plugin-poem = "0.1"
sa-token-storage-memory = "0.1"
poem = "3.0"
tokio = { version = "1", features = ["full"] }
```

### 基础使用

```rust
use std::sync::Arc;
use poem::{Route, Server, listener::TcpListener, handler, web::Json};
use sa_token_plugin_poem::{SaTokenState, SaTokenMiddleware, SaTokenExtractor};
use sa_token_storage_memory::MemoryStorage;
use sa_token_core::StpUtil;

#[handler]
async fn user_info(token: SaTokenExtractor) -> String {
    format!("User ID: {}", token.login_id())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // 1. 创建 sa-token 状态
    let sa_token_state = SaTokenState::builder()
        .storage(Arc::new(MemoryStorage::new()))
        .token_name("Authorization")
        .timeout(7200)
        .build();
    
    // 2. 初始化全局 StpUtil
    StpUtil::init_manager((*sa_token_state.manager).clone());
    
    // 3. 创建路由并应用中间件
    let app = Route::new()
        .at("/api/user/info", poem::get(user_info))
        .with(SaTokenMiddleware::new(sa_token_state.manager.clone()))
        .data(sa_token_state);
    
    // 4. 启动服务器
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
```

## 核心组件

### SaTokenState

应用状态管理，提供 Builder 模式配置：

```rust
let state = SaTokenState::builder()
    .storage(Arc::new(MemoryStorage::new()))
    .token_name("Authorization")
    .timeout(7200)                    // Token 超时时间（秒）
    .token_style(TokenStyle::Uuid)    // Token 生成方式
    .build();
```

### 中间件

#### SaTokenMiddleware

基础中间件，自动提取和验证 Token：

```rust
let app = Route::new()
    .at("/api/user", poem::get(handler))
    .with(SaTokenMiddleware::new(manager));
```

#### SaCheckLoginMiddleware

强制登录检查中间件：

```rust
let app = Route::new()
    .at("/api/user", poem::get(handler))
    .with(SaCheckLoginMiddleware::new(manager));
```

### 提取器 (Extractor)

#### SaTokenExtractor

提取 Token 信息，如果未登录会返回 401 错误：

```rust
#[handler]
async fn user_info(token: SaTokenExtractor) -> String {
    format!("User ID: {}", token.login_id())
}
```

#### OptionalSaTokenExtractor

可选的 Token 提取器，未登录不会报错：

```rust
#[handler]
async fn user_info(token: OptionalSaTokenExtractor) -> String {
    match token.0 {
        Some(t) => format!("User ID: {}", t.login_id()),
        None => "Guest".to_string(),
    }
}
```

#### LoginIdExtractor

直接提取登录 ID：

```rust
#[handler]
async fn user_info(LoginIdExtractor(user_id): LoginIdExtractor) -> String {
    format!("User ID: {}", user_id)
}
```

## 权限和角色管理

使用 `StpUtil` 进行权限和角色管理：

```rust
use sa_token_core::StpUtil;

// 设置用户权限
StpUtil::set_permissions(
    "user_123",
    vec!["user:read".to_string(), "user:write".to_string()],
).await?;

// 设置用户角色
StpUtil::set_roles(
    "user_123",
    vec!["admin".to_string()],
).await?;

// 检查权限
if StpUtil::has_permission("user_123", "user:read").await {
    // 有权限
}

// 检查角色
if StpUtil::has_role("user_123", "admin").await {
    // 有角色
}
```

## Token 提取顺序

中间件按以下优先级顺序查找 Token：

1. **HTTP Header** - `Authorization: <token>`
2. **Cookie** - `Authorization=<token>`
3. **Query Parameter** - `?Authorization=<token>`

## 完整示例

查看 [examples/poem-full-example](../../examples/poem-full-example) 获取完整的使用示例，包括：

- ✅ 用户登录和登出
- ✅ Token 验证
- ✅ 权限和角色管理
- ✅ 受保护的路由
- ✅ 多种提取器使用

## API 文档

完整的 API 文档请访问：[docs.rs/sa-token-plugin-poem](https://docs.rs/sa-token-plugin-poem)

## 许可证

MIT OR Apache-2.0

