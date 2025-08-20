/*!
 * 路由配置模块
 *
 * 负责定义应用程序的所有路由和路由组织结构。
 * 包含公开路由和需要身份验证的受保护路由。
 */

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    config::Config,
    db::DbPool,
    handlers::{
        get_all_users, get_profile, get_sessions, login, logout, logout_all, logout_device,
        register,
    },
    middleware::auth_middleware,
    redis::RedisManager,
};

/// 应用程序状态
///
/// 包含在整个应用程序生命周期中需要共享的数据，
/// 如数据库连接池、Redis管理器和配置信息。
#[derive(Clone)]
pub struct AppState {
    /// 数据库连接池
    pub pool: DbPool,
    /// Redis管理器
    pub redis: RedisManager,
    /// 应用配置
    pub config: Config,
}

/// 创建应用程序路由
///
/// 组织应用程序的所有路由，包括：
/// - 公开的身份验证路由 (`/api/auth`)
/// - 需要身份验证的受保护路由 (`/api`)
/// - 健康检查路由 (`/health`)
///
/// # 参数
///
/// * `pool` - 数据库连接池
/// * `redis_manager` - Redis管理器
/// * `config` - 应用配置
///
/// # 返回值
///
/// 返回配置好的 Axum Router
pub fn create_routes(pool: DbPool, redis_manager: RedisManager, config: Config) -> Router {
    // 创建应用状态，包含共享的数据库连接池、Redis管理器和配置
    let app_state = AppState {
        pool,
        redis: redis_manager,
        config: config.clone(),
    };

    // 公开的身份验证路由
    // 这些路由不需要用户登录即可访问
    let auth_routes = Router::new()
        .route("/register", post(register)) // 用户注册
        .route("/login", post(login)) // 用户登录
        .route("/logout", post(logout)) // 退出登录（需要token）
        .route("/logout-all", post(logout_all)) // 退出所有设备（需要token）
        .route("/sessions", get(get_sessions)) // 获取活跃会话列表（需要token）
        .route("/logout-device/:device_type", post(logout_device)); // 撤销特定设备登录（需要token）

    // 受保护的路由
    // 这些路由需要有效的 JWT Token 才能访问
    let protected_routes = Router::new()
        .route("/profile", get(get_profile)) // 获取用户个人信息
        .route("/users", get(get_all_users)) // 获取所有用户列表
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        )); // 应用身份验证中间件

    // 组合所有路由
    Router::new()
        .nest("/api/auth", auth_routes) // 挂载身份验证路由到 /api/auth
        .nest("/api", protected_routes) // 挂载受保护路由到 /api
        .route("/health", get(health_check)) // 健康检查端点
        .with_state(app_state) // 设置应用状态
}

/// 健康检查处理器
///
/// 提供一个简单的健康检查端点，用于监控服务是否正常运行。
///
/// # 返回值
///
/// 返回字符串 "OK" 表示服务正常
async fn health_check() -> &'static str {
    "OK"
}
