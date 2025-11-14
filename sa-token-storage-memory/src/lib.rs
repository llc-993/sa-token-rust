// Author: 金书记
//
//! # sa-token-storage-memory
//! 
//! 内存存储实现
//! 
//! 适用于：
//! - 开发测试环境
//! - 单机部署
//! - 不需要持久化的场景

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use sa_token_adapter::storage::{SaStorage, StorageResult, StorageError};

/// 内存存储项
#[derive(Debug, Clone)]
struct StorageItem {
    value: String,
    expire_at: Option<DateTime<Utc>>,
}

impl StorageItem {
    fn new(value: String, ttl: Option<Duration>) -> Self {
        let expire_at = ttl.map(|d| Utc::now() + chrono::Duration::from_std(d).unwrap());
        Self { value, expire_at }
    }
    
    fn is_expired(&self) -> bool {
        if let Some(expire_at) = self.expire_at {
            Utc::now() > expire_at
        } else {
            false
        }
    }
}

/// 内存存储实现
#[derive(Debug, Clone)]
pub struct MemoryStorage {
    data: Arc<RwLock<HashMap<String, StorageItem>>>,
}

impl MemoryStorage {
    /// 创建新的内存存储
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// 清理过期的数据
    pub async fn cleanup_expired(&self) {
        let mut data = self.data.write().await;
        data.retain(|_, item| !item.is_expired());
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SaStorage for MemoryStorage {
    async fn get(&self, key: &str) -> StorageResult<Option<String>> {
        let data = self.data.read().await;
        
        if let Some(item) = data.get(key) {
            if item.is_expired() {
                // 数据已过期
                drop(data);
                self.delete(key).await?;
                Ok(None)
            } else {
                Ok(Some(item.value.clone()))
            }
        } else {
            Ok(None)
        }
    }
    
    async fn set(&self, key: &str, value: &str, ttl: Option<Duration>) -> StorageResult<()> {
        let mut data = self.data.write().await;
        let item = StorageItem::new(value.to_string(), ttl);
        data.insert(key.to_string(), item);
        Ok(())
    }
    
    async fn delete(&self, key: &str) -> StorageResult<()> {
        let mut data = self.data.write().await;
        data.remove(key);
        Ok(())
    }
    
    async fn exists(&self, key: &str) -> StorageResult<bool> {
        let data = self.data.read().await;
        if let Some(item) = data.get(key) {
            Ok(!item.is_expired())
        } else {
            Ok(false)
        }
    }
    
    async fn expire(&self, key: &str, ttl: Duration) -> StorageResult<()> {
        let mut data = self.data.write().await;
        if let Some(item) = data.get_mut(key) {
            item.expire_at = Some(Utc::now() + chrono::Duration::from_std(ttl).unwrap());
        }
        Ok(())
    }
    
    async fn ttl(&self, key: &str) -> StorageResult<Option<Duration>> {
        let data = self.data.read().await;
        if let Some(item) = data.get(key) {
            if let Some(expire_at) = item.expire_at {
                let now = Utc::now();
                if expire_at > now {
                    let duration = (expire_at - now).to_std()
                        .map_err(|e| StorageError::InternalError(e.to_string()))?;
                    Ok(Some(duration))
                } else {
                    Ok(Some(Duration::from_secs(0)))
                }
            } else {
                Ok(None) // 永不过期
            }
        } else {
            Ok(None) // 键不存在
        }
    }
    
    async fn clear(&self) -> StorageResult<()> {
        let mut data = self.data.write().await;
        data.clear();
        Ok(())
    }
    
    async fn keys(&self, pattern: &str) -> StorageResult<Vec<String>> {
        let data = self.data.read().await;
        let mut result = Vec::new();
        
        // 将模式转换为正则表达式
        let pattern = pattern.replace("*", ".*");
        let regex = match regex::Regex::new(&pattern) {
            Ok(r) => r,
            Err(e) => return Err(StorageError::OperationFailed(format!("Invalid pattern: {}", e))),
        };
        
        // 筛选匹配的键
        for (key, item) in data.iter() {
            if !item.is_expired() && regex.is_match(key) {
                result.push(key.clone());
            }
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_memory_storage() {
        let storage = MemoryStorage::new();
        
        // 测试设置和获取
        storage.set("key1", "value1", None).await.unwrap();
        let value = storage.get("key1").await.unwrap();
        assert_eq!(value, Some("value1".to_string()));
        
        // 测试删除
        storage.delete("key1").await.unwrap();
        let value = storage.get("key1").await.unwrap();
        assert_eq!(value, None);
        
        // 测试存在性
        storage.set("key2", "value2", None).await.unwrap();
        assert!(storage.exists("key2").await.unwrap());
        assert!(!storage.exists("key3").await.unwrap());
    }
    
    #[tokio::test]
    async fn test_ttl() {
        let storage = MemoryStorage::new();
        
        // 设置带过期时间的键
        storage.set("key1", "value1", Some(Duration::from_secs(1))).await.unwrap();
        
        // 立即获取应该成功
        let value = storage.get("key1").await.unwrap();
        assert_eq!(value, Some("value1".to_string()));
        
        // 等待过期
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        // 过期后应该返回 None
        let value = storage.get("key1").await.unwrap();
        assert_eq!(value, None);
    }
}
