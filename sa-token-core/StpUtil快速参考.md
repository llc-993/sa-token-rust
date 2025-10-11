# StpUtil 快速参考

## ✨ 简介

`StpUtil` 是 sa-token-rust 版本的核心工具类，类似于 Java 版本的 `StpUtil`，提供便捷的认证和授权操作。

---

## 📋 API 速查表

### 登录相关

| 方法 | 说明 | 示例 |
|------|------|------|
| `login(manager, id)` | 登录 | `let token = StpUtil::login(&manager, "user_123").await?;` |
| `logout(manager, token)` | 登出 | `StpUtil::logout(&manager, &token).await?;` |
| `kick_out(manager, login_id)` | 踢人下线 | `StpUtil::kick_out(&manager, "user_123").await?;` |
| `logout_by_login_id(manager, login_id)` | 强制登出 | `StpUtil::logout_by_login_id(&manager, "user_123").await?;` |

### Token 验证

| 方法 | 说明 | 示例 |
|------|------|------|
| `is_login(manager, token)` | 检查是否已登录 | `if StpUtil::is_login(&manager, &token).await { ... }` |
| `check_login(manager, token)` | 检查登录（未登录抛异常） | `StpUtil::check_login(&manager, &token).await?;` |
| `get_login_id(manager, token)` | 获取登录ID | `let id = StpUtil::get_login_id(&manager, &token).await?;` |
| `get_login_id_or_default(manager, token, default)` | 获取登录ID（带默认值） | `let id = StpUtil::get_login_id_or_default(&manager, &token, "guest").await;` |
| `get_token_info(manager, token)` | 获取Token信息 | `let info = StpUtil::get_token_info(&manager, &token).await?;` |

### Session 操作

| 方法 | 说明 | 示例 |
|------|------|------|
| `get_session(manager, login_id)` | 获取Session | `let session = StpUtil::get_session(&manager, "user_123").await?;` |
| `save_session(manager, session)` | 保存Session | `StpUtil::save_session(&manager, &session).await?;` |
| `delete_session(manager, login_id)` | 删除Session | `StpUtil::delete_session(&manager, "user_123").await?;` |
| `set_session_value(manager, login_id, key, value)` | 设置Session值 | `StpUtil::set_session_value(&manager, "user_123", "age", 25).await?;` |
| `get_session_value(manager, login_id, key)` | 获取Session值 | `let age: Option<i32> = StpUtil::get_session_value(&manager, "user_123", "age").await?;` |

### Token 管理

| 方法 | 说明 | 示例 |
|------|------|------|
| `get_token_timeout(manager, token)` | 获取Token剩余时间 | `let timeout = StpUtil::get_token_timeout(&manager, &token).await?;` |
| `renew_timeout(manager, token, seconds)` | 续期Token | `StpUtil::renew_timeout(&manager, &token, 3600).await?;` |
| `create_token(value)` | 创建Token值 | `let token = StpUtil::create_token("custom-value");` |
| `is_valid_token_format(token)` | 验证Token格式 | `if StpUtil::is_valid_token_format("abc123") { ... }` |

### 批量操作

| 方法 | 说明 | 示例 |
|------|------|------|
| `kick_out_batch(manager, login_ids)` | 批量踢人下线 | `let results = StpUtil::kick_out_batch(&manager, &["u1", "u2"]).await?;` |

---

## 🚀 快速开始

### 1. 初始化

```rust
use std::sync::Arc;
use sa_token_core::{StpUtil, SaTokenConfig, SaTokenManager};
use sa_token_storage_memory::MemoryStorage;

let storage = Arc::new(MemoryStorage::new());
let config = SaTokenConfig::builder()
    .token_name("satoken")
    .timeout(7200)
    .build();
let manager = SaTokenManager::new(storage, config);
```

### 2. 常用操作

```rust
// 登录
let token = StpUtil::login(&manager, "user_123").await?;

// 检查登录
if StpUtil::is_login(&manager, &token).await {
    // 已登录
}

// 获取登录ID
let login_id = StpUtil::get_login_id(&manager, &token).await?;

// 设置Session
StpUtil::set_session_value(&manager, &login_id, "username", "张三").await?;

// 获取Session
let username: Option<String> = StpUtil::get_session_value(&manager, &login_id, "username").await?;

// 登出
StpUtil::logout(&manager, &token).await?;
```

---

## 📊 与 Java 版本对比

| 操作 | Java 版本 | Rust 版本 |
|------|-----------|-----------|
| **登录** | `StpUtil.login(10001)` | `StpUtil::login(&manager, "10001").await?` |
| **检查登录** | `StpUtil.checkLogin()` | `StpUtil::check_login(&manager, &token).await?` |
| **获取ID** | `Object id = StpUtil.getLoginId()` | `let id = StpUtil::get_login_id(&manager, &token).await?` |
| **登出** | `StpUtil.logout()` | `StpUtil::logout(&manager, &token).await?` |
| **Session设置** | `StpUtil.getSession().set("name", "张三")` | `StpUtil::set_session_value(&manager, id, "name", "张三").await?` |
| **Session获取** | `String name = (String)StpUtil.getSession().get("name")` | `let name: Option<String> = StpUtil::get_session_value(&manager, id, "name").await?` |

### 主要区别

1. ✅ **需要传递 manager** - Rust 版本需要显式传递 `SaTokenManager` 实例
2. ✅ **异步操作** - 所有方法都是异步的，需要 `.await`
3. ✅ **错误处理** - 使用 `Result` 类型，需要 `?` 处理错误
4. ✅ **类型安全** - Session 值获取时需要指定类型

---

## 💡 在 Web 框架中使用

### Axum 示例

```rust
use axum::{extract::State, Json};
use serde::Deserialize;

#[derive(Clone)]
struct AppState {
    sa_token: Arc<SaTokenManager>,
}

#[derive(Deserialize)]
struct LoginReq {
    username: String,
    password: String,
}

// 登录接口
async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginReq>,
) -> Result<String, String> {
    // 验证密码（简化）
    if req.password != "123456" {
        return Err("密码错误".to_string());
    }
    
    // 登录
    let token = StpUtil::login(&state.sa_token, &req.username)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(token.to_string())
}

// 用户信息接口（需要登录）
async fn user_info(
    State(state): State<AppState>,
    token: String,
) -> Result<String, String> {
    let token = TokenValue::new(token);
    
    // 检查登录
    StpUtil::check_login(&state.sa_token, &token)
        .await
        .map_err(|_| "未登录".to_string())?;
    
    // 获取登录ID
    let login_id = StpUtil::get_login_id(&state.sa_token, &token)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(format!("用户ID: {}", login_id))
}
```

---

## 🎯 最佳实践

### 1. 在 AppState 中保存 Manager

```rust
#[derive(Clone)]
pub struct AppState {
    pub sa_token: Arc<SaTokenManager>,
}
```

### 2. 错误统一处理

```rust
impl From<SaTokenError> for ApiError {
    fn from(err: SaTokenError) -> Self {
        match err {
            SaTokenError::NotLogin => ApiError::Unauthorized("未登录".to_string()),
            SaTokenError::TokenExpired => ApiError::Unauthorized("Token已过期".to_string()),
            _ => ApiError::InternalError(err.to_string()),
        }
    }
}
```

### 3. 配合过程宏使用

```rust
// ✅ 推荐：权限检查使用宏
#[sa_check_permission("user:delete")]
async fn delete_user() { }

// ✅ StpUtil 用于登录和Session
async fn login(req: LoginReq) -> Result<String, String> {
    let token = StpUtil::login(&manager, &req.username).await?;
    StpUtil::set_session_value(&manager, &req.username, "nickname", &req.nickname).await?;
    Ok(token.to_string())
}
```

---

## 📚 相关文档

- [详细文档](./STPUTIL.md)
- [完整示例](../examples/axum-full-example/)
- [过程宏文档](../sa-token-macro/README.md)
- [Java 版 sa-token](https://sa-token.cc/)

---

## 🤝 对比总结

| 特性 | Java 版 StpUtil | Rust 版 StpUtil |
|------|----------------|-----------------|
| **静态方法** | ✅ | ❌ (需要传manager) |
| **类型安全** | ⚠️ | ✅ |
| **异步支持** | ❌ | ✅ |
| **编译时检查** | ❌ | ✅ |
| **Session类型** | Object | 泛型 `T` |
| **错误处理** | 异常 | `Result` |

**Rust 版本的优势**：
- ✅ 更强的类型安全
- ✅ 编译时错误检查
- ✅ 原生异步支持
- ✅ 零成本抽象

**使用建议**：
- 对于简单的 token 和 session 操作，使用 `StpUtil`
- 对于权限和角色验证，使用过程宏 `#[sa_check_permission]`、`#[sa_check_role]`
- 结合两者，发挥各自优势

