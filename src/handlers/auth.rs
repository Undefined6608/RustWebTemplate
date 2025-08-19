/*!
 * 身份验证处理器
 * 
 * 处理用户注册和登录相关的 HTTP 请求。
 * 包括用户账户创建、身份验证和 JWT Token 生成。
 */

use axum::{extract::State, Json};

use crate::{
    error::Result,
    models::{AuthResponse, CreateUserRequest, LoginRequest},
    routes::AppState,
    services::UserService,
    utils::generate_jwt,
};

/// 用户注册处理器
/// 
/// 处理用户注册请求，创建新用户账户并返回 JWT Token。
/// 
/// # 请求
/// 
/// - **方法**: POST
/// - **路径**: `/api/auth/register`
/// - **请求体**: JSON 格式的 `CreateUserRequest`
///   ```json
///   {
///     "email": "user@example.com",
///     "password": "password123",
///     "name": "用户名"
///   }
///   ```
/// 
/// # 响应
/// 
/// 成功时返回 `AuthResponse`，包含 JWT Token 和用户信息：
/// ```json
/// {
///   "token": "jwt_token_here",
///   "user": {
///     "id": "user_uuid",
///     "email": "user@example.com",
///     "name": "用户名",
///     "created_at": "2023-01-01T00:00:00Z"
///   }
/// }
/// ```
/// 
/// # 错误
/// 
/// - `409 Conflict`: 邮箱已存在
/// - `400 Bad Request`: 请求数据格式错误
/// - `500 Internal Server Error`: 服务器内部错误
/// 
/// # 参数
/// 
/// * `app_state` - 应用程序状态，包含数据库连接池和配置
/// * `request` - 用户注册请求数据
pub async fn register(
    State(app_state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<AuthResponse>> {
    // 调用用户服务创建新用户
    let user = UserService::create_user(&app_state.pool, request).await?;
    
    // 为新用户生成 JWT Token
    let token = generate_jwt(user.id, &app_state.config.jwt_secret)?;
    
    // 构造响应数据
    let response = AuthResponse {
        token,
        user: user.into(), // 转换为 UserResponse，隐藏敏感信息
    };

    Ok(Json(response))
}

/// 用户登录处理器
/// 
/// 处理用户登录请求，验证用户凭据并返回 JWT Token。
/// 
/// # 请求
/// 
/// - **方法**: POST
/// - **路径**: `/api/auth/login`
/// - **请求体**: JSON 格式的 `LoginRequest`
///   ```json
///   {
///     "email": "user@example.com",
///     "password": "password123"
///   }
///   ```
/// 
/// # 响应
/// 
/// 成功时返回 `AuthResponse`，包含 JWT Token 和用户信息：
/// ```json
/// {
///   "token": "jwt_token_here",
///   "user": {
///     "id": "user_uuid",
///     "email": "user@example.com",
///     "name": "用户名",
///     "created_at": "2023-01-01T00:00:00Z"
///   }
/// }
/// ```
/// 
/// # 错误
/// 
/// - `401 Unauthorized`: 邮箱或密码错误
/// - `400 Bad Request`: 请求数据格式错误
/// - `500 Internal Server Error`: 服务器内部错误
/// 
/// # 参数
/// 
/// * `app_state` - 应用程序状态，包含数据库连接池和配置
/// * `request` - 用户登录请求数据
pub async fn login(
    State(app_state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    // 验证用户凭据
    let user = UserService::authenticate_user(&app_state.pool, request).await?;
    
    // 为通过验证的用户生成 JWT Token
    let token = generate_jwt(user.id, &app_state.config.jwt_secret)?;
    
    // 构造响应数据
    let response = AuthResponse {
        token,
        user: user.into(), // 转换为 UserResponse，隐藏敏感信息
    };

    Ok(Json(response))
}
