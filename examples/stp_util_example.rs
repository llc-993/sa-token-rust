// Author: 金书记
//
//! StpUtil 使用示例
//! 
//! 演示如何使用 StpUtil 工具类进行认证和权限操作

use std::sync::Arc;
use sa_token_core::{StpUtil, SaTokenConfig, SaTokenManager};
use sa_token_storage_memory::MemoryStorage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 StpUtil 使用示例");
    println!("=".repeat(50));
    
    // 1. 初始化
    let storage = Arc::new(MemoryStorage::new());
    let config = SaTokenConfig::builder()
        .token_name("satoken")
        .timeout(7200)  // 2小时
        .build();
    let manager = SaTokenManager::new(storage, config);
    
    println!("\n1️⃣ 用户登录");
    println!("-".repeat(50));
    
    // 2. 登录
    let user_id = "user_123";
    let token = StpUtil::login(&manager, user_id).await?;
    println!("✅ 用户 {} 登录成功", user_id);
    println!("   Token: {}", token.as_str());
    
    // 3. 检查登录状态
    println!("\n2️⃣ 检查登录状态");
    println!("-".repeat(50));
    let is_login = StpUtil::is_login(&manager, &token).await;
    println!("✅ 是否已登录: {}", if is_login { "是" } else { "否" });
    
    // 4. 获取登录 ID
    println!("\n3️⃣ 获取登录 ID");
    println!("-".repeat(50));
    let login_id = StpUtil::get_login_id(&manager, &token).await?;
    println!("✅ 当前登录 ID: {}", login_id);
    
    // 5. 获取 Token 信息
    println!("\n4️⃣ 获取 Token 信息");
    println!("-".repeat(50));
    let token_info = StpUtil::get_token_info(&manager, &token).await?;
    println!("✅ Token 信息:");
    println!("   - 登录 ID: {}", token_info.login_id);
    println!("   - 创建时间: {}", token_info.create_time);
    println!("   - 登录类型: {}", token_info.login_type);
    
    // 6. Session 操作
    println!("\n5️⃣ Session 操作");
    println!("-".repeat(50));
    
    // 设置 Session 值
    StpUtil::set_session_value(&manager, &login_id, "username", "张三").await?;
    StpUtil::set_session_value(&manager, &login_id, "age", 25).await?;
    println!("✅ 已设置 Session 值");
    
    // 获取 Session 值
    let username: Option<String> = StpUtil::get_session_value(&manager, &login_id, "username").await?;
    let age: Option<i32> = StpUtil::get_session_value(&manager, &login_id, "age").await?;
    println!("✅ Session 值:");
    println!("   - username: {:?}", username);
    println!("   - age: {:?}", age);
    
    // 7. Token 有效期
    println!("\n6️⃣ Token 有效期");
    println!("-".repeat(50));
    if let Some(timeout) = StpUtil::get_token_timeout(&manager, &token).await? {
        println!("✅ Token 剩余有效时间: {} 秒", timeout);
        println!("   约 {} 小时", timeout / 3600);
    } else {
        println!("✅ Token 永久有效");
    }
    
    // 8. 续期 Token
    println!("\n7️⃣ 续期 Token");
    println!("-".repeat(50));
    StpUtil::renew_timeout(&manager, &token, 3600).await?;
    println!("✅ Token 已续期至 1 小时");
    
    if let Some(timeout) = StpUtil::get_token_timeout(&manager, &token).await? {
        println!("   新的剩余时间: {} 秒", timeout);
    }
    
    // 9. 验证登录
    println!("\n8️⃣ 验证登录");
    println!("-".repeat(50));
    match StpUtil::check_login(&manager, &token).await {
        Ok(_) => println!("✅ 登录验证通过"),
        Err(e) => println!("❌ 登录验证失败: {}", e),
    }
    
    // 10. 登出
    println!("\n9️⃣ 登出");
    println!("-".repeat(50));
    StpUtil::logout(&manager, &token).await?;
    println!("✅ 用户已登出");
    
    // 11. 再次检查登录状态
    let is_login_after = StpUtil::is_login(&manager, &token).await;
    println!("✅ 登出后是否已登录: {}", if is_login_after { "是" } else { "否" });
    
    // 12. 尝试验证登录（应该失败）
    println!("\n🔟 验证登录（应该失败）");
    println!("-".repeat(50));
    match StpUtil::check_login(&manager, &token).await {
        Ok(_) => println!("✅ 登录验证通过"),
        Err(e) => println!("❌ 登录验证失败（符合预期）: {}", e),
    }
    
    println!("\n" + "=".repeat(50));
    println!("✅ 所有示例执行完成！");
    
    Ok(())
}
