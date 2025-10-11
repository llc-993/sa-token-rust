# StpUtil API Reference

[中文文档](StpUtil_zh-CN.md) | English

`StpUtil` is a utility class that provides a simplified, static API for common authentication and authorization operations. It wraps `SaTokenManager` functionality in an easy-to-use interface.

## Table of Contents

- [Initialization](#initialization)
- [Login Operations](#login-operations)
- [Logout Operations](#logout-operations)
- [Token Validation](#token-validation)
- [Session Management](#session-management)
- [Permission Management](#permission-management)
- [Role Management](#role-management)
- [Advanced Usage](#advanced-usage)

## Initialization

`StpUtil` is automatically initialized when you create `SaTokenState` using any web framework plugin:

```rust
use sa_token_core::StpUtil;
use sa_token_plugin_axum::SaTokenState;  // or any other framework plugin
use sa_token_storage_memory::MemoryStorage;
use std::sync::Arc;

// StpUtil is automatically initialized when building state
let state = SaTokenState::builder()
    .storage(Arc::new(MemoryStorage::new()))
    .token_name("Authorization")
    .timeout(86400)
    .build();

// StpUtil is ready to use!
StpUtil::login("user_id").await?;
```

**Note**: The initialization happens automatically in `SaTokenState::builder().build()`, so you don't need to call any initialization method manually. This works for all supported web frameworks (Axum, Actix-web, Poem, Rocket, Warp).

## Login Operations

### Basic Login

```rust
use sa_token_core::StpUtil;

// Login with string ID
let token = StpUtil::login("user_10001").await?;
println!("Generated token: {}", token.value());

// Login with numeric ID
let token = StpUtil::login(10001).await?;  // i32, i64, u32, u64 supported
```

### Login with Device Identification

```rust
// Login with device info (for multi-device management)
let token = StpUtil::login_by_device("user_10001", "mobile_ios").await?;
```

## Logout Operations

### Logout Current User

```rust
// Logout by login_id
StpUtil::logout("user_10001").await?;

// Logout by token
StpUtil::logout_by_token(&token).await?;
```

### Logout from Specific Device

```rust
// Logout from a specific device
StpUtil::logout_by_device("user_10001", "mobile_ios").await?;
```

### Kick User Offline

```rust
// Force logout (kick offline)
StpUtil::kick_out("user_10001").await?;
```

## Token Validation

### Check if Logged In

```rust
// Check if user is logged in
let is_logged_in = StpUtil::is_login("user_10001").await;

if is_logged_in {
    println!("User is logged in");
} else {
    println!("User is not logged in");
}
```

### Validate Token

```rust
use sa_token_core::token::TokenValue;

let token = TokenValue::new("your_token_string".to_string());

// Check if token is valid
let is_valid = StpUtil::is_valid(&token).await;

// Get token info
let token_info = StpUtil::get_token_info(&token).await?;
println!("Login ID: {}", token_info.login_id);
println!("Device: {:?}", token_info.device);
```

### Get Login ID from Token

```rust
// Get login_id from token
let login_id = StpUtil::get_login_id_by_token(&token).await?;
```

## Session Management

### Get Session

```rust
// Get user session
let session = StpUtil::get_session("user_10001").await?;

// Store data in session
session.set("username", "John Doe".to_string()).await;
session.set("email", "john@example.com".to_string()).await;

// Retrieve data from session
let username: Option<String> = session.get("username").await;
println!("Username: {:?}", username);
```

### Session Operations

```rust
// Check if session exists
let exists = session.has("email").await;

// Remove session data
session.remove("email").await;

// Clear all session data
session.clear().await;

// Get all session keys
let keys = session.keys().await;
```

### Delete Session

```rust
// Delete user session
StpUtil::delete_session("user_10001").await?;
```

## Permission Management

### Set Permissions

```rust
// Set user permissions
StpUtil::set_permissions(
    "user_10001",
    vec![
        "user:list".to_string(),
        "user:add".to_string(),
        "user:edit".to_string(),
        "user:delete".to_string(),
    ]
).await?;
```

### Check Permissions

```rust
// Check if user has a permission
let has_permission = StpUtil::has_permission("user_10001", "user:delete").await;

if has_permission {
    println!("User can delete");
} else {
    println!("User cannot delete");
}
```

### Check Multiple Permissions

```rust
// Check if user has all permissions (AND)
let has_all = StpUtil::has_permissions_and(
    "user_10001",
    &["user:list", "user:add"]
).await;

// Check if user has any permission (OR)
let has_any = StpUtil::has_permissions_or(
    "user_10001",
    &["user:delete", "admin:all"]
).await;
```

### Get User Permissions

```rust
// Get all permissions for a user
let permissions = StpUtil::get_permissions("user_10001").await;
println!("User permissions: {:?}", permissions);
```

### Clear Permissions

```rust
// Clear all permissions for a user
StpUtil::clear_permissions("user_10001").await?;
```

## Role Management

### Set Roles

```rust
// Set user roles
StpUtil::set_roles(
    "user_10001",
    vec![
        "user".to_string(),
        "vip".to_string(),
    ]
).await?;

// Set admin role
StpUtil::set_roles(
    "admin_10001",
    vec!["admin".to_string()]
).await?;
```

### Check Roles

```rust
// Check if user has a role
let is_admin = StpUtil::has_role("user_10001", "admin").await;

if is_admin {
    println!("User is admin");
}
```

### Check Multiple Roles

```rust
// Check if user has all roles (AND)
let has_all_roles = StpUtil::has_roles_and(
    "user_10001",
    &["user", "vip"]
).await;

// Check if user has any role (OR)
let has_any_role = StpUtil::has_roles_or(
    "user_10001",
    &["admin", "moderator"]
).await;
```

### Get User Roles

```rust
// Get all roles for a user
let roles = StpUtil::get_roles("user_10001").await;
println!("User roles: {:?}", roles);
```

### Clear Roles

```rust
// Clear all roles for a user
StpUtil::clear_roles("user_10001").await?;
```

## Advanced Usage

### Complete Login Flow Example

```rust
use sa_token_core::StpUtil;

// 1. User login
let login_id = "user_10001";
let token = StpUtil::login(login_id).await?;

// 2. Set user permissions
StpUtil::set_permissions(
    login_id,
    vec![
        "user:list".to_string(),
        "user:add".to_string(),
        "post:create".to_string(),
    ]
).await?;

// 3. Set user roles
StpUtil::set_roles(
    login_id,
    vec!["user".to_string(), "author".to_string()]
).await?;

// 4. Store additional data in session
let session = StpUtil::get_session(login_id).await?;
session.set("username", "John Doe".to_string()).await;
session.set("email", "john@example.com".to_string()).await;
session.set("last_login", chrono::Utc::now().to_string()).await;

// Return token to client
Ok(token.value().to_string())
```

### Token Validation in Middleware

```rust
use sa_token_core::StpUtil;
use sa_token_core::token::TokenValue;

async fn validate_request(token_string: &str) -> Result<String, String> {
    let token = TokenValue::new(token_string.to_string());
    
    // Validate token
    if !StpUtil::is_valid(&token).await {
        return Err("Invalid token".to_string());
    }
    
    // Get login_id
    let login_id = StpUtil::get_login_id_by_token(&token).await
        .map_err(|_| "Cannot get login_id".to_string())?;
    
    // Check if user is still logged in
    if !StpUtil::is_login(&login_id).await {
        return Err("User not logged in".to_string());
    }
    
    Ok(login_id)
}
```

### Permission-based Access Control

```rust
use sa_token_core::StpUtil;

async fn delete_user(operator_id: &str, target_user_id: &str) -> Result<(), String> {
    // Check if operator has delete permission
    if !StpUtil::has_permission(operator_id, "user:delete").await {
        return Err("No permission to delete users".to_string());
    }
    
    // Additional check: admin can delete anyone, user can only delete self
    let is_admin = StpUtil::has_role(operator_id, "admin").await;
    
    if !is_admin && operator_id != target_user_id {
        return Err("Can only delete your own account".to_string());
    }
    
    // Proceed with deletion
    // ... your deletion logic
    
    Ok(())
}
```

### Multi-device Session Management

```rust
use sa_token_core::StpUtil;

// User logs in from different devices
let token_web = StpUtil::login_by_device("user_10001", "web").await?;
let token_mobile = StpUtil::login_by_device("user_10001", "mobile_ios").await?;
let token_app = StpUtil::login_by_device("user_10001", "desktop_app").await?;

// Logout from specific device
StpUtil::logout_by_device("user_10001", "mobile_ios").await?;

// User is still logged in on other devices
assert!(StpUtil::is_login("user_10001").await);

// Logout from all devices
StpUtil::logout("user_10001").await?;
```

### Working with Generic Types

```rust
use sa_token_core::StpUtil;

// StpUtil supports any type that implements Display
// This includes: String, &str, i32, i64, u32, u64, etc.

// String login_id
let token1 = StpUtil::login("user_string".to_string()).await?;

// &str login_id
let token2 = StpUtil::login("user_str").await?;

// Numeric login_id
let token3 = StpUtil::login(10001_i32).await?;
let token4 = StpUtil::login(20001_i64).await?;
let token5 = StpUtil::login(30001_u32).await?;

// All methods accept generic types
StpUtil::set_permissions(10001, vec!["user:list".to_string()]).await?;
StpUtil::has_role(20001_i64, "admin").await;
let session = StpUtil::get_session(30001_u32).await?;
```

## Error Handling

All `StpUtil` methods return `Result` types. Handle errors appropriately:

```rust
use sa_token_core::StpUtil;

match StpUtil::login("user_10001").await {
    Ok(token) => {
        println!("Login successful: {}", token.value());
    }
    Err(e) => {
        eprintln!("Login failed: {:?}", e);
    }
}

// Or use the ? operator
let token = StpUtil::login("user_10001").await?;
```

## Best Practices

1. **Automatic Initialization**: `StpUtil` is automatically initialized when you build `SaTokenState`, no manual initialization needed.

2. **Error Handling**: Always handle errors from `StpUtil` methods appropriately.

3. **Permission Naming**: Use consistent naming conventions for permissions (e.g., `resource:action`).

4. **Role Hierarchy**: Design a clear role hierarchy (e.g., admin > moderator > user).

5. **Session Data**: Store minimal, non-sensitive data in sessions.

6. **Logout on Security Events**: Always call `logout` or `kick_out` when security-sensitive events occur (password change, etc.).

7. **Token Validation**: Always validate tokens before processing requests.

8. **Generic Types**: Leverage generic `LoginId` support for cleaner code with different ID types.

## See Also

- [Main Documentation](../README.md)
- [Examples](../examples/)
- [Web Framework Integration](../README.md#framework-integration-examples)

