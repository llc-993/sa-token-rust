# sa-token-rust Documentation

## 📚 Available Documentation

All documentation is available in both English and Chinese (中文).

### Core Documentation

| Document | English | 中文 | Description |
|----------|---------|------|-------------|
| **StpUtil API Reference** | [StpUtil.md](./StpUtil.md) | [StpUtil_zh-CN.md](./StpUtil_zh-CN.md) | Complete guide to the StpUtil utility class |
| **Event Listeners** | [EVENT_LISTENER.md](./EVENT_LISTENER.md) | [EVENT_LISTENER_zh-CN.md](./EVENT_LISTENER_zh-CN.md) | Comprehensive event listening system documentation |
| **Event Listener Quick Start** | [EVENT_LISTENER_QUICKSTART.md](./EVENT_LISTENER_QUICKSTART.md) | [EVENT_LISTENER_QUICKSTART_zh-CN.md](./EVENT_LISTENER_QUICKSTART_zh-CN.md) | Quick start guide for event listeners |
| **Permission Matching Rules** | [PermissionMatching.md](./PermissionMatching.md) | [PermissionMatching.md](./PermissionMatching.md) | Permission checking and wildcard matching (bilingual) |

## 📖 Quick Links

### Getting Started
- [Main README (English)](../README.md)
- [主 README (中文)](../README_zh-CN.md)
- [Examples](../examples/)

### Core Features

#### Authentication & Authorization
- **StpUtil**: Static utility class for authentication operations
  - Login/Logout management
  - Token validation
  - Session management
  - Permission & role checking

#### Event System
- **Event Listeners**: Monitor authentication events
  - Login events
  - Logout events
  - Kick-out events
  - Token renewal events
  - Custom event handlers

#### Permission System
- **Permission Matching**: Flexible permission checking
  - Exact matching
  - Wildcard patterns (`user:*`)
  - Hierarchical permissions

## 🚀 Running Examples

```bash
# Event listener example
cargo run --example event_listener_example

# Web framework examples
cd examples/axum-full-example && cargo run
cd examples/poem-full-example && cargo run
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## 📄 License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE))
- MIT License ([LICENSE-MIT](../LICENSE-MIT))

at your option.

