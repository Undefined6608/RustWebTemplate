use std::str::FromStr;
use std::fmt::Display;
use serde_json::{Value, from_str, to_string};
use url::Url;

/// 类型转换工具结构体
pub struct ConvertUtils;

impl ConvertUtils {
    /// 安全地将任何实现了 Display trait 的类型转换为字符串
    pub fn to_string<T: Display>(value: T) -> String {
        value.to_string()
    }

    /// 安全地将字符串转换为指定类型
    pub fn from_string<T: FromStr>(s: &str) -> Result<T, T::Err> {
        s.parse()
    }

    /// 字符串转 i32
    pub fn str_to_i32(s: &str) -> Option<i32> {
        s.trim().parse().ok()
    }

    /// 字符串转 i64
    pub fn str_to_i64(s: &str) -> Option<i64> {
        s.trim().parse().ok()
    }

    /// 字符串转 f32
    pub fn str_to_f32(s: &str) -> Option<f32> {
        s.trim().parse().ok()
    }

    /// 字符串转 f64
    pub fn str_to_f64(s: &str) -> Option<f64> {
        s.trim().parse().ok()
    }

    /// 字符串转 bool
    pub fn str_to_bool(s: &str) -> Option<bool> {
        match s.trim().to_lowercase().as_str() {
            "true" | "1" | "yes" | "on" | "是" | "真" => Some(true),
            "false" | "0" | "no" | "off" | "否" | "假" => Some(false),
            _ => None,
        }
    }

    /// 数字转字符串（带格式化）
    pub fn number_to_string_formatted(num: f64, decimal_places: usize) -> String {
        format!("{:.1$}", num, decimal_places)
    }

    /// 字节数组转十六进制字符串
    pub fn bytes_to_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    /// 十六进制字符串转字节数组
    pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // 简单的十六进制解码实现
        if hex.len() % 2 != 0 {
            return Err("Invalid hex string length".into());
        }
        
        let mut bytes = Vec::new();
        for chunk in hex.as_bytes().chunks(2) {
            let hex_str = std::str::from_utf8(chunk)?;
            let byte = u8::from_str_radix(hex_str, 16)?;
            bytes.push(byte);
        }
        
        Ok(bytes)
    }

    /// JSON 字符串转 Value
    pub fn json_str_to_value(json_str: &str) -> Result<Value, serde_json::Error> {
        from_str(json_str)
    }

    /// Value 转 JSON 字符串
    pub fn value_to_json_str(value: &Value) -> Result<String, serde_json::Error> {
        to_string(value)
    }

    /// 美化 JSON 字符串
    pub fn prettify_json(json_str: &str) -> Result<String, serde_json::Error> {
        let value: Value = from_str(json_str)?;
        serde_json::to_string_pretty(&value)
    }

    /// 压缩 JSON 字符串
    pub fn minify_json(json_str: &str) -> Result<String, serde_json::Error> {
        let value: Value = from_str(json_str)?;
        to_string(&value)
    }

    /// URL 编码
    pub fn url_encode(s: &str) -> String {
        urlencoding::encode(s).to_string()
    }

    /// URL 解码
    pub fn url_decode(s: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(urlencoding::decode(s)?)
    }

    /// 解析 URL
    pub fn parse_url(url_str: &str) -> Result<ParsedUrl, url::ParseError> {
        let url = Url::parse(url_str)?;
        
        Ok(ParsedUrl {
            scheme: url.scheme().to_string(),
            host: url.host_str().map(|s| s.to_string()),
            port: url.port(),
            path: url.path().to_string(),
            query: url.query().map(|s| s.to_string()),
            fragment: url.fragment().map(|s| s.to_string()),
        })
    }

    /// 构建 URL
    pub fn build_url(
        scheme: &str,
        host: &str,
        port: Option<u16>,
        path: &str,
        query_params: Option<&[(&str, &str)]>,
    ) -> Result<String, url::ParseError> {
        let mut url = Url::parse(&format!("{}://{}", scheme, host))?;
        
        if let Some(port) = port {
            url.set_port(Some(port)).map_err(|_| url::ParseError::InvalidPort)?;
        }
        
        url.set_path(path);
        
        if let Some(params) = query_params {
            let mut query_pairs = url.query_pairs_mut();
            for &(key, value) in params {
                query_pairs.append_pair(key, value);
            }
        }
        
        Ok(url.to_string())
    }

    /// 数组转换为分隔字符串
    pub fn array_to_delimited_string<T: Display>(arr: &[T], delimiter: &str) -> String {
        arr.iter()
            .map(|item| item.to_string())
            .collect::<Vec<_>>()
            .join(delimiter)
    }

    /// 分隔字符串转换为数组
    pub fn delimited_string_to_array(s: &str, delimiter: &str) -> Vec<String> {
        if s.is_empty() {
            return Vec::new();
        }
        
        s.split(delimiter)
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
            .collect()
    }

    /// CSV 行转换为数组
    pub fn csv_row_to_array(csv_row: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current_field = String::new();
        let mut in_quotes = false;
        let mut chars = csv_row.chars().peekable();
        
        while let Some(ch) = chars.next() {
            match ch {
                '"' => {
                    if in_quotes && chars.peek() == Some(&'"') {
                        // 转义的引号
                        current_field.push('"');
                        chars.next(); // 跳过下一个引号
                    } else {
                        in_quotes = !in_quotes;
                    }
                }
                ',' if !in_quotes => {
                    result.push(current_field.clone());
                    current_field.clear();
                }
                _ => {
                    current_field.push(ch);
                }
            }
        }
        
        result.push(current_field);
        result
    }

    /// 数组转换为 CSV 行
    pub fn array_to_csv_row(arr: &[String]) -> String {
        arr.iter()
            .map(|field| {
                if field.contains(',') || field.contains('"') || field.contains('\n') {
                    format!("\"{}\"", field.replace('"', "\"\""))
                } else {
                    field.clone()
                }
            })
            .collect::<Vec<_>>()
            .join(",")
    }

    /// 字节大小转换为人类可读格式
    pub fn bytes_to_human_readable(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
        const THRESHOLD: f64 = 1024.0;
        
        if bytes == 0 {
            return "0 B".to_string();
        }
        
        let mut size = bytes as f64;
        let mut unit_index = 0;
        
        while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
            size /= THRESHOLD;
            unit_index += 1;
        }
        
        if unit_index == 0 {
            format!("{} {}", bytes, UNITS[unit_index])
        } else {
            format!("{:.2} {}", size, UNITS[unit_index])
        }
    }

    /// 人类可读格式转换为字节大小
    pub fn human_readable_to_bytes(size_str: &str) -> Option<u64> {
        let size_str = size_str.trim().to_uppercase();
        let (number_part, unit_part) = if let Some(pos) = size_str.find(char::is_alphabetic) {
            (&size_str[..pos], &size_str[pos..])
        } else {
            (size_str.as_str(), "B")
        };
        
        let number: f64 = number_part.trim().parse().ok()?;
        
        let multiplier = match unit_part.trim() {
            "B" => 1,
            "KB" => 1024,
            "MB" => 1024_u64.pow(2),
            "GB" => 1024_u64.pow(3),
            "TB" => 1024_u64.pow(4),
            "PB" => 1024_u64.pow(5),
            _ => return None,
        };
        
        Some((number * multiplier as f64) as u64)
    }

    /// 颜色格式转换：HEX 转 RGB
    pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
        let hex = hex.trim_start_matches('#');
        
        if hex.len() != 6 {
            return None;
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        
        Some((r, g, b))
    }

    /// 颜色格式转换：RGB 转 HEX
    pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    /// 温度转换：摄氏度转华氏度
    pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
        celsius * 9.0 / 5.0 + 32.0
    }

    /// 温度转换：华氏度转摄氏度
    pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }

    /// 温度转换：摄氏度转开尔文
    pub fn celsius_to_kelvin(celsius: f64) -> f64 {
        celsius + 273.15
    }

    /// 温度转换：开尔文转摄氏度
    pub fn kelvin_to_celsius(kelvin: f64) -> f64 {
        kelvin - 273.15
    }

    /// 长度转换：米转英尺
    pub fn meters_to_feet(meters: f64) -> f64 {
        meters * 3.28084
    }

    /// 长度转换：英尺转米
    pub fn feet_to_meters(feet: f64) -> f64 {
        feet / 3.28084
    }

    /// 重量转换：千克转磅
    pub fn kg_to_pounds(kg: f64) -> f64 {
        kg * 2.20462
    }

    /// 重量转换：磅转千克
    pub fn pounds_to_kg(pounds: f64) -> f64 {
        pounds / 2.20462
    }

    /// 枚举转换为字符串（需要实现 Debug trait）
    pub fn enum_to_string<T: std::fmt::Debug>(value: &T) -> String {
        format!("{:?}", value)
    }

    /// 安全的类型转换（使用 TryFrom）
    pub fn safe_convert<T, U>(value: T) -> Result<U, <U as TryFrom<T>>::Error>
    where
        U: TryFrom<T>,
    {
        U::try_from(value)
    }
}

/// URL 解析结果
#[derive(Debug, Clone)]
pub struct ParsedUrl {
    pub scheme: String,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

// 添加必要的 use 语句
use std::convert::TryFrom;

// 为了支持 URL 解码，我们需要手动实现一个简单版本
mod urlencoding {
    pub fn encode(input: &str) -> String {
        input.chars()
            .map(|c| {
                match c {
                    'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                    _ => format!("%{:02X}", c as u8),
                }
            })
            .collect()
    }
    
    pub fn decode(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = Vec::new();
        let mut chars = input.chars();
        
        while let Some(c) = chars.next() {
            match c {
                '%' => {
                    let hex1 = chars.next().unwrap_or('0');
                    let hex2 = chars.next().unwrap_or('0');
                    let hex_str = format!("{}{}", hex1, hex2);
                    if let Ok(byte) = u8::from_str_radix(&hex_str, 16) {
                        result.push(byte);
                    }
                }
                '+' => result.push(b' '),
                _ => result.push(c as u8),
            }
        }
        
        String::from_utf8(result).map_err(|e| e.into())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_conversions() {
        assert_eq!(ConvertUtils::str_to_i32("123"), Some(123));
        assert_eq!(ConvertUtils::str_to_bool("true"), Some(true));
        assert_eq!(ConvertUtils::str_to_bool("false"), Some(false));
        assert_eq!(ConvertUtils::str_to_bool("invalid"), None);
    }

    #[test]
    fn test_bytes_conversion() {
        let bytes = b"hello";
        let hex = ConvertUtils::bytes_to_hex(bytes);
        let back_to_bytes = ConvertUtils::hex_to_bytes(&hex).unwrap();
        assert_eq!(bytes, &back_to_bytes[..]);
    }

    #[test]
    fn test_human_readable_bytes() {
        assert_eq!(ConvertUtils::bytes_to_human_readable(1024), "1.00 KB");
        assert_eq!(ConvertUtils::bytes_to_human_readable(1048576), "1.00 MB");
        
        assert_eq!(ConvertUtils::human_readable_to_bytes("1 KB"), Some(1024));
        assert_eq!(ConvertUtils::human_readable_to_bytes("1 MB"), Some(1048576));
    }

    #[test]
    fn test_color_conversion() {
        assert_eq!(ConvertUtils::hex_to_rgb("#FF0000"), Some((255, 0, 0)));
        assert_eq!(ConvertUtils::rgb_to_hex(255, 0, 0), "#FF0000");
    }

    #[test]
    fn test_temperature_conversion() {
        assert!((ConvertUtils::celsius_to_fahrenheit(0.0) - 32.0).abs() < f64::EPSILON);
        assert!((ConvertUtils::fahrenheit_to_celsius(32.0) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_array_delimited_conversion() {
        let arr = vec![1, 2, 3, 4];
        let delimited = ConvertUtils::array_to_delimited_string(&arr, ",");
        assert_eq!(delimited, "1,2,3,4");
        
        let back_to_array = ConvertUtils::delimited_string_to_array(&delimited, ",");
        assert_eq!(back_to_array, vec!["1", "2", "3", "4"]);
    }

    #[test]
    fn test_csv_conversion() {
        let csv_row = r#"name,"description with, comma","quoted ""value""",123"#;
        let fields = ConvertUtils::csv_row_to_array(csv_row);
        assert_eq!(fields[0], "name");
        assert_eq!(fields[1], "description with, comma");
        assert_eq!(fields[2], r#"quoted "value""#);
        assert_eq!(fields[3], "123");
    }
}
