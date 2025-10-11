# sa-token-rust Axum 完整示例

这是一个完整的 Axum + sa-token-rust 集成示例，展示了如何在实际项目中使用 sa-token-rust。

## 功能特性

1. ✅ 完整的认证流程（登录、注册）
2. ✅ 权限管理（加载、检查、验证）
3. ✅ 角色管理（加载、检查、验证）
4. ✅ 使用过程宏进行声明式认证
5. ✅ 多种认证场景示例

## 项目结构

```
axum-full-example/
├── src/
│   ├── main.rs         # 主程序入口
│   ├── auth.rs         # 认证相关（登录、请求/响应类型）
│   └── permission.rs   # 权限服务（管理用户权限和角色）
├── Cargo.toml
└── README.md
```

## 运行示例

```bash
cd examples/axum-full-example
cargo run
```

服务器将在 `http://localhost:3000` 启动。

## 测试账号

| 用户名 | 密码 | 权限 | 角色 |
|--------|------|------|------|
| admin | admin123 | user:list, user:read, user:write, user:delete, admin:* | admin, user |
| user | user123 | user:read, user:write | user |
| guest | guest123 | user:read | guest |

## API 端点

### 公开接口（无需认证）

- `GET /` - 首页
- `GET /api/health` - 健康检查
- `POST /api/login` - 登录
- `POST /api/register` - 注册

### 需要登录

- `GET /api/user/info` - 获取用户信息
- `GET /api/user/profile` - 获取用户资料

### 需要特定权限

- `GET /api/user/list` - 列出所有用户（需要 `user:list` 权限）
- `POST /api/user/delete` - 删除用户（需要 `user:delete` 权限）

### 需要管理员角色

- `GET /api/admin/panel` - 管理员面板（需要 `admin` 角色）
- `GET /api/admin/stats` - 管理员统计（需要 `admin` 角色）

### 需要多个权限

- `POST /api/user/manage` - 管理用户（需要 `user:read` 和 `user:write` 权限）

## 使用示例

### 1. 登录

```bash
curl -X POST http://localhost:3000/api/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123"}'
```

响应：
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "token": "xxx-xxx-xxx",
    "user_info": {
      "id": "admin",
      "username": "admin",
      "nickname": "管理员",
      "email": "admin@example.com"
    }
  }
}
```

### 2. 访问需要登录的接口

```bash
curl http://localhost:3000/api/user/info \
  -H "Authorization: Bearer <your-token>"
```

### 3. 访问需要权限的接口

```bash
# 列出用户（需要 user:list 权限）
curl http://localhost:3000/api/user/list \
  -H "Authorization: Bearer <admin-token>"
```

### 4. 访问管理员接口

```bash
# 管理员面板（需要 admin 角色）
curl http://localhost:3000/api/admin/panel \
  -H "Authorization: Bearer <admin-token>"
```

## 如何加载权限

在实际应用中，权限通常存储在数据库中。本示例展示了如何管理和检查权限：

### 初始化权限数据

```rust
// 在 main.rs 中
async fn init_test_data(permission_service: &PermissionService) {
    // 为管理员添加权限
    permission_service.add_user_permissions("admin", vec![
        "user:list".to_string(),
        "user:read".to_string(),
        "user:write".to_string(),
        "user:delete".to_string(),
        "admin:*".to_string(),  // 通配符权限
    ]).await;
    
    // 为管理员添加角色
    permission_service.add_user_roles("admin", vec![
        "admin".to_string(),
        "user".to_string(),
    ]).await;
}
```

### 权限服务使用

```rust
// 检查单个权限
let has_perm = permission_service.has_permission("admin", "user:delete").await;

// 检查多个权限（AND）
let has_all = permission_service.has_all_permissions("admin", &["user:read", "user:write"]).await;

// 检查多个权限（OR）
let has_any = permission_service.has_any_permission("admin", &["user:admin", "user:super"]).await;

// 检查角色
let has_role = permission_service.has_role("admin", "admin").await;
```

### 从数据库加载权限

实际应用中，应该从数据库加载权限：

```rust
impl PermissionService {
    pub async fn load_from_database(&self, user_id: &str) {
        // 从数据库查询用户权限
        // let permissions = sqlx::query!("SELECT permission FROM user_permissions WHERE user_id = ?", user_id)
        //     .fetch_all(&pool)
        //     .await?;
        
        // 从数据库查询用户角色
        // let roles = sqlx::query!("SELECT role FROM user_roles WHERE user_id = ?", user_id)
        //     .fetch_all(&pool)
        //     .await?;
        
        // 加载到内存
        // self.add_user_permissions(user_id, permissions).await;
        // self.add_user_roles(user_id, roles).await;
    }
}
```

## 使用宏的优势

### 声明式认证

使用宏可以让代码更清晰：

```rust
// ❌ 不使用宏
async fn delete_user() -> Result<String, Error> {
    // 手动检查登录
    if !is_logged_in() {
        return Err(Error::Unauthorized);
    }
    
    // 手动检查权限
    if !has_permission("user:delete") {
        return Err(Error::Forbidden);
    }
    
    // 业务逻辑
    Ok("User deleted")
}

// ✅ 使用宏
#[sa_check_permission("user:delete")]
async fn delete_user() -> Result<String, Error> {
    // 直接写业务逻辑
    Ok("User deleted")
}
```

### 减少样板代码

```rust
// 复杂的权限检查变得简单
#[sa_check_permissions_and("user:read", "user:write")]
async fn manage_user() -> String {
    "Manage user"
}

// 多角色检查
#[sa_check_roles_or("admin", "moderator")]
async fn moderate_content() -> String {
    "Moderate content"
}
```

### 公开接口无需认证

```rust
#[sa_ignore]
async fn health_check() -> String {
    "OK"
}
```

## 扩展建议

1. **添加数据库支持** - 使用 sqlx 或 diesel 持久化权限
2. **实现真实的中间件** - 在中间件中读取宏的元数据并执行验证
3. **添加缓存** - 使用 Redis 缓存权限数据
4. **完善错误处理** - 返回更友好的错误信息
5. **添加日志审计** - 记录所有权限检查操作

## 下一步

- 查看 [宏文档](../../docs/MACROS.md) 了解更多宏的使用
- 查看 [架构文档](../../docs/ARCHITECTURE.md) 了解系统设计
- 实现自己的权限加载逻辑

