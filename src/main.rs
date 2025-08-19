/*!
 * 主程序入口
 * 
 * 这是 Rust Web 应用的启动文件，负责：
 * - 初始化日志系统
 * - 加载配置信息
 * - 建立数据库连接池
 * - 建立Redis连接
 * - 配置路由和中间件
 * - 启动 HTTP 服务器
 */

use hello_rust::{config::Config, db::create_pool, redis::RedisManager, routes::create_routes};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// 应用程序主入口点
/// 
/// 使用 tokio 异步运行时启动 Web 服务器
/// 
/// # 返回值
/// 
/// 返回 `anyhow::Result<()>`，如果启动过程中出现错误则返回错误信息
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化追踪日志系统
    // 使用环境变量配置日志级别，默认为 debug 级别
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hello_rust=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 从环境变量加载应用配置
    let config = Config::from_env()?;
    tracing::info!("Starting server with config: {:#?}", config);

    // 创建数据库连接池
    // 连接池负责管理数据库连接，提高性能和资源利用率
    let pool = create_pool(&config.database_url).await?;
    tracing::info!("Database connection established");

    // 创建Redis连接管理器
    let redis_manager = RedisManager::new(&config).await?;
    tracing::info!("Redis connection established");

    // 创建应用路由和中间件栈
    let app = create_routes(pool, redis_manager, config.clone())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http()) // HTTP 请求追踪中间件
                .layer(CorsLayer::permissive())    // CORS 跨域支持中间件
        );

    // 启动 TCP 监听器，绑定到配置的地址和端口
    let listener = tokio::net::TcpListener::bind(&config.server_address()).await?;
    tracing::info!("Server listening on {}", config.server_address());

    // 启动 Axum HTTP 服务器
    axum::serve(listener, app).await?;

    Ok(())
}
