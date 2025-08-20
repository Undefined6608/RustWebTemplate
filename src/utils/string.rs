use regex::Regex;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

/// 字符串工具结构体
pub struct StringUtils;

impl StringUtils {
    /// 判断字符串是否为空或只包含空白字符
    pub fn is_blank(s: &str) -> bool {
        s.trim().is_empty()
    }

    /// 判断字符串是否不为空且不只包含空白字符
    pub fn is_not_blank(s: &str) -> bool {
        !Self::is_blank(s)
    }

    /// 截断字符串到指定长度
    pub fn truncate(s: &str, max_length: usize) -> String {
        let mut result = String::new();
        let mut length = 0;

        for grapheme in s.graphemes(true) {
            if length + grapheme.len() > max_length {
                break;
            }
            result.push_str(grapheme);
            length += grapheme.len();
        }

        result
    }

    /// 截断字符串并添加省略号
    pub fn truncate_with_ellipsis(s: &str, max_length: usize) -> String {
        if s.len() <= max_length {
            s.to_string()
        } else {
            let truncated = Self::truncate(s, max_length.saturating_sub(3));
            format!("{}...", truncated)
        }
    }

    /// 移除所有空白字符
    pub fn remove_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }

    /// 驼峰命名转下划线命名
    pub fn camel_to_snake(s: &str) -> String {
        let mut result = String::new();
        let mut prev_lowercase = false;

        for c in s.chars() {
            if c.is_uppercase() && prev_lowercase {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap_or(c));
            prev_lowercase = c.is_lowercase();
        }

        result
    }

    /// 下划线命名转驼峰命名
    pub fn snake_to_camel(s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;

        for c in s.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap_or(c));
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }

        result
    }

    /// 首字母大写
    pub fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }

    /// 每个单词首字母大写
    pub fn title_case(s: &str) -> String {
        s.split_whitespace()
            .map(Self::capitalize)
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// 反转字符串（支持 Unicode）
    pub fn reverse(s: &str) -> String {
        s.graphemes(true).rev().collect()
    }

    /// 计算字符串的字符数（支持 Unicode）
    pub fn char_count(s: &str) -> usize {
        s.graphemes(true).count()
    }

    /// 左填充
    pub fn pad_left(s: &str, width: usize, pad_char: char) -> String {
        let current_width = Self::char_count(s);
        if current_width >= width {
            s.to_string()
        } else {
            let padding = pad_char.to_string().repeat(width - current_width);
            format!("{}{}", padding, s)
        }
    }

    /// 右填充
    pub fn pad_right(s: &str, width: usize, pad_char: char) -> String {
        let current_width = Self::char_count(s);
        if current_width >= width {
            s.to_string()
        } else {
            let padding = pad_char.to_string().repeat(width - current_width);
            format!("{}{}", s, padding)
        }
    }

    /// 居中填充
    pub fn pad_center(s: &str, width: usize, pad_char: char) -> String {
        let current_width = Self::char_count(s);
        if current_width >= width {
            s.to_string()
        } else {
            let total_padding = width - current_width;
            let left_padding = total_padding / 2;
            let right_padding = total_padding - left_padding;

            let left_pad = pad_char.to_string().repeat(left_padding);
            let right_pad = pad_char.to_string().repeat(right_padding);

            format!("{}{}{}", left_pad, s, right_pad)
        }
    }

    /// 移除前缀
    pub fn remove_prefix(s: &str, prefix: &str) -> String {
        if s.starts_with(prefix) {
            s[prefix.len()..].to_string()
        } else {
            s.to_string()
        }
    }

    /// 移除后缀
    pub fn remove_suffix(s: &str, suffix: &str) -> String {
        if s.ends_with(suffix) {
            s[..s.len() - suffix.len()].to_string()
        } else {
            s.to_string()
        }
    }

    /// 提取数字
    pub fn extract_numbers(s: &str) -> Vec<String> {
        let re = Regex::new(r"\d+").unwrap();
        re.find_iter(s).map(|m| m.as_str().to_string()).collect()
    }

    /// 提取邮箱地址
    pub fn extract_emails(s: &str) -> Vec<String> {
        let re = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
        re.find_iter(s).map(|m| m.as_str().to_string()).collect()
    }

    /// 提取 URL
    pub fn extract_urls(s: &str) -> Vec<String> {
        let re = Regex::new(r"https?://[^\s]+").unwrap();
        re.find_iter(s).map(|m| m.as_str().to_string()).collect()
    }

    /// 验证邮箱格式
    pub fn is_valid_email(email: &str) -> bool {
        let re = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}$").unwrap();
        re.is_match(email)
    }

    /// 验证手机号格式（中国）
    pub fn is_valid_phone_cn(phone: &str) -> bool {
        let re = Regex::new(r"^1[3-9]\d{9}$").unwrap();
        re.is_match(phone)
    }

    /// 验证身份证号格式（中国）
    pub fn is_valid_id_card_cn(id_card: &str) -> bool {
        let re = Regex::new(
            r"^[1-9]\d{5}(18|19|20)\d{2}((0[1-9])|(1[0-2]))(([0-2][1-9])|10|20|30|31)\d{3}[0-9Xx]$",
        )
        .unwrap();
        re.is_match(id_card)
    }

    /// 生成随机字符串
    pub fn random_string(length: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789";
        let mut rng = rand::thread_rng();

        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// 生成随机数字字符串
    pub fn random_numeric_string(length: usize) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        (0..length)
            .map(|_| rng.gen_range(0..10).to_string())
            .collect()
    }

    /// 计算字符串相似度（编辑距离）
    pub fn similarity(s1: &str, s2: &str) -> f64 {
        let distance = Self::levenshtein_distance(s1, s2);
        let max_len = s1.len().max(s2.len());

        if max_len == 0 {
            1.0
        } else {
            1.0 - (distance as f64 / max_len as f64)
        }
    }

    /// 计算编辑距离
    pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let s1_len = s1_chars.len();
        let s2_len = s2_chars.len();

        let mut matrix = vec![vec![0; s2_len + 1]; s1_len + 1];

        for i in 0..=s1_len {
            matrix[i][0] = i;
        }

        for j in 0..=s2_len {
            matrix[0][j] = j;
        }

        for i in 1..=s1_len {
            for j in 1..=s2_len {
                let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                    0
                } else {
                    1
                };
                matrix[i][j] = [
                    matrix[i - 1][j] + 1,        // 删除
                    matrix[i][j - 1] + 1,        // 插入
                    matrix[i - 1][j - 1] + cost, // 替换
                ]
                .iter()
                .min()
                .unwrap()
                .clone();
            }
        }

        matrix[s1_len][s2_len]
    }

    /// 字符串模板替换
    pub fn template_replace(template: &str, variables: &HashMap<String, String>) -> String {
        let mut result = template.to_string();

        for (key, value) in variables {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, value);
        }

        result
    }

    /// 移除 HTML 标签
    pub fn strip_html(html: &str) -> String {
        let re = Regex::new(r"<[^>]*>").unwrap();
        re.replace_all(html, "").to_string()
    }

    /// 转义 HTML 字符
    pub fn escape_html(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    }

    /// 单词计数
    pub fn word_count(s: &str) -> usize {
        s.split_whitespace().count()
    }

    /// 计算字符串哈希值
    pub fn hash_string(s: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_blank() {
        assert!(StringUtils::is_blank(""));
        assert!(StringUtils::is_blank("   "));
        assert!(StringUtils::is_blank("\t\n"));
        assert!(!StringUtils::is_blank("hello"));
    }

    #[test]
    fn test_camel_snake_conversion() {
        assert_eq!(StringUtils::camel_to_snake("camelCase"), "camel_case");
        assert_eq!(StringUtils::snake_to_camel("snake_case"), "snakeCase");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(StringUtils::truncate("Hello World", 5), "Hello");
        assert_eq!(
            StringUtils::truncate_with_ellipsis("Hello World", 8),
            "Hello..."
        );
    }

    #[test]
    fn test_validation() {
        assert!(StringUtils::is_valid_email("test@example.com"));
        assert!(!StringUtils::is_valid_email("invalid-email"));
    }

    #[test]
    fn test_similarity() {
        assert!(StringUtils::similarity("hello", "hello") == 1.0);
        assert!(StringUtils::similarity("hello", "world") < 1.0);
    }
}
