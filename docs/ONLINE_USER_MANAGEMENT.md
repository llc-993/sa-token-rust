# Online User Management & Real-time Push | 在线用户管理与实时推送

[English](#english) | [中文](#中文) | [ภาษาไทย](#ภาษาไทย) | [Tiếng Việt](#tiếng-việt) | [ភាសាខ្មែរ](#ភាសាខ្មែរ) | [Bahasa Melayu](#bahasa-melayu) | [မြန်မာဘာသာ](#မြန်မာဘာသာ)

---

## English

### Overview

The Online User Management module provides real-time tracking of user online status and message push capabilities. Perfect for building chat applications, live notifications, and real-time collaboration tools.

### Key Features

- **Online Status Tracking** - Track user connections in real-time
- **Multi-Device Support** - Users can connect from multiple devices
- **Real-time Push** - Send messages to specific users or broadcast to all
- **Kick-Out Notifications** - Force logout with notifications
- **Activity Tracking** - Monitor user activity timestamps
- **Extensible Pushers** - Implement custom push mechanisms

### Quick Start

```rust
use sa_token_core::{OnlineManager, OnlineUser, InMemoryPusher};
use std::sync::Arc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create online manager
    let manager = Arc::new(OnlineManager::new());
    
    // Register message pusher
    let pusher = Arc::new(InMemoryPusher::new());
    manager.register_pusher(pusher.clone()).await;
    
    // Mark user as online
    let user = OnlineUser {
        login_id: "user123".to_string(),
        token: "token123".to_string(),
        device: "web".to_string(),
        connect_time: chrono::Utc::now(),
        last_activity: chrono::Utc::now(),
        metadata: HashMap::new(),
    };
    manager.mark_online(user).await;
    
    // Push message to user
    manager.push_to_user("user123", "Hello!".to_string()).await?;
    
    // Broadcast to all users
    manager.broadcast("System announcement".to_string()).await?;
    
    // Check online status
    if manager.is_online("user123").await {
        println!("User is online");
    }
    
    Ok(())
}
```

### API Reference

#### OnlineManager Methods

- `new()` - Create manager
- `mark_online(user)` - Mark user online
- `mark_offline(login_id, token)` - Mark specific session offline
- `mark_offline_all(login_id)` - Mark all user sessions offline
- `is_online(login_id)` - Check if user is online
- `get_online_count()` - Get total online users
- `get_online_users()` - Get list of online user IDs
- `push_to_user(login_id, content)` - Push to single user
- `push_to_users(login_ids, content)` - Push to multiple users
- `broadcast(content)` - Push to all online users
- `kick_out_notify(login_id, reason)` - Force logout with notification

### Message Types

- `MessageType::Text` - Plain text
- `MessageType::Binary` - Binary data
- `MessageType::KickOut` - Logout notification
- `MessageType::Notification` - System notification
- `MessageType::Custom(String)` - Custom type

---

## 中文

### 概述

在线用户管理模块提供实时的用户在线状态跟踪和消息推送功能。非常适合构建聊天应用、实时通知和实时协作工具。

### 核心功能

- **在线状态跟踪** - 实时跟踪用户连接
- **多设备支持** - 用户可从多个设备连接
- **实时推送** - 向特定用户或所有用户发送消息
- **强制下线通知** - 强制登出并发送通知
- **活动跟踪** - 监控用户活动时间戳
- **可扩展推送器** - 实现自定义推送机制

### 快速开始

```rust
use sa_token_core::{OnlineManager, OnlineUser, InMemoryPusher};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建在线管理器
    let manager = Arc::new(OnlineManager::new());
    
    // 注册消息推送器
    let pusher = Arc::new(InMemoryPusher::new());
    manager.register_pusher(pusher.clone()).await;
    
    // 标记用户上线
    let user = OnlineUser {
        login_id: "user123".to_string(),
        token: "token123".to_string(),
        device: "web".to_string(),
        connect_time: chrono::Utc::now(),
        last_activity: chrono::Utc::now(),
        metadata: HashMap::new(),
    };
    manager.mark_online(user).await;
    
    // 推送消息给用户
    manager.push_to_user("user123", "你好！".to_string()).await?;
    
    // 广播给所有用户
    manager.broadcast("系统公告".to_string()).await?;
    
    Ok(())
}
```

### API 参考

#### OnlineManager 方法

- `new()` - 创建管理器
- `mark_online(user)` - 标记用户上线
- `mark_offline(login_id, token)` - 标记特定会话离线
- `mark_offline_all(login_id)` - 标记用户所有会话离线
- `is_online(login_id)` - 检查用户是否在线
- `get_online_count()` - 获取在线用户总数
- `push_to_user(login_id, content)` - 推送给单个用户
- `broadcast(content)` - 推送给所有在线用户
- `kick_out_notify(login_id, reason)` - 强制登出并通知

---

## ภาษาไทย

### ภาพรวม

โมดูลการจัดการผู้ใช้ออนไลน์ให้การติดตามสถานะออนไลน์แบบเรียลไทม์และความสามารถในการส่งข้อความ เหมาะสำหรับการสร้างแอปพลิเคชันแชท การแจ้งเตือนสด และเครื่องมือทำงานร่วมกันแบบเรียลไทม์

### คุณสมบัติหลัก

- **ติดตามสถานะออนไลน์** - ติดตามการเชื่อมต่อของผู้ใช้แบบเรียลไทม์
- **รองรับหลายอุปกรณ์** - ผู้ใช้สามารถเชื่อมต่อจากหลายอุปกรณ์
- **การส่งแบบเรียลไทม์** - ส่งข้อความไปยังผู้ใช้เฉพาะหรือกระจายไปยังทุกคน
- **การแจ้งเตือนการถีบออก** - บังคับออกจากระบบพร้อมการแจ้งเตือน

### เริ่มต้นอย่างรวดเร็ว

```rust
use sa_token_core::{OnlineManager, OnlineUser};

let manager = Arc::new(OnlineManager::new());

// ทำเครื่องหมายผู้ใช้เป็นออนไลน์
manager.mark_online(user).await;

// ส่งข้อความไปยังผู้ใช้
manager.push_to_user("user123", "สวัสดี!".to_string()).await?;

// กระจายไปยังผู้ใช้ทั้งหมด
manager.broadcast("ประกาศระบบ".to_string()).await?;
```

---

## Tiếng Việt

### Tổng quan

Module quản lý người dùng trực tuyến cung cấp theo dõi trạng thái trực tuyến theo thời gian thực và khả năng đẩy tin nhắn. Hoàn hảo để xây dựng ứng dụng trò chuyện, thông báo trực tiếp và công cụ cộng tác thời gian thực.

### Tính năng chính

- **Theo dõi trạng thái trực tuyến** - Theo dõi kết nối người dùng theo thời gian thực
- **Hỗ trợ nhiều thiết bị** - Người dùng có thể kết nối từ nhiều thiết bị
- **Đẩy thời gian thực** - Gửi tin nhắn đến người dùng cụ thể hoặc phát sóng cho tất cả
- **Thông báo đá ra** - Buộc đăng xuất với thông báo

### Bắt đầu nhanh

```rust
use sa_token_core::{OnlineManager, OnlineUser};

let manager = Arc::new(OnlineManager::new());

// Đánh dấu người dùng trực tuyến
manager.mark_online(user).await;

// Đẩy tin nhắn đến người dùng
manager.push_to_user("user123", "Xin chào!".to_string()).await?;

// Phát sóng đến tất cả người dùng
manager.broadcast("Thông báo hệ thống".to_string()).await?;
```

---

## ភាសាខ្មែរ

### ទិដ្ឋភាពទូទៅ

ម៉ូឌុលការគ្រប់គ្រងអ្នកប្រើប្រាស់លើបណ្តាញផ្តល់នូវការតាមដានស្ថានភាពលើបណ្តាញតាមពេលវេលាជាក់ស្តែង និងសមត្ថភាពក្នុងការរុញសារ។

### លក្ខណៈពិសេសសំខាន់

- **ការតាមដានស្ថានភាពលើបណ្តាញ** - តាមដានការតភ្ជាប់របស់អ្នកប្រើប្រាស់តាមពេលវេលាជាក់ស្តែង
- **ការគាំទ្រឧបករណ៍ច្រើន** - អ្នកប្រើប្រាស់អាចតភ្ជាប់ពីឧបករណ៍ច្រើន
- **ការរុញតាមពេលវេលាជាក់ស្តែង** - ផ្ញើសារទៅអ្នកប្រើប្រាស់ជាក់លាក់ ឬផ្សព្វផ្សាយទៅគ្រប់គ្នា

```rust
let manager = Arc::new(OnlineManager::new());

manager.mark_online(user).await;
manager.push_to_user("user123", "សួស្តី!".to_string()).await?;
```

---

## Bahasa Melayu

### Gambaran Keseluruhan

Modul Pengurusan Pengguna Dalam Talian menyediakan penjejakan status dalam talian masa nyata dan keupayaan tolak mesej. Sempurna untuk membina aplikasi sembang, pemberitahuan langsung dan alat kerjasama masa nyata.

### Ciri Utama

- **Penjejakan Status Dalam Talian** - Jejak sambungan pengguna secara masa nyata
- **Sokongan Pelbagai Peranti** - Pengguna boleh sambung dari pelbagai peranti
- **Tolak Masa Nyata** - Hantar mesej kepada pengguna tertentu atau siarkan kepada semua
- **Pemberitahuan Tendang Keluar** - Paksa log keluar dengan pemberitahuan

### Permulaan Pantas

```rust
let manager = Arc::new(OnlineManager::new());

manager.mark_online(user).await;
manager.push_to_user("user123", "Hello!".to_string()).await?;
manager.broadcast("Pengumuman sistem".to_string()).await?;
```

---

## မြန်မာဘာသာ

### အကျဉ်းချုပ်

Online User Management module သည် အသုံးပြုသူ၏ online အခြေအနေကို အချိန်နှင့်တပြေးညီ ခြေရာခံပြီး message push လုပ်ဆောင်ချက်များ ပေးပါသည်။ Chat applications၊ live notifications နှင့် real-time collaboration tools များ တည်ဆောက်ရန် အကောင်းဆုံးဖြစ်ပါသည်။

### အဓိက လုပ်ဆောင်ချက်များ

- **Online အခြေအနေ ခြေရာခံခြင်း** - အသုံးပြုသူချိတ်ဆက်မှုများကို အချိန်နှင့်တပြေးညီ ခြေရာခံရန်
- **ပေါင်းစပ်ကိရိယာများစွာ ပံ့ပိုးမှု** - အသုံးပြုသူများသည် ကိရိယာများစွာမှ ချိတ်ဆက်နိုင်ပါသည်
- **အချိန်နှင့်တပြေးညီ Push** - သီးခြားအသုံးပြုသူများ သို့ message ပို့ရန် သို့မဟုတ် အားလုံးသို့ ထုတ်လွှင့်ရန်

### လျင်မြန်စွာစတင်ခြင်း

```rust
let manager = Arc::new(OnlineManager::new());

manager.mark_online(user).await;
manager.push_to_user("user123", "မင်္ဂလာပါ!".to_string()).await?;
manager.broadcast("စနစ်ကြေငြာချက်".to_string()).await?;
```

---

## Related Documentation

- [WebSocket Authentication](./WEBSOCKET_AUTH.md)
- [Distributed Session](./DISTRIBUTED_SESSION.md)
- [Event Listener Guide](./EVENT_LISTENER.md)

## License

MIT OR Apache-2.0

