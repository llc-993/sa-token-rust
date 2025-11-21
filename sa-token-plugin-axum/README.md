# sa-token-plugin-axum

Axum framework integration for sa-token-rust.

## Features

- ⚡ **High Performance**: Built for Axum 0.7+
- 🎯 **Easy Integration**: Middleware and extractors
- 🔧 **Flexible**: Multiple configuration options
- 🛡️ **Complete**: Full authentication and authorization support

## Installation

```toml
[dependencies]
sa-token-plugin-axum = { version = "0.1.9", features = ["redis"] }
axum = "0.8"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use axum::{Router, routing::get};
use sa_token_plugin_axum::{SaTokenState, SaTokenMiddleware, LoginIdExtractor};
use sa_token_storage_memory::MemoryStorage;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let state = SaTokenState::builder()
        .storage(Arc::new(MemoryStorage::new()))
        .timeout(7200)
        .build();
    
    let app = Router::new()
        .route("/user/info", get(user_info))
        .layer(SaTokenMiddleware::new(state.clone()))
        .with_state(state);
    
    // Start server...
}

async fn user_info(LoginIdExtractor(login_id): LoginIdExtractor) -> String {
    format!("User ID: {}", login_id)
}
```

## Configuration

```rust
let state = SaTokenState::builder()
    .storage(Arc::new(storage))
    .token_name("Authorization")
    .timeout(86400)
    .auto_renew(true)
    .token_style(TokenStyle::Random64)
    .build();
```

## Extractors

- `SaTokenExtractor`: Required token
- `OptionalSaTokenExtractor`: Optional token
- `LoginIdExtractor`: Get current login ID

## Author

**金书记**

## License

Licensed under either of Apache-2.0 or MIT.
