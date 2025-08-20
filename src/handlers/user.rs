/*!
 * 用户管理处理器
 *
 * 处理用户信息相关的 HTTP 请求，包括获取个人资料和用户列表。
 * 所有处理器都需要身份验证。
 */

use axum::{extract::State, Extension, Json};
use uuid::Uuid;

use crate::{error::Result, models::UserResponse, routes::AppState, services::UserService};

/// 获取用户个人资料处理器
///
/// 返回当前登录用户的个人信息。
/// 用户 ID 从 JWT Token 中提取，由身份验证中间件注入。
///
/// # 请求
///
/// - **方法**: GET
/// - **路径**: `/api/profile`
/// - **请求头**: `Authorization: Bearer <jwt_token>`
///
/// # 响应
///
/// 成功时返回用户信息：
/// ```json
/// {
///   "id": "user_uuid",
///   "email": "user@example.com",
///   "name": "用户名",
///   "created_at": "2023-01-01T00:00:00Z"
/// }
/// ```
///
/// # 错误
///
/// - `401 Unauthorized`: JWT Token 无效或已过期
/// - `404 Not Found`: 用户不存在
/// - `500 Internal Server Error`: 服务器内部错误
///
/// # 参数
///
/// * `app_state` - 应用程序状态，包含数据库连接池
/// * `user_id` - 从 JWT Token 中提取的用户 ID（由身份验证中间件注入）
pub async fn get_profile(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<UserResponse>> {
    // 根据用户 ID 查询用户信息
    let user = UserService::get_user_by_id(&app_state.pool, user_id).await?;

    // 转换为响应格式并返回
    Ok(Json(user.into()))
}

/// 获取所有用户列表处理器
///
/// 返回系统中所有用户的列表。
/// 需要身份验证，但不进行特殊权限检查。
///
/// # 请求
///
/// - **方法**: GET
/// - **路径**: `/api/users`
/// - **请求头**: `Authorization: Bearer <jwt_token>`
///
/// # 响应
///
/// 成功时返回用户列表：
/// ```json
/// [
///   {
///     "id": "user1_uuid",
///     "email": "user1@example.com",
///     "name": "用户1",
///     "created_at": "2023-01-01T00:00:00Z"
///   },
///   {
///     "id": "user2_uuid",
///     "email": "user2@example.com",
///     "name": "用户2",
///     "created_at": "2023-01-02T00:00:00Z"
///   }
/// ]
/// ```
///
/// # 错误
///
/// - `401 Unauthorized`: JWT Token 无效或已过期
/// - `500 Internal Server Error`: 服务器内部错误
///
/// # 参数
///
/// * `app_state` - 应用程序状态，包含数据库连接池
/// * `_user_id` - 从 JWT Token 中提取的用户 ID（用于验证身份，但不使用）
pub async fn get_all_users(
    State(app_state): State<AppState>,
    Extension(_user_id): Extension<Uuid>, // 需要身份验证，但不使用具体的用户 ID
) -> Result<Json<Vec<UserResponse>>> {
    // 获取所有用户列表
    let users = UserService::get_all_users(&app_state.pool).await?;

    // 将 User 转换为 UserResponse，隐藏敏感信息如密码哈希
    let user_responses: Vec<UserResponse> = users.into_iter().map(|user| user.into()).collect();

    Ok(Json(user_responses))
}
