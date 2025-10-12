# sa-token-core

Core library for sa-token-rust authentication and authorization framework.

## Features

- 🔐 **Token Management**: Generate, validate, and refresh tokens
- 📦 **Session Management**: User session storage and retrieval
- 🛡️ **Permission & Role System**: Fine-grained access control
- 🎯 **StpUtil**: Simplified utility API
- ⚡ **High Performance**: Async/await support with zero-copy design

## Installation

```toml
[dependencies]
sa-token-core = "0.1.2"
sa-token-adapter = "0.1.2"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use sa_token_core::{SaTokenManager, SaTokenConfig, StpUtil};
use sa_token_storage_memory::MemoryStorage;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create manager
    let storage = Arc::new(MemoryStorage::new());
    let config = SaTokenConfig::default();
    let manager = SaTokenManager::new(storage, config);
    
    // Initialize StpUtil
    StpUtil::init_manager(manager);
    
    // User login
    let token = StpUtil::login("user_123").await?;
    println!("Token: {}", token.value());
    
    // Check login status
    let is_logged_in = StpUtil::is_login_by_login_id("user_123").await;
    println!("Is logged in: {}", is_logged_in);
    
    Ok(())
}
```

## Core Components

### SaTokenManager

Main manager for authentication operations:
- Token generation and validation
- Session management
- Permission and role checking

### StpUtil

Utility class providing simplified API:
- Login/Logout operations
- Permission/Role management
- Token information retrieval

See [StpUtil Documentation](../docs/StpUtil.md) for details.

## Documentation

- [Main Documentation](../README.md)
- [StpUtil Usage](../docs/StpUtil.md)
- [Permission Matching Rules](../docs/PermissionMatching.md)

## Author

**金书记**

## License

Licensed under either of:
- Apache License, Version 2.0
- MIT License
