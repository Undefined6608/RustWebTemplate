/*!
 * Redis 工具演示示例
 * 
 * 演示如何使用项目中集成的 Redis 工具和缓存功能。
 * 
 * 运行方式：
 * ```bash
 * cargo run --example redis_demo
 * ```
 * 
 * 确保 Redis 服务正在运行（端口 6379）
 */

use hello_rust::{config::Config, redis::RedisManager, utils::redis::CacheHelper};
use serde::{Deserialize, Serialize};

/// 用户信息示例结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserInfo {
    id: u32,
    username: String,
    email: String,
    last_login: String,
}

/// 会话信息示例结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
struct SessionInfo {
    user_id: u32,
    login_time: String,
    ip_address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    println!("🚀 Redis 工具演示开始");
    
    // 加载配置
    let config = Config::from_env()?;
    println!("✅ 配置加载完成");
    
    // 创建 Redis 管理器
    let redis_manager = RedisManager::new(&config).await?;
    println!("✅ Redis 连接建立成功");
    
    // 创建缓存辅助工具
    let cache_helper = CacheHelper::new(
        hello_rust::redis::RedisUtils::new(redis_manager.clone())
    );
    println!("✅ 缓存工具初始化完成");
    
    // 演示基础 Redis 操作
    println!("\n📦 演示基础 Redis 操作:");
    demo_basic_operations(&redis_manager).await?;
    
    // 演示用户缓存功能
    println!("\n👤 演示用户缓存功能:");
    demo_user_caching(&cache_helper).await?;
    
    // 演示会话管理
    println!("\n🔐 演示会话管理:");
    demo_session_management(&cache_helper).await?;
    
    // 演示限流功能
    println!("\n🚦 演示限流功能:");
    demo_rate_limiting(&cache_helper).await?;
    
    // 演示验证码功能
    println!("\n📧 演示验证码功能:");
    demo_verification_codes(&cache_helper).await?;
    
    // 演示列表操作
    println!("\n📋 演示列表操作:");
    demo_list_operations(&cache_helper).await?;
    
    // 演示批量操作
    println!("\n⚡ 演示批量操作:");
    demo_batch_operations(&cache_helper).await?;
    
    // 健康检查
    println!("\n💓 Redis 健康检查:");
    let is_healthy = cache_helper.health_check().await?;
    println!("Redis 状态: {}", if is_healthy { "健康 ✅" } else { "异常 ❌" });
    
    println!("\n🎉 Redis 工具演示完成！");
    
    Ok(())
}

/// 演示基础 Redis 操作
async fn demo_basic_operations(redis_manager: &RedisManager) -> Result<(), Box<dyn std::error::Error>> {
    let redis_utils = hello_rust::redis::RedisUtils::new(redis_manager.clone());
    
    // 字符串操作
    println!("  💾 字符串操作:");
    redis_utils.set_string("demo:string", "Hello Redis!", Some(300)).await?;
    let value = redis_utils.get_string("demo:string").await?;
    println!("    设置并获取字符串: {:?}", value);
    
    // JSON 对象操作
    println!("  📄 JSON 对象操作:");
    let user = UserInfo {
        id: 1,
        username: "demo_user".to_string(),
        email: "demo@example.com".to_string(),
        last_login: "2024-01-01 12:00:00".to_string(),
    };
    redis_utils.set_json("demo:user", &user, Some(300)).await?;
    let retrieved_user: Option<UserInfo> = redis_utils.get_json("demo:user").await?;
    println!("    设置并获取 JSON 对象: {:?}", retrieved_user);
    
    // 计数器操作
    println!("  🔢 计数器操作:");
    let count1 = redis_utils.increment("demo:counter", Some(5)).await?;
    let count2 = redis_utils.increment("demo:counter", None).await?;
    println!("    递增 5: {}, 递增 1: {}", count1, count2);
    
    // 列表操作
    println!("  📝 列表操作:");
    redis_utils.list_push_left("demo:list", "item1").await?;
    redis_utils.list_push_left("demo:list", "item2").await?;
    redis_utils.list_push_right("demo:list", "item3").await?;
    let length = redis_utils.list_length("demo:list").await?;
    println!("    列表长度: {}", length);
    
    let popped = redis_utils.list_pop_right("demo:list").await?;
    println!("    弹出项目: {:?}", popped);
    
    // 集合操作
    println!("  🎯 集合操作:");
    redis_utils.set_add("demo:set", "member1").await?;
    redis_utils.set_add("demo:set", "member2").await?;
    redis_utils.set_add("demo:set", "member1").await?; // 重复添加
    let members = redis_utils.set_members("demo:set").await?;
    println!("    集合成员: {:?}", members);
    
    Ok(())
}

/// 演示用户缓存功能
async fn demo_user_caching(cache_helper: &CacheHelper) -> Result<(), Box<dyn std::error::Error>> {
    let user = UserInfo {
        id: 123,
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
        last_login: "2024-01-01 15:30:00".to_string(),
    };
    
    // 缓存用户
    cache_helper.cache_user(user.id, &user, Some(600)).await?;
    println!("  ✅ 用户信息已缓存 (ID: {})", user.id);
    
    // 获取缓存的用户
    let cached_user: Option<UserInfo> = cache_helper.get_cached_user(user.id).await?;
    println!("  📥 获取缓存用户: {:?}", cached_user);
    
    // 清除用户缓存
    let cleared = cache_helper.clear_user_cache(user.id).await?;
    println!("  🗑️  清除用户缓存: {}", if cleared { "成功" } else { "失败" });
    
    // 再次尝试获取（应该为空）
    let cached_user_after_clear: Option<UserInfo> = cache_helper.get_cached_user(user.id).await?;
    println!("  📭 清除后获取用户: {:?}", cached_user_after_clear);
    
    Ok(())
}

/// 演示会话管理
async fn demo_session_management(cache_helper: &CacheHelper) -> Result<(), Box<dyn std::error::Error>> {
    let session_id = "sess_abc123def456";
    let session_info = SessionInfo {
        user_id: 123,
        login_time: "2024-01-01 16:00:00".to_string(),
        ip_address: "192.168.1.100".to_string(),
    };
    
    // 设置会话
    cache_helper.set_session(session_id, &session_info, 1800).await?; // 30分钟
    println!("  ✅ 会话已设置 (ID: {})", session_id);
    
    // 获取会话
    let retrieved_session: Option<SessionInfo> = cache_helper.get_session(session_id).await?;
    println!("  📥 获取会话信息: {:?}", retrieved_session);
    
    // 延长会话
    let extended = cache_helper.extend_session(session_id, 3600).await?; // 延长到1小时
    println!("  ⏰ 延长会话: {}", if extended { "成功" } else { "失败" });
    
    // 删除会话
    let deleted = cache_helper.delete_session(session_id).await?;
    println!("  🗑️  删除会话: {}", if deleted { "成功" } else { "失败" });
    
    Ok(())
}

/// 演示限流功能
async fn demo_rate_limiting(cache_helper: &CacheHelper) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = "user_456";
    let limit = 5;
    let window = 60; // 60秒
    
    println!("  🚦 限流测试 (限制: {}/{}秒):", limit, window);
    
    // 模拟多次请求
    for i in 1..=7 {
        let allowed = cache_helper.rate_limit(user_id, limit, window).await?;
        let count = cache_helper.get_rate_limit_count(user_id).await?;
        println!("    请求 {}: {} (当前计数: {})", 
                i, 
                if allowed { "允许 ✅" } else { "限制 ❌" }, 
                count);
        
        // 小延迟
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    Ok(())
}

/// 演示验证码功能
async fn demo_verification_codes(cache_helper: &CacheHelper) -> Result<(), Box<dyn std::error::Error>> {
    let email = "user@example.com";
    let code = "123456";
    
    // 设置验证码
    cache_helper.set_verification_code(email, code, 300).await?; // 5分钟有效
    println!("  ✅ 验证码已发送到 {}", email);
    
    // 验证正确的验证码
    let valid = cache_helper.verify_and_consume_code(email, code).await?;
    println!("  🔍 验证正确验证码: {}", if valid { "成功 ✅" } else { "失败 ❌" });
    
    // 再次尝试验证（应该失败，因为已被消费）
    let valid_again = cache_helper.verify_and_consume_code(email, code).await?;
    println!("  🔍 再次验证（已消费）: {}", if valid_again { "成功 ✅" } else { "失败 ❌" });
    
    // 验证错误的验证码
    cache_helper.set_verification_code(email, code, 300).await?;
    let invalid = cache_helper.verify_and_consume_code(email, "654321").await?;
    println!("  🔍 验证错误验证码: {}", if invalid { "成功 ✅" } else { "失败 ❌" });
    
    Ok(())
}

/// 演示列表操作
async fn demo_list_operations(cache_helper: &CacheHelper) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug, Serialize, Deserialize)]
    struct LogEntry {
        timestamp: String,
        message: String,
        level: String,
    }
    
    let log_entries = vec![
        LogEntry {
            timestamp: "2024-01-01 10:00:00".to_string(),
            message: "应用启动".to_string(),
            level: "INFO".to_string(),
        },
        LogEntry {
            timestamp: "2024-01-01 10:01:00".to_string(),
            message: "用户登录".to_string(),
            level: "INFO".to_string(),
        },
        LogEntry {
            timestamp: "2024-01-01 10:02:00".to_string(),
            message: "数据库连接错误".to_string(),
            level: "ERROR".to_string(),
        },
    ];
    
    // 添加日志条目到列表
    for entry in &log_entries {
        cache_helper.add_to_list("demo:logs", entry, Some(10)).await?;
    }
    println!("  ✅ 添加了 {} 条日志条目", log_entries.len());
    
    // 获取最近的日志条目
    let recent_logs: Vec<LogEntry> = cache_helper.get_list_items("demo:logs", 0, 4).await?;
    println!("  📋 最近的日志条目 ({}条):", recent_logs.len());
    for (i, log) in recent_logs.iter().enumerate() {
        println!("    {}: [{}] {} - {}", i + 1, log.level, log.timestamp, log.message);
    }
    
    Ok(())
}

/// 演示批量操作
async fn demo_batch_operations(cache_helper: &CacheHelper) -> Result<(), Box<dyn std::error::Error>> {
    // 批量设置
    let items = vec![
        ("demo:batch:1".to_string(), "value1".to_string()),
        ("demo:batch:2".to_string(), "value2".to_string()),
        ("demo:batch:3".to_string(), "value3".to_string()),
    ];
    
    cache_helper.batch_set(items.clone(), Some(300)).await?;
    println!("  ✅ 批量设置了 {} 个键值对", items.len());
    
    // 批量获取
    let keys: Vec<String> = items.iter().map(|(k, _)| k.clone()).collect();
    let values = cache_helper.batch_get(keys).await?;
    println!("  📥 批量获取结果:");
    for (i, value) in values.iter().enumerate() {
        println!("    键 {}: {:?}", i + 1, value);
    }
    
    Ok(())
}
