/*!
 * 用户业务逻辑服务
 * 
 * 处理所有与用户相关的业务逻辑，包括用户创建、身份验证、
 * 用户查询等操作。该服务封装了复杂的业务规则和数据操作。
 */

use uuid::Uuid;

use crate::{
    db::DbPool,
    error::{AppError, Result},
    models::{CreateUserRequest, LoginRequest, User},
    utils::{hash_password, verify_password},
};

/// 用户服务结构体
/// 
/// 提供用户管理相关的业务逻辑方法。
/// 采用静态方法设计，无需实例化即可使用。
pub struct UserService;

impl UserService {
    /// 创建新用户
    /// 
    /// 处理用户注册逻辑，包括邮箱重复检查、密码加密和数据库插入。
    /// 
    /// # 业务规则
    /// 
    /// 1. 检查邮箱是否已被注册
    /// 2. 使用 Argon2 算法对密码进行哈希处理
    /// 3. 在数据库中创建新用户记录
    /// 4. 自动设置创建时间和更新时间
    /// 
    /// # 参数
    /// 
    /// * `pool` - 数据库连接池
    /// * `request` - 用户注册请求数据
    /// 
    /// # 返回值
    /// 
    /// 返回 `Result<User>`，成功时包含新创建的用户信息
    /// 
    /// # 错误
    /// 
    /// - `AppError::Conflict`: 邮箱已存在
    /// - `AppError::PasswordHash`: 密码哈希失败
    /// - `AppError::Database`: 数据库操作失败
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// let request = CreateUserRequest {
    ///     email: "user@example.com".to_string(),
    ///     password: "securePassword123".to_string(),
    ///     name: "张三".to_string(),
    /// };
    /// 
    /// let user = UserService::create_user(&pool, request).await?;
    /// println!("Created user: {}", user.email);
    /// ```
    pub async fn create_user(pool: &DbPool, request: CreateUserRequest) -> Result<User> {
        // 检查邮箱是否已经被注册
        let existing_user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(&request.email)
        .fetch_optional(pool)
        .await?;

        if existing_user.is_some() {
            return Err(AppError::Conflict("User with this email already exists".to_string()));
        }

        // 对密码进行哈希处理
        let password_hash = hash_password(&request.password)?;

        // 在数据库中创建新用户
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, password_hash, name)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(&request.email)
        .bind(&password_hash)
        .bind(&request.name)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    /// 验证用户身份
    /// 
    /// 处理用户登录逻辑，验证邮箱和密码的正确性。
    /// 
    /// # 验证流程
    /// 
    /// 1. 根据邮箱查找用户
    /// 2. 使用 Argon2 验证密码哈希
    /// 3. 返回用户信息（如果验证成功）
    /// 
    /// # 安全考虑
    /// 
    /// - 对于不存在的邮箱和错误的密码都返回相同的错误信息，
    ///   避免泄露用户是否存在的信息
    /// - 使用安全的密码哈希验证算法
    /// 
    /// # 参数
    /// 
    /// * `pool` - 数据库连接池
    /// * `request` - 用户登录请求数据
    /// 
    /// # 返回值
    /// 
    /// 返回 `Result<User>`，成功时包含用户完整信息
    /// 
    /// # 错误
    /// 
    /// - `AppError::Authentication`: 邮箱或密码错误
    /// - `AppError::Database`: 数据库操作失败
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// let request = LoginRequest {
    ///     email: "user@example.com".to_string(),
    ///     password: "securePassword123".to_string(),
    /// };
    /// 
    /// let user = UserService::authenticate_user(&pool, request).await?;
    /// println!("User {} logged in", user.email);
    /// ```
    pub async fn authenticate_user(pool: &DbPool, request: LoginRequest) -> Result<User> {
        // 根据邮箱查找用户
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(&request.email)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::Authentication("Invalid email or password".to_string()))?;

        // 验证密码
        let is_valid = verify_password(&request.password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::Authentication("Invalid email or password".to_string()));
        }

        Ok(user)
    }

    /// 根据用户 ID 获取用户信息
    /// 
    /// 查询指定 ID 的用户详细信息，通常用于获取当前登录用户的资料。
    /// 
    /// # 参数
    /// 
    /// * `pool` - 数据库连接池
    /// * `user_id` - 用户唯一标识符
    /// 
    /// # 返回值
    /// 
    /// 返回 `Result<User>`，成功时包含用户完整信息
    /// 
    /// # 错误
    /// 
    /// - `AppError::NotFound`: 用户不存在
    /// - `AppError::Database`: 数据库操作失败
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use uuid::Uuid;
    /// 
    /// let user_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000")?;
    /// let user = UserService::get_user_by_id(&pool, user_id).await?;
    /// println!("Found user: {}", user.name);
    /// ```
    pub async fn get_user_by_id(pool: &DbPool, user_id: Uuid) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user)
    }

    /// 获取所有用户列表
    /// 
    /// 查询系统中的所有用户，按创建时间倒序排列。
    /// 通常用于管理界面显示用户列表。
    /// 
    /// # 排序规则
    /// 
    /// 用户按创建时间降序排列（最新注册的用户排在前面）
    /// 
    /// # 参数
    /// 
    /// * `pool` - 数据库连接池
    /// 
    /// # 返回值
    /// 
    /// 返回 `Result<Vec<User>>`，成功时包含所有用户的列表
    /// 
    /// # 错误
    /// 
    /// - `AppError::Database`: 数据库操作失败
    /// 
    /// # 注意事项
    /// 
    /// 在用户量很大的系统中，这个方法可能需要添加分页功能
    /// 以避免一次性加载过多数据。
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// let users = UserService::get_all_users(&pool).await?;
    /// println!("Total users: {}", users.len());
    /// 
    /// for user in users {
    ///     println!("User: {} ({})", user.name, user.email);
    /// }
    /// ```
    pub async fn get_all_users(pool: &DbPool) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }
}
