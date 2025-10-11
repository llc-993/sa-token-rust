//! Session 管理模块

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Session 对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaSession {
    /// Session ID
    pub id: String,
    
    /// 创建时间
    pub create_time: DateTime<Utc>,
    
    /// 数据存储
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
    
    /// 设置值
    pub fn set<T: Serialize>(&mut self, key: impl Into<String>, value: T) -> Result<(), serde_json::Error> {
        let json_value = serde_json::to_value(value)?;
        self.data.insert(key.into(), json_value);
        Ok(())
    }
    
    /// 获取值
    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        self.data.get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
    
    /// 删除值
    pub fn remove(&mut self, key: &str) -> Option<serde_json::Value> {
        self.data.remove(key)
    }
    
    /// 清空 session
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    /// 检查 key 是否存在
    pub fn has(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}

