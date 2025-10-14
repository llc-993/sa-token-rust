//! Distributed Session Management Module | 分布式 Session 管理模块
//!
//! # Code Flow Logic | 代码流程逻辑
//!
//! ## English
//! 
//! ### Overview
//! This module provides distributed session management for microservices architecture.
//! It enables session sharing across multiple services with service authentication,
//! session attributes management, and automatic timeout handling.
//! 
//! ### Service Registration Flow
//! ```text
//! 1. Create ServiceCredential
//!    ├─→ service_id: Unique service identifier
//!    ├─→ service_name: Human-readable name
//!    ├─→ secret_key: Service authentication secret
//!    ├─→ permissions: List of permissions
//!    └─→ created_at: Registration timestamp
//!    ↓
//! 2. DistributedSessionManager.register_service(credential)
//!    └─→ Store in service_credentials HashMap
//!    ↓
//! 3. Service Registered Successfully
//! ```
//! 
//! ### Service Authentication Flow
//! ```text
//! 1. Service Requests Access
//!    ├─→ Provide service_id
//!    └─→ Provide secret_key
//!    ↓
//! 2. DistributedSessionManager.verify_service(service_id, secret)
//!    ├─→ Look up service in credentials map
//!    ├─→ Compare secret_key
//!    └─→ Return ServiceCredential if valid
//!    ↓
//! 3. Authentication Result
//!    ├─→ Success: Return ServiceCredential
//!    └─→ Failure: Return PermissionDenied error
//! ```
//! 
//! ### Session Creation Flow
//! ```text
//! 1. User Logs In on Any Service
//!    ↓
//! 2. create_session(login_id, token)
//!    ├─→ Generate unique session_id (UUID)
//!    ├─→ Record service_id (which service created it)
//!    ├─→ Set timestamps (create_time, last_access)
//!    └─→ Initialize empty attributes HashMap
//!    ↓
//! 3. Save to Distributed Storage
//!    ├─→ storage.save_session(session, ttl)
//!    └─→ TTL: session_timeout duration
//!    ↓
//! 4. Return DistributedSession
//! ```
//! 
//! ### Cross-Service Session Access Flow
//! ```text
//! 1. Service A Creates Session
//!    session_id: "uuid-123"
//!    service_id: "service-a"
//!    login_id: "user123"
//!    ↓
//! 2. Service B Accesses Session
//!    get_session("uuid-123")
//!    ↓
//! 3. Retrieve from Distributed Storage
//!    ├─→ storage.get_session(session_id)
//!    └─→ Returns full session data
//!    ↓
//! 4. Service B Reads/Modifies Session
//!    ├─→ Access attributes
//!    ├─→ Update last_access time
//!    └─→ Save back to storage
//! ```
//! 
//! ### Session Attributes Flow
//! ```text
//! 1. set_attribute(session_id, key, value)
//!    ├─→ Get current session
//!    ├─→ session.attributes.insert(key, value)
//!    ├─→ Update last_access timestamp
//!    └─→ Save updated session
//!    ↓
//! 2. get_attribute(session_id, key)
//!    ├─→ Get session from storage
//!    └─→ Return attributes.get(key)
//!    ↓
//! 3. remove_attribute(session_id, key)
//!    ├─→ Get current session
//!    ├─→ session.attributes.remove(key)
//!    └─→ Save updated session
//! ```
//! 
//! ### Session Cleanup Flow
//! ```text
//! 1. delete_session(session_id)
//!    └─→ storage.delete_session(session_id)
//!    ↓
//! 2. delete_all_sessions(login_id)
//!    ├─→ get_sessions_by_login_id(login_id)
//!    └─→ For each session: delete_session(id)
//!    ↓
//! 3. Automatic Cleanup (TTL-based)
//!    └─→ Storage backend expires sessions after timeout
//! ```
//! 
//! ### Multi-Session Management
//! ```text
//! One user can have multiple sessions:
//! 
//! user123:
//!   ├─→ Session 1 (created by service-a, web device)
//!   ├─→ Session 2 (created by service-b, mobile device)
//!   └─→ Session 3 (created by service-c, desktop app)
//! 
//! All sessions share the same login_id but have unique session_ids
//! Each service can access any session via distributed storage
//! ```
//!
//! ## 中文
//! 
//! ### 概述
//! 本模块为微服务架构提供分布式 Session 管理。
//! 它支持多个服务之间的 Session 共享，包括服务认证、Session 属性管理和自动超时处理。
//! 
//! ### 服务注册流程
//! ```text
//! 1. 创建 ServiceCredential
//!    ├─→ service_id: 唯一服务标识符
//!    ├─→ service_name: 可读服务名称
//!    ├─→ secret_key: 服务认证密钥
//!    ├─→ permissions: 权限列表
//!    └─→ created_at: 注册时间戳
//!    ↓
//! 2. DistributedSessionManager.register_service(credential)
//!    └─→ 存储到 service_credentials HashMap
//!    ↓
//! 3. 服务注册成功
//! ```
//! 
//! ### 服务认证流程
//! ```text
//! 1. 服务请求访问
//!    ├─→ 提供 service_id
//!    └─→ 提供 secret_key
//!    ↓
//! 2. DistributedSessionManager.verify_service(service_id, secret)
//!    ├─→ 在凭证映射中查找服务
//!    ├─→ 比较 secret_key
//!    └─→ 如果有效则返回 ServiceCredential
//!    ↓
//! 3. 认证结果
//!    ├─→ 成功: 返回 ServiceCredential
//!    └─→ 失败: 返回 PermissionDenied 错误
//! ```
//! 
//! ### Session 创建流程
//! ```text
//! 1. 用户在任意服务上登录
//!    ↓
//! 2. create_session(login_id, token)
//!    ├─→ 生成唯一 session_id (UUID)
//!    ├─→ 记录 service_id (创建它的服务)
//!    ├─→ 设置时间戳 (create_time, last_access)
//!    └─→ 初始化空的 attributes HashMap
//!    ↓
//! 3. 保存到分布式存储
//!    ├─→ storage.save_session(session, ttl)
//!    └─→ TTL: session_timeout 持续时间
//!    ↓
//! 4. 返回 DistributedSession
//! ```
//! 
//! ### 跨服务 Session 访问流程
//! ```text
//! 1. 服务 A 创建 Session
//!    session_id: "uuid-123"
//!    service_id: "service-a"
//!    login_id: "user123"
//!    ↓
//! 2. 服务 B 访问 Session
//!    get_session("uuid-123")
//!    ↓
//! 3. 从分布式存储获取
//!    ├─→ storage.get_session(session_id)
//!    └─→ 返回完整 session 数据
//!    ↓
//! 4. 服务 B 读取/修改 Session
//!    ├─→ 访问属性
//!    ├─→ 更新 last_access 时间
//!    └─→ 保存回存储
//! ```
//! 
//! ### Session 属性流程
//! ```text
//! 1. set_attribute(session_id, key, value)
//!    ├─→ 获取当前 session
//!    ├─→ session.attributes.insert(key, value)
//!    ├─→ 更新 last_access 时间戳
//!    └─→ 保存更新后的 session
//!    ↓
//! 2. get_attribute(session_id, key)
//!    ├─→ 从存储获取 session
//!    └─→ 返回 attributes.get(key)
//!    ↓
//! 3. remove_attribute(session_id, key)
//!    ├─→ 获取当前 session
//!    ├─→ session.attributes.remove(key)
//!    └─→ 保存更新后的 session
//! ```
//! 
//! ### Session 清理流程
//! ```text
//! 1. delete_session(session_id)
//!    └─→ storage.delete_session(session_id)
//!    ↓
//! 2. delete_all_sessions(login_id)
//!    ├─→ get_sessions_by_login_id(login_id)
//!    └─→ 对每个 session: delete_session(id)
//!    ↓
//! 3. 自动清理 (基于 TTL)
//!    └─→ 存储后端在超时后自动过期 sessions
//! ```
//! 
//! ### 多 Session 管理
//! ```text
//! 一个用户可以有多个 sessions:
//! 
//! user123:
//!   ├─→ Session 1 (service-a 创建, web 设备)
//!   ├─→ Session 2 (service-b 创建, 移动设备)
//!   └─→ Session 3 (service-c 创建, 桌面应用)
//! 
//! 所有 sessions 共享相同的 login_id 但有唯一的 session_ids
//! 每个服务都可以通过分布式存储访问任何 session
//! ```

use crate::error::SaTokenError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use chrono::{DateTime, Utc};

/// Distributed session data structure
/// 分布式 Session 数据结构
///
/// Represents a session that can be shared across multiple services
/// 表示可以在多个服务之间共享的 Session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedSession {
    /// Unique session identifier | 唯一 Session 标识符
    pub session_id: String,
    
    /// User login ID | 用户登录 ID
    pub login_id: String,
    
    /// Authentication token | 认证 Token
    pub token: String,
    
    /// ID of the service that created this session | 创建此 Session 的服务 ID
    pub service_id: String,
    
    /// Session creation time | Session 创建时间
    pub create_time: DateTime<Utc>,
    
    /// Last access time | 最后访问时间
    pub last_access: DateTime<Utc>,
    
    /// Session attributes (key-value pairs) | Session 属性（键值对）
    pub attributes: HashMap<String, String>,
}

/// Service credential for inter-service authentication
/// 服务间认证的服务凭证
///
/// Contains service identification and permission information
/// 包含服务标识和权限信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCredential {
    /// Unique service identifier | 唯一服务标识符
    pub service_id: String,
    
    /// Human-readable service name | 可读的服务名称
    pub service_name: String,
    
    /// Service authentication secret key | 服务认证密钥
    pub secret_key: String,
    
    /// Service registration time | 服务注册时间
    pub created_at: DateTime<Utc>,
    
    /// List of permissions this service has | 该服务拥有的权限列表
    pub permissions: Vec<String>,
}

/// Distributed session storage trait
/// 分布式 Session 存储 trait
///
/// Implement this trait to provide custom storage backends
/// 实现此 trait 以提供自定义存储后端
#[async_trait]
pub trait DistributedSessionStorage: Send + Sync {
    /// Save a session to storage with optional TTL
    /// 保存 Session 到存储，可选 TTL
    ///
    /// # Arguments | 参数
    /// * `session` - Session to save | 要保存的 Session
    /// * `ttl` - Time-to-live duration | 生存时间
    async fn save_session(&self, session: DistributedSession, ttl: Option<Duration>) -> Result<(), SaTokenError>;
    
    /// Get a session from storage
    /// 从存储获取 Session
    ///
    /// # Arguments | 参数
    /// * `session_id` - Session identifier | Session 标识符
    async fn get_session(&self, session_id: &str) -> Result<Option<DistributedSession>, SaTokenError>;
    
    /// Delete a session from storage
    /// 从存储删除 Session
    ///
    /// # Arguments | 参数
    /// * `session_id` - Session identifier | Session 标识符
    async fn delete_session(&self, session_id: &str) -> Result<(), SaTokenError>;
    
    /// Get all sessions for a specific user
    /// 获取特定用户的所有 Sessions
    ///
    /// # Arguments | 参数
    /// * `login_id` - User login ID | 用户登录 ID
    async fn get_sessions_by_login_id(&self, login_id: &str) -> Result<Vec<DistributedSession>, SaTokenError>;
}

/// Distributed session manager
/// 分布式 Session 管理器
///
/// Manages distributed sessions and service authentication
/// 管理分布式 Sessions 和服务认证
pub struct DistributedSessionManager {
    /// Session storage backend | Session 存储后端
    storage: Arc<dyn DistributedSessionStorage>,
    
    /// Current service ID | 当前服务 ID
    service_id: String,
    
    /// Default session timeout | 默认 Session 超时时间
    session_timeout: Duration,
    
    /// Registered service credentials | 已注册的服务凭证
    service_credentials: Arc<tokio::sync::RwLock<HashMap<String, ServiceCredential>>>,
}

impl DistributedSessionManager {
    /// Create a new distributed session manager
    /// 创建新的分布式 Session 管理器
    ///
    /// # Arguments | 参数
    /// * `storage` - Session storage implementation | Session 存储实现
    /// * `service_id` - ID of this service | 此服务的 ID
    /// * `session_timeout` - Default session timeout | 默认 Session 超时时间
    ///
    /// # Example | 示例
    /// ```rust,ignore
    /// let storage = Arc::new(MyDistributedStorage::new());
    /// let manager = DistributedSessionManager::new(
    ///     storage,
    ///     "my-service".to_string(),
    ///     Duration::from_secs(3600),
    /// );
    /// ```
    pub fn new(
        storage: Arc<dyn DistributedSessionStorage>,
        service_id: String,
        session_timeout: Duration,
    ) -> Self {
        Self {
            storage,
            service_id,
            session_timeout,
            service_credentials: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Register a service for inter-service authentication
    /// 注册服务以进行服务间认证
    ///
    /// # Arguments | 参数
    /// * `credential` - Service credential information | 服务凭证信息
    ///
    /// # Example | 示例
    /// ```rust,ignore
    /// let credential = ServiceCredential {
    ///     service_id: "api-gateway".to_string(),
    ///     service_name: "API Gateway".to_string(),
    ///     secret_key: "secret123".to_string(),
    ///     created_at: Utc::now(),
    ///     permissions: vec!["read".to_string(), "write".to_string()],
    /// };
    /// manager.register_service(credential).await;
    /// ```
    pub async fn register_service(&self, credential: ServiceCredential) {
        let mut credentials = self.service_credentials.write().await;
        credentials.insert(credential.service_id.clone(), credential);
    }

    /// Verify a service's credentials
    /// 验证服务的凭证
    ///
    /// # Arguments | 参数
    /// * `service_id` - Service identifier | 服务标识符
    /// * `secret` - Service secret key | 服务密钥
    ///
    /// # Returns | 返回值
    /// * `Ok(ServiceCredential)` - Service authenticated | 服务已认证
    /// * `Err(PermissionDenied)` - Invalid credentials | 凭证无效
    ///
    /// # Example | 示例
    /// ```rust,ignore
    /// match manager.verify_service("api-gateway", "secret123").await {
    ///     Ok(cred) => println!("Service {} verified", cred.service_name),
    ///     Err(e) => println!("Verification failed: {}", e),
    /// }
    /// ```
    pub async fn verify_service(&self, service_id: &str, secret: &str) -> Result<ServiceCredential, SaTokenError> {
        let credentials = self.service_credentials.read().await;
        if let Some(cred) = credentials.get(service_id) {
            if cred.secret_key == secret {
                return Ok(cred.clone());
            }
        }
        Err(SaTokenError::PermissionDenied)
    }

    /// Create a new distributed session
    /// 创建新的分布式 Session
    ///
    /// # Arguments | 参数
    /// * `login_id` - User login ID | 用户登录 ID
    /// * `token` - Authentication token | 认证 Token
    ///
    /// # Returns | 返回值
    /// * `Ok(DistributedSession)` - Session created | Session 已创建
    /// * `Err(SaTokenError)` - Creation failed | 创建失败
    ///
    /// # Example | 示例
    /// ```rust,ignore
    /// let session = manager.create_session(
    ///     "user123".to_string(),
    ///     "token456".to_string(),
    /// ).await?;
    /// println!("Session created: {}", session.session_id);
    /// ```
    pub async fn create_session(
        &self,
        login_id: String,
        token: String,
    ) -> Result<DistributedSession, SaTokenError> {
        let session = DistributedSession {
            session_id: uuid::Uuid::new_v4().to_string(),
            login_id,
            token,
            service_id: self.service_id.clone(),
            create_time: Utc::now(),
            last_access: Utc::now(),
            attributes: HashMap::new(),
        };

        self.storage.save_session(session.clone(), Some(self.session_timeout)).await?;
        Ok(session)
    }

    /// Get a session by ID
    /// 通过 ID 获取 Session
    ///
    /// # Arguments | 参数
    /// * `session_id` - Session identifier | Session 标识符
    ///
    /// # Returns | 返回值
    /// * `Ok(DistributedSession)` - Session found | 找到 Session
    /// * `Err(SessionNotFound)` - Session not found | 未找到 Session
    ///
    /// # Example | 示例
    /// ```rust,ignore
    /// let session = manager.get_session("session-id-123").await?;
    /// println!("User: {}", session.login_id);
    /// ```
    pub async fn get_session(&self, session_id: &str) -> Result<DistributedSession, SaTokenError> {
        self.storage.get_session(session_id).await?
            .ok_or(SaTokenError::SessionNotFound)
    }

    /// Update an existing session
    /// 更新现有 Session
    ///
    /// # Arguments | 参数
    /// * `session` - Updated session data | 更新后的 Session 数据
    ///
    /// # Example | 示例
    /// ```rust,ignore
    /// let mut session = manager.get_session("session-id").await?;
    /// session.attributes.insert("role".to_string(), "admin".to_string());
    /// manager.update_session(session).await?;
    /// ```
    pub async fn update_session(&self, session: DistributedSession) -> Result<(), SaTokenError> {
        self.storage.save_session(session, Some(self.session_timeout)).await
    }

    /// Delete a session
    /// 删除 Session
    ///
    /// # Arguments | 参数
    /// * `session_id` - Session identifier | Session 标识符
    ///
    /// # Example | 示例
    /// ```rust,ignore
    /// manager.delete_session("session-id-123").await?;
    /// ```
    pub async fn delete_session(&self, session_id: &str) -> Result<(), SaTokenError> {
        self.storage.delete_session(session_id).await
    }

    /// Refresh a session (update last access time)
    /// 刷新 Session（更新最后访问时间）
    ///
    /// # Arguments | 参数
    /// * `session_id` - Session identifier | Session 标识符
    ///
    /// # Example | 示例
    /// ```rust,ignore
    /// manager.refresh_session("session-id-123").await?;
    /// ```
    pub async fn refresh_session(&self, session_id: &str) -> Result<(), SaTokenError> {
        let mut session = self.get_session(session_id).await?;
        session.last_access = Utc::now();
        self.update_session(session).await
    }

    /// Set a session attribute
    /// 设置 Session 属性
    ///
    /// # Arguments | 参数
    /// * `session_id` - Session identifier | Session 标识符
    /// * `key` - Attribute key | 属性键
    /// * `value` - Attribute value | 属性值
    ///
    /// # Example | 示例
    /// ```rust,ignore
    /// manager.set_attribute("session-id", "theme".to_string(), "dark".to_string()).await?;
    /// ```
    pub async fn set_attribute(
        &self,
        session_id: &str,
        key: String,
        value: String,
    ) -> Result<(), SaTokenError> {
        let mut session = self.get_session(session_id).await?;
        session.attributes.insert(key, value);
        session.last_access = Utc::now();
        self.update_session(session).await
    }

    /// Get a session attribute
    /// 获取 Session 属性
    ///
    /// # Arguments | 参数
    /// * `session_id` - Session identifier | Session 标识符
    /// * `key` - Attribute key | 属性键
    ///
    /// # Returns | 返回值
    /// * `Some(value)` - Attribute found | 找到属性
    /// * `None` - Attribute not found | 未找到属性
    ///
    /// # Example | 示例
    /// ```rust,ignore
    /// if let Some(theme) = manager.get_attribute("session-id", "theme").await? {
    ///     println!("Theme: {}", theme);
    /// }
    /// ```
    pub async fn get_attribute(
        &self,
        session_id: &str,
        key: &str,
    ) -> Result<Option<String>, SaTokenError> {
        let session = self.get_session(session_id).await?;
        Ok(session.attributes.get(key).cloned())
    }

    /// Remove a session attribute
    /// 移除 Session 属性
    ///
    /// # Arguments | 参数
    /// * `session_id` - Session identifier | Session 标识符
    /// * `key` - Attribute key | 属性键
    ///
    /// # Example | 示例
    /// ```rust,ignore
    /// manager.remove_attribute("session-id", "temp_data").await?;
    /// ```
    pub async fn remove_attribute(
        &self,
        session_id: &str,
        key: &str,
    ) -> Result<(), SaTokenError> {
        let mut session = self.get_session(session_id).await?;
        session.attributes.remove(key);
        session.last_access = Utc::now();
        self.update_session(session).await
    }

    /// Get all sessions for a specific user
    /// 获取特定用户的所有 Sessions
    ///
    /// # Arguments | 参数
    /// * `login_id` - User login ID | 用户登录 ID
    ///
    /// # Returns | 返回值
    /// Vector of sessions | Sessions 向量
    ///
    /// # Example | 示例
    /// ```rust,ignore
    /// let sessions = manager.get_sessions_by_login_id("user123").await?;
    /// println!("User has {} active sessions", sessions.len());
    /// ```
    pub async fn get_sessions_by_login_id(&self, login_id: &str) -> Result<Vec<DistributedSession>, SaTokenError> {
        self.storage.get_sessions_by_login_id(login_id).await
    }

    /// Delete all sessions for a specific user
    /// 删除特定用户的所有 Sessions
    ///
    /// # Arguments | 参数
    /// * `login_id` - User login ID | 用户登录 ID
    ///
    /// # Example | 示例
    /// ```rust,ignore
    /// manager.delete_all_sessions("user123").await?;
    /// ```
    pub async fn delete_all_sessions(&self, login_id: &str) -> Result<(), SaTokenError> {
        let sessions = self.get_sessions_by_login_id(login_id).await?;
        for session in sessions {
            self.delete_session(&session.session_id).await?;
        }
        Ok(())
    }
}

/// In-memory distributed session storage implementation
/// 内存分布式 Session 存储实现
///
/// For testing and development purposes
/// 用于测试和开发目的
pub struct InMemoryDistributedStorage {
    /// Sessions storage: session_id -> DistributedSession
    /// Sessions 存储: session_id -> DistributedSession
    sessions: Arc<tokio::sync::RwLock<HashMap<String, DistributedSession>>>,
    
    /// Login index: login_id -> Vec<session_id>
    /// 登录索引: login_id -> Vec<session_id>
    login_index: Arc<tokio::sync::RwLock<HashMap<String, Vec<String>>>>,
}

impl InMemoryDistributedStorage {
    /// Create a new in-memory storage
    /// 创建新的内存存储
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            login_index: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryDistributedStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DistributedSessionStorage for InMemoryDistributedStorage {
    async fn save_session(&self, session: DistributedSession, _ttl: Option<Duration>) -> Result<(), SaTokenError> {
        let session_id = session.session_id.clone();
        let login_id = session.login_id.clone();
        
        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session);
        
        let mut index = self.login_index.write().await;
        index.entry(login_id)
            .or_insert_with(Vec::new)
            .push(session_id);
        
        Ok(())
    }

    async fn get_session(&self, session_id: &str) -> Result<Option<DistributedSession>, SaTokenError> {
        let sessions = self.sessions.read().await;
        Ok(sessions.get(session_id).cloned())
    }

    async fn delete_session(&self, session_id: &str) -> Result<(), SaTokenError> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.remove(session_id) {
            let mut index = self.login_index.write().await;
            if let Some(session_ids) = index.get_mut(&session.login_id) {
                session_ids.retain(|id| id != session_id);
                if session_ids.is_empty() {
                    index.remove(&session.login_id);
                }
            }
        }
        Ok(())
    }

    async fn get_sessions_by_login_id(&self, login_id: &str) -> Result<Vec<DistributedSession>, SaTokenError> {
        let index = self.login_index.read().await;
        let session_ids = index.get(login_id).cloned().unwrap_or_default();
        
        let sessions = self.sessions.read().await;
        let mut result = Vec::new();
        for session_id in session_ids {
            if let Some(session) = sessions.get(&session_id) {
                result.push(session.clone());
            }
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_distributed_session_manager() {
        let storage = Arc::new(InMemoryDistributedStorage::new());
        let manager = DistributedSessionManager::new(
            storage,
            "service1".to_string(),
            Duration::from_secs(3600),
        );

        let session = manager.create_session(
            "user1".to_string(),
            "token1".to_string(),
        ).await.unwrap();

        let retrieved = manager.get_session(&session.session_id).await.unwrap();
        assert_eq!(retrieved.login_id, "user1");
    }

    #[tokio::test]
    async fn test_session_attributes() {
        let storage = Arc::new(InMemoryDistributedStorage::new());
        let manager = DistributedSessionManager::new(
            storage,
            "service1".to_string(),
            Duration::from_secs(3600),
        );

        let session = manager.create_session(
            "user2".to_string(),
            "token2".to_string(),
        ).await.unwrap();

        manager.set_attribute(
            &session.session_id,
            "key1".to_string(),
            "value1".to_string(),
        ).await.unwrap();

        let value = manager.get_attribute(&session.session_id, "key1").await.unwrap();
        assert_eq!(value, Some("value1".to_string()));
    }

    #[tokio::test]
    async fn test_service_verification() {
        let storage = Arc::new(InMemoryDistributedStorage::new());
        let manager = DistributedSessionManager::new(
            storage,
            "service1".to_string(),
            Duration::from_secs(3600),
        );

        let credential = ServiceCredential {
            service_id: "service2".to_string(),
            service_name: "Service 2".to_string(),
            secret_key: "secret123".to_string(),
            created_at: Utc::now(),
            permissions: vec!["read".to_string(), "write".to_string()],
        };

        manager.register_service(credential.clone()).await;

        let verified = manager.verify_service("service2", "secret123").await.unwrap();
        assert_eq!(verified.service_id, "service2");

        let result = manager.verify_service("service2", "wrong_secret").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_all_sessions() {
        let storage = Arc::new(InMemoryDistributedStorage::new());
        let manager = DistributedSessionManager::new(
            storage,
            "service1".to_string(),
            Duration::from_secs(3600),
        );

        manager.create_session("user3".to_string(), "token1".to_string()).await.unwrap();
        manager.create_session("user3".to_string(), "token2".to_string()).await.unwrap();

        let sessions = manager.get_sessions_by_login_id("user3").await.unwrap();
        assert_eq!(sessions.len(), 2);

        manager.delete_all_sessions("user3").await.unwrap();

        let sessions = manager.get_sessions_by_login_id("user3").await.unwrap();
        assert_eq!(sessions.len(), 0);
    }
}
