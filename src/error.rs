/*!
 * 错误处理模块
 *
 * 定义应用程序的统一错误类型和错误处理机制。
 * 所有错误都会自动转换为适当的 HTTP 响应。
 */

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// 应用程序通用结果类型
///
/// 简化错误处理，统一使用 `AppError` 作为错误类型
pub type Result<T> = std::result::Result<T, AppError>;

/// 应用程序错误枚举
///
/// 定义了应用程序中可能出现的所有错误类型，
/// 每种错误都会映射到相应的 HTTP 状态码。
#[derive(Error, Debug)]
pub enum AppError {
    /// 数据库操作错误
    ///
    /// 包括连接失败、查询错误、约束违反等数据库相关错误
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// JWT Token 相关错误
    ///
    /// 包括 Token 签名、验证、解析等 JWT 相关错误
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    /// 密码哈希处理错误
    ///
    /// 密码加密或验证过程中的错误
    #[error("Password hashing error")]
    PasswordHash,

    /// 数据验证错误
    ///
    /// 用户输入数据格式不正确或不符合业务规则
    #[error("Validation error: {0}")]
    Validation(String),

    /// 身份验证错误
    ///
    /// 用户身份验证失败，如密码错误、Token 无效等
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// 授权错误
    ///
    /// 用户没有执行某操作的权限
    #[error("Authorization error: {0}")]
    Authorization(String),

    /// 资源未找到错误
    ///
    /// 请求的资源不存在
    #[error("Not found: {0}")]
    NotFound(String),

    /// 资源冲突错误
    ///
    /// 资源已存在或状态冲突，如用户邮箱重复
    #[error("Conflict: {0}")]
    Conflict(String),

    /// 内部服务器错误
    ///
    /// 其他未预期的系统错误
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    /// 将应用程序错误转换为 HTTP 响应
    ///
    /// 根据错误类型返回相应的 HTTP 状态码和错误消息。
    /// 敏感的错误信息（如数据库错误）会被隐藏，只返回通用的错误消息。
    ///
    /// # 错误映射
    ///
    /// - `Database` -> 500 Internal Server Error
    /// - `Jwt` -> 401 Unauthorized  
    /// - `PasswordHash` -> 500 Internal Server Error
    /// - `Validation` -> 400 Bad Request
    /// - `Authentication` -> 401 Unauthorized
    /// - `Authorization` -> 403 Forbidden
    /// - `NotFound` -> 404 Not Found
    /// - `Conflict` -> 409 Conflict
    /// - `Internal` -> 500 Internal Server Error
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            // 数据库错误：记录详细错误日志，但不向客户端暴露敏感信息
            AppError::Database(err) => {
                tracing::error!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }

            // JWT 错误：Token 无效或已过期
            AppError::Jwt(_) => (StatusCode::UNAUTHORIZED, "Invalid token"),

            // 密码哈希错误：记录错误日志，返回通用错误消息
            AppError::PasswordHash => {
                tracing::error!("Password hashing error occurred");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }

            // 验证错误：返回具体的验证失败原因
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),

            // 身份验证错误：用户名密码错误等
            AppError::Authentication(msg) => (StatusCode::UNAUTHORIZED, msg.as_str()),

            // 授权错误：权限不足
            AppError::Authorization(msg) => (StatusCode::FORBIDDEN, msg.as_str()),

            // 资源未找到错误
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.as_str()),

            // 资源冲突错误：如邮箱已存在
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.as_str()),

            // 内部错误：记录详细错误日志
            AppError::Internal(err) => {
                tracing::error!("Internal error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        // 构造 JSON 错误响应
        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
