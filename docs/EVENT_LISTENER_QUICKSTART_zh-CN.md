# 事件监听快速开始

[English](./EVENT_LISTENER_QUICKSTART.md) | 中文

---

## 概述

sa-token-rust 提供了强大的事件监听功能，可以监听登录、登出、踢出下线等操作。

## 快速开始

### 1. 创建自定义监听器

```rust
use async_trait::async_trait;
use sa_token_core::SaTokenListener;

struct MyListener;

#[async_trait]
impl SaTokenListener for MyListener {
    async fn on_login(&self, login_id: &str, token: &str, login_type: &str) {
        println!("用户 {} 登录了", login_id);
        // 在这里添加您的业务逻辑
    }

    async fn on_logout(&self, login_id: &str, token: &str, login_type: &str) {
        println!("用户 {} 登出了", login_id);
    }

    async fn on_kick_out(&self, login_id: &str, token: &str, login_type: &str) {
        println!("用户 {} 被踢出下线", login_id);
    }
}
```

### 2. 注册监听器

```rust
use sa_token_core::{SaTokenManager, StpUtil};
use std::sync::Arc;

// 方式一：通过 Manager 注册
let manager = SaTokenManager::new(storage, config);
manager.event_bus().register(Arc::new(MyListener)).await;

// 方式二：通过 StpUtil 注册
StpUtil::init_manager(manager);
StpUtil::register_listener(Arc::new(MyListener)).await;
```

### 3. 使用内置的日志监听器

```rust
use sa_token_core::LoggingListener;

manager.event_bus().register(Arc::new(LoggingListener)).await;
```

### 4. 自动触发事件

一旦注册了监听器，相关操作会自动触发事件：

```rust
// 登录 - 触发 Login 事件
let token = StpUtil::login("user_123").await?;

// 登出 - 触发 Logout 事件
StpUtil::logout(&token).await?;

// 踢出下线 - 触发 KickOut 事件
StpUtil::kick_out("user_123").await?;
```

## 支持的事件类型

- **Login**: 登录事件
- **Logout**: 登出事件
- **KickOut**: 踢出下线事件
- **RenewTimeout**: Token 续期事件
- **Replaced**: 被顶下线事件
- **Banned**: 被封禁事件

## 运行示例

```bash
cargo run --example event_listener_example
```

## 更多信息

查看完整文档：[EVENT_LISTENER_zh-CN.md](./EVENT_LISTENER_zh-CN.md)

