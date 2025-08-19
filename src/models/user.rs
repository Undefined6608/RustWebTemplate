/*!
 * 用户数据模型
 * 
 * 定义用户相关的所有数据结构，包括数据库实体、
 * API 请求/响应格式和数据转换规则。
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 用户数据库实体
/// 
/// 对应数据库中的 `users` 表，包含用户的完整信息。
/// 该结构体用于数据库查询和内部数据处理。
/// 
/// # 字段说明
/// 
/// - `id`: 用户唯一标识符 (UUID)
/// - `email`: 用户邮箱地址，用于登录和联系
/// - `password_hash`: 经过 Argon2 哈希处理的密码
/// - `name`: 用户显示名称
/// - `created_at`: 账户创建时间
/// - `updated_at`: 最后更新时间
/// 
/// # 安全注意事项
/// 
/// - 密码以哈希形式存储，永不保存明文密码
/// - 响应给客户端时会隐藏密码哈希字段
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    /// 用户唯一标识符
    pub id: Uuid,
    
    /// 用户邮箱地址（用于登录）
    pub email: String,
    
    /// 密码的 Argon2 哈希值
    pub password_hash: String,
    
    /// 用户显示名称
    pub name: String,
    
    /// 账户创建时间
    pub created_at: DateTime<Utc>,
    
    /// 最后更新时间
    pub updated_at: DateTime<Utc>,
}

/// 用户注册请求
/// 
/// 用于接收客户端的用户注册数据。
/// 包含创建新用户账户所需的基本信息。
/// 
/// # 验证规则
/// 
/// - `email`: 必须是有效的邮箱格式
/// - `password`: 建议最少 8 位字符，包含数字和字母
/// - `name`: 用户显示名称，不能为空
/// 
/// # 示例 JSON
/// 
/// ```json
/// {
///   "email": "user@example.com",
///   "password": "securePassword123",
///   "name": "张三"
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    /// 用户邮箱地址
    pub email: String,
    
    /// 用户密码（明文，服务端会进行哈希处理）
    pub password: String,
    
    /// 用户显示名称
    pub name: String,
}

/// 用户登录请求
/// 
/// 用于接收客户端的登录凭据。
/// 
/// # 示例 JSON
/// 
/// ```json
/// {
///   "email": "user@example.com",
///   "password": "securePassword123"
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// 登录邮箱
    pub email: String,
    
    /// 登录密码
    pub password: String,
}

/// 用户信息响应
/// 
/// 返回给客户端的用户信息，不包含敏感数据如密码哈希。
/// 用于个人资料、用户列表等 API 响应。
/// 
/// # 示例 JSON
/// 
/// ```json
/// {
///   "id": "123e4567-e89b-12d3-a456-426614174000",
///   "email": "user@example.com",
///   "name": "张三",
///   "created_at": "2023-01-01T00:00:00Z"
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct UserResponse {
    /// 用户 ID
    pub id: Uuid,
    
    /// 用户邮箱
    pub email: String,
    
    /// 用户名称
    pub name: String,
    
    /// 创建时间
    pub created_at: DateTime<Utc>,
}

/// 身份验证响应
/// 
/// 用于注册和登录成功后返回给客户端的数据。
/// 包含 JWT Token 和用户基本信息。
/// 
/// # 示例 JSON
/// 
/// ```json
/// {
///   "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
///   "user": {
///     "id": "123e4567-e89b-12d3-a456-426614174000",
///     "email": "user@example.com",
///     "name": "张三",
///     "created_at": "2023-01-01T00:00:00Z"
///   }
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// JWT 访问令牌
    pub token: String,
    
    /// 用户信息
    pub user: UserResponse,
}

/// 从 User 实体转换为 UserResponse
/// 
/// 自动过滤掉敏感信息（如密码哈希），只保留可以安全
/// 返回给客户端的用户信息。
/// 
/// # 示例
/// 
/// ```rust
/// let user: User = get_user_from_db().await?;
/// let response: UserResponse = user.into();
/// ```
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            created_at: user.created_at,
            // 注意：不包含 password_hash 和 updated_at
        }
    }
}
