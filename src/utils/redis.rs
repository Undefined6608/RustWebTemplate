/*!
 * Redis 工具函数模块
 *
 * 提供基于 RedisUtils 的高级缓存功能和常用操作。
 */

use crate::{redis::RedisUtils, AppError, Result};
use serde::{Deserialize, Serialize};

/// 缓存前缀常量
pub mod cache_keys {
    /// 用户缓存前缀
    pub const USER_PREFIX: &str = "user:";
    /// 会话缓存前缀  
    pub const SESSION_PREFIX: &str = "session:";
    /// 限流缓存前缀
    pub const RATE_LIMIT_PREFIX: &str = "rate_limit:";
    /// 临时验证码前缀
    pub const VERIFICATION_PREFIX: &str = "verification:";
}

/// 缓存辅助工具结构体
pub struct CacheHelper {
    redis_utils: RedisUtils,
}

impl CacheHelper {
    /// 创建新的缓存辅助工具实例
    pub fn new(redis_utils: RedisUtils) -> Self {
        Self { redis_utils }
    }

    /// 缓存用户信息
    ///
    /// # 参数
    ///
    /// * `user_id` - 用户ID
    /// * `user_data` - 用户数据
    /// * `ttl_seconds` - 缓存时间（秒），None表示使用默认过期时间
    ///
    /// # 返回值
    ///
    /// 返回 `Result<(), AppError>`
    pub async fn cache_user<T>(
        &self,
        user_id: u32,
        user_data: &T,
        ttl_seconds: Option<u64>,
    ) -> Result<()>
    where
        T: Serialize,
    {
        let key = format!("{}{}", cache_keys::USER_PREFIX, user_id);
        self.redis_utils.set_json(key, user_data, ttl_seconds).await
    }

    /// 获取缓存的用户信息
    ///
    /// # 参数
    ///
    /// * `user_id` - 用户ID
    ///
    /// # 返回值
    ///
    /// 返回 `Result<Option<T>, AppError>`
    pub async fn get_cached_user<T>(&self, user_id: u32) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let key = format!("{}{}", cache_keys::USER_PREFIX, user_id);
        self.redis_utils.get_json(key).await
    }

    /// 清除用户缓存
    ///
    /// # 参数
    ///
    /// * `user_id` - 用户ID
    ///
    /// # 返回值
    ///
    /// 返回 `Result<bool, AppError>`
    pub async fn clear_user_cache(&self, user_id: u32) -> Result<bool> {
        let key = format!("{}{}", cache_keys::USER_PREFIX, user_id);
        self.redis_utils.delete(key).await
    }

    /// 设置会话信息
    ///
    /// # 参数
    ///
    /// * `session_id` - 会话ID
    /// * `session_data` - 会话数据
    /// * `ttl_seconds` - 会话过期时间（秒）
    ///
    /// # 返回值
    ///
    /// 返回 `Result<(), AppError>`
    pub async fn set_session<T>(
        &self,
        session_id: &str,
        session_data: &T,
        ttl_seconds: u64,
    ) -> Result<()>
    where
        T: Serialize,
    {
        let key = format!("{}{}", cache_keys::SESSION_PREFIX, session_id);
        self.redis_utils
            .set_json(key, session_data, Some(ttl_seconds))
            .await
    }

    /// 获取会话信息
    ///
    /// # 参数
    ///
    /// * `session_id` - 会话ID
    ///
    /// # 返回值
    ///
    /// 返回 `Result<Option<T>, AppError>`
    pub async fn get_session<T>(&self, session_id: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let key = format!("{}{}", cache_keys::SESSION_PREFIX, session_id);
        self.redis_utils.get_json(key).await
    }

    /// 删除会话
    ///
    /// # 参数
    ///
    /// * `session_id` - 会话ID
    ///
    /// # 返回值
    ///
    /// 返回 `Result<bool, AppError>`
    pub async fn delete_session(&self, session_id: &str) -> Result<bool> {
        let key = format!("{}{}", cache_keys::SESSION_PREFIX, session_id);
        self.redis_utils.delete(key).await
    }

    /// 延长会话过期时间
    ///
    /// # 参数
    ///
    /// * `session_id` - 会话ID
    /// * `ttl_seconds` - 新的过期时间（秒）
    ///
    /// # 返回值
    ///
    /// 返回 `Result<bool, AppError>`
    pub async fn extend_session(&self, session_id: &str, ttl_seconds: u64) -> Result<bool> {
        let key = format!("{}{}", cache_keys::SESSION_PREFIX, session_id);
        self.redis_utils.expire(key, ttl_seconds).await
    }

    /// 实现限流功能
    ///
    /// # 参数
    ///
    /// * `identifier` - 限流标识符（如用户ID、IP地址等）
    /// * `limit` - 限制次数
    /// * `window_seconds` - 时间窗口（秒）
    ///
    /// # 返回值
    ///
    /// 返回 `Result<bool, AppError>` - true表示允许请求，false表示超出限制
    pub async fn rate_limit(
        &self,
        identifier: &str,
        limit: i64,
        window_seconds: u64,
    ) -> Result<bool> {
        let key = format!("{}{}", cache_keys::RATE_LIMIT_PREFIX, identifier);

        // 获取当前计数
        let current = self.redis_utils.increment(&key, None).await?;

        // 如果是第一次请求，设置过期时间
        if current == 1 {
            self.redis_utils.expire(&key, window_seconds).await?;
        }

        // 检查是否超出限制
        Ok(current <= limit)
    }

    /// 获取当前限流计数
    ///
    /// # 参数
    ///
    /// * `identifier` - 限流标识符
    ///
    /// # 返回值
    ///
    /// 返回 `Result<i64, AppError>` - 当前计数
    pub async fn get_rate_limit_count(&self, identifier: &str) -> Result<i64> {
        let key = format!("{}{}", cache_keys::RATE_LIMIT_PREFIX, identifier);

        if let Some(count_str) = self.redis_utils.get_string(&key).await? {
            count_str.parse::<i64>().map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to parse rate limit count: {}", e))
            })
        } else {
            Ok(0)
        }
    }

    /// 设置验证码
    ///
    /// # 参数
    ///
    /// * `identifier` - 标识符（如邮箱、手机号等）
    /// * `code` - 验证码
    /// * `ttl_seconds` - 验证码有效期（秒）
    ///
    /// # 返回值
    ///
    /// 返回 `Result<(), AppError>`
    pub async fn set_verification_code(
        &self,
        identifier: &str,
        code: &str,
        ttl_seconds: u64,
    ) -> Result<()> {
        let key = format!("{}{}", cache_keys::VERIFICATION_PREFIX, identifier);
        self.redis_utils
            .set_string(key, code, Some(ttl_seconds))
            .await
    }

    /// 验证并消费验证码
    ///
    /// # 参数
    ///
    /// * `identifier` - 标识符
    /// * `code` - 要验证的验证码
    ///
    /// # 返回值
    ///
    /// 返回 `Result<bool, AppError>` - true表示验证通过，false表示验证失败
    pub async fn verify_and_consume_code(&self, identifier: &str, code: &str) -> Result<bool> {
        let key = format!("{}{}", cache_keys::VERIFICATION_PREFIX, identifier);

        if let Some(stored_code) = self.redis_utils.get_string(&key).await? {
            if stored_code == code {
                // 验证成功，删除验证码
                self.redis_utils.delete(key).await?;
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    /// 添加到列表缓存（如活动日志、消息队列等）
    ///
    /// # 参数
    ///
    /// * `list_key` - 列表键名
    /// * `item` - 要添加的项目
    /// * `max_length` - 列表最大长度，超出时会移除旧项目
    ///
    /// # 返回值
    ///
    /// 返回 `Result<(), AppError>`
    pub async fn add_to_list<T>(
        &self,
        list_key: &str,
        item: &T,
        max_length: Option<u32>,
    ) -> Result<()>
    where
        T: Serialize,
    {
        let json_item = serde_json::to_string(item)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("JSON serialization failed: {}", e)))?;

        // 添加到列表左端
        self.redis_utils.list_push_left(list_key, json_item).await?;

        // 如果设置了最大长度，则修剪列表
        if let Some(max_len) = max_length {
            use redis::AsyncCommands;
            let mut conn = self.redis_utils.manager.connection().clone();
            let _: () = conn
                .ltrim(list_key, 0, (max_len as isize) - 1)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis ltrim failed: {}", e)))?;
        }

        Ok(())
    }

    /// 从列表缓存获取项目
    ///
    /// # 参数
    ///
    /// * `list_key` - 列表键名
    /// * `start` - 开始位置（0开始）
    /// * `end` - 结束位置（-1表示到最后）
    ///
    /// # 返回值
    ///
    /// 返回 `Result<Vec<T>, AppError>`
    pub async fn get_list_items<T>(&self, list_key: &str, start: i64, end: i64) -> Result<Vec<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        use redis::AsyncCommands;

        let mut conn = self.redis_utils.manager.connection().clone();
        let items: Vec<String> = conn
            .lrange(list_key, start as isize, end as isize)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis lrange failed: {}", e)))?;

        let mut result = Vec::new();
        for item_str in items {
            let item: T = serde_json::from_str(item_str.as_str()).map_err(|e| {
                AppError::Internal(anyhow::anyhow!("JSON deserialization failed: {}", e))
            })?;
            result.push(item);
        }

        Ok(result)
    }

    /// 批量设置缓存
    ///
    /// # 参数
    ///
    /// * `items` - 键值对列表
    /// * `ttl_seconds` - 过期时间（秒）
    ///
    /// # 返回值
    ///
    /// 返回 `Result<(), AppError>`
    pub async fn batch_set(
        &self,
        items: Vec<(String, String)>,
        ttl_seconds: Option<u64>,
    ) -> Result<()> {
        use redis::AsyncCommands;

        let mut conn = self.redis_utils.manager.connection().clone();

        for (key, value) in items {
            if let Some(seconds) = ttl_seconds {
                let _: () = conn.set_ex(key, value, seconds).await.map_err(|e| {
                    AppError::Internal(anyhow::anyhow!("Redis batch set_ex failed: {}", e))
                })?;
            } else {
                let _: () = conn.set(key, value).await.map_err(|e| {
                    AppError::Internal(anyhow::anyhow!("Redis batch set failed: {}", e))
                })?;
            }
        }

        Ok(())
    }

    /// 批量获取缓存
    ///
    /// # 参数
    ///
    /// * `keys` - 键列表
    ///
    /// # 返回值
    ///
    /// 返回 `Result<Vec<Option<String>>, AppError>`
    pub async fn batch_get(&self, keys: Vec<String>) -> Result<Vec<Option<String>>> {
        use redis::AsyncCommands;

        let mut conn = self.redis_utils.manager.connection().clone();
        let values: Vec<Option<String>> = conn
            .mget(keys)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis mget failed: {}", e)))?;

        Ok(values)
    }

    /// 检查缓存健康状态
    ///
    /// # 返回值
    ///
    /// 返回 `Result<bool, AppError>` - true表示Redis连接正常
    pub async fn health_check(&self) -> Result<bool> {
        // 通过简单的设置和删除操作来检查连接状态
        let test_key = "health_check_test";
        let test_value = "ok";

        match self
            .redis_utils
            .set_string(test_key, test_value, Some(1))
            .await
        {
            Ok(_) => {
                // 尝试删除测试键
                let _ = self.redis_utils.delete(test_key).await;
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }
}
