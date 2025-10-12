# sa-token-rust

[中文文档](README_zh-CN.md) | English

A lightweight, high-performance authentication and authorization framework for Rust, inspired by [sa-token](https://github.com/dromara/sa-token).

## ✨ Features

- 🚀 **Multiple Web Framework Support**: Axum, Actix-web, Poem, Rocket, Warp
- 🔐 **Complete Authentication**: Login, logout, token validation, session management
- 🛡️ **Fine-grained Authorization**: Permission and role-based access control
- 💾 **Flexible Storage**: Memory, Redis, and database storage backends
- 🎯 **Easy to Use**: Procedural macros and utility classes for simple integration
- ⚡ **High Performance**: Zero-copy design, async/await support
- 🔧 **Highly Configurable**: Token timeout, cookie options, custom token names

## 📦 Architecture

```
sa-token-rust/
├── sa-token-core/              # Core library (Token, Session, Manager)
├── sa-token-adapter/           # Adapter interfaces (Storage, Request/Response)
├── sa-token-macro/             # Procedural macros (#[sa_check_login], etc.)
├── sa-token-storage-memory/    # Memory storage implementation
├── sa-token-storage-redis/     # Redis storage implementation
├── sa-token-storage-database/  # Database storage implementation (placeholder)
├── sa-token-plugin-axum/       # Axum framework integration
├── sa-token-plugin-actix-web/  # Actix-web framework integration
├── sa-token-plugin-poem/       # Poem framework integration
├── sa-token-plugin-rocket/     # Rocket framework integration
├── sa-token-plugin-warp/       # Warp framework integration
└── examples/                   # Example projects
    ├── axum-full-example/      # Complete Axum example
    └── poem-full-example/      # Complete Poem example
```

## 🎯 Core Components

### 1. **sa-token-core**
Core authentication and authorization logic:
- `SaTokenManager`: Main manager for token and session operations
- `StpUtil`: Utility class providing simplified API ([Documentation](docs/StpUtil.md))
- Token generation, validation, and refresh
- Session management
- Permission and role checking

### 2. **sa-token-adapter**
Abstraction layer for framework integration:
- `SaStorage`: Storage interface for tokens and sessions
- `SaRequest` / `SaResponse`: Request/response abstraction

### 3. **sa-token-macro**
Procedural macros for annotation-style authentication:
- `#[sa_check_login]`: Require login
- `#[sa_check_permission("user:list")]`: Check permission ([Matching Rules](docs/PermissionMatching.md))
- `#[sa_check_role("admin")]`: Check role
- `#[sa_check_permissions_and(...)]`: Check multiple permissions (AND)
- `#[sa_check_permissions_or(...)]`: Check multiple permissions (OR)
- `#[sa_ignore]`: Skip authentication

### 4. **Web Framework Plugins**
All plugins provide:
- State management with Builder pattern
- Dual middleware (basic + login-required)
- Three extractors (required, optional, LoginId)
- Request/Response adapters
- Token extraction from Header/Cookie/Query
- Bearer token support

## 🚀 Quick Start

### 1. Add Dependencies

```toml
[dependencies]
sa-token-core = "0.1"
sa-token-storage-memory = "0.1"
sa-token-plugin-axum = "0.1"  # Choose your framework
tokio = { version = "1", features = ["full"] }
axum = "0.7"
```

### 2. Initialize sa-token

#### Option A: Using Memory Storage (Development)

```rust
use sa_token_core::StpUtil;
use sa_token_plugin_axum::SaTokenState;
use sa_token_storage_memory::MemoryStorage;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create state (StpUtil is automatically initialized)
    let state = SaTokenState::builder()
        .storage(Arc::new(MemoryStorage::new()))
        .token_name("Authorization")
        .timeout(86400)  // 24 hours
        .build();
    
    // StpUtil is ready to use!
    // Your application code...
}
```

#### Option B: Using Redis Storage (Production)

**Method 1: Redis URL (Recommended for simple scenarios)**

```rust
use sa_token_storage_redis::RedisStorage;
use sa_token_plugin_axum::SaTokenState;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Redis with password
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

**Method 2: RedisConfig Structure (Recommended for configuration files)**

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

**Method 3: Builder Pattern (Most flexible)**

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

### 3. User Login

```rust
use sa_token_core::StpUtil;

// User login
let token = StpUtil::login("user_id_10001").await?;
println!("Token: {}", token.value());

// Set permissions and roles
StpUtil::set_permissions(
    "user_id_10001",
    vec!["user:list".to_string(), "user:add".to_string()]
).await?;

StpUtil::set_roles(
    "user_id_10001",
    vec!["admin".to_string()]
).await?;
```

### 4. Check Authentication (Axum Example)

```rust
use axum::{Router, routing::get};
use sa_token_plugin_axum::{SaTokenMiddleware, LoginIdExtractor};

async fn user_info(LoginIdExtractor(login_id): LoginIdExtractor) -> String {
    format!("Current user: {}", login_id)
}

async fn admin_panel(login_id: LoginIdExtractor) -> String {
    // Check permission
    if !StpUtil::has_permission(&login_id.0, "admin:panel").await {
        return "No permission".to_string();
    }
    format!("Welcome admin: {}", login_id.0)
}

let app = Router::new()
    .route("/user/info", get(user_info))
    .route("/admin/panel", get(admin_panel))
    .layer(SaTokenMiddleware::new(state));
```

### 5. Using Procedural Macros

```rust
use sa_token_macro::*;

#[sa_check_login]
async fn protected_route() -> &'static str {
    "This route requires login"
}

#[sa_check_permission("user:delete")]
async fn delete_user(user_id: String) -> &'static str {
    "User deleted"
}

#[sa_check_role("admin")]
async fn admin_only() -> &'static str {
    "Admin only content"
}
```

## 📚 Framework Integration Examples

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
    format!("User: {}", login_id.0)
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
        format!("User info")
    });

warp::serve(routes)
    .run(([127, 0, 0, 1], 8080))
    .await;
```

## 📖 Documentation

- [StpUtil API Reference](docs/StpUtil.md) - Complete guide to StpUtil utility class
- [Permission Matching Rules](docs/PermissionMatching.md#english) - How permission checking works
- [Examples](examples/) - Working examples for all supported frameworks

## 🔧 Advanced Usage

### Custom Storage

Implement the `SaStorage` trait for your own storage backend:

```rust
use sa_token_adapter::storage::SaStorage;
use async_trait::async_trait;

pub struct CustomStorage;

#[async_trait]
impl SaStorage for CustomStorage {
    async fn get(&self, key: &str) -> Option<String> {
        // Your implementation
    }
    
    async fn set(&self, key: &str, value: String, timeout: Option<i64>) {
        // Your implementation
    }
    
    // ... other methods
}
```

### Token Configuration

```rust
let state = SaTokenState::builder()
    .storage(Arc::new(MemoryStorage::new()))
    .token_name("X-Token")           // Custom token name
    .timeout(7200)                    // Token timeout (seconds)
    .active_timeout(1800)             // Activity timeout (seconds)
    .build();
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## 📄 License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 👨‍💻 Author

**金书记**

## 🙏 Acknowledgments

This project is inspired by [sa-token](https://github.com/dromara/sa-token) Java framework.

