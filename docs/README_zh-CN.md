# sa-token-rust 文档

English | [中文文档](./README_zh-CN.md)

欢迎使用 **sa-token-rust** 文档！所有文档均提供中英文双语版本。

> 💡 **新功能！** [简化使用指南](../README_zh-CN.md#-简化使用方式推荐) - 只需一个依赖即可开始！

---

## 📚 文档索引

### 🚀 入门指南

| 文档 | 说明 |
|----------|-------------|
| [主 README](../README_zh-CN.md) | 完整的项目概述和快速开始 |
| [简化使用方式](../README_zh-CN.md#-简化使用方式推荐) | 一行导入，包含所有功能 |
| [架构概览](../README_zh-CN.md#-架构) | 项目结构和设计 |

---

### 📖 核心文档

| 文档 | English | 中文 | 说明 |
|----------|---------|------|-------------|
| **StpUtil API 参考** | [StpUtil.md](./StpUtil.md) | [StpUtil_zh-CN.md](./StpUtil_zh-CN.md) | StpUtil 工具类完整指南 |
| **权限匹配规则** | [PermissionMatching.md](./PermissionMatching.md#english) | [PermissionMatching.md](./PermissionMatching.md#中文) | 权限检查和通配符匹配 |

---

### 🎯 功能指南

#### 认证与授权

| 功能 | English | 中文 | 说明 |
|---------|---------|------|-------------|
| **事件监听** | [EVENT_LISTENER.md](./EVENT_LISTENER.md) | [EVENT_LISTENER_zh-CN.md](./EVENT_LISTENER_zh-CN.md) | 监听登录、登出和踢出下线事件 |
| **事件监听快速开始** | [QUICKSTART.md](./EVENT_LISTENER_QUICKSTART.md) | [QUICKSTART_zh-CN.md](./EVENT_LISTENER_QUICKSTART_zh-CN.md) | 5分钟快速上手事件监听 |
| **JWT 指南** | [JWT_GUIDE.md](./JWT_GUIDE.md) | [JWT_GUIDE_zh-CN.md](./JWT_GUIDE_zh-CN.md) | 完整的 JWT 实现（8种算法） |
| **OAuth2 指南** | [OAUTH2_GUIDE.md](./OAUTH2_GUIDE.md) | [OAUTH2_GUIDE_zh-CN.md](./OAUTH2_GUIDE_zh-CN.md) | OAuth2 授权码模式 |

#### 实时通信与 WebSocket

| 功能 | 多语言支持 | 说明 |
|---------|------------------------|-------------|
| **WebSocket 认证** | [WEBSOCKET_AUTH.md](./WEBSOCKET_AUTH.md) | 🌍 7种语言：EN, 中文, ไทย, Tiếng Việt, ខ្មែរ, Melayu, မြန်မာ |
| **在线用户管理** | [ONLINE_USER_MANAGEMENT.md](./ONLINE_USER_MANAGEMENT.md) | 追踪在线用户并推送实时消息 |

#### 分布式系统

| 功能 | 多语言支持 | 说明 |
|---------|------------------------|-------------|
| **分布式 Session** | [DISTRIBUTED_SESSION.md](./DISTRIBUTED_SESSION.md) | 微服务跨服务会话共享 |
| **SSO 单点登录** | [SSO_GUIDE.md](./SSO_GUIDE.md#中文) | 基于票据的 SSO 和统一登出（7 种语言）|

#### 错误处理

| 功能 | 多语言支持 | 说明 |
|---------|------------------------|-------------|
| **错误参考手册** | [ERROR_REFERENCE.md](./ERROR_REFERENCE.md) | 完整的错误代码参考（32种类型，7种语言） |

---

### 💻 代码示例

所有示例位于 [`examples/`](../examples/) 目录：

| 示例 | 说明 |
|---------|-------------|
| `event_listener_example.rs` | 事件监听器与自定义处理器 |
| `jwt_example.rs` | JWT 生成、验证和刷新 |
| `token_styles_example.rs` | 所有7种 Token 生成风格 |
| `security_features_example.rs` | Nonce（防重放攻击）和 Refresh Token |
| `oauth2_example.rs` | 完整的 OAuth2 授权流程 |
| `websocket_online_example.rs` | WebSocket 认证 + 在线用户追踪 |
| `distributed_session_example.rs` | 微服务分布式会话 |

**运行示例：**
```bash
# 基础示例
cargo run --example event_listener_example
cargo run --example jwt_example

# WebSocket + 在线用户
cargo run --example websocket_online_example

# 分布式会话
cargo run --example distributed_session_example
```

---

### 🔧 框架集成

| 框架 | 插件包 | 状态 |
|-----------|---------------|--------|
| **Axum** | `sa-token-plugin-axum` | ✅ 稳定 |
| **Actix-web** | `sa-token-plugin-actix-web` | ✅ 稳定 |
| **Poem** | `sa-token-plugin-poem` | ✅ 稳定 |
| **Rocket** | `sa-token-plugin-rocket` | ✅ 稳定 |
| **Warp** | `sa-token-plugin-warp` | ✅ 稳定 |

**快速开始任意框架：**

```toml
[dependencies]
# 一站式包（选择你的框架）
sa-token-plugin-axum = "0.1.3"  # 或 actix-web, poem, rocket, warp
```

```rust
use sa_token_plugin_axum::*;  // ✨ 包含所有功能！
```

详见 [主 README - 简化使用方式](../README_zh-CN.md#-简化使用方式推荐)。

---

### 🌍 语言支持

**双语文档（English + 中文）：**
- StpUtil API 参考
- 事件监听
- JWT 指南
- OAuth2 指南
- 权限匹配

**多语言文档（7种语言）：**
- WebSocket 认证
- 在线用户管理
- 分布式 Session
- SSO 单点登录
- 错误参考手册

**支持的语言：**
- 🇬🇧 English（英语）
- 🇨🇳 中文（Chinese）
- 🇹🇭 ภาษาไทย（泰语）
- 🇻🇳 Tiếng Việt（越南语）
- 🇰🇭 ភាសាខ្មែរ（高棉语）
- 🇲🇾 Bahasa Melayu（马来语）
- 🇲🇲 မြန်မာဘာသာ（缅甸语）

---

## 🎯 功能概览

### 核心功能

- **认证**：登录、登出、Token 验证、Session 管理
- **授权**：基于权限和角色的访问控制，支持通配符
- **事件系统**：监听登录、登出、踢出下线和自定义事件
- **Token 风格**：UUID、Random、JWT、Hash、Timestamp、Tik（7种选项）

### 高级功能

- **JWT 支持**：8种算法（HS256/384/512, RS256/384/512, ES256/384）
- **OAuth2**：完整的授权码模式实现
- **安全特性**：Nonce（防重放攻击）、Refresh Token 机制
- **WebSocket**：使用多种 Token 来源认证 WebSocket 连接
- **在线用户**：实时追踪和消息推送
- **分布式 Session**：微服务跨服务会话共享

### 存储选项

- **Memory**（默认）：用于开发的内存存储
- **Redis**：生产就绪的 Redis 后端
- **Database**：自定义数据库存储（可扩展）

**通过 features 轻松切换：**
```toml
sa-token-plugin-axum = { version = "0.1.3", features = ["redis"] }
```

---

## 🔍 快速查找

**你在寻找：**

- **入门指南？** → [主 README](../README_zh-CN.md) → [简化使用方式](../README_zh-CN.md#-简化使用方式推荐)
- **认证基础？** → [StpUtil API 参考](./StpUtil_zh-CN.md)
- **事件监听？** → [事件监听指南](./EVENT_LISTENER_zh-CN.md)
- **JWT Token？** → [JWT 指南](./JWT_GUIDE_zh-CN.md)
- **OAuth2？** → [OAuth2 指南](./OAUTH2_GUIDE_zh-CN.md)
- **WebSocket 认证？** → [WebSocket 认证](./WEBSOCKET_AUTH.md)
- **实时功能？** → [在线用户管理](./ONLINE_USER_MANAGEMENT.md)
- **微服务？** → [分布式 Session](./DISTRIBUTED_SESSION.md)
- **单点登录？** → [SSO 指南](./SSO_GUIDE.md#中文)
- **错误代码？** → [错误参考手册](./ERROR_REFERENCE.md)
- **代码示例？** → [示例目录](../examples/)

---

## 🤝 贡献

欢迎贡献！请随时提交：
- 🐛 Bug 报告
- 💡 功能请求
- 📖 文档改进
- 🔧 Pull requests

查看 [CONTRIBUTING.md](../CONTRIBUTING.md) 了解贡献指南。

---

## 📄 许可证

本项目采用双重许可：

- **Apache License, Version 2.0** ([LICENSE-APACHE](../LICENSE-APACHE))
- **MIT License** ([LICENSE-MIT](../LICENSE-MIT))

你可以任选其一。

---

## 📞 支持

- 📚 **文档**：你正在这里！
- 💬 **Issues**：[GitHub Issues](https://github.com/your-repo/sa-token-rust/issues)
- 🌟 **给个星**：如果这个项目对你有帮助，请在 GitHub 上给个 ⭐！

---

**由 sa-token 社区用 ❤️ 制作**

