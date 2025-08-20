use base64::{engine::general_purpose, Engine as _};
use hex;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use uuid::Uuid;

/// 加密工具结构体
pub struct CryptoUtils;

impl CryptoUtils {
    /// Base64 编码
    pub fn base64_encode(data: &[u8]) -> String {
        general_purpose::STANDARD.encode(data)
    }

    /// Base64 解码
    pub fn base64_decode(encoded: &str) -> Result<Vec<u8>, base64::DecodeError> {
        general_purpose::STANDARD.decode(encoded)
    }

    /// Base64 URL 安全编码
    pub fn base64_url_encode(data: &[u8]) -> String {
        general_purpose::URL_SAFE_NO_PAD.encode(data)
    }

    /// Base64 URL 安全解码
    pub fn base64_url_decode(encoded: &str) -> Result<Vec<u8>, base64::DecodeError> {
        general_purpose::URL_SAFE_NO_PAD.decode(encoded)
    }

    /// 十六进制编码
    pub fn hex_encode(data: &[u8]) -> String {
        hex::encode(data)
    }

    /// 十六进制解码
    pub fn hex_decode(encoded: &str) -> Result<Vec<u8>, hex::FromHexError> {
        hex::decode(encoded)
    }

    /// 计算字符串的哈希值
    pub fn hash_string(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }

    /// 计算字节数组的哈希值
    pub fn hash_bytes(data: &[u8]) -> u64 {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }

    /// 生成随机字节数组
    pub fn random_bytes(length: usize) -> Vec<u8> {
        use rand::RngCore;
        let mut bytes = vec![0u8; length];
        rand::thread_rng().fill_bytes(&mut bytes);
        bytes
    }

    /// 生成随机十六进制字符串
    pub fn random_hex(length: usize) -> String {
        let bytes = Self::random_bytes(length);
        Self::hex_encode(&bytes)
    }

    /// 生成随机 Base64 字符串
    pub fn random_base64(length: usize) -> String {
        let bytes = Self::random_bytes(length);
        Self::base64_encode(&bytes)
    }

    /// 生成 UUID v4
    pub fn generate_uuid() -> String {
        Uuid::new_v4().to_string()
    }

    /// 生成不带连字符的 UUID
    pub fn generate_uuid_simple() -> String {
        Uuid::new_v4().simple().to_string()
    }

    /// 简单的凯撒密码加密
    pub fn caesar_encrypt(text: &str, shift: u8) -> String {
        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let shifted = (c as u8 - base + shift) % 26 + base;
                    shifted as char
                } else {
                    c
                }
            })
            .collect()
    }

    /// 简单的凯撒密码解密
    pub fn caesar_decrypt(text: &str, shift: u8) -> String {
        Self::caesar_encrypt(text, 26 - (shift % 26))
    }

    /// ROT13 编码/解码
    pub fn rot13(text: &str) -> String {
        Self::caesar_encrypt(text, 13)
    }

    /// 简单的异或加密/解密
    pub fn xor_encrypt_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
        if key.is_empty() {
            return data.to_vec();
        }

        data.iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ key[i % key.len()])
            .collect()
    }

    /// 字符串异或加密/解密
    pub fn xor_string_encrypt_decrypt(text: &str, key: &str) -> String {
        let encrypted_bytes = Self::xor_encrypt_decrypt(text.as_bytes(), key.as_bytes());
        String::from_utf8_lossy(&encrypted_bytes).to_string()
    }

    /// 生成安全的随机密码
    pub fn generate_password(
        length: usize,
        include_uppercase: bool,
        include_lowercase: bool,
        include_numbers: bool,
        include_symbols: bool,
    ) -> String {
        use rand::seq::SliceRandom;

        let mut charset = Vec::new();

        if include_lowercase {
            charset.extend_from_slice(b"abcdefghijklmnopqrstuvwxyz");
        }
        if include_uppercase {
            charset.extend_from_slice(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        if include_numbers {
            charset.extend_from_slice(b"0123456789");
        }
        if include_symbols {
            charset.extend_from_slice(b"!@#$%^&*()_+-=[]{}|;:,.<>?");
        }

        if charset.is_empty() {
            return String::new();
        }

        (0..length)
            .map(|_| *charset.choose(&mut rand::thread_rng()).unwrap() as char)
            .collect()
    }

    /// 生成默认配置的安全密码
    pub fn generate_secure_password(length: usize) -> String {
        Self::generate_password(length, true, true, true, true)
    }

    /// 检查密码强度
    pub fn check_password_strength(password: &str) -> PasswordStrength {
        let mut score = 0;
        let mut feedback = Vec::new();

        // 长度检查
        if password.len() >= 8 {
            score += 1;
        } else {
            feedback.push("密码长度至少需要8位".to_string());
        }

        if password.len() >= 12 {
            score += 1;
        }

        // 包含小写字母
        if password.chars().any(|c| c.is_ascii_lowercase()) {
            score += 1;
        } else {
            feedback.push("密码应包含小写字母".to_string());
        }

        // 包含大写字母
        if password.chars().any(|c| c.is_ascii_uppercase()) {
            score += 1;
        } else {
            feedback.push("密码应包含大写字母".to_string());
        }

        // 包含数字
        if password.chars().any(|c| c.is_ascii_digit()) {
            score += 1;
        } else {
            feedback.push("密码应包含数字".to_string());
        }

        // 包含特殊字符
        if password
            .chars()
            .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c))
        {
            score += 1;
        } else {
            feedback.push("密码应包含特殊字符".to_string());
        }

        let level = match score {
            0..=2 => StrengthLevel::Weak,
            3..=4 => StrengthLevel::Medium,
            _ => StrengthLevel::Strong,
        };

        PasswordStrength {
            level,
            score,
            feedback,
        }
    }

    /// 计算文件哈希（模拟，实际使用时需要加载真实的哈希库）
    pub fn hash_data_simple(_data: &[u8]) -> String {
        Self::hex_encode(&Self::random_bytes(32)) // 简化实现
    }

    /// 时间戳签名（简单实现）
    pub fn timestamp_signature(data: &str, secret: &str) -> String {
        use crate::utils::time::TimeUtils;

        let timestamp = TimeUtils::timestamp();
        let payload = format!("{}.{}", timestamp, data);
        let hash = Self::hash_string(&format!("{}{}", payload, secret));

        format!("{}.{:x}", payload, hash)
    }

    /// 验证时间戳签名
    pub fn verify_timestamp_signature(signature: &str, secret: &str, max_age_seconds: i64) -> bool {
        use crate::utils::time::TimeUtils;

        let parts: Vec<&str> = signature.split('.').collect();
        if parts.len() != 3 {
            return false;
        }

        let timestamp: i64 = match parts[0].parse() {
            Ok(ts) => ts,
            Err(_) => return false,
        };

        let data = parts[1];
        let provided_hash = parts[2];

        // 检查时间戳是否过期
        let current_timestamp = TimeUtils::timestamp();
        if current_timestamp - timestamp > max_age_seconds {
            return false;
        }

        // 验证签名
        let payload = format!("{}.{}", timestamp, data);
        let expected_hash = format!("{:x}", Self::hash_string(&format!("{}{}", payload, secret)));

        provided_hash == expected_hash
    }

    /// URL 安全的 Base64 编码字符串
    pub fn url_safe_encode(data: &str) -> String {
        Self::base64_url_encode(data.as_bytes())
    }

    /// URL 安全的 Base64 解码字符串
    pub fn url_safe_decode(encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        let bytes = Self::base64_url_decode(encoded)?;
        Ok(String::from_utf8(bytes)?)
    }
}

/// 密码强度等级
#[derive(Debug, Clone, PartialEq)]
pub enum StrengthLevel {
    Weak,
    Medium,
    Strong,
}

/// 密码强度检查结果
#[derive(Debug, Clone)]
pub struct PasswordStrength {
    pub level: StrengthLevel,
    pub score: u8,
    pub feedback: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encoding() {
        let data = b"Hello, World!";
        let encoded = CryptoUtils::base64_encode(data);
        let decoded = CryptoUtils::base64_decode(&encoded).unwrap();
        assert_eq!(data, &decoded[..]);
    }

    #[test]
    fn test_hex_encoding() {
        let data = b"Hello";
        let encoded = CryptoUtils::hex_encode(data);
        let decoded = CryptoUtils::hex_decode(&encoded).unwrap();
        assert_eq!(data, &decoded[..]);
    }

    #[test]
    fn test_caesar_cipher() {
        let text = "Hello";
        let encrypted = CryptoUtils::caesar_encrypt(text, 3);
        let decrypted = CryptoUtils::caesar_decrypt(&encrypted, 3);
        assert_eq!(text, decrypted);
    }

    #[test]
    fn test_rot13() {
        let text = "Hello";
        let encoded = CryptoUtils::rot13(text);
        let decoded = CryptoUtils::rot13(&encoded);
        assert_eq!(text, decoded);
    }

    #[test]
    fn test_xor_encryption() {
        let data = b"Hello, World!";
        let key = b"secret";
        let encrypted = CryptoUtils::xor_encrypt_decrypt(data, key);
        let decrypted = CryptoUtils::xor_encrypt_decrypt(&encrypted, key);
        assert_eq!(data, &decrypted[..]);
    }

    #[test]
    fn test_password_generation() {
        let password = CryptoUtils::generate_secure_password(12);
        assert_eq!(password.len(), 12);
    }

    #[test]
    fn test_password_strength() {
        let weak_password = "123";
        let strong_password = "Str0ng!P@ssw0rd";

        let weak_strength = CryptoUtils::check_password_strength(weak_password);
        let strong_strength = CryptoUtils::check_password_strength(strong_password);

        assert_eq!(weak_strength.level, StrengthLevel::Weak);
        assert_eq!(strong_strength.level, StrengthLevel::Strong);
    }

    #[test]
    fn test_uuid_generation() {
        let uuid1 = CryptoUtils::generate_uuid();
        let uuid2 = CryptoUtils::generate_uuid();
        assert_ne!(uuid1, uuid2);
        assert_eq!(uuid1.len(), 36); // 包含连字符的 UUID 长度

        let simple_uuid = CryptoUtils::generate_uuid_simple();
        assert_eq!(simple_uuid.len(), 32); // 不包含连字符的 UUID 长度
    }
}
