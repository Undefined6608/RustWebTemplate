/*!
 * 身份验证处理器
 *
 * 处理用户身份验证相关的 HTTP 请求。
 * 包括用户账户创建、身份验证、JWT Token 生成和撤销。
 */

use axum::{
    extract::Request,
    extract::State,
    http::header::{AUTHORIZATION, USER_AGENT},
    Json,
};
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    models::{AuthResponse, CreateUserRequest, LoginRequest},
    routes::AppState,
    services::{TokenService, UserService},
    utils::DeviceInfo,
};

/// 从HTTP请求中提取设备信息
///
/// # 参数
///
/// * `request` - HTTP 请求对象
///
/// # 返回值
///
/// 返回解析后的设备信息
fn extract_device_info(request: &Request) -> DeviceInfo {
    // 从请求头中获取 User-Agent
    let user_agent = request
        .headers()
        .get(USER_AGENT)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("Unknown");

    // 从请求头中获取设备类型提示（可选的自定义头部）
    let device_type_hint = request
        .headers()
        .get("X-Device-Type")
        .and_then(|header| header.to_str().ok());

    DeviceInfo::from_user_agent(user_agent, device_type_hint)
}

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
    request: Request,
) -> Result<Json<AuthResponse>> {
    // 提取设备信息
    let device_info = extract_device_info(&request);

    // 提取IP地址（从连接信息或代理头部）
    let ip_address = request
        .headers()
        .get("X-Forwarded-For")
        .or_else(|| request.headers().get("X-Real-IP"))
        .and_then(|header| header.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string());

    // 提取JSON请求体
    let (_, body) = request.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|e| AppError::Validation(format!("读取请求体失败: {}", e)))?;
    let create_user_request: CreateUserRequest = serde_json::from_slice(&bytes)
        .map_err(|e| AppError::Validation(format!("JSON解析失败: {}", e)))?;

    // 调用用户服务创建新用户
    let user = UserService::create_user(&app_state.pool, create_user_request).await?;

    // 使用 TokenService 生成并存储 token 到 Redis
    let token = TokenService::create_token(
        &app_state.redis,
        user.id,
        &app_state.config.jwt_secret,
        device_info,
        ip_address,
    )
    .await?;

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
    request: Request,
) -> Result<Json<AuthResponse>> {
    // 提取设备信息
    let device_info = extract_device_info(&request);

    // 提取IP地址（从连接信息或代理头部）
    let ip_address = request
        .headers()
        .get("X-Forwarded-For")
        .or_else(|| request.headers().get("X-Real-IP"))
        .and_then(|header| header.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string());

    // 提取JSON请求体
    let (_, body) = request.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|e| AppError::Validation(format!("读取请求体失败: {}", e)))?;
    let login_request: LoginRequest = serde_json::from_slice(&bytes)
        .map_err(|e| AppError::Validation(format!("JSON解析失败: {}", e)))?;

    // 验证用户凭据
    let user = UserService::authenticate_user(&app_state.pool, login_request).await?;

    // 使用 TokenService 生成并存储 token 到 Redis（会自动撤销同设备类型的其他登录）
    let token = TokenService::create_token(
        &app_state.redis,
        user.id,
        &app_state.config.jwt_secret,
        device_info,
        ip_address,
    )
    .await?;

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
    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        AppError::Authentication("Invalid authorization header format".to_string())
    })?;

    // 先验证 token 以获取用户 ID
    let claims =
        TokenService::verify_token(&app_state.redis, token, &app_state.config.jwt_secret).await?;
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
    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        AppError::Authentication("Invalid authorization header format".to_string())
    })?;

    // 先验证 token 以获取用户 ID
    let claims =
        TokenService::verify_token(&app_state.redis, token, &app_state.config.jwt_secret).await?;
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

/// 获取用户活跃会话列表处理器
///
/// 返回用户在所有设备类型上的活跃登录会话信息。
///
/// # 请求
///
/// - **方法**: GET
/// - **路径**: `/api/auth/sessions`
/// - **请求头**: 必须包含有效的 Authorization header
///
/// # 响应
///
/// 成功时返回用户的所有活跃会话：
/// ```json
/// {
///   "sessions": [
///     {
///       "device_type": "web",
///       "device_name": "Chrome on Windows 10",
///       "created_at": "2023-01-01T10:00:00Z",
///       "ip_address": "192.168.1.100",
///       "is_current": true
///     },
///     {
///       "device_type": "mobile",
///       "device_name": "iOS Device",
///       "created_at": "2023-01-01T09:00:00Z",
///       "ip_address": "192.168.1.101",
///       "is_current": false
///     }
///   ]
/// }
/// ```
///
/// # 参数
///
/// * `app_state` - 应用程序状态
/// * `request` - HTTP 请求对象
pub async fn get_sessions(
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
    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        AppError::Authentication("Invalid authorization header format".to_string())
    })?;

    // 先验证 token 以获取用户 ID
    let claims =
        TokenService::verify_token(&app_state.redis, token, &app_state.config.jwt_secret).await?;
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Authentication("Invalid user ID in token".to_string()))?;

    // 获取用户所有设备的活跃会话
    let device_sessions = TokenService::get_user_device_sessions(&app_state.redis, user_id).await?;

    // 转换为响应格式
    let mut sessions = Vec::new();
    for (device_type, token_info) in device_sessions {
        let session = serde_json::json!({
            "device_type": device_type.to_string(),
            "device_name": token_info.device_info.display_name(),
            "created_at": chrono::DateTime::from_timestamp(token_info.created_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
            "ip_address": token_info.ip_address,
            "is_current": false // 后面可以通过比较token来确定是否为当前会话
        });
        sessions.push(session);
    }

    // 返回会话列表
    Ok(Json(serde_json::json!({
        "sessions": sessions
    })))
}

/// 撤销特定设备类型的登录会话处理器
///
/// 撤销用户在指定设备类型上的登录会话。
///
/// # 请求
///
/// - **方法**: POST
/// - **路径**: `/api/auth/logout-device/{device_type}`
/// - **请求头**: 必须包含有效的 Authorization header
///
/// # 响应
///
/// 成功时返回撤销结果：
/// ```json
/// {
///   "message": "已撤销Web设备的登录会话"
/// }
/// ```
///
/// # 参数
///
/// * `app_state` - 应用程序状态
/// * `request` - HTTP 请求对象
/// * `device_type` - 要撤销的设备类型
pub async fn logout_device(
    State(app_state): State<AppState>,
    axum::extract::Path(device_type_str): axum::extract::Path<String>,
    request: Request,
) -> Result<Json<serde_json::Value>> {
    // 从请求头中提取 Authorization 字段
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| AppError::Authentication("Missing authorization header".to_string()))?;

    // 验证 Authorization 头的格式
    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        AppError::Authentication("Invalid authorization header format".to_string())
    })?;

    // 先验证 token 以获取用户 ID
    let claims =
        TokenService::verify_token(&app_state.redis, token, &app_state.config.jwt_secret).await?;
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Authentication("Invalid user ID in token".to_string()))?;

    // 解析设备类型
    let device_type = crate::utils::DeviceType::from_str(&device_type_str);

    // 撤销指定设备类型的token
    TokenService::revoke_device_tokens(&app_state.redis, user_id, &device_type).await?;

    let device_name = match device_type {
        crate::utils::DeviceType::Web => "Web",
        crate::utils::DeviceType::Mobile => "移动",
        crate::utils::DeviceType::Desktop => "桌面",
        crate::utils::DeviceType::Api => "API",
    };

    // 返回成功响应
    Ok(Json(serde_json::json!({
        "message": format!("已撤销{}设备的登录会话", device_name)
    })))
}
