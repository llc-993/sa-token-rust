// Author: 金书记
//
//! Session 管理模块

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Session 对象 | Session Object
/// 
/// 用于存储用户会话数据的对象
/// Object for storing user session data
/// 
/// # 字段说明 | Field Description
/// - `id`: Session 唯一标识 | Session unique identifier
/// - `create_time`: 创建时间 | Creation time
/// - `data`: 存储的键值对数据 | Stored key-value data
/// 
/// # 使用示例 | Usage Example
/// 
/// ```rust,ignore
/// let mut session = SaSession::new("session_123");
/// session.set("username", "张三")?;
/// session.set("age", 25)?;
/// 
/// let username: Option<String> = session.get("username");
/// println!("Username: {:?}", username);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaSession {
    /// Session ID
    pub id: String,
    
    /// 创建时间 | Creation time
    pub create_time: DateTime<Utc>,
    
    /// 数据存储 | Data storage
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}

impl SaSession {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            create_time: Utc::now(),
            data: HashMap::new(),
        }
    }
    
    /// 设置值 | Set Value
    /// 
    /// # 参数 | Parameters
    /// - `key`: 键名 | Key name
    /// - `value`: 要存储的值 | Value to store
    /// 
    /// # 返回 | Returns
    /// - `Ok(())`: 设置成功 | Set successfully
    /// - `Err`: 序列化失败 | Serialization failed
    pub fn set<T: Serialize>(&mut self, key: impl Into<String>, value: T) -> Result<(), serde_json::Error> {
        let json_value = serde_json::to_value(value)?;
        self.data.insert(key.into(), json_value);
        Ok(())
    }
    
    /// 获取值 | Get Value
    /// 
    /// # 参数 | Parameters
    /// - `key`: 键名 | Key name
    /// 
    /// # 返回 | Returns
    /// - `Some(value)`: 找到值并成功反序列化 | Found value and deserialized successfully
    /// - `None`: 键不存在或反序列化失败 | Key not found or deserialization failed
    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        self.data.get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
    
    /// 删除值 | Remove Value
    /// 
    /// # 参数 | Parameters
    /// - `key`: 键名 | Key name
    /// 
    /// # 返回 | Returns
    /// 被删除的值，如果键不存在则返回 None
    /// Removed value, or None if key doesn't exist
    pub fn remove(&mut self, key: &str) -> Option<serde_json::Value> {
        self.data.remove(key)
    }
    
    /// 清空 session | Clear Session
    /// 
    /// 删除所有存储的数据 | Remove all stored data
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    /// 检查 key 是否存在 | Check if Key Exists
    /// 
    /// # 参数 | Parameters
    /// - `key`: 键名 | Key name
    /// 
    /// # 返回 | Returns
    /// - `true`: 键存在 | Key exists
    /// - `false`: 键不存在 | Key doesn't exist
    pub fn has(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}
