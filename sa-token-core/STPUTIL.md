# StpUtil 使用指南

## 简介

`StpUtil` 是 sa-token-rust 的便捷工具类，类似于 Java 版 sa-token 的 `StpUtil`。它提供了一系列静态方法，用于简化认证和授权操作。

## Java 版本对比

### Java 版 StpUtil

```java
// 登录
StpUtil.login(userId);

// 检查登录
StpUtil.checkLogin();

// 获取登录ID
String loginId = StpUtil.getLoginId();

// 登出
StpUtil.logout();

// 检查权限
StpUtil.checkPermission("user:delete");

// 检查角色
StpUtil.checkRole("admin");
```

### Rust 版 StpUtil

```rust
use sa_token_core::StpUtil;

// 登录
let token = StpUtil::login(&manager, user_id).await?;

// 检查登录
StpUtil::check_login(&manager, &token).await?;

// 获取登录ID
let login_id = StpUtil::get_login_id(&manager, &token).await?;

// 登出
StpUtil::logout(&manager, &token).await?;

// 注意：权限和角色检查建议使用过程宏
#[sa_check_permission("user:delete")]
async fn delete_user() { }

#[sa_check_role("admin")]
async fn admin_panel() { }
```

## 核心API

### 1. 登录相关

#### 1.1 登录
```rust
let token = StpUtil::login(&manager, "user_123").await?;
println!("登录成功，Token: {}", token.as_str());
```

#### 1.2 登出
```rust
StpUtil::logout(&manager, &token).await?;
```

#### 1.3 踢人下线
```rust
// 根据登录ID踢人下线
StpUtil::kick_out(&manager, "user_123").await?;
```

#### 1.4 强制登出
```rust
// 登出指定用户的所有token
StpUtil::logout_by_login_id(&manager, "user_123").await?;
```

### 2. Token 验证

#### 2.1 检查是否已登录
```rust
if StpUtil::is_login(&manager, &token).await {
    println!("已登录");
} else {
    println!("未登录");
}
```

#### 2.2 检查登录（未登录抛异常）
```rust
// 如果未登录，会返回错误
StpUtil::check_login(&manager, &token).await?;
```

#### 2.3 获取登录ID
```rust
// 获取登录ID
let login_id = StpUtil::get_login_id(&manager, &token).await?;

// 获取登录ID，未登录返回默认值
let login_id = StpUtil::get_login_id_or_default(&manager, &token, "guest").await;
```

#### 2.4 获取Token信息
```rust
let token_info = StpUtil::get_token_info(&manager, &token).await?;
println!("登录ID: {}", token_info.login_id);
println!("创建时间: {}", token_info.create_time);
```

### 3. Session 操作

#### 3.1 获取Session
```rust
let session = StpUtil::get_session(&manager, "user_123").await?;
```

#### 3.2 设置Session值
```rust
// 设置字符串
StpUtil::set_session_value(&manager, "user_123", "username", "张三").await?;

// 设置数字
StpUtil::set_session_value(&manager, "user_123", "age", 25).await?;

// 设置复杂对象
#[derive(Serialize, Deserialize)]
struct UserInfo {
    name: String,
    email: String,
}

let user_info = UserInfo {
    name: "张三".to_string(),
    email: "zhangsan@example.com".to_string(),
};
StpUtil::set_session_value(&manager, "user_123", "user_info", user_info).await?;
```

#### 3.3 获取Session值
```rust
// 获取字符串
let username: Option<String> = StpUtil::get_session_value(&manager, "user_123", "username").await?;

// 获取数字
let age: Option<i32> = StpUtil::get_session_value(&manager, "user_123", "age").await?;

// 获取复杂对象
let user_info: Option<UserInfo> = StpUtil::get_session_value(&manager, "user_123", "user_info").await?;
```

#### 3.4 删除Session
```rust
StpUtil::delete_session(&manager, "user_123").await?;
```

### 4. Token 管理

#### 4.1 获取Token剩余有效时间
```rust
if let Some(timeout) = StpUtil::get_token_timeout(&manager, &token).await? {
    println!("Token剩余 {} 秒", timeout);
} else {
    println!("Token永久有效");
}
```

#### 4.2 续期Token
```rust
// 续期到1小时
StpUtil::renew_timeout(&manager, &token, 3600).await?;
```

#### 4.3 创建Token
```rust
// 创建一个token值（不登录）
let token = StpUtil::create_token("custom-token-value");
```

#### 4.4 验证Token格式
```rust
if StpUtil::is_valid_token_format("1234567890abcdef") {
    println!("Token格式有效");
}
```

### 5. 批量操作

#### 5.1 批量踢人下线
```rust
let user_ids = vec!["user_1", "user_2", "user_3"];
let results = StpUtil::kick_out_batch(&manager, &user_ids).await?;

for (i, result) in results.iter().enumerate() {
    match result {
        Ok(_) => println!("用户 {} 已踢下线", user_ids[i]),
        Err(e) => println!("用户 {} 踢下线失败: {}", user_ids[i], e),
    }
}
```

## 完整使用示例

```rust
use std::sync::Arc;
use sa_token_core::{StpUtil, SaTokenConfig, SaTokenManager};
use sa_token_storage_memory::MemoryStorage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 初始化
    let storage = Arc::new(MemoryStorage::new());
    let config = SaTokenConfig::builder()
        .token_name("satoken")
        .timeout(7200)  // 2小时
        .build();
    let manager = SaTokenManager::new(storage, config);
    
    // 2. 登录
    let user_id = "user_123";
    let token = StpUtil::login(&manager, user_id).await?;
    println!("✅ 登录成功，Token: {}", token.as_str());
    
    // 3. 检查登录状态
    if StpUtil::is_login(&manager, &token).await {
        println!("✅ 用户已登录");
    }
    
    // 4. 获取登录ID
    let login_id = StpUtil::get_login_id(&manager, &token).await?;
    println!("✅ 当前登录ID: {}", login_id);
    
    // 5. 设置Session
    StpUtil::set_session_value(&manager, &login_id, "username", "张三").await?;
    StpUtil::set_session_value(&manager, &login_id, "age", 25).await?;
    
    // 6. 获取Session
    let username: Option<String> = StpUtil::get_session_value(&manager, &login_id, "username").await?;
    println!("✅ 用户名: {:?}", username);
    
    // 7. 获取Token剩余时间
    if let Some(timeout) = StpUtil::get_token_timeout(&manager, &token).await? {
        println!("✅ Token剩余 {} 秒", timeout);
    }
    
    // 8. 续期Token
    StpUtil::renew_timeout(&manager, &token, 3600).await?;
    println!("✅ Token已续期");
    
    // 9. 登出
    StpUtil::logout(&manager, &token).await?;
    println!("✅ 已登出");
    
    Ok(())
}
```

## 在 Axum 中使用

```rust
use axum::{extract::State, Json};
use sa_token_core::StpUtil;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppState {
    sa_token_manager: Arc<SaTokenManager>,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

// 登录接口
async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, String> {
    // 1. 验证用户名密码（这里简化处理）
    if req.password != "123456" {
        return Err("密码错误".to_string());
    }
    
    // 2. 登录
    let user_id = req.username;
    let token = StpUtil::login(&state.sa_token_manager, &user_id)
        .await
        .map_err(|e| e.to_string())?;
    
    // 3. 设置Session
    StpUtil::set_session_value(&state.sa_token_manager, &user_id, "username", &user_id)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(Json(LoginResponse {
        token: token.to_string(),
    }))
}

// 需要登录的接口
async fn user_info(
    State(state): State<AppState>,
    token: TokenExtractor,
) -> Result<Json<UserInfo>, String> {
    // 1. 检查登录
    StpUtil::check_login(&state.sa_token_manager, &token.0)
        .await
        .map_err(|_| "未登录".to_string())?;
    
    // 2. 获取登录ID
    let login_id = StpUtil::get_login_id(&state.sa_token_manager, &token.0)
        .await
        .map_err(|e| e.to_string())?;
    
    // 3. 获取Session
    let username: Option<String> = StpUtil::get_session_value(&state.sa_token_manager, &login_id, "username")
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(Json(UserInfo {
        id: login_id,
        username: username.unwrap_or_default(),
    }))
}

// 登出接口
async fn logout(
    State(state): State<AppState>,
    token: TokenExtractor,
) -> Result<String, String> {
    StpUtil::logout(&state.sa_token_manager, &token.0)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok("登出成功".to_string())
}
```

## 最佳实践

### 1. Manager 管理

建议在应用状态中保存 `SaTokenManager` 实例：

```rust
#[derive(Clone)]
pub struct AppState {
    pub sa_token: Arc<SaTokenManager>,
}
```

### 2. 错误处理

```rust
match StpUtil::login(&manager, user_id).await {
    Ok(token) => println!("登录成功: {}", token.as_str()),
    Err(SaTokenError::StorageError(e)) => println!("存储错误: {}", e),
    Err(e) => println!("其他错误: {}", e),
}
```

### 3. 结合过程宏

对于需要权限验证的接口，建议使用过程宏而不是手动调用：

```rust
// ✅ 推荐：使用过程宏
#[sa_check_login]
async fn user_profile() -> String {
    "用户资料".to_string()
}

// ❌ 不推荐：手动检查
async fn user_profile(token: TokenValue) -> Result<String, String> {
    StpUtil::check_login(&manager, &token).await
        .map_err(|e| e.to_string())?;
    Ok("用户资料".to_string())
}
```

## 与 Java 版本的差异

| 功能 | Java 版本 | Rust 版本 |
|------|-----------|-----------|
| 登录 | `StpUtil.login(id)` | `StpUtil::login(&manager, id).await?` |
| 检查登录 | `StpUtil.checkLogin()` | `StpUtil::check_login(&manager, &token).await?` |
| 获取登录ID | `StpUtil.getLoginId()` | `StpUtil::get_login_id(&manager, &token).await?` |
| Session | `StpUtil.getSession()` | `StpUtil::get_session(&manager, login_id).await?` |
| 权限检查 | `StpUtil.checkPermission(...)` | 使用 `#[sa_check_permission(...)]` 宏 |
| 角色检查 | `StpUtil.checkRole(...)` | 使用 `#[sa_check_role(...)]` 宏 |

**主要差异**：
1. Rust 版本需要显式传递 `manager` 实例
2. 所有异步操作需要 `.await`
3. 错误处理使用 `Result` 类型
4. 权限检查推荐使用过程宏而不是工具方法

## 常见问题

### Q: 为什么需要传递 manager 实例？

**A**: 因为 Rust 没有全局静态状态的概念（不推荐使用）。建议将 manager 保存在应用状态中，然后通过参数传递。

### Q: 可以像 Java 一样不传 manager 吗？

**A**: 可以使用 `once_cell` 或 `lazy_static` 创建全局单例，但不推荐。Rust 的设计哲学是显式依赖管理。

### Q: 为什么权限检查推荐使用宏？

**A**: 过程宏在编译时生成代码，性能更好且代码更简洁。StpUtil 主要用于 token 和 session 管理。

## 参考链接

- [完整示例](../examples/axum-full-example/)
- [过程宏文档](../sa-token-macro/README.md)
- [Java版 sa-token 文档](https://sa-token.cc/)

