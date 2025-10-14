# WebSocket Authentication Guide | WebSocket 认证指南

[English](#english) | [中文](#中文) | [ภาษาไทย](#ภาษาไทย) | [Tiếng Việt](#tiếng-việt) | [ភាសាខ្មែរ](#ភាសាខ្មែរ) | [Bahasa Melayu](#bahasa-melayu) | [မြန်မာဘာသာ](#မြန်မာဘာသာ)

---

## English

### Overview

The WebSocket Authentication module provides secure authentication for WebSocket connections in sa-token-rust. It supports multiple token extraction methods and integrates seamlessly with the core authentication system.

### Features

- **Multiple Token Sources**
  - Authorization Header (Bearer Token)
  - WebSocket Protocol Header
  - Query Parameters
- **Token Validation** - Automatic expiration checking
- **Session Management** - Unique session IDs for each connection
- **Extensible** - Custom token extractors

### Quick Start

#### 1. Basic Usage

```rust
use sa_token_core::{SaTokenManager, SaTokenConfig, WsAuthManager};
use sa_token_storage_memory::MemoryStorage;
use std::sync::Arc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize manager
    let storage = Arc::new(MemoryStorage::new());
    let config = SaTokenConfig::default();
    let manager = Arc::new(SaTokenManager::new(storage, config));
    
    // Create WebSocket auth manager
    let ws_auth = WsAuthManager::new(manager.clone());
    
    // User logs in
    let token = manager.login("user123").await?;
    
    // Authenticate WebSocket connection
    let mut headers = HashMap::new();
    headers.insert(
        "Authorization".to_string(),
        format!("Bearer {}", token.as_str())
    );
    
    let auth_info = ws_auth.authenticate(&headers, &HashMap::new()).await?;
    
    println!("User {} connected", auth_info.login_id);
    println!("Session ID: {}", auth_info.session_id);
    
    Ok(())
}
```

#### 2. Token from Query Parameter

```rust
// Extract token from URL query parameter
let mut query = HashMap::new();
query.insert("token".to_string(), token.as_str().to_string());

let auth_info = ws_auth.authenticate(&HashMap::new(), &query).await?;
```

#### 3. Custom Token Extractor

```rust
use sa_token_core::WsTokenExtractor;
use async_trait::async_trait;

struct CustomExtractor;

#[async_trait]
impl WsTokenExtractor for CustomExtractor {
    async fn extract_token(
        &self,
        headers: &HashMap<String, String>,
        query: &HashMap<String, String>
    ) -> Option<String> {
        // Custom extraction logic
        headers.get("X-Custom-Token").cloned()
    }
}

// Use custom extractor
let custom_extractor = Arc::new(CustomExtractor);
let ws_auth = WsAuthManager::with_extractor(manager, custom_extractor);
```

### API Reference

#### WsAuthManager

**Methods:**
- `new(manager)` - Create with default extractor
- `with_extractor(manager, extractor)` - Create with custom extractor
- `authenticate(headers, query)` - Authenticate connection
- `verify_token(token)` - Verify token validity
- `refresh_ws_session(auth_info)` - Refresh session

#### WsAuthInfo

**Fields:**
- `login_id` - User identifier
- `token` - Authentication token
- `session_id` - Unique session ID
- `connect_time` - Connection timestamp
- `metadata` - Custom metadata

### Best Practices

1. **Always verify tokens on reconnection**
2. **Use HTTPS/WSS in production**
3. **Implement token refresh for long-lived connections**
4. **Handle token expiration gracefully**
5. **Log authentication events for security auditing**

---

## 中文

### 概述

WebSocket 认证模块为 sa-token-rust 中的 WebSocket 连接提供安全认证。它支持多种 Token 提取方法，并与核心认证系统无缝集成。

### 功能特性

- **多种 Token 来源**
  - Authorization 请求头（Bearer Token）
  - WebSocket Protocol 请求头
  - 查询参数
- **Token 验证** - 自动过期检查
- **会话管理** - 每个连接的唯一会话 ID
- **可扩展** - 自定义 Token 提取器

### 快速开始

#### 1. 基本用法

```rust
use sa_token_core::{SaTokenManager, SaTokenConfig, WsAuthManager};
use sa_token_storage_memory::MemoryStorage;
use std::sync::Arc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化管理器
    let storage = Arc::new(MemoryStorage::new());
    let config = SaTokenConfig::default();
    let manager = Arc::new(SaTokenManager::new(storage, config));
    
    // 创建 WebSocket 认证管理器
    let ws_auth = WsAuthManager::new(manager.clone());
    
    // 用户登录
    let token = manager.login("user123").await?;
    
    // 认证 WebSocket 连接
    let mut headers = HashMap::new();
    headers.insert(
        "Authorization".to_string(),
        format!("Bearer {}", token.as_str())
    );
    
    let auth_info = ws_auth.authenticate(&headers, &HashMap::new()).await?;
    
    println!("用户 {} 已连接", auth_info.login_id);
    println!("会话 ID: {}", auth_info.session_id);
    
    Ok(())
}
```

#### 2. 从查询参数提取 Token

```rust
// 从 URL 查询参数提取 Token
let mut query = HashMap::new();
query.insert("token".to_string(), token.as_str().to_string());

let auth_info = ws_auth.authenticate(&HashMap::new(), &query).await?;
```

#### 3. 自定义 Token 提取器

```rust
use sa_token_core::WsTokenExtractor;
use async_trait::async_trait;

struct CustomExtractor;

#[async_trait]
impl WsTokenExtractor for CustomExtractor {
    async fn extract_token(
        &self,
        headers: &HashMap<String, String>,
        query: &HashMap<String, String>
    ) -> Option<String> {
        // 自定义提取逻辑
        headers.get("X-Custom-Token").cloned()
    }
}

// 使用自定义提取器
let custom_extractor = Arc::new(CustomExtractor);
let ws_auth = WsAuthManager::with_extractor(manager, custom_extractor);
```

### API 参考

#### WsAuthManager

**方法:**
- `new(manager)` - 使用默认提取器创建
- `with_extractor(manager, extractor)` - 使用自定义提取器创建
- `authenticate(headers, query)` - 认证连接
- `verify_token(token)` - 验证 Token 有效性
- `refresh_ws_session(auth_info)` - 刷新会话

#### WsAuthInfo

**字段:**
- `login_id` - 用户标识符
- `token` - 认证 Token
- `session_id` - 唯一会话 ID
- `connect_time` - 连接时间戳
- `metadata` - 自定义元数据

### 最佳实践

1. **始终在重新连接时验证 Token**
2. **在生产环境中使用 HTTPS/WSS**
3. **为长连接实现 Token 刷新**
4. **优雅地处理 Token 过期**
5. **记录认证事件以进行安全审计**

---

## ภาษาไทย

### ภาพรวม

โมดูลการยืนยันตัวตน WebSocket ให้การยืนยันตัวตนที่ปลอดภัยสำหรับการเชื่อมต่อ WebSocket ใน sa-token-rust รองรับหลายวิธีในการดึง Token และผสานรวมได้อย่างราบรื่นกับระบบการยืนยันตัวตนหลัก

### ฟีเจอร์

- **แหล่ง Token หลายแหล่ง**
  - Authorization Header (Bearer Token)
  - WebSocket Protocol Header
  - Query Parameters
- **การตรวจสอบ Token** - ตรวจสอบการหมดอายุอัตโนมัติ
- **การจัดการเซสชัน** - Session ID ที่ไม่ซ้ำกันสำหรับแต่ละการเชื่อมต่อ
- **ขยายได้** - ตัวดึง Token แบบกำหนดเอง

### เริ่มต้นอย่างรวดเร็ว

#### 1. การใช้งานพื้นฐาน

```rust
use sa_token_core::{SaTokenManager, SaTokenConfig, WsAuthManager};
use sa_token_storage_memory::MemoryStorage;
use std::sync::Arc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // เริ่มต้น manager
    let storage = Arc::new(MemoryStorage::new());
    let config = SaTokenConfig::default();
    let manager = Arc::new(SaTokenManager::new(storage, config));
    
    // สร้าง WebSocket auth manager
    let ws_auth = WsAuthManager::new(manager.clone());
    
    // ผู้ใช้ล็อกอิน
    let token = manager.login("user123").await?;
    
    // ยืนยันตัวตนการเชื่อมต่อ WebSocket
    let mut headers = HashMap::new();
    headers.insert(
        "Authorization".to_string(),
        format!("Bearer {}", token.as_str())
    );
    
    let auth_info = ws_auth.authenticate(&headers, &HashMap::new()).await?;
    
    println!("ผู้ใช้ {} เชื่อมต่อแล้ว", auth_info.login_id);
    println!("Session ID: {}", auth_info.session_id);
    
    Ok(())
}
```

### การใช้งาน API

#### WsAuthManager

**เมธอด:**
- `new(manager)` - สร้างด้วยตัวดึงเริ่มต้น
- `with_extractor(manager, extractor)` - สร้างด้วยตัวดึงแบบกำหนดเอง
- `authenticate(headers, query)` - ยืนยันตัวตนการเชื่อมต่อ
- `verify_token(token)` - ตรวจสอบความถูกต้องของ Token
- `refresh_ws_session(auth_info)` - รีเฟรชเซสชัน

---

## Tiếng Việt

### Tổng quan

Module xác thực WebSocket cung cấp xác thực an toàn cho các kết nối WebSocket trong sa-token-rust. Nó hỗ trợ nhiều phương thức trích xuất token và tích hợp liền mạch với hệ thống xác thực cốt lõi.

### Tính năng

- **Nhiều nguồn Token**
  - Authorization Header (Bearer Token)
  - WebSocket Protocol Header
  - Query Parameters
- **Xác thực Token** - Kiểm tra hết hạn tự động
- **Quản lý phiên** - Session ID duy nhất cho mỗi kết nối
- **Có thể mở rộng** - Bộ trích xuất token tùy chỉnh

### Bắt đầu nhanh

#### 1. Sử dụng cơ bản

```rust
use sa_token_core::{SaTokenManager, SaTokenConfig, WsAuthManager};
use sa_token_storage_memory::MemoryStorage;
use std::sync::Arc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Khởi tạo manager
    let storage = Arc::new(MemoryStorage::new());
    let config = SaTokenConfig::default();
    let manager = Arc::new(SaTokenManager::new(storage, config));
    
    // Tạo WebSocket auth manager
    let ws_auth = WsAuthManager::new(manager.clone());
    
    // Người dùng đăng nhập
    let token = manager.login("user123").await?;
    
    // Xác thực kết nối WebSocket
    let mut headers = HashMap::new();
    headers.insert(
        "Authorization".to_string(),
        format!("Bearer {}", token.as_str())
    );
    
    let auth_info = ws_auth.authenticate(&headers, &HashMap::new()).await?;
    
    println!("Người dùng {} đã kết nối", auth_info.login_id);
    println!("Session ID: {}", auth_info.session_id);
    
    Ok(())
}
```

### Tham chiếu API

#### WsAuthManager

**Phương thức:**
- `new(manager)` - Tạo với bộ trích xuất mặc định
- `with_extractor(manager, extractor)` - Tạo với bộ trích xuất tùy chỉnh
- `authenticate(headers, query)` - Xác thực kết nối
- `verify_token(token)` - Xác minh tính hợp lệ của token
- `refresh_ws_session(auth_info)` - Làm mới phiên

---

## ភាសាខ្មែរ

### ទិដ្ឋភាពទូទៅ

ម៉ូឌុលការផ្ទៀងផ្ទាត់ភាពត្រឹមត្រូវ WebSocket ផ្តល់ការផ្ទៀងផ្ទាត់ភាពត្រឹមត្រូវសុវត្ថិភាពសម្រាប់ការតភ្ជាប់ WebSocket ក្នុង sa-token-rust។ វាគាំទ្រវិធីសាស្ត្រច្រើនក្នុងការទាញយក Token និងរួមបញ្ចូលយ៉ាងរលូនជាមួយនឹងប្រព័ន្ធផ្ទៀងផ្ទាត់ភាពត្រឹមត្រូវស្នូល។

### លក្ខណៈពិសេស

- **ប្រភព Token ច្រើន**
  - Authorization Header (Bearer Token)
  - WebSocket Protocol Header
  - Query Parameters
- **ការផ្ទៀងផ្ទាត់ Token** - ពិនិត្យការផុតកំណត់ដោយស្វ័យប្រវត្តិ
- **ការគ្រប់គ្រងសម័យ** - Session ID តែមួយគត់សម្រាប់ការតភ្ជាប់នីមួយៗ
- **អាចពង្រីក** - ឧបករណ៍ទាញយក Token តាមតម្រូវការ

### ចាប់ផ្តើមរហ័ស

```rust
use sa_token_core::{SaTokenManager, SaTokenConfig, WsAuthManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = Arc::new(SaTokenManager::new(storage, config));
    let ws_auth = WsAuthManager::new(manager.clone());
    
    let token = manager.login("user123").await?;
    
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), format!("Bearer {}", token.as_str()));
    
    let auth_info = ws_auth.authenticate(&headers, &HashMap::new()).await?;
    
    println!("អ្នកប្រើប្រាស់ {} បានតភ្ជាប់", auth_info.login_id);
    
    Ok(())
}
```

---

## Bahasa Melayu

### Gambaran Keseluruhan

Modul Pengesahan WebSocket menyediakan pengesahan selamat untuk sambungan WebSocket dalam sa-token-rust. Ia menyokong pelbagai kaedah pengekstrakan token dan berintegrasi dengan lancar dengan sistem pengesahan teras.

### Ciri-ciri

- **Pelbagai Sumber Token**
  - Authorization Header (Bearer Token)
  - WebSocket Protocol Header
  - Query Parameters
- **Pengesahan Token** - Pemeriksaan tamat tempoh automatik
- **Pengurusan Sesi** - Session ID unik untuk setiap sambungan
- **Boleh Dikembangkan** - Pengekstrak token tersuai

### Permulaan Pantas

```rust
use sa_token_core::{SaTokenManager, SaTokenConfig, WsAuthManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = Arc::new(SaTokenManager::new(storage, config));
    let ws_auth = WsAuthManager::new(manager.clone());
    
    let token = manager.login("user123").await?;
    
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), format!("Bearer {}", token.as_str()));
    
    let auth_info = ws_auth.authenticate(&headers, &HashMap::new()).await?;
    
    println!("Pengguna {} telah sambung", auth_info.login_id);
    
    Ok(())
}
```

### Rujukan API

**Kaedah WsAuthManager:**
- `new(manager)` - Cipta dengan pengekstrak lalai
- `authenticate(headers, query)` - Sahkan sambungan
- `verify_token(token)` - Sahkan kesahihan token

---

## မြန်မာဘာသာ

### အကျဉ်းချုပ်

WebSocket Authentication module သည် sa-token-rust တွင် WebSocket connections များအတွက် လုံခြုံသော authentication ပေးပါသည်။ ၎င်းသည် token ထုတ်ယူရန် နည်းလမ်းများစွာကို ပံ့ပိုးပြီး core authentication system နှင့် ချောမွေ့စွာ ပေါင်းစပ်ပါသည်။

### လုပ်ဆောင်ချက်များ

- **Token ရင်းမြစ်များစွာ**
  - Authorization Header (Bearer Token)
  - WebSocket Protocol Header
  - Query Parameters
- **Token အတည်ပြုခြင်း** - အလိုအလျောက် သက်တမ်းကုန်ဆုံးမှု စစ်ဆေးခြင်း
- **Session စီမံခန့်ခွဲမှု** - ချိတ်ဆက်မှုတစ်ခုချင်းစီအတွက် ထူးခြား Session ID
- **တိုးချဲ့နိုင်သော** - စိတ်ကြိုက် token ထုတ်ယူကိရိယာများ

### လျင်မြန်စွာ စတင်ခြင်း

```rust
use sa_token_core::{SaTokenManager, SaTokenConfig, WsAuthManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = Arc::new(SaTokenManager::new(storage, config));
    let ws_auth = WsAuthManager::new(manager.clone());
    
    let token = manager.login("user123").await?;
    
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), format!("Bearer {}", token.as_str()));
    
    let auth_info = ws_auth.authenticate(&headers, &HashMap::new()).await?;
    
    println!("အသုံးပြုသူ {} ချိတ်ဆက်ပြီး", auth_info.login_id);
    
    Ok(())
}
```

### API ကိုးကား

**WsAuthManager နည်းလမ်းများ:**
- `new(manager)` - မူလ extractor ဖြင့် ဖန်တီးရန်
- `authenticate(headers, query)` - ချိတ်ဆက်မှု အတည်ပြုရန်
- `verify_token(token)` - Token တရားဝင်မှု စစ်ဆေးရန်

---

## Related Documentation

- [Online User Management](./ONLINE_USER_MANAGEMENT.md)
- [Distributed Session](./DISTRIBUTED_SESSION.md)
- [Event Listener Guide](./EVENT_LISTENER.md)
- [JWT Guide](./JWT_GUIDE.md)

## License

MIT OR Apache-2.0

