/*!
 * 应用配置模块
 * 
 * 负责从环境变量加载和管理应用程序的配置信息。
 * 支持 .env 文件和系统环境变量。
 */

use serde::{Deserialize, Serialize};
use std::env;

/// 应用程序配置结构体
/// 
/// 包含应用程序运行所需的所有配置项，包括：
/// - 数据库连接信息
/// - JWT 密钥
/// - 服务器监听地址和端口
/// - 开发环境设置
/// - 数据库连接池配置
/// - CORS 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 数据库连接 URL
    /// 格式：postgresql://用户名:密码@主机:端口/数据库名
    pub database_url: String,
    
    /// JWT Token 签名密钥
    /// 生产环境中必须使用安全的随机字符串
    pub jwt_secret: String,
    
    /// 服务器监听端口
    pub port: u16,
    
    /// 服务器监听主机地址
    /// 0.0.0.0 表示监听所有网络接口
    pub host: String,
    
    /// 是否为开发模式
    pub development_mode: bool,
    
    /// 数据库连接池最大连接数
    pub db_max_connections: u32,
    
    /// 数据库连接池最小连接数
    pub db_min_connections: u32,
    
    /// 数据库连接超时时间（秒）
    pub db_connection_timeout: u64,
    
    /// CORS 允许的源列表
    pub cors_allowed_origins: Option<Vec<String>>,
    
    /// Redis 连接 URL
    /// 格式：redis://用户名:密码@主机:端口/数据库编号
    pub redis_url: String,
    
    /// Redis 连接池最大连接数
    pub redis_max_connections: u32,
    
    /// Redis 连接超时时间（秒）
    pub redis_connection_timeout: u64,
    
    /// Redis 键的默认过期时间（秒）
    pub redis_default_expiry: Option<u64>,
}

impl Config {
    /// 从环境变量创建配置实例
    /// 
    /// 首先尝试加载 .env 文件，然后读取环境变量。
    /// 如果某些配置项未设置，则使用默认值。
    /// 
    /// # 环境变量
    /// 
    /// - `DATABASE_URL`: 数据库连接 URL
    /// - `JWT_SECRET`: JWT 签名密钥
    /// - `PORT`: 服务器端口号
    /// - `HOST`: 服务器主机地址
    /// - `DEVELOPMENT_MODE`: 开发模式开关
    /// - `DB_MAX_CONNECTIONS`: 数据库连接池最大连接数
    /// - `DB_MIN_CONNECTIONS`: 数据库连接池最小连接数
    /// - `DB_CONNECTION_TIMEOUT`: 数据库连接超时时间
    /// - `CORS_ALLOWED_ORIGINS`: CORS 允许的源列表（逗号分隔）
    /// - `REDIS_URL`: Redis 连接 URL
    /// - `REDIS_MAX_CONNECTIONS`: Redis 连接池最大连接数
    /// - `REDIS_CONNECTION_TIMEOUT`: Redis 连接超时时间
    /// - `REDIS_DEFAULT_EXPIRY`: Redis 键的默认过期时间
    /// 
    /// # 返回值
    /// 
    /// 返回 `anyhow::Result<Config>`，如果配置解析失败则返回错误
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// use hello_rust::Config;
    /// 
    /// let config = Config::from_env().expect("Failed to load config");
    /// println!("Server will listen on {}", config.server_address());
    /// ```
    pub fn from_env() -> anyhow::Result<Self> {
        // 尝试加载 .env 文件（如果存在）
        dotenvy::dotenv().ok();

        Ok(Config {
            // 数据库连接 URL，默认连接到本地 PostgreSQL
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://postgres:password@localhost/hello_rust".to_string()),
            
            // JWT 密钥，生产环境中应该使用强随机密钥
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key-change-this-in-production".to_string()),
            
            // 服务器端口，默认 3000
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?, // 解析为 u16，如果解析失败会返回错误
            
            // 服务器主机地址，默认监听所有接口
            host: env::var("HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            
            // 开发模式，默认为 false
            development_mode: env::var("DEVELOPMENT_MODE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            
            // 数据库连接池最大连接数，默认 10
            db_max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            
            // 数据库连接池最小连接数，默认 1
            db_min_connections: env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .unwrap_or(1),
            
            // 数据库连接超时时间，默认 30 秒
            db_connection_timeout: env::var("DB_CONNECTION_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            
            // CORS 允许的源列表，从逗号分隔的字符串解析
            cors_allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                .ok()
                .map(|origins| {
                    origins
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                }),
            
            // Redis 连接 URL，默认连接到本地 Redis
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379/0".to_string()),
            
            // Redis 连接池最大连接数，默认 10
            redis_max_connections: env::var("REDIS_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            
            // Redis 连接超时时间，默认 30 秒
            redis_connection_timeout: env::var("REDIS_CONNECTION_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            
            // Redis 键的默认过期时间，可选配置
            redis_default_expiry: env::var("REDIS_DEFAULT_EXPIRY")
                .ok()
                .and_then(|s| s.parse().ok()),
        })
    }

    /// 获取服务器完整地址
    /// 
    /// 将主机地址和端口组合成完整的服务器地址。
    /// 
    /// # 返回值
    /// 
    /// 返回格式为 "主机:端口" 的字符串
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// use hello_rust::Config;
    /// 
    /// let config = Config::from_env().unwrap();
    /// let address = config.server_address(); // 例如: "0.0.0.0:3000"
    /// ```
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
