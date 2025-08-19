/*!
 * Hello Rust Web 应用程序库
 * 
 * 这是一个使用 Rust 构建的现代化 Web 应用程序模板，包含：
 * - JWT 身份验证
 * - PostgreSQL 数据库集成
 * - RESTful API 设计
 * - 丰富的工具函数库
 * - 完整的错误处理
 * 
 * # 模块组织
 * 
 * - `config`: 应用配置管理
 * - `db`: 数据库连接和操作
 * - `error`: 统一错误处理
 * - `redis`: Redis 缓存和工具
 * - `handlers`: HTTP 请求处理器
 * - `middleware`: 中间件（如身份验证）
 * - `models`: 数据模型定义
 * - `routes`: 路由配置
 * - `services`: 业务逻辑服务
 * - `utils`: 通用工具函数
 */

// 核心模块
pub mod config;
pub mod db;
pub mod error;
pub mod redis;

// Web 相关模块
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;

// 业务逻辑模块
pub mod services;

// 工具函数模块
pub mod utils;

// 重新导出常用类型，方便外部使用
pub use config::Config;
pub use error::{AppError, Result};
pub use redis::{RedisManager, RedisUtils};
