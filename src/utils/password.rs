/*!
 * 密码哈希和验证工具
 *
 * 使用 Argon2 算法提供安全的密码哈希和验证功能。
 * Argon2 是现代密码哈希的金标准，抗彩虹表和暴力破解攻击。
 */

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::error::Result;

/// 哈希密码
///
/// 使用 Argon2 算法对明文密码进行安全哈希处理。
/// 每次调用都会生成新的随机盐值，确保相同密码产生不同的哈希值。
///
/// # 安全特性
///
/// - **Argon2**: 2015年密码哈希竞赛获奖算法
/// - **随机盐值**: 每次哈希都使用新的随机盐值
/// - **内存困难**: 抗 ASIC 和 GPU 加速攻击
/// - **时间困难**: 可调节的时间成本参数
///
/// # 参数
///
/// * `password` - 要哈希的明文密码
///
/// # 返回值
///
/// 返回 `Result<String>`，成功时包含 Argon2 哈希字符串
///
/// # 错误
///
/// - `AppError::PasswordHash`: 密码哈希处理失败
///
/// # 哈希格式
///
/// 返回的哈希字符串包含：
/// - 算法标识符 (argon2id)
/// - 参数配置 (内存、迭代次数、并行度)
/// - 盐值 (Base64 编码)
/// - 哈希值 (Base64 编码)
///
/// 格式示例：
/// ```
/// $argon2id$v=19$m=19456,t=2,p=1$salt$hash
/// ```
///
/// # 示例
///
/// ```rust
/// use crate::utils::password::hash_password;
///
/// let password = "my_secure_password";
/// let hash = hash_password(password)?;
/// println!("Password hash: {}", hash);
///
/// // 每次调用都会产生不同的哈希值
/// let hash2 = hash_password(password)?;
/// assert_ne!(hash, hash2); // 不同的哈希值
/// ```
pub fn hash_password(password: &str) -> Result<String> {
    // 生成随机盐值
    let salt = SaltString::generate(&mut OsRng);

    // 使用默认的 Argon2 配置
    let argon2 = Argon2::default();

    // 对密码进行哈希处理
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| crate::error::AppError::PasswordHash)?
        .to_string();

    Ok(password_hash)
}

/// 验证密码
///
/// 验证明文密码是否与存储的哈希值匹配。
/// 使用恒定时间比较算法，防止时序攻击。
///
/// # 验证过程
///
/// 1. 解析存储的哈希字符串，提取算法参数和盐值
/// 2. 使用相同的参数和盐值对输入密码进行哈希
/// 3. 使用恒定时间算法比较两个哈希值
///
/// # 参数
///
/// * `password` - 要验证的明文密码
/// * `hash` - 存储的密码哈希值
///
/// # 返回值
///
/// 返回 `Result<bool>`，成功时：
/// - `true`: 密码正确
/// - `false`: 密码错误
///
/// # 错误
///
/// - `AppError::PasswordHash`: 哈希格式无效或验证失败
///
/// # 安全特性
///
/// - **恒定时间**: 验证时间不依赖于密码内容，防止时序攻击
/// - **格式验证**: 自动验证哈希字符串的格式和完整性
/// - **算法兼容**: 支持不同版本的 Argon2 参数
///
/// # 示例
///
/// ```rust
/// use crate::utils::password::{hash_password, verify_password};
///
/// let password = "my_secure_password";
/// let hash = hash_password(password)?;
///
/// // 验证正确密码
/// assert!(verify_password(password, &hash)?);
///
/// // 验证错误密码
/// assert!(!verify_password("wrong_password", &hash)?);
/// ```
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    // 解析存储的哈希字符串
    let parsed_hash = PasswordHash::new(hash).map_err(|_| crate::error::AppError::PasswordHash)?;

    // 使用默认的 Argon2 验证器
    let argon2 = Argon2::default();

    // 验证密码（使用恒定时间比较）
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),  // 密码正确
        Err(_) => Ok(false), // 密码错误
    }
}
