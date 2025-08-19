/*!
 * 数据库连接模块
 * 
 * 负责管理 PostgreSQL 数据库连接池和数据库迁移。
 * 使用 SQLx 作为数据库访问层。
 */

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

/// 数据库连接池类型别名
/// 
/// 使用 PostgreSQL 连接池，提供连接复用和连接管理功能
pub type DbPool = Pool<Postgres>;

/// 创建数据库连接池
/// 
/// 建立与 PostgreSQL 数据库的连接池，并自动运行数据库迁移。
/// 连接池可以有效管理数据库连接，避免频繁建立和关闭连接的开销。
/// 
/// # 参数
/// 
/// * `database_url` - PostgreSQL 数据库连接 URL
///   格式：`postgresql://用户名:密码@主机:端口/数据库名`
/// 
/// # 连接池配置
/// 
/// - 最大连接数：10
/// - 连接获取超时：30 秒
/// 
/// # 返回值
/// 
/// 返回 `anyhow::Result<DbPool>`，如果连接失败或迁移失败则返回错误
/// 
/// # 错误
/// 
/// - 数据库连接失败
/// - 数据库迁移执行失败
/// 
/// # 示例
/// 
/// ```no_run
/// use hello_rust::db::create_pool;
/// 
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let database_url = "postgresql://postgres:password@localhost/mydb";
///     let pool = create_pool(database_url).await?;
///     
///     // 使用连接池执行数据库操作
///     // ...
///     
///     Ok(())
/// }
/// ```
pub async fn create_pool(database_url: &str) -> anyhow::Result<DbPool> {
    // 创建 PostgreSQL 连接池
    let pool = PgPoolOptions::new()
        .max_connections(10)                      // 设置最大连接数为 10
        .acquire_timeout(Duration::from_secs(30)) // 设置连接获取超时为 30 秒
        .connect(database_url)                    // 连接到数据库
        .await?;

    // 自动运行数据库迁移
    // 这会执行 migrations/ 目录下的所有迁移文件
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
