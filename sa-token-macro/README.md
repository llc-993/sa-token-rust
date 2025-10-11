# sa-token-macro

sa-token-rust 的过程宏库，提供类似 Java 注解的认证授权标记功能。

## 功能特性

- ✅ `#[sa_check_login]` - 检查用户登录状态
- ✅ `#[sa_check_permission("perm")]` - 检查单个权限
- ✅ `#[sa_check_role("role")]` - 检查单个角色
- ✅ `#[sa_check_permissions_and("p1", "p2")]` - 检查多个权限（AND逻辑）
- ✅ `#[sa_check_permissions_or("p1", "p2")]` - 检查多个权限（OR逻辑）
- ✅ `#[sa_check_roles_and("r1", "r2")]` - 检查多个角色（AND逻辑）
- ✅ `#[sa_check_roles_or("r1", "r2")]` - 检查多个角色（OR逻辑）
- ✅ `#[sa_ignore]` - 忽略所有认证检查

## 安装

```toml
[dependencies]
sa-token-macro = "0.1"
```

## 使用示例

### 基础认证

```rust
use sa_token_macro::*;

// 检查登录
#[sa_check_login]
async fn user_info() -> String {
    "User info".to_string()
}

// 检查权限
#[sa_check_permission("user:delete")]
async fn delete_user(id: u64) -> String {
    format!("Delete user {}", id)
}

// 检查角色
#[sa_check_role("admin")]
async fn admin_panel() -> String {
    "Admin panel".to_string()
}
```

### 多权限/角色检查

```rust
// AND 逻辑：必须同时拥有所有权限
#[sa_check_permissions_and("user:read", "user:write")]
async fn manage_user() -> String {
    "Manage user".to_string()
}

// OR 逻辑：拥有任一权限即可
#[sa_check_permissions_or("admin:all", "super:all")]
async fn admin_action() -> String {
    "Admin action".to_string()
}

// AND 逻辑：必须同时拥有所有角色
#[sa_check_roles_and("admin", "super")]
async fn super_admin() -> String {
    "Super admin".to_string()
}

// OR 逻辑：拥有任一角色即可
#[sa_check_roles_or("admin", "moderator")]
async fn moderate() -> String {
    "Moderate".to_string()
}
```

### 忽略认证

```rust
// 单个函数忽略认证
#[sa_ignore]
async fn public_api() -> String {
    "Public API".to_string()
}

// 整个结构体忽略认证
#[sa_ignore]
struct PublicController;

impl PublicController {
    async fn home() -> String {
        "Home".to_string()
    }
}

// impl 块忽略认证
struct ApiController;

#[sa_ignore]
impl ApiController {
    async fn version() -> String {
        "v1.0.0".to_string()
    }
}
```

### 混合使用

```rust
struct UserController;

impl UserController {
    // 公开接口
    #[sa_ignore]
    async fn register() -> String {
        "Register".to_string()
    }
    
    // 需要登录
    #[sa_check_login]
    async fn profile() -> String {
        "Profile".to_string()
    }
    
    // 需要权限
    #[sa_check_permission("user:update")]
    async fn update() -> String {
        "Update".to_string()
    }
    
    // 需要角色
    #[sa_check_role("admin")]
    async fn delete() -> String {
        "Delete".to_string()
    }
}
```

## 工作原理

这些宏主要是在编译时为函数、结构体或 impl 块添加元数据标记。实际的认证逻辑由框架中间件在运行时执行。

中间件会检查这些标记来决定：
- 是否需要验证登录状态
- 需要检查哪些权限或角色
- 是否应该跳过认证（`#[sa_ignore]`）

## 优先级

`#[sa_ignore]` 的优先级最高。如果同时使用了 `#[sa_ignore]` 和其他认证宏，将跳过所有认证检查。

```rust
// sa_ignore 会覆盖 sa_check_login
#[sa_ignore]
#[sa_check_login]
async fn example() -> String {
    // 实际上不会进行登录检查
    "Example".to_string()
}
```

## 注意事项

1. 这些宏需要配合框架插件（如 `sa-token-plugin-axum`）使用
2. 宏本身不执行任何验证逻辑，只添加元数据标记
3. 实际的认证逻辑在框架中间件中实现
4. 建议在框架文档中查看如何正确配置中间件

## 示例

查看 `examples/basic_usage.rs` 获取完整示例。

运行示例：

```bash
cargo run --example basic_usage
```

## 许可证

MIT OR Apache-2.0

