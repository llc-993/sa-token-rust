# Distributed Session Management | 分布式 Session 管理

[English](#english) | [中文](#中文) | [ภาษาไทย](#ภาษาไทย) | [Tiếng Việt](#tiếng-việt) | [ភាសាខ្មែរ](#ភាសាខ្មែរ) | [Bahasa Melayu](#bahasa-melayu) | [မြန်မာဘာသာ](#မြန်မာဘာသာ)

---

## English

### Overview

The Distributed Session Management module enables session sharing across multiple microservices. It provides service authentication, cross-service session access, and attribute management with automatic timeout handling.

### Key Features

- **Cross-Service Session Sharing** - Share sessions across microservices
- **Service Authentication** - Verify service credentials
- **Session Attributes** - Store custom key-value pairs
- **Multi-Session Support** - One user can have multiple sessions
- **Automatic Cleanup** - TTL-based session expiration
- **Pluggable Storage** - Use custom storage backends

### Quick Start

```rust
use sa_token_core::{
    DistributedSessionManager, InMemoryDistributedStorage, ServiceCredential
};
use std::sync::Arc;
use std::time::Duration;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create distributed session manager
    let storage = Arc::new(InMemoryDistributedStorage::new());
    let manager = DistributedSessionManager::new(
        storage,
        "service-main".to_string(),
        Duration::from_secs(3600), // 1 hour TTL
    );
    
    // Register a service
    let credential = ServiceCredential {
        service_id: "api-gateway".to_string(),
        service_name: "API Gateway".to_string(),
        secret_key: "secret123".to_string(),
        created_at: Utc::now(),
        permissions: vec!["read".to_string(), "write".to_string()],
    };
    manager.register_service(credential).await;
    
    // Verify service
    let verified = manager.verify_service("api-gateway", "secret123").await?;
    println!("Service verified: {}", verified.service_name);
    
    // Create session
    let session = manager.create_session(
        "user123".to_string(),
        "token456".to_string(),
    ).await?;
    
    // Set session attribute
    manager.set_attribute(
        &session.session_id,
        "role".to_string(),
        "admin".to_string(),
    ).await?;
    
    // Get session attribute
    if let Some(role) = manager.get_attribute(&session.session_id, "role").await? {
        println!("User role: {}", role);
    }
    
    // Get all sessions for user
    let sessions = manager.get_sessions_by_login_id("user123").await?;
    println!("User has {} active sessions", sessions.len());
    
    Ok(())
}
```

### Service Authentication Flow

```text
Service A                   Manager                    Service B
   |                           |                           |
   |-- register_service ------>|                           |
   |<----- registered ---------|                           |
   |                           |                           |
   |                           |<-- verify_service(id, secret)
   |                           |--- check credentials ---->|
   |                           |<----- verified ----------|
```

### Cross-Service Session Access

```text
Service A creates session:
  session_id: "uuid-123"
  login_id: "user123"
  attributes: {"role": "admin"}

Service B accesses session:
  get_session("uuid-123") -> Full session data
  Can read/modify attributes
  Updates last_access timestamp
```

### API Reference

#### DistributedSessionManager

**Methods:**
- `new(storage, service_id, timeout)` - Create manager
- `register_service(credential)` - Register a service
- `verify_service(id, secret)` - Verify service credentials
- `create_session(login_id, token)` - Create new session
- `get_session(session_id)` - Get session by ID
- `update_session(session)` - Update existing session
- `delete_session(session_id)` - Delete session
- `set_attribute(id, key, value)` - Set session attribute
- `get_attribute(id, key)` - Get session attribute
- `remove_attribute(id, key)` - Remove session attribute
- `get_sessions_by_login_id(login_id)` - Get all user sessions
- `delete_all_sessions(login_id)` - Delete all user sessions

---

## 中文

### 概述

分布式 Session 管理模块支持跨多个微服务共享 Session。它提供服务认证、跨服务 Session 访问和属性管理，并自动处理超时。

### 核心功能

- **跨服务 Session 共享** - 在微服务间共享 Session
- **服务认证** - 验证服务凭证
- **Session 属性** - 存储自定义键值对
- **多 Session 支持** - 一个用户可以有多个 Session
- **自动清理** - 基于 TTL 的 Session 过期
- **可插拔存储** - 使用自定义存储后端

### 快速开始

```rust
use sa_token_core::{
    DistributedSessionManager, InMemoryDistributedStorage, ServiceCredential
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建分布式 Session 管理器
    let storage = Arc::new(InMemoryDistributedStorage::new());
    let manager = DistributedSessionManager::new(
        storage,
        "service-main".to_string(),
        Duration::from_secs(3600), // 1 小时 TTL
    );
    
    // 注册服务
    let credential = ServiceCredential {
        service_id: "api-gateway".to_string(),
        service_name: "API Gateway".to_string(),
        secret_key: "secret123".to_string(),
        created_at: Utc::now(),
        permissions: vec!["read".to_string(), "write".to_string()],
    };
    manager.register_service(credential).await;
    
    // 验证服务
    let verified = manager.verify_service("api-gateway", "secret123").await?;
    
    // 创建 Session
    let session = manager.create_session(
        "user123".to_string(),
        "token456".to_string(),
    ).await?;
    
    // 设置 Session 属性
    manager.set_attribute(
        &session.session_id,
        "role".to_string(),
        "admin".to_string(),
    ).await?;
    
    Ok(())
}
```

### 服务认证流程

```text
服务 A                      管理器                     服务 B
   |                           |                           |
   |-- 注册服务 ------------->|                           |
   |<----- 已注册 ------------|                           |
   |                           |                           |
   |                           |<-- 验证服务(id, secret) --|
   |                           |--- 检查凭证 ------------->|
   |                           |<----- 已验证 ------------|
```

---

## ภาษาไทย

### ภาพรวม

โมดูลการจัดการ Distributed Session ช่วยให้สามารถแชร์ session ข้ามไมโครเซอร์วิสหลายตัว มีการยืนยันตัวตนของเซอร์วิส การเข้าถึง session ข้ามเซอร์วิส และการจัดการแอตทริบิวต์พร้อมการจัดการหมดเวลาอัตโนมัติ

### คุณสมบัติหลัก

- **การแชร์ Session ข้ามเซอร์วิส** - แชร์ sessions ข้ามไมโครเซอร์วิส
- **การยืนยันตัวตนเซอร์วิส** - ตรวจสอบข้อมูลรับรองเซอร์วิส
- **แอตทริบิวต์ Session** - เก็บคู่คีย์-ค่าแบบกำหนดเอง
- **รองรับหลาย Session** - ผู้ใช้หนึ่งคนสามารถมีหลาย sessions

### เริ่มต้นอย่างรวดเร็ว

```rust
let manager = DistributedSessionManager::new(
    storage,
    "service-main".to_string(),
    Duration::from_secs(3600),
);

// สร้าง session
let session = manager.create_session(
    "user123".to_string(),
    "token456".to_string(),
).await?;

// ตั้งค่าแอตทริบิวต์
manager.set_attribute(&session.session_id, "role".to_string(), "admin".to_string()).await?;
```

---

## Tiếng Việt

### Tổng quan

Module quản lý Distributed Session cho phép chia sẻ session qua nhiều microservices. Nó cung cấp xác thực dịch vụ, truy cập session liên dịch vụ và quản lý thuộc tính với xử lý timeout tự động.

### Tính năng chính

- **Chia sẻ Session liên dịch vụ** - Chia sẻ sessions qua microservices
- **Xác thực dịch vụ** - Xác minh thông tin xác thực dịch vụ
- **Thuộc tính Session** - Lưu trữ các cặp key-value tùy chỉnh
- **Hỗ trợ nhiều Session** - Một người dùng có thể có nhiều sessions

### Bắt đầu nhanh

```rust
let manager = DistributedSessionManager::new(
    storage,
    "service-main".to_string(),
    Duration::from_secs(3600),
);

let session = manager.create_session("user123".to_string(), "token456".to_string()).await?;

manager.set_attribute(&session.session_id, "role".to_string(), "admin".to_string()).await?;
```

---

## ភាសាខ្មែរ

### ទិដ្ឋភាពទូទៅ

ម៉ូឌុលការគ្រប់គ្រង Distributed Session បើកឱ្យមានការចែករំលែក session ឆ្លងកាត់ microservices ជាច្រើន។ វាផ្តល់នូវការផ្ទៀងផ្ទាត់សេវាកម្ម ការចូលប្រើ session ឆ្លងកាត់សេវាកម្ម និងការគ្រប់គ្រងគុណលក្ខណៈជាមួយនឹងការ្រប់គ្រងការផុតកំណត់ដោយស្វ័យប្រវត្តិ។

```rust
let manager = DistributedSessionManager::new(
    storage,
    "service-main".to_string(),
    Duration::from_secs(3600),
);

let session = manager.create_session("user123".to_string(), "token456".to_string()).await?;
```

---

## Bahasa Melayu

### Gambaran Keseluruhan

Modul Pengurusan Distributed Session membolehkan perkongsian session merentasi pelbagai microservices. Ia menyediakan pengesahan perkhidmatan, akses session merentas perkhidmatan dan pengurusan atribut dengan pengendalian timeout automatik.

### Ciri Utama

- **Perkongsian Session Merentas Perkhidmatan** - Kongsi sessions merentasi microservices
- **Pengesahan Perkhidmatan** - Sahkan kelayakan perkhidmatan
- **Atribut Session** - Simpan pasangan key-value tersuai

### Permulaan Pantas

```rust
let manager = DistributedSessionManager::new(
    storage,
    "service-main".to_string(),
    Duration::from_secs(3600),
);

let session = manager.create_session("user123".to_string(), "token456".to_string()).await?;

manager.set_attribute(&session.session_id, "role".to_string(), "admin".to_string()).await?;
```

---

## မြန်မာဘာသာ

### အကျဉ်းချုပ်

Distributed Session Management module သည် microservices များစွာတွင် session sharing လုပ်နိုင်ပါသည်။ ၎င်းသည် service authentication၊ cross-service session access နှင့် attribute management တို့ကို automatic timeout handling ဖြင့် ပေးပါသည်။

### အဓိက လုပ်ဆောင်ချက်များ

- **Cross-Service Session Sharing** - microservices များတွင် sessions များ sharing လုပ်ရန်
- **Service Authentication** - service credentials များ verify လုပ်ရန်
- **Session Attributes** - custom key-value pairs များ သိမ်းဆည်းရန်

### လျင်မြန်စွာ စတင်ခြင်း

```rust
let manager = DistributedSessionManager::new(
    storage,
    "service-main".to_string(),
    Duration::from_secs(3600),
);

let session = manager.create_session("user123".to_string(), "token456".to_string()).await?;

manager.set_attribute(&session.session_id, "role".to_string(), "admin".to_string()).await?;
```

---

## Use Cases

### 1. Microservices Architecture
Share user sessions across API Gateway, User Service, Order Service, etc.

### 2. Multi-Region Deployment
Synchronize sessions across different geographic regions.

### 3. Load Balancing
Maintain session consistency across multiple server instances.

### 4. Service Mesh
Enable session access across service mesh components.

## Best Practices

1. **Use appropriate TTL** - Set session timeout based on security requirements
2. **Implement cleanup** - Regularly clean up expired sessions
3. **Secure service credentials** - Use strong secret keys for service authentication
4. **Monitor session count** - Track active sessions per user
5. **Use persistent storage** - Implement Redis/Database storage for production

## Related Documentation

- [WebSocket Authentication](./WEBSOCKET_AUTH.md)
- [Online User Management](./ONLINE_USER_MANAGEMENT.md)
- [Event Listener Guide](./EVENT_LISTENER.md)

## License

MIT OR Apache-2.0

