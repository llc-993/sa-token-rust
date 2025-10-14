# sa-token 事件监听

[English](./EVENT_LISTENER.md) | 中文

---

sa-token-rust 提供了完整的事件监听机制，允许您监听和响应各种认证相关的事件。

## 目录

- [概述](#概述)
- [事件类型](#事件类型)
- [基本使用](#基本使用)
- [自定义监听器](#自定义监听器)
- [内置监听器](#内置监听器)
- [实际应用场景](#实际应用场景)
- [API 参考](#api-参考)

## 概述

事件监听系统基于观察者模式实现，允许您在认证操作（登录、登出、踢出下线等）发生时执行自定义逻辑。

### 核心组件

- **SaTokenEvent**: 事件数据结构
- **SaTokenEventType**: 事件类型枚举
- **SaTokenListener**: 监听器 trait
- **SaTokenEventBus**: 事件总线，管理和分发事件

## 事件类型

sa-token 支持以下事件类型：

| 事件类型 | 说明 | 触发时机 |
|---------|------|---------|
| `Login` | 登录事件 | 用户成功登录时 |
| `Logout` | 登出事件 | 用户主动登出时 |
| `KickOut` | 踢出下线事件 | 管理员强制踢出用户时 |
| `RenewTimeout` | Token续期事件 | Token 过期时间被更新时 |
| `Replaced` | 被顶下线事件 | 用户在其他设备登录导致当前设备下线 |
| `Banned` | 被封禁事件 | 用户账号被封禁时 |

## 基本使用

### 1. 注册监听器

有两种方式注册监听器：

#### 方式一：通过 SaTokenManager

```rust
use sa_token_core::{SaTokenManager, LoggingListener};
use std::sync::Arc;

// 创建管理器
let manager = SaTokenManager::new(storage, config);

// 注册监听器
manager.event_bus()
    .register(Arc::new(LoggingListener))
    .await;
```

#### 方式二：通过 StpUtil

```rust
use sa_token_core::{StpUtil, LoggingListener};
use std::sync::Arc;

// 初始化 StpUtil
StpUtil::init_manager(manager);

// 注册监听器
StpUtil::register_listener(Arc::new(LoggingListener)).await;
```

### 2. 自动触发事件

一旦注册了监听器，相关操作会自动触发事件：

```rust
// 登录 - 触发 Login 事件
let token = StpUtil::login("user_123").await?;

// 登出 - 触发 Logout 事件
StpUtil::logout(&token).await?;

// 踢出下线 - 触发 KickOut 事件
StpUtil::kick_out("user_123").await?;
```

## 自定义监听器

### 实现监听器 trait

```rust
use async_trait::async_trait;
use sa_token_core::SaTokenListener;

struct MyListener;

#[async_trait]
impl SaTokenListener for MyListener {
    async fn on_login(&self, login_id: &str, token: &str, login_type: &str) {
        println!("用户 {} 登录了，token: {}", login_id, token);
        
        // 在这里添加您的业务逻辑
        // 例如：
        // - 记录登录日志到数据库
        // - 更新用户最后登录时间
        // - 发送登录通知
        // - 统计登录次数
    }

    async fn on_logout(&self, login_id: &str, token: &str, login_type: &str) {
        println!("用户 {} 登出了", login_id);
    }

    async fn on_kick_out(&self, login_id: &str, token: &str, login_type: &str) {
        println!("用户 {} 被踢出下线", login_id);
    }

    // 其他事件方法是可选的
    // async fn on_renew_timeout(...) {}
    // async fn on_replaced(...) {}
    // async fn on_banned(...) {}
}
```

### 注册并使用

```rust
// 注册监听器
StpUtil::register_listener(Arc::new(MyListener)).await;

// 正常使用 sa-token，事件会自动触发
let token = StpUtil::login("user_123").await?;
```

## 内置监听器

### LoggingListener

内置的日志监听器，使用 tracing 记录所有事件：

```rust
use sa_token_core::LoggingListener;
use std::sync::Arc;

// 注册日志监听器
StpUtil::register_listener(Arc::new(LoggingListener)).await;
```

输出示例：
```
INFO  用户登录 login_id="user_123" token="abc..." login_type="default"
INFO  用户登出 login_id="user_123" token="abc..." login_type="default"
WARN  用户被踢出下线 login_id="user_123" token="abc..." login_type="default"
```

## 实际应用场景

### 场景1：登录日志记录

```rust
struct LoginLogListener {
    db_pool: Arc<DbPool>,
}

#[async_trait]
impl SaTokenListener for LoginLogListener {
    async fn on_login(&self, login_id: &str, token: &str, _login_type: &str) {
        // 记录到数据库
        let log = LoginLog {
            user_id: login_id.to_string(),
            token: token.to_string(),
            login_time: Utc::now(),
            ip_address: get_client_ip(),
        };
        
        if let Err(e) = self.db_pool.insert_login_log(log).await {
            eprintln!("记录登录日志失败: {}", e);
        }
    }
}
```

### 场景2：安全监控

```rust
struct SecurityMonitorListener {
    alert_service: Arc<AlertService>,
}

#[async_trait]
impl SaTokenListener for SecurityMonitorListener {
    async fn on_login(&self, login_id: &str, _token: &str, _login_type: &str) {
        // 检查异常登录
        if self.is_suspicious_login(login_id).await {
            self.alert_service.send_alert(
                format!("检测到用户异常登录: {}", login_id)
            ).await;
        }
    }

    async fn on_kick_out(&self, login_id: &str, _token: &str, _login_type: &str) {
        // 记录安全事件
        self.alert_service.log_security_event(
            "用户被踢出下线",
            login_id,
        ).await;
    }
}

impl SecurityMonitorListener {
    async fn is_suspicious_login(&self, login_id: &str) -> bool {
        // 实现异常检测逻辑
        // 例如：检查登录频率、IP地理位置、设备指纹等
        false
    }
}
```

### 场景3：实时统计

```rust
struct StatisticsListener {
    redis: Arc<RedisClient>,
}

#[async_trait]
impl SaTokenListener for StatisticsListener {
    async fn on_login(&self, _login_id: &str, _token: &str, _login_type: &str) {
        // 增加在线用户计数
        let _ = self.redis.incr("online_users").await;
        
        // 增加今日登录次数
        let key = format!("login_count:{}", today());
        let _ = self.redis.incr(&key).await;
    }

    async fn on_logout(&self, _login_id: &str, _token: &str, _login_type: &str) {
        // 减少在线用户计数
        let _ = self.redis.decr("online_users").await;
    }
}
```

### 场景4：WebSocket 推送通知

```rust
struct WebSocketNotifyListener {
    ws_manager: Arc<WebSocketManager>,
}

#[async_trait]
impl SaTokenListener for WebSocketNotifyListener {
    async fn on_kick_out(&self, login_id: &str, _token: &str, _login_type: &str) {
        // 通过 WebSocket 通知用户被踢出
        let message = json!({
            "type": "kicked_out",
            "message": "您的账号已在其他设备登录",
            "timestamp": Utc::now()
        });
        
        let _ = self.ws_manager
            .send_to_user(login_id, message)
            .await;
    }
}
```

### 场景5：多监听器协作

```rust
// 同时注册多个监听器
async fn setup_listeners(manager: &SaTokenManager) {
    // 日志记录
    manager.event_bus()
        .register(Arc::new(LoggingListener))
        .await;
    
    // 数据库记录
    let db_listener = LoginLogListener {
        db_pool: Arc::clone(&db_pool),
    };
    manager.event_bus()
        .register(Arc::new(db_listener))
        .await;
    
    // 安全监控
    let security_listener = SecurityMonitorListener {
        alert_service: Arc::clone(&alert_service),
    };
    manager.event_bus()
        .register(Arc::new(security_listener))
        .await;
    
    // 实时统计
    let stats_listener = StatisticsListener {
        redis: Arc::clone(&redis_client),
    };
    manager.event_bus()
        .register(Arc::new(stats_listener))
        .await;
    
    // WebSocket 通知
    let ws_listener = WebSocketNotifyListener {
        ws_manager: Arc::clone(&ws_manager),
    };
    manager.event_bus()
        .register(Arc::new(ws_listener))
        .await;
}
```

## API 参考

### SaTokenEvent

事件数据结构：

```rust
pub struct SaTokenEvent {
    /// 事件类型
    pub event_type: SaTokenEventType,
    /// 登录ID
    pub login_id: String,
    /// Token 值
    pub token: String,
    /// 登录类型
    pub login_type: String,
    /// 事件发生时间
    pub timestamp: DateTime<Utc>,
    /// 额外数据
    pub extra: Option<serde_json::Value>,
}
```

创建事件的便捷方法：

```rust
// 创建登录事件
let event = SaTokenEvent::login("user_123", "token_abc");

// 创建登出事件
let event = SaTokenEvent::logout("user_123", "token_abc");

// 创建踢出下线事件
let event = SaTokenEvent::kick_out("user_123", "token_abc");

// 设置额外数据
let event = SaTokenEvent::login("user_123", "token_abc")
    .with_login_type("admin")
    .with_extra(json!({"ip": "192.168.1.1"}));
```

### SaTokenListener

监听器 trait 的所有方法都是可选的：

```rust
#[async_trait]
pub trait SaTokenListener: Send + Sync {
    async fn on_login(&self, login_id: &str, token: &str, login_type: &str) {}
    async fn on_logout(&self, login_id: &str, token: &str, login_type: &str) {}
    async fn on_kick_out(&self, login_id: &str, token: &str, login_type: &str) {}
    async fn on_renew_timeout(&self, login_id: &str, token: &str, login_type: &str) {}
    async fn on_replaced(&self, login_id: &str, token: &str, login_type: &str) {}
    async fn on_banned(&self, login_id: &str, login_type: &str) {}
    async fn on_event(&self, event: &SaTokenEvent) {}
}
```

### SaTokenEventBus

事件总线方法：

```rust
// 创建事件总线
let bus = SaTokenEventBus::new();

// 注册监听器
bus.register(Arc::new(MyListener)).await;

// 发布事件
let event = SaTokenEvent::login("user_123", "token_abc");
bus.publish(event).await;

// 清空所有监听器
bus.clear().await;

// 获取监听器数量
let count = bus.listener_count().await;
```

### StpUtil 事件方法

```rust
// 获取事件总线
let bus = StpUtil::event_bus();

// 注册监听器
StpUtil::register_listener(Arc::new(MyListener)).await;
```

## 注意事项

1. **异步执行**: 所有监听器方法都是异步的，会按注册顺序依次执行
2. **错误处理**: 监听器中的错误不会影响主业务流程，建议在监听器内部处理错误
3. **性能考虑**: 避免在监听器中执行耗时操作，考虑使用消息队列异步处理
4. **线程安全**: 监听器必须实现 `Send + Sync`
5. **生命周期**: 监听器通过 `Arc` 包装，可以被多个事件总线共享

## 完整示例

查看 `examples/event_listener_example.rs` 获取完整的可运行示例。

运行示例：

```bash
cargo run --example event_listener_example
```

## 参考

- [快速开始](./EVENT_LISTENER_QUICKSTART_zh-CN.md)
- [英文文档](./EVENT_LISTENER.md)
- [API 文档](https://docs.rs/sa-token-core)

