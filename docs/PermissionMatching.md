# Permission Matching Rules | 权限匹配规则

[English](#english) | [中文](#中文)

---

## English

### Overview

The `#[sa_check_permission]` macro in sa-token-rust provides a flexible permission checking system that supports both exact matching and wildcard patterns. This document explains how the permission matching algorithm works.

### Basic Concepts

#### Permission Format

Permissions follow the format: `module:action`

Examples:
- `user:list` - View user list
- `user:create` - Create user
- `user:update` - Update user
- `user:delete` - Delete user
- `order:refund` - Refund order

### Matching Rules

#### 1. Exact Match

The system first attempts an exact string match.

```rust
// User has permission
["user:delete"]

// Required permission
"user:delete"

// Result: ✅ Match (exact)
```

**Match Table:**
| User Permission | Required Permission | Result |
|----------------|---------------------|--------|
| `user:delete`  | `user:delete`       | ✅ Match |
| `user:create`  | `user:delete`       | ❌ No match |
| `order:list`   | `user:delete`       | ❌ No match |

#### 2. Wildcard Match (`*`)

If exact match fails, the system checks for wildcard patterns.

**Module Wildcard:** `module:*`
- Matches all actions in the specified module
- Format: `{prefix}:*`
- Example: `user:*` matches `user:list`, `user:create`, `user:delete`, etc.

```rust
// User has permission
["user:*"]

// Required permissions (all match)
"user:list"    // ✅
"user:create"  // ✅
"user:update"  // ✅
"user:delete"  // ✅

// No match
"order:list"   // ❌ (different module)
```

**Match Table:**
| User Permission | Required Permission | Result |
|----------------|---------------------|--------|
| `user:*`       | `user:delete`       | ✅ Wildcard match |
| `user:*`       | `user:list`         | ✅ Wildcard match |
| `user:*`       | `user:create`       | ✅ Wildcard match |
| `admin:*`      | `user:delete`       | ❌ No match (different prefix) |
| `order:*`      | `user:list`         | ❌ No match (different prefix) |

#### 3. Global Wildcard (`*`)

A single `*` grants all permissions.

```rust
// User has permission
["*"]

// All permissions match
"user:delete"   // ✅
"order:create"  // ✅
"admin:config"  // ✅
```

**Match Table:**
| User Permission | Required Permission | Result |
|----------------|---------------------|--------|
| `*`            | `user:delete`       | ✅ Global wildcard |
| `*`            | `order:list`        | ✅ Global wildcard |
| `*`            | `admin:config`      | ✅ Global wildcard |

### Algorithm Flow

```
┌─────────────────────────────────────┐
│ Start: Check Permission             │
└────────────┬────────────────────────┘
             │
             ▼
┌─────────────────────────────────────┐
│ Get user's permission list          │
└────────────┬────────────────────────┘
             │
             ▼
┌─────────────────────────────────────┐
│ Step 1: Exact Match                 │
│ Does list contain exact permission? │
└────────┬────────────────────────────┘
         │
    Yes  │  No
    ┌────┴────┐
    │         │
    ▼         ▼
┌────────┐  ┌──────────────────────────┐
│ PASS ✅│  │ Step 2: Wildcard Match   │
└────────┘  │ Check each permission:   │
            │ - Ends with ":*"?        │
            │ - Required starts with   │
            │   permission prefix?     │
            └────────┬─────────────────┘
                     │
                Yes  │  No
                ┌────┴────┐
                │         │
                ▼         ▼
            ┌────────┐  ┌────────┐
            │ PASS ✅│  │ DENY ❌│
            └────────┘  └────────┘
```

### Implementation

The matching logic is implemented in `sa-token-core/src/util.rs`:

```rust
pub async fn has_permission(login_id: impl LoginId, permission: &str) -> bool {
    let manager = Self::get_manager();
    let map = manager.user_permissions.read().await;
    
    if let Some(permissions) = map.get(&login_id.to_login_id()) {
        // 1. Exact match
        if permissions.contains(&permission.to_string()) {
            return true;
        }
        
        // 2. Wildcard match
        for perm in permissions {
            if perm.ends_with(":*") {
                let prefix = &perm[..perm.len() - 2];
                if permission.starts_with(prefix) {
                    return true;
                }
            }
        }
    }
    
    false
}
```

### Usage Examples

#### Example 1: Exact Permission

```rust
use sa_token_core::StpUtil;
use sa_token_macro::sa_check_permission;

// Initialize permissions
StpUtil::set_permissions("user_123", vec![
    "user:list".to_string(),
    "user:create".to_string(),
]).await?;

// Check exact permission
#[sa_check_permission("user:list")]
async fn list_users() -> &'static str {
    let login_id = StpUtil::get_login_id_as_string()?;
    
    // Manual check (recommended)
    if !StpUtil::has_permission(&login_id, "user:list").await {
        return "Permission denied";
    }
    
    "User list"
}
```

#### Example 2: Wildcard Permission

```rust
// Admin has all user module permissions
StpUtil::set_permissions("admin_001", vec![
    "user:*".to_string(),    // All user operations
    "order:*".to_string(),   // All order operations
]).await?;

// These all pass for admin_001
#[sa_check_permission("user:list")]
async fn list_users() { /* ... */ }

#[sa_check_permission("user:create")]
async fn create_user() { /* ... */ }

#[sa_check_permission("user:delete")]
async fn delete_user() { /* ... */ }
```

#### Example 3: Multiple Permissions

```rust
// Check multiple permissions (AND logic)
if StpUtil::has_permissions_and(&login_id, &["user:read", "user:write"]).await {
    println!("Has both read and write permissions");
}

// Check multiple permissions (OR logic)
if StpUtil::has_permissions_or(&login_id, &["admin:*", "user:*"]).await {
    println!("Has admin or user module permissions");
}
```

#### Example 4: Dynamic Permission

```rust
#[sa_check_permission("order:refund")]
async fn refund_order(order_id: u64, amount: f64) -> Result<String, StatusCode> {
    let login_id = StpUtil::get_login_id_as_string()?;
    
    // Dynamic permission based on business logic
    let required_permission = if amount > 1000.0 {
        "order:refund:advanced"  // High-value refunds need advanced permission
    } else {
        "order:refund"
    };
    
    if !StpUtil::has_permission(&login_id, required_permission).await {
        return Err(StatusCode::FORBIDDEN);
    }
    
    Ok(format!("Refunded ${}", amount))
}
```

### Best Practices

#### 1. Permission Naming Convention

Follow the `module:action` format:

```
✅ Good:
- user:list
- user:create
- user:update
- user:delete
- order:create
- order:refund
- admin:config

❌ Bad:
- userList (no separator)
- user_create (wrong separator)
- deleteUser (action first)
```

#### 2. Wildcard Usage

Use wildcards sparingly for administrative roles:

```rust
// Regular user - specific permissions
StpUtil::set_permissions("user_123", vec![
    "user:list".to_string(),
    "user:view".to_string(),
]).await?;

// Admin - module wildcard
StpUtil::set_permissions("admin_001", vec![
    "user:*".to_string(),
    "order:*".to_string(),
]).await?;

// Super admin - global wildcard (use with caution)
StpUtil::set_permissions("superadmin_001", vec![
    "*".to_string(),
]).await?;
```

#### 3. Hierarchical Permissions

Organize permissions hierarchically:

```rust
// Level 1: Module
"user:*"      // All user operations

// Level 2: Action
"user:list"
"user:create"
"user:update"
"user:delete"

// Level 3: Resource-specific (custom implementation)
"user:update:self"     // Only update own profile
"user:update:any"      // Update any user
```

### Performance Considerations

- **Exact Match First:** The system checks exact matches before wildcards, optimizing for the most common case.
- **In-Memory Storage:** Permissions are stored in memory (`HashMap`) for fast access.
- **Async Operations:** All permission checks are async to support Redis or database backends.

### Security Notes

⚠️ **Important:**
1. **Manual Checks Required:** The `#[sa_check_permission]` macro only adds metadata. You must manually call `StpUtil::has_permission()` in your function.
2. **Validate Before Use:** Always check permissions before performing sensitive operations.
3. **Limit Wildcards:** Use global wildcards (`*`) only for super admin accounts.
4. **Audit Trails:** Consider logging permission checks for security auditing.

---

## 中文

### 概述

sa-token-rust 中的 `#[sa_check_permission]` 宏提供了一个灵活的权限检查系统，支持精确匹配和通配符模式。本文档详细说明了权限匹配算法的工作原理。

### 基本概念

#### 权限格式

权限遵循格式：`模块:操作`

示例：
- `user:list` - 查看用户列表
- `user:create` - 创建用户
- `user:update` - 更新用户
- `user:delete` - 删除用户
- `order:refund` - 退款

### 匹配规则

#### 1. 精确匹配

系统首先尝试精确字符串匹配。

```rust
// 用户拥有的权限
["user:delete"]

// 需要的权限
"user:delete"

// 结果：✅ 匹配（精确）
```

**匹配表：**
| 用户权限       | 需要的权限     | 结果 |
|---------------|---------------|------|
| `user:delete` | `user:delete` | ✅ 匹配 |
| `user:create` | `user:delete` | ❌ 不匹配 |
| `order:list`  | `user:delete` | ❌ 不匹配 |

#### 2. 通配符匹配 (`*`)

如果精确匹配失败，系统会检查通配符模式。

**模块通配符：** `模块:*`
- 匹配指定模块中的所有操作
- 格式：`{前缀}:*`
- 示例：`user:*` 匹配 `user:list`、`user:create`、`user:delete` 等

```rust
// 用户拥有的权限
["user:*"]

// 需要的权限（全部匹配）
"user:list"    // ✅
"user:create"  // ✅
"user:update"  // ✅
"user:delete"  // ✅

// 不匹配
"order:list"   // ❌（不同模块）
```

**匹配表：**
| 用户权限  | 需要的权限     | 结果 |
|----------|---------------|------|
| `user:*` | `user:delete` | ✅ 通配符匹配 |
| `user:*` | `user:list`   | ✅ 通配符匹配 |
| `user:*` | `user:create` | ✅ 通配符匹配 |
| `admin:*`| `user:delete` | ❌ 不匹配（前缀不同） |
| `order:*`| `user:list`   | ❌ 不匹配（前缀不同） |

#### 3. 全局通配符 (`*`)

单个 `*` 授予所有权限。

```rust
// 用户拥有的权限
["*"]

// 所有权限都匹配
"user:delete"   // ✅
"order:create"  // ✅
"admin:config"  // ✅
```

**匹配表：**
| 用户权限 | 需要的权限     | 结果 |
|---------|---------------|------|
| `*`     | `user:delete` | ✅ 全局通配符 |
| `*`     | `order:list`  | ✅ 全局通配符 |
| `*`     | `admin:config`| ✅ 全局通配符 |

### 算法流程

```
┌─────────────────────────────────────┐
│ 开始：检查权限                       │
└────────────┬────────────────────────┘
             │
             ▼
┌─────────────────────────────────────┐
│ 获取用户的权限列表                   │
└────────────┬────────────────────────┘
             │
             ▼
┌─────────────────────────────────────┐
│ 步骤 1：精确匹配                     │
│ 列表中是否包含精确权限？             │
└────────┬────────────────────────────┘
         │
    是   │  否
    ┌────┴────┐
    │         │
    ▼         ▼
┌────────┐  ┌──────────────────────────┐
│ 通过 ✅│  │ 步骤 2：通配符匹配        │
└────────┘  │ 检查每个权限：            │
            │ - 是否以 ":*" 结尾？      │
            │ - 需要的权限是否以        │
            │   权限前缀开头？          │
            └────────┬─────────────────┘
                     │
                是   │  否
                ┌────┴────┐
                │         │
                ▼         ▼
            ┌────────┐  ┌────────┐
            │ 通过 ✅│  │ 拒绝 ❌│
            └────────┘  └────────┘
```

### 实现代码

匹配逻辑在 `sa-token-core/src/util.rs` 中实现：

```rust
pub async fn has_permission(login_id: impl LoginId, permission: &str) -> bool {
    let manager = Self::get_manager();
    let map = manager.user_permissions.read().await;
    
    if let Some(permissions) = map.get(&login_id.to_login_id()) {
        // 1. 精确匹配
        if permissions.contains(&permission.to_string()) {
            return true;
        }
        
        // 2. 通配符匹配
        for perm in permissions {
            if perm.ends_with(":*") {
                let prefix = &perm[..perm.len() - 2];
                if permission.starts_with(prefix) {
                    return true;
                }
            }
        }
    }
    
    false
}
```

### 使用示例

#### 示例 1：精确权限

```rust
use sa_token_core::StpUtil;
use sa_token_macro::sa_check_permission;

// 初始化权限
StpUtil::set_permissions("user_123", vec![
    "user:list".to_string(),
    "user:create".to_string(),
]).await?;

// 检查精确权限
#[sa_check_permission("user:list")]
async fn list_users() -> &'static str {
    let login_id = StpUtil::get_login_id_as_string()?;
    
    // 手动检查（推荐）
    if !StpUtil::has_permission(&login_id, "user:list").await {
        return "权限不足";
    }
    
    "用户列表"
}
```

#### 示例 2：通配符权限

```rust
// 管理员拥有所有用户模块权限
StpUtil::set_permissions("admin_001", vec![
    "user:*".to_string(),    // 所有用户操作
    "order:*".to_string(),   // 所有订单操作
]).await?;

// admin_001 可以访问以下所有接口
#[sa_check_permission("user:list")]
async fn list_users() { /* ... */ }

#[sa_check_permission("user:create")]
async fn create_user() { /* ... */ }

#[sa_check_permission("user:delete")]
async fn delete_user() { /* ... */ }
```

#### 示例 3：多个权限

```rust
// 检查多个权限（AND 逻辑）
if StpUtil::has_permissions_and(&login_id, &["user:read", "user:write"]).await {
    println!("同时拥有读写权限");
}

// 检查多个权限（OR 逻辑）
if StpUtil::has_permissions_or(&login_id, &["admin:*", "user:*"]).await {
    println!("拥有管理员或用户模块权限");
}
```

#### 示例 4：动态权限

```rust
#[sa_check_permission("order:refund")]
async fn refund_order(order_id: u64, amount: f64) -> Result<String, StatusCode> {
    let login_id = StpUtil::get_login_id_as_string()?;
    
    // 根据业务逻辑动态决定权限
    let required_permission = if amount > 1000.0 {
        "order:refund:advanced"  // 大额退款需要高级权限
    } else {
        "order:refund"
    };
    
    if !StpUtil::has_permission(&login_id, required_permission).await {
        return Err(StatusCode::FORBIDDEN);
    }
    
    Ok(format!("已退款 ¥{}", amount))
}
```

### 最佳实践

#### 1. 权限命名规范

遵循 `模块:操作` 格式：

```
✅ 正确：
- user:list
- user:create
- user:update
- user:delete
- order:create
- order:refund
- admin:config

❌ 错误：
- userList（没有分隔符）
- user_create（错误的分隔符）
- deleteUser（操作在前）
```

#### 2. 通配符使用

谨慎使用通配符，仅用于管理员角色：

```rust
// 普通用户 - 具体权限
StpUtil::set_permissions("user_123", vec![
    "user:list".to_string(),
    "user:view".to_string(),
]).await?;

// 管理员 - 模块通配符
StpUtil::set_permissions("admin_001", vec![
    "user:*".to_string(),
    "order:*".to_string(),
]).await?;

// 超级管理员 - 全局通配符（谨慎使用）
StpUtil::set_permissions("superadmin_001", vec![
    "*".to_string(),
]).await?;
```

#### 3. 分层权限

按层次组织权限：

```rust
// 层级 1：模块
"user:*"      // 所有用户操作

// 层级 2：操作
"user:list"
"user:create"
"user:update"
"user:delete"

// 层级 3：资源特定（自定义实现）
"user:update:self"     // 只能更新自己的资料
"user:update:any"      // 可以更新任何用户
```

### 性能考虑

- **精确匹配优先：** 系统先检查精确匹配，优化最常见的情况
- **内存存储：** 权限存储在内存（`HashMap`）中以实现快速访问
- **异步操作：** 所有权限检查都是异步的，支持 Redis 或数据库后端

### 安全注意事项

⚠️ **重要：**
1. **需要手动检查：** `#[sa_check_permission]` 宏只添加元数据，必须在函数中手动调用 `StpUtil::has_permission()`
2. **使用前验证：** 在执行敏感操作前始终检查权限
3. **限制通配符：** 仅对超级管理员账户使用全局通配符 (`*`)
4. **审计跟踪：** 考虑记录权限检查日志以进行安全审计

---

## Related Documentation

- [StpUtil API](./StpUtil.md) - Complete StpUtil API reference
- [README](../README.md) - Project overview and quick start

## 相关文档

- [StpUtil API](./StpUtil_zh-CN.md) - 完整的 StpUtil API 参考
- [README](../README_zh-CN.md) - 项目概述和快速开始

