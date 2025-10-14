// Author: 金书记
//
//! 事件监听示例
//! 
//! 演示如何使用 sa-token 的事件监听功能

use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
use sa_token_core::{
    SaTokenManager, SaTokenConfig, StpUtil,
    SaTokenListener, LoggingListener, WsAuthManager,
};
use sa_token_storage_memory::MemoryStorage;

/// 自定义监听器 - 记录用户行为
struct UserBehaviorListener {
    websocket_sessions: Arc<tokio::sync::RwLock<HashMap<String, usize>>>,
}

impl UserBehaviorListener {
    fn new() -> Self {
        Self {
            websocket_sessions: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl SaTokenListener for UserBehaviorListener {
    async fn on_login(&self, login_id: &str, token: &str, login_type: &str) {
        if login_type == "websocket" {
            // WebSocket 认证
            let mut sessions = self.websocket_sessions.write().await;
            let count = sessions.entry(login_id.to_string()).or_insert(0);
            *count += 1;
            
            println!("📝 [用户行为记录] WebSocket 连接");
            println!("   - 用户ID: {}", login_id);
            println!("   - Token: {}...", &token[..20.min(token.len())]);
            println!("   - 登录类型: 🌐 {}", login_type);
            println!("   - 该用户的 WebSocket 连接数: {}", *count);
        } else {
            // 普通登录
            println!("📝 [用户行为记录] 用户登录");
            println!("   - 用户ID: {}", login_id);
            println!("   - Token: {}...", &token[..20.min(token.len())]);
            println!("   - 登录类型: {}", login_type);
        }
        
        // 这里可以添加实际的业务逻辑，例如：
        // - 记录登录日志到数据库
        // - 更新用户最后登录时间
        // - 发送登录通知
        // - 统计登录次数
    }

    async fn on_logout(&self, login_id: &str, token: &str, login_type: &str) {
        println!("📝 [用户行为记录] 用户登出");
        println!("   - 用户ID: {}", login_id);
        println!("   - Token: {}", token);
        println!("   - 登录类型: {}", login_type);
        
        // 业务逻辑：
        // - 记录登出日志
        // - 清理用户缓存
        // - 更新在线状态
    }

    async fn on_kick_out(&self, login_id: &str, token: &str, login_type: &str) {
        println!("⚠️  [用户行为记录] 用户被踢出下线");
        println!("   - 用户ID: {}", login_id);
        println!("   - Token: {}", token);
        println!("   - 登录类型: {}", login_type);
        
        // 业务逻辑：
        // - 记录踢出日志
        // - 发送通知给用户
        // - 清理会话数据
    }
}

/// 安全监控监听器 - 监控可疑行为
struct SecurityMonitorListener;

#[async_trait]
impl SaTokenListener for SecurityMonitorListener {
    async fn on_login(&self, login_id: &str, _token: &str, _login_type: &str) {
        // 检查是否存在异常登录
        println!("🔒 [安全监控] 检查登录安全性");
        println!("   - 用户ID: {}", login_id);
        
        // 实际业务逻辑：
        // - 检查登录IP是否在白名单
        // - 检查登录频率是否异常
        // - 检查是否需要二次验证
    }

    async fn on_kick_out(&self, login_id: &str, _token: &str, _login_type: &str) {
        println!("🚨 [安全监控] 用户被强制下线");
        println!("   - 用户ID: {}", login_id);
        
        // 实际业务逻辑：
        // - 记录安全事件
        // - 发送告警通知
        // - 触发安全审计
    }
}

/// 统计监听器 - 统计用户活跃度
struct StatisticsListener {
    login_count: Arc<tokio::sync::RwLock<u64>>,
    logout_count: Arc<tokio::sync::RwLock<u64>>,
}

impl StatisticsListener {
    fn new() -> Self {
        Self {
            login_count: Arc::new(tokio::sync::RwLock::new(0)),
            logout_count: Arc::new(tokio::sync::RwLock::new(0)),
        }
    }

    async fn get_stats(&self) -> (u64, u64) {
        let login_count = *self.login_count.read().await;
        let logout_count = *self.logout_count.read().await;
        (login_count, logout_count)
    }
}

#[async_trait]
impl SaTokenListener for StatisticsListener {
    async fn on_login(&self, _login_id: &str, _token: &str, _login_type: &str) {
        let mut count = self.login_count.write().await;
        *count += 1;
        println!("📊 [统计] 登录次数: {}", *count);
    }

    async fn on_logout(&self, _login_id: &str, _token: &str, _login_type: &str) {
        let mut count = self.logout_count.write().await;
        *count += 1;
        println!("📊 [统计] 登出次数: {}", *count);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== sa-token 事件监听示例 ===\n");
    
    // 1. 创建存储和配置
    let storage = Arc::new(MemoryStorage::new());
    let config = SaTokenConfig::builder()
        .timeout(7200) // 2小时过期
        .build_config();
    
    // 2. 创建管理器
    let manager = SaTokenManager::new(storage, config);
    
    // 3. 注册事件监听器
    println!(">>> 注册事件监听器...\n");
    
    // 方式一：直接通过 manager 注册
    manager.event_bus().register(Arc::new(LoggingListener)).await;
    
    let behavior_listener = Arc::new(UserBehaviorListener::new());
    let behavior_listener_clone: Arc<dyn SaTokenListener> = behavior_listener.clone();
    manager.event_bus().register(behavior_listener_clone).await;
    
    manager.event_bus().register(Arc::new(SecurityMonitorListener)).await;
    
    let stats_listener = Arc::new(StatisticsListener::new());
    let stats_listener_clone: Arc<dyn SaTokenListener> = stats_listener.clone();
    manager.event_bus().register(stats_listener_clone).await;
    
    println!("✅ 已注册 4 个监听器\n");
    
    // 4. 初始化 StpUtil
    StpUtil::init_manager(manager.clone());
    
    // 方式二：通过 StpUtil 注册（如果还有其他监听器）
    // StpUtil::register_listener(Arc::new(AnotherListener)).await;
    
    // 5. 测试登录事件
    println!("\n========================================");
    println!(">>> 测试1: 用户登录");
    println!("========================================\n");
    
    let token1 = StpUtil::login("user_10086").await?;
    println!("\n生成的 Token: {}\n", token1.as_str());
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // 6. 测试第二个用户登录
    println!("\n========================================");
    println!(">>> 测试2: 另一个用户登录");
    println!("========================================\n");
    
    let token2 = StpUtil::login("user_10087").await?;
    println!("\n生成的 Token: {}\n", token2.as_str());
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // 7. 测试登出事件
    println!("\n========================================");
    println!(">>> 测试3: 用户登出");
    println!("========================================\n");
    
    StpUtil::logout(&token1).await?;
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // 8. 测试踢出下线事件
    println!("\n========================================");
    println!(">>> 测试4: 踢出用户下线");
    println!("========================================\n");
    
    StpUtil::kick_out("user_10087").await?;
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // 9. 测试 WebSocket 认证事件
    println!("\n========================================");
    println!(">>> 测试5: WebSocket 认证（触发 Login 事件）");
    println!("========================================\n");
    
    let manager_arc = Arc::new(manager.clone());
    let ws_auth = WsAuthManager::new(manager_arc);
    
    // 先登录获取 token
    let ws_token = StpUtil::login("ws_user_001").await?;
    println!("用户 ws_user_001 已登录\n");
    
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // WebSocket 认证（使用 Authorization Header）
    println!("WebSocket 认证 - 方式1: Authorization Header");
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), format!("Bearer {}", ws_token.as_str()));
    
    let ws_auth_info = ws_auth.authenticate(&headers, &HashMap::new()).await?;
    println!("WebSocket 会话 ID: {}\n", ws_auth_info.session_id);
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // 10. 测试多设备 WebSocket 连接
    println!("\n========================================");
    println!(">>> 测试6: 多设备 WebSocket 连接");
    println!("========================================\n");
    
    println!("同一用户从多个设备连接...\n");
    
    // 设备2: 使用 Query 参数
    println!("WebSocket 认证 - 方式2: Query Parameter");
    let mut query = HashMap::new();
    query.insert("token".to_string(), ws_token.as_str().to_string());
    
    let ws_auth_info2 = ws_auth.authenticate(&HashMap::new(), &query).await?;
    println!("WebSocket 会话 ID: {}\n", ws_auth_info2.session_id);
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // 设备3: 使用 WebSocket Protocol Header
    println!("WebSocket 认证 - 方式3: Sec-WebSocket-Protocol Header");
    let mut headers2 = HashMap::new();
    headers2.insert("Sec-WebSocket-Protocol".to_string(), ws_token.as_str().to_string());
    
    let ws_auth_info3 = ws_auth.authenticate(&headers2, &HashMap::new()).await?;
    println!("WebSocket 会话 ID: {}\n", ws_auth_info3.session_id);
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // 11. 显示 WebSocket 连接统计
    println!("\n========================================");
    println!(">>> WebSocket 连接统计");
    println!("========================================\n");
    
    let ws_sessions = behavior_listener.websocket_sessions.read().await;
    if let Some(count) = ws_sessions.get("ws_user_001") {
        println!("用户 ws_user_001 的 WebSocket 连接数: {}", count);
    }
    println!();
    
    // 12. 显示总体统计信息
    println!("\n========================================");
    println!(">>> 总体统计信息");
    println!("========================================\n");
    
    let (login_count, logout_count) = stats_listener.get_stats().await;
    println!("总登录次数: {} (包括普通登录 + WebSocket 认证)", login_count);
    println!("总登出次数: {}", logout_count);
    
    println!("\n💡 事件监听说明:");
    println!("   • 普通登录: login_type = 'default'");
    println!("   • WebSocket 认证: login_type = 'websocket'");
    println!("   • 监听器可以通过 login_type 区分不同类型的登录");
    println!("   • WebSocket 认证会触发 Login 事件，与普通登录使用相同的事件系统");
    
    println!("\n✅ 事件监听示例运行完成！");
    
    Ok(())
}

