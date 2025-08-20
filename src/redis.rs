/*!
 * Redis 连接管理和工具模块
 *
 * 提供 Redis 连接池管理和常用的缓存操作工具。
 */

use crate::config::Config;
use crate::error::AppError;
use redis::{aio::ConnectionManager, Client, RedisResult};
use serde::{Deserialize, Serialize};

/// Redis 管理器
///
/// 封装 Redis 连接管理器，提供连接池和基础配置
#[derive(Clone)]
pub struct RedisManager {
    /// Redis 连接管理器
    connection_manager: ConnectionManager,
    /// 默认过期时间（秒）
    default_expiry: Option<u64>,
}

impl RedisManager {
    /// 创建新的 Redis 管理器实例
    ///
    /// # 参数
    ///
    /// * `config` - 应用配置，包含 Redis 连接信息
    ///
    /// # 返回值
    ///
    /// 返回 `Result<RedisManager, AppError>`
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use hello_rust::{Config, RedisManager};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = Config::from_env()?;
    ///     let redis_manager = RedisManager::new(&config).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(config: &Config) -> Result<Self, AppError> {
        // 创建 Redis 客户端
        let client = Client::open(config.redis_url.as_str()).map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to create Redis client: {}", e))
        })?;

        // 创建连接管理器
        let connection_manager = client.get_connection_manager().await.map_err(|e| {
            AppError::Internal(anyhow::anyhow!(
                "Failed to create Redis connection manager: {}",
                e
            ))
        })?;

        Ok(RedisManager {
            connection_manager,
            default_expiry: config.redis_default_expiry,
        })
    }

    /// 获取连接管理器的引用
    pub fn connection(&self) -> &ConnectionManager {
        &self.connection_manager
    }

    /// 获取默认过期时间
    pub fn default_expiry(&self) -> Option<u64> {
        self.default_expiry
    }
}

/// Redis 工具结构体
///
/// 提供常用的 Redis 操作方法
pub struct RedisUtils {
    pub manager: RedisManager,
}

impl RedisUtils {
    /// 创建新的 Redis 工具实例
    pub fn new(manager: RedisManager) -> Self {
        Self { manager }
    }

    /// 设置字符串值
    ///
    /// # 参数
    ///
    /// * `key` - 键名
    /// * `value` - 值
    /// * `expiry` - 过期时间（秒），None 表示使用默认过期时间
    ///
    /// # 返回值
    ///
    /// 返回 `Result<(), AppError>`
    pub async fn set_string<K, V>(
        &self,
        key: K,
        value: V,
        expiry: Option<u64>,
    ) -> Result<(), AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
        V: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();

        // 确定过期时间
        let exp = expiry.or(self.manager.default_expiry());

        if let Some(seconds) = exp {
            let _: () = conn
                .set_ex(key, value, seconds)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis set_ex failed: {}", e)))?;
        } else {
            let _: () = conn
                .set(key, value)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis set failed: {}", e)))?;
        }

        Ok(())
    }

    /// 获取字符串值
    ///
    /// # 参数
    ///
    /// * `key` - 键名
    ///
    /// # 返回值
    ///
    /// 返回 `Result<Option<String>, AppError>`，如果键不存在返回 None
    pub async fn get_string<K>(&self, key: K) -> Result<Option<String>, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();
        let result: RedisResult<String> = conn.get(key).await;

        match result {
            Ok(value) => Ok(Some(value)),
            Err(e) if e.kind() == redis::ErrorKind::TypeError => Ok(None),
            Err(e) => Err(AppError::Internal(anyhow::anyhow!(
                "Redis get failed: {}",
                e
            ))),
        }
    }

    /// 设置 JSON 对象
    ///
    /// # 参数
    ///
    /// * `key` - 键名
    /// * `value` - 可序列化的值
    /// * `expiry` - 过期时间（秒），None 表示使用默认过期时间
    ///
    /// # 返回值
    ///
    /// 返回 `Result<(), AppError>`
    pub async fn set_json<K, V>(
        &self,
        key: K,
        value: &V,
        expiry: Option<u64>,
    ) -> Result<(), AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
        V: Serialize,
    {
        let json_value = serde_json::to_string(value)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("JSON serialization failed: {}", e)))?;

        self.set_string(key, json_value, expiry).await
    }

    /// 获取 JSON 对象
    ///
    /// # 参数
    ///
    /// * `key` - 键名
    ///
    /// # 返回值
    ///
    /// 返回 `Result<Option<T>, AppError>`，如果键不存在返回 None
    pub async fn get_json<K, T>(&self, key: K) -> Result<Option<T>, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
        T: for<'de> Deserialize<'de>,
    {
        if let Some(json_str) = self.get_string(key).await? {
            let value = serde_json::from_str(&json_str).map_err(|e| {
                AppError::Internal(anyhow::anyhow!("JSON deserialization failed: {}", e))
            })?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    /// 删除键
    ///
    /// # 参数
    ///
    /// * `key` - 键名
    ///
    /// # 返回值
    ///
    /// 返回 `Result<bool, AppError>`，true 表示键存在并被删除，false 表示键不存在
    pub async fn delete<K>(&self, key: K) -> Result<bool, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();
        let deleted: u32 = conn
            .del(key)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis del failed: {}", e)))?;

        Ok(deleted > 0)
    }

    /// 检查键是否存在
    ///
    /// # 参数
    ///
    /// * `key` - 键名
    ///
    /// # 返回值
    ///
    /// 返回 `Result<bool, AppError>`
    pub async fn exists<K>(&self, key: K) -> Result<bool, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();
        let exists: bool = conn
            .exists(key)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis exists failed: {}", e)))?;

        Ok(exists)
    }

    /// 设置键的过期时间
    ///
    /// # 参数
    ///
    /// * `key` - 键名
    /// * `seconds` - 过期时间（秒）
    ///
    /// # 返回值
    ///
    /// 返回 `Result<bool, AppError>`，true 表示成功设置，false 表示键不存在
    pub async fn expire<K>(&self, key: K, seconds: u64) -> Result<bool, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();
        let result: bool = conn
            .expire(key, seconds as i64)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis expire failed: {}", e)))?;

        Ok(result)
    }

    /// 获取键的剩余生存时间
    ///
    /// # 参数
    ///
    /// * `key` - 键名
    ///
    /// # 返回值
    ///
    /// 返回 `Result<Option<u64>, AppError>`
    /// - Some(seconds) - 剩余秒数
    /// - None - 键不存在或没有过期时间
    pub async fn ttl<K>(&self, key: K) -> Result<Option<u64>, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();
        let ttl: i64 = conn
            .ttl(key)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis ttl failed: {}", e)))?;

        match ttl {
            -2 => Ok(None), // 键不存在
            -1 => Ok(None), // 键存在但没有过期时间
            seconds if seconds > 0 => Ok(Some(seconds as u64)),
            _ => Ok(None),
        }
    }

    /// 原子性递增
    ///
    /// # 参数
    ///
    /// * `key` - 键名
    /// * `increment` - 递增值，默认为 1
    ///
    /// # 返回值
    ///
    /// 返回 `Result<i64, AppError>` - 递增后的值
    pub async fn increment<K>(&self, key: K, increment: Option<i64>) -> Result<i64, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();

        let result = if let Some(inc) = increment {
            conn.incr(key, inc).await
        } else {
            conn.incr(key, 1).await
        };

        result.map_err(|e| AppError::Internal(anyhow::anyhow!("Redis incr failed: {}", e)))
    }

    /// 原子性递减
    ///
    /// # 参数
    ///
    /// * `key` - 键名
    /// * `decrement` - 递减值，默认为 1
    ///
    /// # 返回值
    ///
    /// 返回 `Result<i64, AppError>` - 递减后的值
    pub async fn decrement<K>(&self, key: K, decrement: Option<i64>) -> Result<i64, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();

        let result = if let Some(dec) = decrement {
            conn.incr(key, -dec).await
        } else {
            conn.incr(key, -1).await
        };

        result.map_err(|e| AppError::Internal(anyhow::anyhow!("Redis decr failed: {}", e)))
    }

    /// 列表左推
    ///
    /// # 参数
    ///
    /// * `key` - 列表键名
    /// * `value` - 要推入的值
    ///
    /// # 返回值
    ///
    /// 返回 `Result<u32, AppError>` - 推入后列表的长度
    pub async fn list_push_left<K, V>(&self, key: K, value: V) -> Result<u32, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
        V: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();
        let length: u32 = conn
            .lpush(key, value)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis lpush failed: {}", e)))?;

        Ok(length)
    }

    /// 列表右推
    ///
    /// # 参数
    ///
    /// * `key` - 列表键名
    /// * `value` - 要推入的值
    ///
    /// # 返回值
    ///
    /// 返回 `Result<u32, AppError>` - 推入后列表的长度
    pub async fn list_push_right<K, V>(&self, key: K, value: V) -> Result<u32, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
        V: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();
        let length: u32 = conn
            .rpush(key, value)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis rpush failed: {}", e)))?;

        Ok(length)
    }

    /// 列表左弹
    ///
    /// # 参数
    ///
    /// * `key` - 列表键名
    ///
    /// # 返回值
    ///
    /// 返回 `Result<Option<String>, AppError>`
    pub async fn list_pop_left<K>(&self, key: K) -> Result<Option<String>, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();
        let result: RedisResult<String> = conn.lpop(key, None).await;

        match result {
            Ok(value) => Ok(Some(value)),
            Err(e) if e.kind() == redis::ErrorKind::TypeError => Ok(None),
            Err(e) => Err(AppError::Internal(anyhow::anyhow!(
                "Redis lpop failed: {}",
                e
            ))),
        }
    }

    /// 列表右弹
    ///
    /// # 参数
    ///
    /// * `key` - 列表键名
    ///
    /// # 返回值
    ///
    /// 返回 `Result<Option<String>, AppError>`
    pub async fn list_pop_right<K>(&self, key: K) -> Result<Option<String>, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();
        let result: RedisResult<String> = conn.rpop(key, None).await;

        match result {
            Ok(value) => Ok(Some(value)),
            Err(e) if e.kind() == redis::ErrorKind::TypeError => Ok(None),
            Err(e) => Err(AppError::Internal(anyhow::anyhow!(
                "Redis rpop failed: {}",
                e
            ))),
        }
    }

    /// 获取列表长度
    ///
    /// # 参数
    ///
    /// * `key` - 列表键名
    ///
    /// # 返回值
    ///
    /// 返回 `Result<u32, AppError>`
    pub async fn list_length<K>(&self, key: K) -> Result<u32, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();
        let length: u32 = conn
            .llen(key)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis llen failed: {}", e)))?;

        Ok(length)
    }

    /// 集合添加成员
    ///
    /// # 参数
    ///
    /// * `key` - 集合键名
    /// * `member` - 要添加的成员
    ///
    /// # 返回值
    ///
    /// 返回 `Result<bool, AppError>` - true 表示成员是新的，false 表示成员已存在
    pub async fn set_add<K, V>(&self, key: K, member: V) -> Result<bool, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
        V: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();
        let added: u32 = conn
            .sadd(key, member)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis sadd failed: {}", e)))?;

        Ok(added > 0)
    }

    /// 集合移除成员
    ///
    /// # 参数
    ///
    /// * `key` - 集合键名
    /// * `member` - 要移除的成员
    ///
    /// # 返回值
    ///
    /// 返回 `Result<bool, AppError>` - true 表示成员存在并被移除，false 表示成员不存在
    pub async fn set_remove<K, V>(&self, key: K, member: V) -> Result<bool, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
        V: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();
        let removed: u32 = conn
            .srem(key, member)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis srem failed: {}", e)))?;

        Ok(removed > 0)
    }

    /// 检查集合成员是否存在
    ///
    /// # 参数
    ///
    /// * `key` - 集合键名
    /// * `member` - 要检查的成员
    ///
    /// # 返回值
    ///
    /// 返回 `Result<bool, AppError>`
    pub async fn set_is_member<K, V>(&self, key: K, member: V) -> Result<bool, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
        V: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();
        let is_member: bool = conn
            .sismember(key, member)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis sismember failed: {}", e)))?;

        Ok(is_member)
    }

    /// 获取集合所有成员
    ///
    /// # 参数
    ///
    /// * `key` - 集合键名
    ///
    /// # 返回值
    ///
    /// 返回 `Result<Vec<String>, AppError>`
    pub async fn set_members<K>(&self, key: K) -> Result<Vec<String>, AppError>
    where
        K: redis::ToRedisArgs + Send + Sync,
    {
        use redis::AsyncCommands;

        let mut conn = self.manager.connection().clone();
        let members: Vec<String> = conn
            .smembers(key)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis smembers failed: {}", e)))?;

        Ok(members)
    }
}
