# Event Listener Quick Start

English | [中文](./EVENT_LISTENER_QUICKSTART_zh-CN.md)

---

## Overview

sa-token-rust provides powerful event listening capabilities to monitor login, logout, kick-out, and other operations.

## Quick Start

### 1. Create Custom Listener

```rust
use async_trait::async_trait;
use sa_token_core::SaTokenListener;

struct MyListener;

#[async_trait]
impl SaTokenListener for MyListener {
    async fn on_login(&self, login_id: &str, token: &str, login_type: &str) {
        println!("User {} logged in", login_id);
        // Add your business logic here
    }

    async fn on_logout(&self, login_id: &str, token: &str, login_type: &str) {
        println!("User {} logged out", login_id);
    }

    async fn on_kick_out(&self, login_id: &str, token: &str, login_type: &str) {
        println!("User {} was kicked out", login_id);
    }
}
```

### 2. Register Listener

```rust
use sa_token_core::{SaTokenManager, StpUtil};
use std::sync::Arc;

// Method 1: Register via Manager
let manager = SaTokenManager::new(storage, config);
manager.event_bus().register(Arc::new(MyListener)).await;

// Method 2: Register via StpUtil
StpUtil::init_manager(manager);
StpUtil::register_listener(Arc::new(MyListener)).await;
```

### 3. Use Built-in Logging Listener

```rust
use sa_token_core::LoggingListener;

manager.event_bus().register(Arc::new(LoggingListener)).await;
```

### 4. Automatic Event Triggering

Once listeners are registered, events are automatically triggered:

```rust
// Login - triggers Login event
let token = StpUtil::login("user_123").await?;

// Logout - triggers Logout event
StpUtil::logout(&token).await?;

// Kick out - triggers KickOut event
StpUtil::kick_out("user_123").await?;
```

## Supported Event Types

- **Login**: Login event
- **Logout**: Logout event
- **KickOut**: Kick-out event
- **RenewTimeout**: Token renewal event
- **Replaced**: Replaced event
- **Banned**: Banned event

## Run Example

```bash
cargo run --example event_listener_example
```

## More Information

See full documentation: [EVENT_LISTENER.md](./EVENT_LISTENER.md)
