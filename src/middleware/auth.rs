/*!
 * 身份验证中间件
 *
 * 负责验证 HTTP 请求中的 JWT Token，确保只有经过身份验证的用户
 * 才能访问受保护的资源。验证成功后会将用户 ID 注入到请求扩展中。
 */

use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    routes::AppState,
    services::TokenService,
};

/// 身份验证中间件函数
///
/// 验证 HTTP 请求中的 JWT Token，确保用户已经登录。
/// 该中间件会：
/// 1. 从 Authorization 请求头中提取 JWT Token
/// 2. 验证 Token 的有效性和签名
/// 3. 从 Token 中提取用户 ID
/// 4. 将用户 ID 注入到请求扩展中，供后续处理器使用
///
/// # 请求头格式
///
/// ```
/// Authorization: Bearer <jwt_token>
/// ```
///
/// # 工作流程
///
/// 1. **提取 Authorization 头**: 从请求头中获取 `Authorization` 字段
/// 2. **验证格式**: 确保头部格式为 `Bearer <token>`
/// 3. **验证 Token**: 使用配置的密钥验证 JWT Token 的签名和有效期
/// 4. **提取用户信息**: 从 Token 的 claims 中提取用户 ID
/// 5. **注入用户 ID**: 将用户 ID 添加到请求扩展中
/// 6. **继续处理**: 调用下一个中间件或处理器
///
/// # 错误处理
///
/// - `401 Unauthorized`: 缺少 Authorization 头
/// - `401 Unauthorized`: Authorization 头格式不正确
/// - `401 Unauthorized`: JWT Token 无效、已过期或签名错误
/// - `401 Unauthorized`: Token 中的用户 ID 格式不正确
///
/// # 参数
///
/// * `config` - 应用配置，包含 JWT 密钥
/// * `request` - HTTP 请求对象
/// * `next` - 下一个中间件或处理器
///
/// # 返回值
///
/// 返回 `Result<Response>`，成功时继续处理请求，失败时返回身份验证错误
///
/// # 示例
///
/// ```rust
/// // 在路由中应用身份验证中间件
/// use axum::{middleware, Router};
/// use crate::middleware::auth_middleware;
///
/// let protected_routes = Router::new()
///     .route("/profile", get(get_profile))
///     .layer(middleware::from_fn_with_state(config, auth_middleware));
/// ```
pub async fn auth_middleware(
    State(app_state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response> {
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

    // 使用 TokenService 验证 token（包括 Redis 存在性检查）
    let claims =
        TokenService::verify_token(&app_state.redis, token, &app_state.config.jwt_secret).await?;

    // 从 Token claims 中提取用户 ID
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Authentication("Invalid user ID in token".to_string()))?;

    // 将用户 ID 注入到请求扩展中，供后续处理器使用
    request.extensions_mut().insert(user_id);

    // 继续处理请求
    Ok(next.run(request).await)
}
