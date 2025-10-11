//! sa-token-macro 基础使用示例

use sa_token_macro::*;

// ============ 登录检查示例 ============

#[sa_check_login]
async fn user_info() -> String {
    "User info - requires login".to_string()
}

// ============ 权限检查示例 ============

#[sa_check_permission("user:read")]
async fn get_user(id: u64) -> String {
    format!("Get user {} - requires user:read permission", id)
}

#[sa_check_permission("user:write")]
async fn update_user(id: u64, name: String) -> String {
    format!("Update user {} to {} - requires user:write permission", id, name)
}

#[sa_check_permission("user:delete")]
async fn delete_user(id: u64) -> String {
    format!("Delete user {} - requires user:delete permission", id)
}

// ============ 角色检查示例 ============

#[sa_check_role("admin")]
async fn admin_panel() -> String {
    "Admin panel - requires admin role".to_string()
}

#[sa_check_role("moderator")]
async fn moderate_content(content_id: u64) -> String {
    format!("Moderate content {} - requires moderator role", content_id)
}

// ============ 多权限检查示例 ============

#[sa_check_permissions_and("user:read", "user:write")]
async fn manage_user() -> String {
    "Manage user - requires both user:read AND user:write permissions".to_string()
}

#[sa_check_permissions_or("admin:all", "super:all")]
async fn super_admin_action() -> String {
    "Super admin action - requires admin:all OR super:all permission".to_string()
}

// ============ 多角色检查示例 ============

#[sa_check_roles_and("admin", "super")]
async fn super_admin_panel() -> String {
    "Super admin panel - requires both admin AND super roles".to_string()
}

#[sa_check_roles_or("admin", "moderator")]
async fn moderate_or_admin() -> String {
    "Moderate or admin - requires admin OR moderator role".to_string()
}

// ============ 忽略认证示例 ============

#[sa_ignore]
async fn public_api() -> String {
    "Public API - no authentication required".to_string()
}

#[sa_ignore]
async fn health_check() -> String {
    "OK - health check doesn't need auth".to_string()
}

// ============ 结构体级别的忽略认证 ============

#[sa_ignore]
struct PublicController;

impl PublicController {
    async fn home() -> String {
        "Home page - public access".to_string()
    }
    
    async fn about() -> String {
        "About page - public access".to_string()
    }
}

// ============ impl块级别的忽略认证 ============

struct ApiController;

#[sa_ignore]
impl ApiController {
    async fn version() -> String {
        "v1.0.0 - version API is public".to_string()
    }
    
    async fn status() -> String {
        "running - status API is public".to_string()
    }
}

// ============ 混合使用示例 ============

struct UserController;

impl UserController {
    // 公开接口
    #[sa_ignore]
    async fn register(username: String) -> String {
        format!("Register user: {} - public", username)
    }
    
    // 需要登录
    #[sa_check_login]
    async fn profile() -> String {
        "User profile - requires login".to_string()
    }
    
    // 需要特定权限
    #[sa_check_permission("user:update_profile")]
    async fn update_profile(data: String) -> String {
        format!("Update profile: {} - requires permission", data)
    }
    
    // 需要管理员角色
    #[sa_check_role("admin")]
    async fn list_all_users() -> String {
        "List all users - requires admin role".to_string()
    }
}

#[tokio::main]
async fn main() {
    println!("=== sa-token-macro 示例 ===\n");
    
    println!("1. 登录检查:");
    println!("   {}", user_info().await);
    
    println!("\n2. 权限检查:");
    println!("   {}", get_user(123).await);
    println!("   {}", update_user(123, "Alice".to_string()).await);
    
    println!("\n3. 角色检查:");
    println!("   {}", admin_panel().await);
    
    println!("\n4. 多权限检查:");
    println!("   {}", manage_user().await);
    
    println!("\n5. 公开API（忽略认证）:");
    println!("   {}", public_api().await);
    println!("   {}", health_check().await);
    
    println!("\n6. 控制器示例:");
    println!("   {}", PublicController::home().await);
    println!("   {}", ApiController::version().await);
    println!("   {}", UserController::register("Bob".to_string()).await);
    
    println!("\n注意：这些宏主要是添加编译时标记，实际的认证逻辑在框架中间件中执行。");
}

