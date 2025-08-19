/*!
 * 身份验证处理器
 * 
 * 处理用户身份验证相关的 HTTP 请求。
 * 包括用户账户创建、身份验证、JWT Token 生成和撤销。
 */

use axum::{extract::State, http::header::AUTHORIZATION, Json, extract::Request};
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    models::{AuthResponse, CreateUserRequest, LoginRequest},
    routes::AppState,
    services::{UserService, TokenService},
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
    
    // 使用 TokenService 生成并存储 token 到 Redis
    let token = TokenService::create_token(
        &app_state.redis,
        user.id,
        &app_state.config.jwt_secret,
        None, // device_info，可以从请求头获取
        None, // ip_address，可以从请求中提取
    ).await?;
    
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
    
    // 使用 TokenService 生成并存储 token 到 Redis
    let token = TokenService::create_token(
        &app_state.redis,
        user.id,
        &app_state.config.jwt_secret,
        None, // device_info，可以从请求头获取
        None, // ip_address，可以从请求中提取
    ).await?;
    
    // 构造响应数据
    let response = AuthResponse {
        token,
        user: user.into(), // 转换为 UserResponse，隐藏敏感信息
    };

    Ok(Json(response))
}

/// 用户退出登录处理器
/// 
/// 撤销用户的当前 token，使其无效。
/// 
/// # 请求
/// 
/// - **方法**: POST
/// - **路径**: `/api/auth/logout`
/// - **请求头**: 必须包含有效的 Authorization header
///   ```
///   Authorization: Bearer <jwt_token>
///   ```
/// 
/// # 响应
/// 
/// 成功时返回简单的成功消息：
/// ```json
/// {
///   "message": "退出登录成功"
/// }
/// ```
/// 
/// # 错误
/// 
/// - `401 Unauthorized`: Token 无效或已过期
/// - `500 Internal Server Error`: 服务器内部错误
/// 
/// # 参数
/// 
/// * `app_state` - 应用程序状态，包含 Redis 管理器和配置
/// * `request` - HTTP 请求对象，用于提取 Authorization header
pub async fn logout(
    State(app_state): State<AppState>,
    request: Request,
) -> Result<Json<serde_json::Value>> {
    // 从请求头中提取 Authorization 字段
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| AppError::Authentication("Missing authorization header".to_string()))?;

    // 验证 Authorization 头的格式，必须是 "Bearer <token>"
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Authentication("Invalid authorization header format".to_string()))?;

    // 先验证 token 以获取用户 ID
    let claims = TokenService::verify_token(&app_state.redis, token, &app_state.config.jwt_secret).await?;
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Authentication("Invalid user ID in token".to_string()))?;

    // 撤销当前 token
    TokenService::revoke_token(&app_state.redis, token, user_id).await?;

    // 返回成功响应
    Ok(Json(serde_json::json!({
        "message": "退出登录成功"
    })))
}

/// 撤销用户所有 token 处理器
/// 
/// 撤销用户的所有 token，使所有设备上的登录都无效。
/// 用于安全场景，如密码更改后强制重新登录。
/// 
/// # 请求
/// 
/// - **方法**: POST
/// - **路径**: `/api/auth/logout-all`
/// - **请求头**: 必须包含有效的 Authorization header
/// 
/// # 响应
/// 
/// 成功时返回撤销的 token 数量：
/// ```json
/// {
///   "message": "已撤销所有登录会话",
///   "revoked_count": 3
/// }
/// ```
/// 
/// # 参数
/// 
/// * `app_state` - 应用程序状态
/// * `request` - HTTP 请求对象
pub async fn logout_all(
    State(app_state): State<AppState>,
    request: Request,
) -> Result<Json<serde_json::Value>> {
    // 从请求头中提取 Authorization 字段
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| AppError::Authentication("Missing authorization header".to_string()))?;

    // 验证 Authorization 头的格式
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Authentication("Invalid authorization header format".to_string()))?;

    // 先验证 token 以获取用户 ID
    let claims = TokenService::verify_token(&app_state.redis, token, &app_state.config.jwt_secret).await?;
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Authentication("Invalid user ID in token".to_string()))?;

    // 获取用户当前的 token 数量
    let token_count = TokenService::get_user_token_count(&app_state.redis, user_id).await?;

    // 撤销用户的所有 token
    TokenService::revoke_all_user_tokens(&app_state.redis, user_id).await?;

    // 返回成功响应
    Ok(Json(serde_json::json!({
        "message": "已撤销所有登录会话",
        "revoked_count": token_count
    })))
}
