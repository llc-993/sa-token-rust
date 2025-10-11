# 架构设计

## 总体架构

sa-token-rust 采用分层架构设计，确保核心逻辑与Web框架解耦。

```
┌─────────────────────────────────────────────────┐
│          应用层 (Application Layer)              │
│         使用 sa-token 的业务代码                 │
└─────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────┐
│        框架插件层 (Framework Plugin Layer)       │
│   Axum  │ Actix-web │ Rocket │ Warp │ Poem     │
└─────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────┐
│         适配器层 (Adapter Layer)                 │
│    SaRequest │ SaResponse │ SaStorage           │
└─────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────┐
│          核心层 (Core Layer)                     │
│   Token │ Session │ Permission │ Context        │
└─────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────┐
│        存储层 (Storage Layer)                    │
│      Memory  │  Redis  │  Database              │
└─────────────────────────────────────────────────┘
```

## 核心组件

### 1. sa-token-core

框架无关的核心逻辑，包括：

- **Token管理**: 生成、验证、刷新token
- **Session管理**: 会话存储与管理
- **权限系统**: 权限和角色验证
- **配置管理**: 框架配置

### 2. sa-token-adapter

定义所有适配器接口：

- `SaStorage`: 存储适配器
- `SaRequest`: 请求上下文适配器
- `SaResponse`: 响应上下文适配器

### 3. sa-token-storage-*

具体的存储实现：

- **Memory**: 基于HashMap的内存存储
- **Redis**: 基于redis-rs的Redis存储
- **Database**: 数据库存储（待实现）

### 4. sa-token-plugin-*

各Web框架的集成插件：

- 实现框架特定的中间件/拦截器
- 实现请求/响应适配器
- 提供框架友好的API

### 5. sa-token-macro

过程宏支持：

- `#[sa_check_login]`
- `#[sa_check_permission]`
- `#[sa_check_role]`

## 设计原则

### 1. 框架无关

核心逻辑完全独立于任何Web框架，通过trait实现适配。

### 2. 存储抽象

通过`SaStorage` trait抽象存储层，支持多种存储后端。

### 3. 异步优先

全部基于async/await，充分利用Tokio的并发能力。

### 4. 类型安全

充分利用Rust的类型系统，在编译时捕获错误。

### 5. 易于扩展

基于trait的设计，用户可以轻松实现自定义存储或框架适配器。

## Token流程

```
1. 用户登录
   ↓
2. 生成Token（根据配置的TokenStyle）
   ↓
3. 创建TokenInfo（包含登录ID、过期时间等）
   ↓
4. 存储到Storage（key: sa:token:{token}, value: TokenInfo JSON）
   ↓
5. 返回Token给客户端
   ↓
6. 客户端携带Token访问受保护资源
   ↓
7. 中间件从请求中提取Token
   ↓
8. 从Storage获取TokenInfo并验证
   ↓
9. 验证通过，继续处理请求
```

## Session流程

```
1. 获取Session（通过login_id）
   ↓
2. 从Storage读取（key: sa:session:{login_id}）
   ↓
3. 如果不存在，创建新Session
   ↓
4. 修改Session数据
   ↓
5. 保存到Storage
```

## 扩展点

### 添加新的存储后端

1. 实现`SaStorage` trait
2. 处理过期时间
3. 实现批量操作（可选，提升性能）

### 添加新的Web框架支持

1. 创建新的plugin crate
2. 实现`SaRequest`和`SaResponse`
3. 实现框架的中间件机制
4. 提供提取器/守卫等便利功能

### 自定义Token生成策略

1. 实现自定义的TokenGenerator
2. 在配置中指定使用

### 自定义权限验证逻辑

1. 实现`PermissionChecker` trait
2. 注入到SaTokenManager

