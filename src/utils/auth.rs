/*!
 * JWT 身份验证工具
 * 
 * 提供 JWT (JSON Web Token) 的生成和验证功能，
 * 用于实现无状态的用户身份验证系统。
 */

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AppError, Result};

/// JWT Token 声明 (Claims)
/// 
/// 包含在 JWT Token 中的用户信息和元数据。
/// 遵循 JWT 标准的声明格式。
/// 
/// # 标准声明字段
/// 
/// - `sub` (Subject): 主题，这里用于存储用户 ID
/// - `exp` (Expiration): 过期时间戳
/// - `iat` (Issued At): 发行时间戳
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// 用户 ID (Subject)
    pub sub: String,
    
    /// 过期时间戳 (Expiration Time)
    pub exp: i64,
    
    /// 发行时间戳 (Issued At)
    pub iat: i64,
}

impl Claims {
    /// 创建新的 JWT 声明
    /// 
    /// 基于用户 ID 创建 JWT 声明，自动设置发行时间和过期时间。
    /// Token 的有效期为 24 小时。
    /// 
    /// # 参数
    /// 
    /// * `user_id` - 用户唯一标识符
    /// 
    /// # 返回值
    /// 
    /// 返回包含用户信息和时间戳的 Claims 结构体
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use uuid::Uuid;
    /// use crate::utils::auth::Claims;
    /// 
    /// let user_id = Uuid::new_v4();
    /// let claims = Claims::new(user_id);
    /// println!("Token will expire at: {}", claims.exp);
    /// ```
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // Token 24小时后过期

        Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }
}

/// 生成 JWT Token
/// 
/// 使用用户 ID 和密钥生成签名的 JWT Token。
/// Token 包含用户标识和过期时间信息。
/// 
/// # 参数
/// 
/// * `user_id` - 用户唯一标识符
/// * `secret` - JWT 签名密钥
/// 
/// # 返回值
/// 
/// 返回 `Result<String>`，成功时包含 JWT Token 字符串
/// 
/// # 错误
/// 
/// - `AppError::Jwt`: JWT 编码失败
/// 
/// # 安全注意事项
/// 
/// - 密钥应该足够长且随机
/// - 生产环境中应使用环境变量存储密钥
/// - Token 有效期为 24 小时，平衡安全性和用户体验
/// 
/// # 示例
/// 
/// ```rust
/// use uuid::Uuid;
/// use crate::utils::auth::generate_jwt;
/// 
/// let user_id = Uuid::new_v4();
/// let secret = "your-secret-key";
/// let token = generate_jwt(user_id, secret)?;
/// println!("Generated token: {}", token);
/// ```
pub fn generate_jwt(user_id: Uuid, secret: &str) -> Result<String> {
    // 创建包含用户信息的声明
    let claims = Claims::new(user_id);
    
    // 使用默认的 JWT 头部 (HS256 算法)
    let header = Header::default();
    
    // 创建编码密钥
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    
    // 编码生成 JWT Token
    encode(&header, &claims, &encoding_key)
        .map_err(AppError::Jwt)
}

/// 验证 JWT Token
/// 
/// 验证 JWT Token 的签名和有效期，提取其中的用户信息。
/// 
/// # 参数
/// 
/// * `token` - 要验证的 JWT Token 字符串
/// * `secret` - JWT 签名密钥（必须与生成时使用的密钥相同）
/// 
/// # 返回值
/// 
/// 返回 `Result<Claims>`，成功时包含 Token 中的用户声明信息
/// 
/// # 错误
/// 
/// - `AppError::Jwt`: Token 格式无效
/// - `AppError::Jwt`: Token 签名验证失败
/// - `AppError::Jwt`: Token 已过期
/// - `AppError::Jwt`: Token 不是在有效时间内发行的
/// 
/// # 验证内容
/// 
/// 1. **签名验证**: 确保 Token 未被篡改
/// 2. **过期时间**: 检查 Token 是否已过期
/// 3. **发行时间**: 验证 Token 发行时间的合理性
/// 4. **格式验证**: 确保 Token 格式正确
/// 
/// # 示例
/// 
/// ```rust
/// use crate::utils::auth::verify_jwt;
/// 
/// let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
/// let secret = "your-secret-key";
/// 
/// match verify_jwt(token, secret) {
///     Ok(claims) => {
///         println!("Valid token for user: {}", claims.sub);
///         println!("Expires at: {}", claims.exp);
///     }
///     Err(e) => println!("Invalid token: {}", e),
/// }
/// ```
pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims> {
    // 创建解码密钥
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    
    // 使用默认验证设置
    let validation = Validation::default();
    
    // 解码并验证 Token
    decode::<Claims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(AppError::Jwt)
}
