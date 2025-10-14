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
- 🎧 **Event Listeners**: Monitor login, logout, kick-out, and other authentication events
- 🔑 **JWT Support**: Full JWT (JSON Web Token) implementation with multiple algorithms
- 🔒 **Security Features**: Nonce for replay attack prevention, refresh token mechanism
- 🌐 **OAuth2 Support**: Complete OAuth2 authorization code flow implementation

## 📦 Architecture

```
sa-token-rust/
├── sa-token-core/              # Core library (Token, Session, Manager)
│   ├── token/                  # Token management
│   │   ├── generator.rs        # Token generation (UUID, Random, JWT, Hash, Timestamp, Tik)
│   │   ├── validator.rs        # Token validation
│   │   ├── jwt.rs              # JWT implementation (HS256/384/512, RS256/384/512, ES256/384)
│   │   └── mod.rs              # Token types (TokenValue, TokenInfo)
│   ├── session/                # Session management
│   ├── permission/             # Permission and role checking
│   ├── event/                  # Event listener system
│   │   └── mod.rs              # Event bus, listeners (Login, Logout, KickOut, etc.)
│   ├── nonce.rs                # Nonce manager (replay attack prevention)
│   ├── refresh.rs              # Refresh token manager
│   ├── oauth2.rs               # OAuth2 authorization code flow
│   ├── manager.rs              # SaTokenManager (core manager)
│   ├── config.rs               # Configuration and builder
│   └── util.rs                 # StpUtil (utility class)
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
├── examples/                   # Example projects
│   ├── event_listener_example.rs      # Event listener demo
│   ├── jwt_example.rs                 # JWT complete demo
│   ├── token_styles_example.rs        # Token styles demo
│   ├── security_features_example.rs   # Nonce & Refresh token demo
│   └── oauth2_example.rs              # OAuth2 authorization flow demo
└── docs/                       # Documentation
    ├── JWT_GUIDE.md / JWT_GUIDE_zh-CN.md
    ├── OAUTH2_GUIDE.md / OAUTH2_GUIDE_zh-CN.md
    ├── EVENT_LISTENER.md / EVENT_LISTENER_zh-CN.md
    └── StpUtil.md / StpUtil_zh-CN.md
```

## 🎯 Core Components

### 1. **sa-token-core**
Core authentication and authorization logic:
- `SaTokenManager`: Main manager for token and session operations
- `StpUtil`: Utility class providing simplified API ([Documentation](docs/StpUtil.md))
- Token generation, validation, and refresh
- Multiple token styles (UUID, Random, JWT, Hash, Timestamp, Tik)
- Session management
- Permission and role checking
- Event listening system ([Documentation](docs/EVENT_LISTENER.md))
- JWT support with multiple algorithms ([JWT Guide](docs/JWT_GUIDE.md))
- Security features: Nonce (replay attack prevention), Refresh Token
- OAuth2 authorization code flow ([OAuth2 Guide](docs/OAUTH2_GUIDE.md))

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

### 6. Event Listeners

Monitor authentication events like login, logout, and kick-out:

```rust
use async_trait::async_trait;
use sa_token_core::SaTokenListener;
use std::sync::Arc;

// Create custom listener
struct MyListener;

#[async_trait]
impl SaTokenListener for MyListener {
    async fn on_login(&self, login_id: &str, token: &str, login_type: &str) {
        println!("User {} logged in", login_id);
        // Add your business logic here:
        // - Log to database
        // - Send notification
        // - Update statistics
    }

    async fn on_logout(&self, login_id: &str, token: &str, login_type: &str) {
        println!("User {} logged out", login_id);
    }

    async fn on_kick_out(&self, login_id: &str, token: &str, login_type: &str) {
        println!("User {} was kicked out", login_id);
    }
}

// Register listener
StpUtil::register_listener(Arc::new(MyListener)).await;

// Or use built-in logging listener
use sa_token_core::LoggingListener;
StpUtil::register_listener(Arc::new(LoggingListener)).await;

// Events are automatically triggered
let token = StpUtil::login("user_123").await?; // Triggers Login event
StpUtil::logout(&token).await?;                 // Triggers Logout event
StpUtil::kick_out("user_123").await?;          // Triggers KickOut event
```

For complete event listener documentation, see [Event Listener Guide](docs/EVENT_LISTENER.md).

### 7. Token Styles

sa-token-rust supports multiple token generation styles to meet different scenarios:

```rust
use sa_token_core::SaTokenConfig;
use sa_token_core::config::TokenStyle;

let config = SaTokenConfig::builder()
    .token_style(TokenStyle::Tik)  // Choose your preferred style
    .build_config();
```

#### Available Token Styles

| Style | Length | Example | Use Case |
|-------|--------|---------|----------|
| **Uuid** | 36 chars | `550e8400-e29b-41d4-a716-446655440000` | Standard UUID format, universally recognized |
| **SimpleUuid** | 32 chars | `550e8400e29b41d4a716446655440000` | UUID without hyphens, more compact |
| **Random32** | 32 chars | `a3f5c9d8e2b7f4a6c1e8d3b9f2a7c5e1` | Random hex string, good security |
| **Random64** | 64 chars | `a3f5c9d8...` | Longer random string, higher security |
| **Random128** | 128 chars | `a3f5c9d8...` | Maximum random length, ultra-secure |
| **Jwt** | Variable | `eyJhbGc...` | Self-contained token with claims ([JWT Guide](docs/JWT_GUIDE.md)) |
| **Hash** ⭐ | 64 chars | `472c7dce...` | SHA256 hash with user info, traceable |
| **Timestamp** ⭐ | ~30 chars | `1760404107094_a8f4f17d88fcddb8` | Includes timestamp, easy to track |
| **Tik** ⭐ | 8 chars | `GIxYHHD5` | Short and shareable, perfect for URLs |

⭐ = New in this version

#### Token Style Examples

```rust
// Uuid style (default)
.token_style(TokenStyle::Uuid)
// Output: 550e8400-e29b-41d4-a716-446655440000

// Hash style - includes user information in hash
.token_style(TokenStyle::Hash)
// Output: 472c7dceee2b3079a1ae70746f43ba99b91636292ba7811b3bc8985a1148836f

// Timestamp style - includes millisecond timestamp
.token_style(TokenStyle::Timestamp)
// Output: 1760404107094_a8f4f17d88fcddb8

// Tik style - short 8-character token
.token_style(TokenStyle::Tik)
// Output: GIxYHHD5

// JWT style - self-contained token with claims
.token_style(TokenStyle::Jwt)
.jwt_secret_key("your-secret-key")
// Output: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

#### Choosing the Right Token Style

- **Uuid/SimpleUuid**: Standard choice, widely compatible
- **Random32/64/128**: When you need random tokens with specific length
- **JWT**: When you need self-contained tokens with embedded information
- **Hash**: When you need tokens that can be traced back to user info
- **Timestamp**: When you need to know when the token was created
- **Tik**: When you need short tokens for sharing (URLs, QR codes, etc.)

Run the example to see all token styles in action:
```bash
cargo run --example token_styles_example
```

### 8. Security Features

#### Nonce (Replay Attack Prevention)

```rust
use sa_token_core::NonceManager;

let nonce_manager = NonceManager::new(storage, 300); // 5 minutes TTL

// Generate nonce
let nonce = nonce_manager.generate();

// Validate and consume (one-time use)
nonce_manager.validate_and_consume(&nonce, "user_123").await?;

// Second use will fail (replay attack detected)
match nonce_manager.validate_and_consume(&nonce, "user_123").await {
    Err(_) => println!("Replay attack prevented!"),
    _ => {}
}
```

#### Refresh Token

```rust
use sa_token_core::RefreshTokenManager;

let refresh_manager = RefreshTokenManager::new(storage, config);

// Generate refresh token
let refresh_token = refresh_manager.generate("user_123");
refresh_manager.store(&refresh_token, &access_token, "user_123").await?;

// Refresh access token when expired
let (new_access_token, user_id) = refresh_manager
    .refresh_access_token(&refresh_token)
    .await?;
```

Run security features example:
```bash
cargo run --example security_features_example
```

### 9. OAuth2 Authorization

Complete OAuth2 authorization code flow implementation:

```rust
use sa_token_core::{OAuth2Manager, OAuth2Client};

let oauth2 = OAuth2Manager::new(storage);

// Register OAuth2 client
let client = OAuth2Client {
    client_id: "web_app_001".to_string(),
    client_secret: "secret_abc123xyz".to_string(),
    redirect_uris: vec!["http://localhost:3000/callback".to_string()],
    grant_types: vec!["authorization_code".to_string()],
    scope: vec!["read".to_string(), "write".to_string()],
};

oauth2.register_client(&client).await?;

// Generate authorization code
let auth_code = oauth2.generate_authorization_code(
    "web_app_001".to_string(),
    "user_123".to_string(),
    "http://localhost:3000/callback".to_string(),
    vec!["read".to_string()],
);

oauth2.store_authorization_code(&auth_code).await?;

// Exchange code for token
let token = oauth2.exchange_code_for_token(
    &auth_code.code,
    "web_app_001",
    "secret_abc123xyz",
    "http://localhost:3000/callback",
).await?;

// Verify access token
let token_info = oauth2.verify_access_token(&token.access_token).await?;

// Refresh token
let new_token = oauth2.refresh_access_token(
    token.refresh_token.as_ref().unwrap(),
    "web_app_001",
    "secret_abc123xyz",
).await?;
```

📖 **[OAuth2 Complete Guide](docs/OAUTH2_GUIDE.md)**

Run OAuth2 example:
```bash
cargo run --example oauth2_example
```

📖 **[Full Event Listener Documentation](docs/EVENT_LISTENER.md)**

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

