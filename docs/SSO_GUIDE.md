# SSO Single Sign-On Guide | SSO 单点登录指南

**Multi-language Documentation | 多语言文档**

Quick navigation | 快速导航:
- [🇬🇧 English](#english)
- [🇨🇳 中文](#中文)
- [🇹🇭 ภาษาไทย](#ภาษาไทย)
- [🇻🇳 Tiếng Việt](#tiếng-việt)
- [🇰🇭 ភាសាខ្មែរ](#ភាសាខ្មែរ)
- [🇲🇾 Bahasa Melayu](#bahasa-melayu)
- [🇲🇲 မြန်မာဘာသာ](#မြန်မာဘာသာ)

---

<a name="english"></a>
## 🇬🇧 English

### Overview

sa-token-rust provides a complete Single Sign-On (SSO) solution based on ticket authentication. Users only need to log in once to access multiple applications seamlessly.

### Key Features

- 🎫 **Ticket-based Authentication**: Secure, one-time use tickets
- 🔐 **Unified Login**: Log in once, access all applications
- 🚪 **Unified Logout**: Log out from all applications at once
- 🌐 **Cross-domain Support**: Configurable origin whitelist
- ⏱️ **Ticket Expiration**: Automatic ticket expiration and cleanup
- 🛡️ **Security Protection**: Service URL matching, replay attack prevention
- 🔄 **Session Management**: Track all logged-in applications

### Core Components

#### 1. SsoServer - SSO Server

The SSO Server is the central authentication service that:
- Manages user authentication
- Generates and validates tickets
- Maintains global session state
- Handles unified logout
- Tracks active client applications

#### 2. SsoClient - SSO Client

Each application acts as an SSO Client that:
- Checks local login status
- Generates login/logout URLs
- Validates tickets from SSO Server
- Creates local sessions
- Handles logout callbacks

#### 3. SsoTicket - Authentication Ticket

A ticket is a short-lived, one-time use authentication token that contains:
- `ticket_id`: Unique ticket identifier (UUID)
- `service`: Target application URL
- `login_id`: User identifier
- `create_time`: Ticket creation time
- `expire_time`: Ticket expiration time
- `used`: Usage status flag

### Architecture Flow

```
┌─────────────┐         ┌─────────────┐         ┌─────────────┐
│   User      │         │ SSO Server  │         │   Client    │
│  Browser    │         │   (Auth)    │         │   App 1     │
└──────┬──────┘         └──────┬──────┘         └──────┬──────┘
       │                       │                       │
       │  1. Access App 1      │                       │
       ├───────────────────────┼──────────────────────>│
       │                       │                       │
       │  2. Redirect to SSO   │                       │
       │<──────────────────────┼───────────────────────┤
       │                       │                       │
       │  3. Login Request     │                       │
       ├──────────────────────>│                       │
       │                       │                       │
       │  4. Create Ticket     │                       │
       │<──────────────────────┤                       │
       │                       │                       │
       │  6. Callback with Ticket                      │
       ├───────────────────────┼──────────────────────>│
       │                       │                       │
       │  7. Validate Ticket   │                       │
       │                       │<──────────────────────┤
       │                       │                       │
       │  8. Ticket Valid      │                       │
       │                       ├──────────────────────>│
       │                       │                       │
       │  9. Create Local Session                      │
       │  10. Access Granted   │                       │
       │<──────────────────────┼───────────────────────┤
```

### Quick Start

#### 1. Basic Setup

```rust
use std::sync::Arc;
use sa_token_core::{SaTokenConfig, SsoServer, SsoClient};
use sa_token_storage_memory::MemoryStorage;

let manager = SaTokenConfig::builder()
    .storage(Arc::new(MemoryStorage::new()))
    .timeout(7200)
    .build();

let manager = Arc::new(manager);
```

#### 2. Create SSO Server

```rust
let sso_server = Arc::new(
    SsoServer::new(manager.clone())
        .with_ticket_timeout(300)  // 5 minutes
);
```

#### 3. Create SSO Clients

```rust
let client1 = Arc::new(SsoClient::new(
    manager.clone(),
    "http://sso.example.com/auth".to_string(),
    "http://app1.example.com".to_string(),
));
```

### Complete Login Flow

#### Step 1: User Logs in at SSO Server

```rust
let ticket = sso_server.login(
    "user_123".to_string(),
    "http://app1.example.com".to_string(),
).await?;
```

#### Step 2: Validate Ticket

```rust
let login_id = sso_server.validate_ticket(
    &ticket.ticket_id,
    "http://app1.example.com",
).await?;
```

#### Step 3: Create Local Session

```rust
let token = client1.login_by_ticket(login_id).await?;
```

### Unified Logout

```rust
let clients = sso_server.logout("user_123").await?;

for client_url in clients {
    // Notify each client to logout
}

client1.handle_logout("user_123").await?;
client2.handle_logout("user_123").await?;
```

### Security Features

**1. One-time Ticket Usage**
```rust
// First validation - succeeds
sso_server.validate_ticket(&ticket_id, service).await?;

// Second validation - fails (ticket already used)
sso_server.validate_ticket(&ticket_id, service).await?; // Error!
```

**2. Service URL Matching**
```rust
// Ticket for App1 cannot be used for App2
sso_server.validate_ticket(&ticket_id, "wrong_service").await?; // ServiceMismatch!
```

### Error Handling

```rust
use sa_token_core::SaTokenError;

match sso_server.validate_ticket(ticket_id, service).await {
    Ok(login_id) => println!("Valid: {}", login_id),
    Err(SaTokenError::InvalidTicket) => println!("Ticket not found"),
    Err(SaTokenError::TicketExpired) => println!("Ticket expired"),
    Err(SaTokenError::ServiceMismatch) => println!("Service mismatch"),
    Err(e) => println!("Other error: {}", e),
}
```

### API Reference

**SsoServer Methods:**
- `new(manager)` - Create new SSO Server
- `with_ticket_timeout(seconds)` - Set ticket expiration time
- `login(login_id, service)` - User login and generate ticket
- `create_ticket(login_id, service)` - Create ticket for logged-in user
- `validate_ticket(ticket_id, service)` - Validate and consume ticket
- `logout(login_id)` - Unified logout
- `is_logged_in(login_id)` - Check if user is logged in
- `get_session(login_id)` - Get user's SSO session
- `get_active_clients(login_id)` - Get list of active clients
- `cleanup_expired_tickets()` - Clean up expired tickets

**SsoClient Methods:**
- `new(manager, server_url, service_url)` - Create new SSO Client
- `with_logout_callback(callback)` - Set logout callback
- `get_login_url()` - Generate login URL
- `get_logout_url()` - Generate logout URL
- `check_local_login(login_id)` - Check local session
- `login_by_ticket(login_id)` - Create local session
- `handle_logout(login_id)` - Handle logout request

### Complete Example

See [sso_example.rs](../examples/sso_example.rs) for a complete working example.

Run the example:
```bash
cargo run --example sso_example
```

### Related Documentation

- [Event Listener Guide](./EVENT_LISTENER.md)
- [WebSocket Authentication](./WEBSOCKET_AUTH.md)
- [Distributed Session](./DISTRIBUTED_SESSION.md)
- [Error Reference](./ERROR_REFERENCE.md)

---

<a name="中文"></a>
## 🇨🇳 中文

### 概述

sa-token-rust 提供了基于票据认证的完整单点登录（SSO）解决方案。用户只需登录一次即可无缝访问多个应用程序。

### 核心特性

- 🎫 **票据认证**：安全的一次性使用票据
- 🔐 **统一登录**：一次登录，访问所有应用
- 🚪 **统一登出**：一次登出，退出所有应用
- 🌐 **跨域支持**：可配置的域名白名单
- ⏱️ **票据过期**：自动票据过期和清理
- 🛡️ **安全保护**：服务URL匹配、防重放攻击
- 🔄 **会话管理**：跟踪所有已登录应用

### 核心组件

#### 1. SsoServer - SSO 服务端

SSO 服务端是中央认证服务，负责：
- 管理用户认证
- 生成和验证票据
- 维护全局会话状态
- 处理统一登出
- 跟踪活跃客户端应用

#### 2. SsoClient - SSO 客户端

每个应用程序作为 SSO 客户端，负责：
- 检查本地登录状态
- 生成登录/登出 URL
- 验证来自 SSO 服务端的票据
- 创建本地会话
- 处理登出回调

#### 3. SsoTicket - 认证票据

票据是一个短期、一次性使用的认证令牌，包含：
- `ticket_id`：唯一票据标识符（UUID）
- `service`：目标应用 URL
- `login_id`：用户标识
- `create_time`：票据创建时间
- `expire_time`：票据过期时间
- `used`：使用状态标记

### 架构流程

```
┌─────────────┐         ┌─────────────┐         ┌─────────────┐
│   用户      │         │ SSO 服务端  │         │   客户端    │
│  浏览器     │         │   (认证)    │         │   应用 1    │
└──────┬──────┘         └──────┬──────┘         └──────┬──────┘
       │                       │                       │
       │  1. 访问应用 1        │                       │
       ├───────────────────────┼──────────────────────>│
       │                       │                       │
       │  2. 重定向到 SSO      │                       │
       │<──────────────────────┼───────────────────────┤
       │                       │                       │
       │  3. 登录请求          │                       │
       ├──────────────────────>│                       │
       │                       │                       │
       │  4. 创建票据          │                       │
       │<──────────────────────┤                       │
       │                       │                       │
       │  6. 带票据回调                                │
       ├───────────────────────┼──────────────────────>│
       │                       │                       │
       │  7. 验证票据          │                       │
       │                       │<──────────────────────┤
       │                       │                       │
       │  8. 票据有效          │                       │
       │                       ├──────────────────────>│
       │                       │                       │
       │  9. 创建本地会话                              │
       │  10. 授予访问权限     │                       │
       │<──────────────────────┼───────────────────────┤
```

### 快速开始

#### 1. 基础设置

```rust
use std::sync::Arc;
use sa_token_core::{SaTokenConfig, SsoServer, SsoClient};
use sa_token_storage_memory::MemoryStorage;

let manager = SaTokenConfig::builder()
    .storage(Arc::new(MemoryStorage::new()))
    .timeout(7200)
    .build();

let manager = Arc::new(manager);
```

#### 2. 创建 SSO 服务端

```rust
let sso_server = Arc::new(
    SsoServer::new(manager.clone())
        .with_ticket_timeout(300)  // 5 分钟
);
```

#### 3. 创建 SSO 客户端

```rust
let client1 = Arc::new(SsoClient::new(
    manager.clone(),
    "http://sso.example.com/auth".to_string(),
    "http://app1.example.com".to_string(),
));
```

### 完整登录流程

#### 步骤 1：用户在 SSO 服务端登录

```rust
let ticket = sso_server.login(
    "user_123".to_string(),
    "http://app1.example.com".to_string(),
).await?;
```

#### 步骤 2：验证票据

```rust
let login_id = sso_server.validate_ticket(
    &ticket.ticket_id,
    "http://app1.example.com",
).await?;
```

#### 步骤 3：创建本地会话

```rust
let token = client1.login_by_ticket(login_id).await?;
```

### 统一登出

```rust
let clients = sso_server.logout("user_123").await?;

for client_url in clients {
    // 通知每个客户端登出
}

client1.handle_logout("user_123").await?;
client2.handle_logout("user_123").await?;
```

### 安全特性

**1. 一次性票据使用**
```rust
// 第一次验证 - 成功
sso_server.validate_ticket(&ticket_id, service).await?;

// 第二次验证 - 失败（票据已使用）
sso_server.validate_ticket(&ticket_id, service).await?; // 错误！
```

**2. 服务 URL 匹配**
```rust
// 应用1的票据不能用于应用2
sso_server.validate_ticket(&ticket_id, "wrong_service").await?; // ServiceMismatch!
```

### 错误处理

```rust
use sa_token_core::SaTokenError;

match sso_server.validate_ticket(ticket_id, service).await {
    Ok(login_id) => println!("有效: {}", login_id),
    Err(SaTokenError::InvalidTicket) => println!("票据未找到"),
    Err(SaTokenError::TicketExpired) => println!("票据已过期"),
    Err(SaTokenError::ServiceMismatch) => println!("服务不匹配"),
    Err(e) => println!("其他错误: {}", e),
}
```

### API 参考

**SsoServer 方法：**
- `new(manager)` - 创建新的 SSO Server
- `with_ticket_timeout(seconds)` - 设置票据过期时间
- `login(login_id, service)` - 用户登录并生成票据
- `create_ticket(login_id, service)` - 为已登录用户创建票据
- `validate_ticket(ticket_id, service)` - 验证并消费票据
- `logout(login_id)` - 统一登出
- `is_logged_in(login_id)` - 检查用户是否已登录
- `get_session(login_id)` - 获取用户的 SSO 会话
- `get_active_clients(login_id)` - 获取活跃客户端列表
- `cleanup_expired_tickets()` - 清理过期票据

**SsoClient 方法：**
- `new(manager, server_url, service_url)` - 创建新的 SSO Client
- `with_logout_callback(callback)` - 设置登出回调
- `get_login_url()` - 生成登录 URL
- `get_logout_url()` - 生成登出 URL
- `check_local_login(login_id)` - 检查本地会话
- `login_by_ticket(login_id)` - 创建本地会话
- `handle_logout(login_id)` - 处理登出请求

### 完整示例

查看 [sso_example.rs](../examples/sso_example.rs) 获取完整的工作示例。

运行示例：
```bash
cargo run --example sso_example
```

### 相关文档

- [事件监听指南](./EVENT_LISTENER.md)
- [WebSocket 认证](./WEBSOCKET_AUTH.md)
- [分布式 Session](./DISTRIBUTED_SESSION.md)
- [错误参考](./ERROR_REFERENCE.md)

---

<a name="ภาษาไทย"></a>
## 🇹🇭 ภาษาไทย

### ภาพรวม

sa-token-rust ให้บริการโซลูชัน Single Sign-On (SSO) แบบสมบูรณ์โดยใช้การตรวจสอบสิทธิ์แบบตั๋ว

### คุณสมบัติหลัก

- 🎫 **การตรวจสอบสิทธิ์แบบตั๋ว**: ตั๋วที่ปลอดภัย ใช้ได้ครั้งเดียว
- 🔐 **เข้าสู่ระบบแบบรวม**: เข้าสู่ระบบครั้งเดียว เข้าถึงทุกแอปพลิเคชัน
- 🚪 **ออกจากระบบแบบรวม**: ออกจากระบบทุกแอปพลิเคชันพร้อมกัน
- 🌐 **รองรับ Cross-domain**: รายการอนุญาต origin ที่กำหนดได้
- ⏱️ **ตั๋วหมดอายุ**: ตั๋วหมดอายุและทำความสะอาดอัตโนมัติ

### การใช้งานเบื้องต้น

```rust
// สร้าง SSO Server
let sso_server = Arc::new(
    SsoServer::new(manager.clone())
        .with_ticket_timeout(300)
);

// สร้าง SSO Client
let client = Arc::new(SsoClient::new(
    manager.clone(),
    "http://sso.example.com/auth".to_string(),
    "http://app1.example.com".to_string(),
));

// ผู้ใช้เข้าสู่ระบบ
let ticket = sso_server.login("user_123".to_string(), service).await?;

// ตรวจสอบตั๋ว
let login_id = sso_server.validate_ticket(&ticket.ticket_id, service).await?;

// สร้าง session ในเครื่อง
let token = client.login_by_ticket(login_id).await?;
```

### การออกจากระบบแบบรวม

```rust
let clients = sso_server.logout("user_123").await?;

for client_url in clients {
    // แจ้งเตือน client ให้ออกจากระบบ
}
```

### ตัวอย่างที่สมบูรณ์

```bash
cargo run --example sso_example
```

---

<a name="tiếng-việt"></a>
## 🇻🇳 Tiếng Việt

### Tổng quan

sa-token-rust cung cấp giải pháp Single Sign-On (SSO) hoàn chỉnh dựa trên xác thực vé.

### Tính năng chính

- 🎫 **Xác thực dựa trên vé**: Vé an toàn, sử dụng một lần
- 🔐 **Đăng nhập thống nhất**: Đăng nhập một lần, truy cập tất cả ứng dụng
- 🚪 **Đăng xuất thống nhất**: Đăng xuất khỏi tất cả ứng dụng cùng lúc
- 🌐 **Hỗ trợ Cross-domain**: Danh sách trắng origin có thể cấu hình
- ⏱️ **Vé hết hạn**: Tự động hết hạn và dọn dẹp vé

### Sử dụng cơ bản

```rust
// Tạo SSO Server
let sso_server = Arc::new(
    SsoServer::new(manager.clone())
        .with_ticket_timeout(300)
);

// Tạo SSO Client
let client = Arc::new(SsoClient::new(
    manager.clone(),
    "http://sso.example.com/auth".to_string(),
    "http://app1.example.com".to_string(),
));

// Người dùng đăng nhập
let ticket = sso_server.login("user_123".to_string(), service).await?;

// Xác thực vé
let login_id = sso_server.validate_ticket(&ticket.ticket_id, service).await?;

// Tạo session cục bộ
let token = client.login_by_ticket(login_id).await?;
```

### Đăng xuất thống nhất

```rust
let clients = sso_server.logout("user_123").await?;

for client_url in clients {
    // Thông báo cho mỗi client đăng xuất
}
```

### Ví dụ hoàn chỉnh

```bash
cargo run --example sso_example
```

---

<a name="ភាសាខ្មែរ"></a>
## 🇰🇭 ភាសាខ្មែរ

### ទិដ្ឋភាពទូទៅ

sa-token-rust ផ្តល់ដំណោះស្រាយ Single Sign-On (SSO) ពេញលេញដោយផ្អែកលើការផ្ទៀងផ្ទាត់សំបុត្រ

### លក្ខណៈពិសេសសំខាន់

- 🎫 **ការផ្ទៀងផ្ទាត់ដោយផ្អែកលើសំបុត្រ**: សំបុត្រសុវត្ថិភាព ប្រើម្តង
- 🔐 **ការចូលរួមបញ្ចូលគ្នា**: ចូលម្តង ចូលដំណើរការកម្មវិធីទាំងអស់
- 🚪 **ការចេញរួមបញ្ចូលគ្នា**: ចេញពីកម្មវិធីទាំងអស់ក្នុងពេលតែមួយ
- 🌐 **ការគាំទ្រ Cross-domain**: បញ្ជីអនុញ្ញាត origin អាចកំណត់បាន
- ⏱️ **ការផុតកំណត់សំបុត្រ**: សំបុត្រផុតកំណត់និងសម្អាតដោយស្វ័យប្រវត្តិ

### ការប្រើប្រាស់មូលដ្ឋាន

```rust
// បង្កើត SSO Server
let sso_server = Arc::new(
    SsoServer::new(manager.clone())
        .with_ticket_timeout(300)
);

// បង្កើត SSO Client
let client = Arc::new(SsoClient::new(
    manager.clone(),
    "http://sso.example.com/auth".to_string(),
    "http://app1.example.com".to_string(),
));

// អ្នកប្រើប្រាស់ចូល
let ticket = sso_server.login("user_123".to_string(), service).await?;

// ផ្ទៀងផ្ទាត់សំបុត្រ
let login_id = sso_server.validate_ticket(&ticket.ticket_id, service).await?;

// បង្កើត session ក្នុងតំបន់
let token = client.login_by_ticket(login_id).await?;
```

### ការចេញរួមបញ្ចូលគ្នា

```rust
let clients = sso_server.logout("user_123").await?;

for client_url in clients {
    // ជូនដំណឹង client នីមួយៗឱ្យចេញ
}
```

### ឧទាហរណ៍ពេញលេញ

```bash
cargo run --example sso_example
```

---

<a name="bahasa-melayu"></a>
## 🇲🇾 Bahasa Melayu

### Gambaran Keseluruhan

sa-token-rust menyediakan penyelesaian Single Sign-On (SSO) lengkap berdasarkan pengesahan tiket.

### Ciri-ciri Utama

- 🎫 **Pengesahan Berasaskan Tiket**: Tiket selamat, guna sekali
- 🔐 **Log Masuk Bersatu**: Log masuk sekali, akses semua aplikasi
- 🚪 **Log Keluar Bersatu**: Log keluar dari semua aplikasi sekaligus
- 🌐 **Sokongan Cross-domain**: Senarai putih origin boleh dikonfigurasi
- ⏱️ **Tamat Tempoh Tiket**: Tiket tamat tempoh dan pembersihan automatik

### Penggunaan Asas

```rust
// Cipta SSO Server
let sso_server = Arc::new(
    SsoServer::new(manager.clone())
        .with_ticket_timeout(300)
);

// Cipta SSO Client
let client = Arc::new(SsoClient::new(
    manager.clone(),
    "http://sso.example.com/auth".to_string(),
    "http://app1.example.com".to_string(),
));

// Pengguna log masuk
let ticket = sso_server.login("user_123".to_string(), service).await?;

// Sahkan tiket
let login_id = sso_server.validate_ticket(&ticket.ticket_id, service).await?;

// Cipta session tempatan
let token = client.login_by_ticket(login_id).await?;
```

### Log Keluar Bersatu

```rust
let clients = sso_server.logout("user_123").await?;

for client_url in clients {
    // Beritahu setiap client untuk log keluar
}
```

### Contoh Lengkap

```bash
cargo run --example sso_example
```

---

<a name="မြန်မာဘာသာ"></a>
## 🇲🇲 မြန်မာဘာသာ

### အကျဉ်းချုပ်

sa-token-rust သည် လက်မှတ်အခြေခံ စစ်မှန်ကြောင်းထောက်ခံချက်ဖြင့် Single Sign-On (SSO) အပြည့်အစုံကို ပံ့ပိုးပေးသည်။

### အဓိကအင်္ဂါရပ်များ

- 🎫 **လက်မှတ်အခြေခံ စစ်မှန်ကြောင်းထောက်ခံချက်**: လုံခြုံသော တစ်ကြိမ်သုံး လက်မှတ်များ
- 🔐 **ပေါင်းစည်း login**: တစ်ကြိမ် login ဝင်ပြီး application အားလုံးကို အသုံးပြု
- 🚪 **ပေါင်းစည်း logout**: application အားလုံးမှ တစ်ပြိုင်နက် logout ထွက်
- 🌐 **Cross-domain ပံ့ပိုးမှု**: ပြင်ဆင်နိုင်သော origin ခွင့်ပြုစာရင်း
- ⏱️ **လက်မှတ်သက်တမ်းကုန်**: အလိုအလျောက် လက်မှတ်သက်တမ်းကုန်နှင့် သန့်ရှင်းရေး

### အခြေခံအသုံးပြုမှု

```rust
// SSO Server ဖန်တီးခြင်း
let sso_server = Arc::new(
    SsoServer::new(manager.clone())
        .with_ticket_timeout(300)
);

// SSO Client ဖန်တီးခြင်း
let client = Arc::new(SsoClient::new(
    manager.clone(),
    "http://sso.example.com/auth".to_string(),
    "http://app1.example.com".to_string(),
));

// အသုံးပြုသူ login ဝင်ခြင်း
let ticket = sso_server.login("user_123".to_string(), service).await?;

// လက်မှတ် စစ်ဆေးခြင်း
let login_id = sso_server.validate_ticket(&ticket.ticket_id, service).await?;

// ဒေသန္တရ session ဖန်တီးခြင်း
let token = client.login_by_ticket(login_id).await?;
```

### ပေါင်းစည်း Logout

```rust
let clients = sso_server.logout("user_123").await?;

for client_url in clients {
    // client တစ်ခုစီကို logout ရန် အကြောင်းကြားပါ
}
```

### အပြည့်အစုံဥပမာ

```bash
cargo run --example sso_example
```

---

## 📖 Additional Resources

- [Main Documentation](../README.md)
- [Examples Directory](../examples/)
- [API Reference](./StpUtil.md)

---

**Version**: 0.1.8  
**Last Updated**: 2025-01-15

