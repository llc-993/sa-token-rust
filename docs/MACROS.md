# sa-token-rust 过程宏使用指南

## 概述

sa-token-rust 提供了一套完整的过程宏，用于声明式地标记认证和授权需求。这些宏类似于 Java sa-token 中的注解。

## 可用的宏

### 1. 登录检查

#### `#[sa_check_login]`

检查用户是否已登录。

```rust
#[sa_check_login]
async fn user_profile() -> impl Responder {
    "User profile"
}
```

### 2. 权限检查

#### `#[sa_check_permission("permission")]`

检查用户是否拥有指定权限。

```rust
#[sa_check_permission("user:delete")]
async fn delete_user(id: u64) -> impl Responder {
    format!("Delete user {}", id)
}
```

#### `#[sa_check_permissions_and("p1", "p2", ...)]`

检查用户是否拥有所有指定权限（AND 逻辑）。

```rust
#[sa_check_permissions_and("user:read", "user:write")]
async fn manage_user() -> impl Responder {
    "Manage user"
}
```

#### `#[sa_check_permissions_or("p1", "p2", ...)]`

检查用户是否拥有任一指定权限（OR 逻辑）。

```rust
#[sa_check_permissions_or("admin:all", "super:all")]
async fn admin_action() -> impl Responder {
    "Admin action"
}
```

### 3. 角色检查

#### `#[sa_check_role("role")]`

检查用户是否拥有指定角色。

```rust
#[sa_check_role("admin")]
async fn admin_panel() -> impl Responder {
    "Admin panel"
}
```

#### `#[sa_check_roles_and("r1", "r2", ...)]`

检查用户是否拥有所有指定角色（AND 逻辑）。

```rust
#[sa_check_roles_and("admin", "super")]
async fn super_admin_panel() -> impl Responder {
    "Super admin panel"
}
```

#### `#[sa_check_roles_or("r1", "r2", ...)]`

检查用户是否拥有任一指定角色（OR 逻辑）。

```rust
#[sa_check_roles_or("admin", "moderator")]
async fn moderate_content() -> impl Responder {
    "Moderate content"
}
```

### 4. 忽略认证 ⭐

#### `#[sa_ignore]`

**最强大的宏！** 忽略所有认证检查，包括登录验证、权限验证、角色验证和路由拦截器。

这个宏可以应用于：
- **函数**：单个路由处理函数忽略认证
- **结构体**：整个控制器的所有方法都忽略认证
- **impl 块**：impl 块中的所有方法都忽略认证

#### 应用于函数

```rust
#[sa_ignore]
async fn public_api() -> impl Responder {
    "Public API - no authentication required"
}

#[sa_ignore]
async fn health_check() -> impl Responder {
    "OK"
}
```

#### 应用于结构体

```rust
#[sa_ignore]
struct PublicController;

impl PublicController {
    // 此控制器的所有方法都不需要认证
    async fn home() -> impl Responder {
        "Home page"
    }
    
    async fn about() -> impl Responder {
        "About page"
    }
}
```

#### 应用于 impl 块

```rust
struct ApiController;

#[sa_ignore]
impl ApiController {
    // 这个 impl 块中的所有方法都忽略认证
    async fn version() -> impl Responder {
        "v1.0.0"
    }
    
    async fn status() -> impl Responder {
        "running"
    }
}
```

#### 使用场景

- ✅ 公开 API 接口
- ✅ 健康检查端点（`/health`, `/ping`）
- ✅ 版本信息接口（`/version`, `/info`）
- ✅ 静态资源访问
- ✅ 登录/注册接口
- ✅ 公开文档页面
- ✅ 第三方回调接口（webhook）

#### 优先级

`#[sa_ignore]` 的优先级**最高**。即使同时使用了其他认证宏，也会被 `#[sa_ignore]` 覆盖：

```rust
// 警告：sa_ignore 会覆盖 sa_check_login
#[sa_ignore]
#[sa_check_login]  // 这个会被忽略
async fn example() -> impl Responder {
    // 实际上不会进行登录检查
    "Example"
}
```

## 混合使用示例

```rust
struct UserController;

impl UserController {
    // 公开接口 - 忽略认证
    #[sa_ignore]
    async fn register(data: RegisterData) -> impl Responder {
        "Register success"
    }
    
    // 需要登录
    #[sa_check_login]
    async fn profile() -> impl Responder {
        "User profile"
    }
    
    // 需要特定权限
    #[sa_check_permission("user:update_profile")]
    async fn update_profile(data: UpdateData) -> impl Responder {
        "Profile updated"
    }
    
    // 需要管理员角色
    #[sa_check_role("admin")]
    async fn delete_user(id: u64) -> impl Responder {
        format!("User {} deleted", id)
    }
    
    // 需要多个权限（AND）
    #[sa_check_permissions_and("user:read", "user:write")]
    async fn manage_all_users() -> impl Responder {
        "Manage all users"
    }
    
    // 需要任一角色（OR）
    #[sa_check_roles_or("admin", "moderator")]
    async fn moderate() -> impl Responder {
        "Moderate content"
    }
}
```

## 工作原理

### 元数据标记

所有宏都通过 `cfg_attr` 添加编译时元数据标记：

```rust
// sa_check_login 生成
#[cfg_attr(feature = "sa-token-metadata", sa_token_check = "login")]

// sa_check_permission 生成
#[cfg_attr(feature = "sa-token-metadata", sa_token_check = "permission")]
#[cfg_attr(feature = "sa-token-metadata", sa_token_permission = "user:delete")]

// sa_check_role 生成
#[cfg_attr(feature = "sa-token-metadata", sa_token_check = "role")]
#[cfg_attr(feature = "sa-token-metadata", sa_token_role = "admin")]

// sa_ignore 生成
#[cfg_attr(feature = "sa-token-metadata", sa_token_ignore = "true")]
```

### 中间件集成

框架中间件（Axum、Actix-web 等）会：
1. 读取这些元数据标记
2. 根据标记决定是否需要认证
3. 执行相应的认证逻辑
4. 如果发现 `sa_token_ignore = "true"`，则跳过所有检查

## 与 Java sa-token 的对比

| Java 注解 | Rust 宏 | 说明 |
|-----------|---------|------|
| `@SaCheckLogin` | `#[sa_check_login]` | 检查登录 |
| `@SaCheckPermission("user:delete")` | `#[sa_check_permission("user:delete")]` | 检查权限 |
| `@SaCheckRole("admin")` | `#[sa_check_role("admin")]` | 检查角色 |
| `@SaCheckPermission(value={"p1","p2"}, mode=AND)` | `#[sa_check_permissions_and("p1", "p2")]` | 多权限AND |
| `@SaCheckPermission(value={"p1","p2"}, mode=OR)` | `#[sa_check_permissions_or("p1", "p2")]` | 多权限OR |
| `@SaIgnore` | `#[sa_ignore]` | 忽略认证 ⭐ |

## 注意事项

1. **编译时标记**：这些宏主要是添加编译时标记，不执行实际验证
2. **中间件配合**：需要配合框架中间件使用才能生效
3. **异步支持**：所有宏都支持 async 函数
4. **泛型支持**：支持带泛型的函数
5. **可见性**：保持原函数的可见性修饰符

## 完整示例

查看 `sa-token-macro/examples/basic_usage.rs` 获取完整示例。

运行示例：
```bash
cd sa-token-rust
cargo run --example basic_usage
```

## 下一步

学习如何在具体框架中使用这些宏：
- [Axum 集成指南](./AXUM_INTEGRATION.md)
- [Actix-web 集成指南](./ACTIX_INTEGRATION.md)

