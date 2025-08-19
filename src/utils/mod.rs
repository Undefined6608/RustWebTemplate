/*!
 * 工具函数模块
 * 
 * 提供各种通用的工具函数和辅助功能，包括：
 * - 身份验证和 JWT 处理
 * - 密码加密和验证
 * - 时间和日期处理
 * - 字符串操作
 * - 数字计算
 * - 集合操作
 * - 加密和编码
 * - 类型转换
 * - 格式化输出
 * 
 * # 设计原则
 * 
 * - **纯函数**: 大部分工具函数都是纯函数，无副作用
 * - **类型安全**: 充分利用 Rust 的类型系统保证安全性
 * - **性能优化**: 针对常用操作进行性能优化
 * - **易于测试**: 提供完整的单元测试覆盖
 * 
 * # 子模块
 * 
 * - `auth`: JWT Token 生成和验证
 * - `password`: 密码哈希和验证
 * - `time`: 时间日期处理和时区转换
 * - `string`: 字符串操作和验证
 * - `number`: 数字计算和统计
 * - `collection`: 集合操作和数据结构
 * - `crypto`: 加密、编码和哈希
 * - `convert`: 类型转换和数据格式转换
 * - `format`: 格式化输出和显示
 */

/// JWT 身份验证工具
pub mod auth;

/// 密码处理工具
pub mod password;

/// 时间日期工具
pub mod time;

/// 字符串处理工具
pub mod string;

/// 数字计算工具
pub mod number;

/// 集合操作工具
pub mod collection;

/// 加密编码工具
pub mod crypto;

/// 类型转换工具
pub mod convert;

/// 格式化工具
pub mod format;

// 重新导出所有工具函数，方便外部使用
pub use auth::*;
pub use password::*;
pub use time::*;
pub use string::*;
pub use number::*;
pub use collection::*;
pub use crypto::*;
pub use convert::*;
pub use format::*;
