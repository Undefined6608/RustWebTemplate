/*!
 * Token 管理服务
 * 
 * 负责 JWT Token 的生成、Redis 存储、验证和撤销功能。
 * 提供完整的 token 生命周期管理。
 */

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    redis::RedisManager,
    utils::{generate_jwt, verify_jwt, Claims},
};

/// Token 信息结构体
/// 
/// 存储在 Redis 中的 token 相关信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenInfo {
    /// 用户 ID
    pub user_id: Uuid,
    /// Token 创建时间
    pub created_at: i64,
    /// Token 过期时间
    pub expires_at: i64,
    /// 设备信息（可选）
    pub device_info: Option<String>,
    /// IP 地址（可选）
    pub ip_address: Option<String>,
}

/// Token 管理服务
pub struct TokenService;

impl TokenService {
    /// Token 在 Redis 中的键前缀
    const TOKEN_PREFIX: &'static str = "auth:token:";
    
    /// 用户 token 集合的键前缀（用于快速查找用户的所有 token）
    const USER_TOKENS_PREFIX: &'static str = "auth:user_tokens:";
    
    /// Token 的默认过期时间（24小时，与JWT保持一致）
    const TOKEN_EXPIRY_SECONDS: u64 = 24 * 60 * 60;

    /// 生成并存储 token
    /// 
    /// # 参数
    /// 
    /// * `redis` - Redis 管理器
    /// * `user_id` - 用户 ID
    /// * `jwt_secret` - JWT 密钥
    /// * `device_info` - 设备信息（可选）
    /// * `ip_address` - IP 地址（可选）
    /// 
    /// # 返回值
    /// 
    /// 返回生成的 JWT token 字符串
    pub async fn create_token(
        redis: &RedisManager,
        user_id: Uuid,
        jwt_secret: &str,
        device_info: Option<String>,
        ip_address: Option<String>,
    ) -> Result<String> {
        // 生成 JWT token
        let token = generate_jwt(user_id, jwt_secret)?;
        
        // 创建 token 信息
        let now = Utc::now();
        let expires_at = now + Duration::hours(24);
        
        let token_info = TokenInfo {
            user_id,
            created_at: now.timestamp(),
            expires_at: expires_at.timestamp(),
            device_info,
            ip_address,
        };

        // 在 Redis 中存储 token 信息
        let token_key = format!("{}{}", Self::TOKEN_PREFIX, token);
        let user_tokens_key = format!("{}{}", Self::USER_TOKENS_PREFIX, user_id);
        
        // 使用 Redis pipeline 提高性能
        use redis::AsyncCommands;
        let mut conn = redis.connection().clone();
        
        // 存储 token 信息，设置过期时间
        let _: () = conn.set_ex(&token_key, 
            serde_json::to_string(&token_info)
                .map_err(|e| AppError::Internal(anyhow::anyhow!("JSON序列化失败: {}", e)))?,
            Self::TOKEN_EXPIRY_SECONDS
        ).await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis存储token失败: {}", e)))?;
        
        // 将 token 添加到用户的 token 集合中
        let _: () = conn.sadd(&user_tokens_key, &token).await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis添加用户token失败: {}", e)))?;
        
        // 为用户 token 集合设置过期时间（比 token 稍长一些）
        let _: () = conn.expire(&user_tokens_key, (Self::TOKEN_EXPIRY_SECONDS + 3600) as i64).await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis设置用户token过期时间失败: {}", e)))?;

        Ok(token)
    }

    /// 验证 token 有效性
    /// 
    /// # 参数
    /// 
    /// * `redis` - Redis 管理器
    /// * `token` - 要验证的 JWT token
    /// * `jwt_secret` - JWT 密钥
    /// 
    /// # 返回值
    /// 
    /// 返回 token 中的用户 Claims 信息
    pub async fn verify_token(
        redis: &RedisManager,
        token: &str,
        jwt_secret: &str,
    ) -> Result<Claims> {
        // 首先验证 JWT token 的签名和格式
        let claims = verify_jwt(token, jwt_secret)?;
        
        // 检查 token 是否在 Redis 中存在（未被撤销）
        let token_key = format!("{}{}", Self::TOKEN_PREFIX, token);
        
        use redis::AsyncCommands;
        let mut conn = redis.connection().clone();
        
        let exists: bool = conn.exists(&token_key).await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis检查token存在性失败: {}", e)))?;
        
        if !exists {
            return Err(AppError::Authentication("Token已被撤销或不存在".to_string()));
        }
        
        // 可选：获取并验证 token 信息
        let token_info_str: Option<String> = conn.get(&token_key).await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis获取token信息失败: {}", e)))?;
        
        if let Some(info_str) = token_info_str {
            let token_info: TokenInfo = serde_json::from_str(&info_str)
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Token信息反序列化失败: {}", e)))?;
            
            // 验证 token 信息中的用户 ID 是否与 JWT claims 一致
            if token_info.user_id.to_string() != claims.sub {
                return Err(AppError::Authentication("Token信息不一致".to_string()));
            }
        }

        Ok(claims)
    }

    /// 撤销单个 token
    /// 
    /// # 参数
    /// 
    /// * `redis` - Redis 管理器
    /// * `token` - 要撤销的 token
    /// * `user_id` - 用户 ID（用于从用户 token 集合中移除）
    pub async fn revoke_token(
        redis: &RedisManager,
        token: &str,
        user_id: Uuid,
    ) -> Result<()> {
        let token_key = format!("{}{}", Self::TOKEN_PREFIX, token);
        let user_tokens_key = format!("{}{}", Self::USER_TOKENS_PREFIX, user_id);
        
        use redis::AsyncCommands;
        let mut conn = redis.connection().clone();
        
        // 删除 token 信息
        let _: () = conn.del(&token_key).await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis删除token失败: {}", e)))?;
        
        // 从用户 token 集合中移除
        let _: () = conn.srem(&user_tokens_key, token).await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis移除用户token失败: {}", e)))?;

        Ok(())
    }

    /// 撤销用户的所有 token
    /// 
    /// # 参数
    /// 
    /// * `redis` - Redis 管理器
    /// * `user_id` - 用户 ID
    pub async fn revoke_all_user_tokens(
        redis: &RedisManager,
        user_id: Uuid,
    ) -> Result<()> {
        let user_tokens_key = format!("{}{}", Self::USER_TOKENS_PREFIX, user_id);
        
        use redis::AsyncCommands;
        let mut conn = redis.connection().clone();
        
        // 获取用户的所有 token
        let tokens: Vec<String> = conn.smembers(&user_tokens_key).await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis获取用户tokens失败: {}", e)))?;
        
        // 删除所有 token 信息
        for token in tokens {
            let token_key = format!("{}{}", Self::TOKEN_PREFIX, token);
            let _: () = conn.del(&token_key).await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis删除token失败: {}", e)))?;
        }
        
        // 删除用户 token 集合
        let _: () = conn.del(&user_tokens_key).await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis删除用户token集合失败: {}", e)))?;

        Ok(())
    }

    /// 获取用户的活跃 token 数量
    /// 
    /// # 参数
    /// 
    /// * `redis` - Redis 管理器
    /// * `user_id` - 用户 ID
    /// 
    /// # 返回值
    /// 
    /// 返回用户当前的活跃 token 数量
    pub async fn get_user_token_count(
        redis: &RedisManager,
        user_id: Uuid,
    ) -> Result<u32> {
        let user_tokens_key = format!("{}{}", Self::USER_TOKENS_PREFIX, user_id);
        
        use redis::AsyncCommands;
        let mut conn = redis.connection().clone();
        
        let count: u32 = conn.scard(&user_tokens_key).await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis获取用户token数量失败: {}", e)))?;

        Ok(count)
    }

    /// 获取 token 信息
    /// 
    /// # 参数
    /// 
    /// * `redis` - Redis 管理器
    /// * `token` - JWT token
    /// 
    /// # 返回值
    /// 
    /// 返回 token 的详细信息
    pub async fn get_token_info(
        redis: &RedisManager,
        token: &str,
    ) -> Result<Option<TokenInfo>> {
        let token_key = format!("{}{}", Self::TOKEN_PREFIX, token);
        
        use redis::AsyncCommands;
        let mut conn = redis.connection().clone();
        
        let token_info_str: Option<String> = conn.get(&token_key).await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis获取token信息失败: {}", e)))?;
        
        if let Some(info_str) = token_info_str {
            let token_info: TokenInfo = serde_json::from_str(&info_str)
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Token信息反序列化失败: {}", e)))?;
            Ok(Some(token_info))
        } else {
            Ok(None)
        }
    }

    /// 清理过期的 token（可选的维护功能）
    /// 
    /// 这个方法可以由定时任务调用，清理 Redis 中可能残留的过期 token
    /// 
    /// # 参数
    /// 
    /// * `redis` - Redis 管理器
    pub async fn cleanup_expired_tokens(redis: &RedisManager) -> Result<u32> {
        use redis::AsyncCommands;
        let mut conn = redis.connection().clone();
        
        let pattern = format!("{}*", Self::TOKEN_PREFIX);
        let mut cleaned_count = 0u32;
        
        // 获取所有 token 键
        let keys: Vec<String> = conn.keys(&pattern).await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis获取token键列表失败: {}", e)))?;
        
        let now = Utc::now().timestamp();
        
        for key in keys {
            // 获取 token 信息
            let token_info_str: Option<String> = conn.get(&key).await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis获取token信息失败: {}", e)))?;
            
            if let Some(info_str) = token_info_str {
                if let Ok(token_info) = serde_json::from_str::<TokenInfo>(&info_str) {
                    // 检查是否过期
                    if token_info.expires_at < now {
                        let _: () = conn.del(&key).await
                            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis删除过期token失败: {}", e)))?;
                        
                        // 从用户 token 集合中移除
                        let token = key.strip_prefix(Self::TOKEN_PREFIX).unwrap_or("");
                        let user_tokens_key = format!("{}{}", Self::USER_TOKENS_PREFIX, token_info.user_id);
                        let _: () = conn.srem(&user_tokens_key, token).await
                            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis移除用户过期token失败: {}", e)))?;
                        
                        cleaned_count += 1;
                    }
                }
            }
        }
        
        Ok(cleaned_count)
    }
}
