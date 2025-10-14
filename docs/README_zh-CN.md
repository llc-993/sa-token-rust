# sa-token-rust 文档

## 📚 可用文档

所有文档均提供中英文双语版本。

### 核心文档

| 文档 | English | 中文 | 说明 |
|----------|---------|------|-------------|
| **StpUtil API 参考** | [StpUtil.md](./StpUtil.md) | [StpUtil_zh-CN.md](./StpUtil_zh-CN.md) | StpUtil 工具类完整指南 |
| **事件监听** | [EVENT_LISTENER.md](./EVENT_LISTENER.md) | [EVENT_LISTENER_zh-CN.md](./EVENT_LISTENER_zh-CN.md) | 完整的事件监听系统文档 |
| **事件监听快速开始** | [EVENT_LISTENER_QUICKSTART.md](./EVENT_LISTENER_QUICKSTART.md) | [EVENT_LISTENER_QUICKSTART_zh-CN.md](./EVENT_LISTENER_QUICKSTART_zh-CN.md) | 事件监听器快速入门指南 |
| **权限匹配规则** | [PermissionMatching.md](./PermissionMatching.md) | [PermissionMatching.md](./PermissionMatching.md) | 权限检查和通配符匹配（双语） |

## 📖 快速链接

### 入门指南
- [主 README (English)](../README.md)
- [主 README (中文)](../README_zh-CN.md)
- [示例代码](../examples/)

### 核心功能

#### 认证与授权
- **StpUtil**: 认证操作的静态工具类
  - 登录/登出管理
  - Token 验证
  - Session 管理
  - 权限和角色检查

#### 事件系统
- **事件监听**: 监听认证事件
  - 登录事件
  - 登出事件
  - 踢出下线事件
  - Token 续期事件
  - 自定义事件处理

#### 权限系统
- **权限匹配**: 灵活的权限检查
  - 精确匹配
  - 通配符模式 (`user:*`)
  - 层级权限

## 🚀 运行示例

```bash
# 事件监听示例
cargo run --example event_listener_example

# Web 框架示例
cd examples/axum-full-example && cargo run
cd examples/poem-full-example && cargo run
```

## 🤝 贡献

欢迎贡献！请随时提交 issues 和 pull requests。

## 📄 许可证

本项目采用以下任一许可证：

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE))
- MIT License ([LICENSE-MIT](../LICENSE-MIT))

由你选择。

