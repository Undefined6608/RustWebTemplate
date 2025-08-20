use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// 格式化工具结构体
pub struct FormatUtils;

impl FormatUtils {
    /// 格式化数字为货币（带千分位分隔符）
    pub fn format_currency(amount: f64, currency_symbol: &str, decimal_places: u32) -> String {
        let formatted_number = Self::format_number_with_commas(amount, decimal_places);
        format!("{}{}", currency_symbol, formatted_number)
    }

    /// 格式化数字（带千分位分隔符）
    pub fn format_number_with_commas(number: f64, decimal_places: u32) -> String {
        let formatted = format!("{:.1$}", number, decimal_places as usize);
        let parts: Vec<&str> = formatted.split('.').collect();
        let integer_part = parts[0];
        let decimal_part = if parts.len() > 1 { parts[1] } else { "" };

        let mut result = String::new();
        let chars: Vec<char> = integer_part.chars().collect();

        for (i, ch) in chars.iter().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                result.insert(0, ',');
            }
            result.insert(0, *ch);
        }

        if !decimal_part.is_empty() {
            result.push('.');
            result.push_str(decimal_part);
        }

        result
    }

    /// 格式化百分比
    pub fn format_percentage(ratio: f64, decimal_places: u32) -> String {
        format!("{:.1$}%", ratio * 100.0, decimal_places as usize)
    }

    /// 格式化文件大小
    pub fn format_file_size(bytes: u64) -> String {
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

    /// 格式化时间持续时间
    pub fn format_duration(seconds: u64) -> String {
        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;

        let mut parts = Vec::new();

        if days > 0 {
            parts.push(format!("{}天", days));
        }
        if hours > 0 {
            parts.push(format!("{}小时", hours));
        }
        if minutes > 0 {
            parts.push(format!("{}分钟", minutes));
        }
        if secs > 0 || parts.is_empty() {
            parts.push(format!("{}秒", secs));
        }

        parts.join("")
    }

    /// 格式化时间持续时间（英文）
    pub fn format_duration_en(seconds: u64) -> String {
        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;

        let mut parts = Vec::new();

        if days > 0 {
            parts.push(format!("{} day{}", days, if days > 1 { "s" } else { "" }));
        }
        if hours > 0 {
            parts.push(format!(
                "{} hour{}",
                hours,
                if hours > 1 { "s" } else { "" }
            ));
        }
        if minutes > 0 {
            parts.push(format!(
                "{} minute{}",
                minutes,
                if minutes > 1 { "s" } else { "" }
            ));
        }
        if secs > 0 || parts.is_empty() {
            parts.push(format!(
                "{} second{}",
                secs,
                if secs > 1 { "s" } else { "" }
            ));
        }

        parts.join(", ")
    }

    /// 格式化相对时间
    pub fn format_relative_time(datetime: &DateTime<Utc>) -> String {
        let now = Utc::now();
        let diff = now.signed_duration_since(*datetime);

        if diff.num_seconds() < 60 {
            "刚刚".to_string()
        } else if diff.num_minutes() < 60 {
            format!("{}分钟前", diff.num_minutes())
        } else if diff.num_hours() < 24 {
            format!("{}小时前", diff.num_hours())
        } else if diff.num_days() < 30 {
            format!("{}天前", diff.num_days())
        } else if diff.num_days() < 365 {
            format!("{}个月前", diff.num_days() / 30)
        } else {
            format!("{}年前", diff.num_days() / 365)
        }
    }

    /// 格式化电话号码（中国）
    pub fn format_phone_cn(phone: &str) -> String {
        let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();

        if digits.len() == 11 && digits.starts_with('1') {
            format!("{}-{}-{}", &digits[0..3], &digits[3..7], &digits[7..11])
        } else {
            phone.to_string()
        }
    }

    /// 格式化身份证号（隐藏中间部分）
    pub fn format_id_card_masked(id_card: &str) -> String {
        if id_card.len() == 18 {
            format!("{}******{}", &id_card[0..6], &id_card[14..18])
        } else if id_card.len() == 15 {
            format!("{}******{}", &id_card[0..6], &id_card[12..15])
        } else {
            id_card.to_string()
        }
    }

    /// 格式化银行卡号（隐藏中间部分）
    pub fn format_bank_card_masked(card_number: &str) -> String {
        let digits: String = card_number.chars().filter(|c| c.is_ascii_digit()).collect();

        if digits.len() >= 8 {
            let visible_start = &digits[0..4];
            let visible_end = &digits[digits.len() - 4..];
            let masked_middle = "*".repeat(digits.len() - 8);
            format!("{} {} {}", visible_start, masked_middle, visible_end)
        } else {
            card_number.to_string()
        }
    }

    /// 格式化邮箱（隐藏部分内容）
    pub fn format_email_masked(email: &str) -> String {
        if let Some(at_pos) = email.find('@') {
            let username = &email[0..at_pos];
            let domain = &email[at_pos..];

            if username.len() <= 2 {
                format!("{}****{}", &username[0..1], domain)
            } else {
                let visible_chars = (username.len() / 3).max(1);
                let visible_start = &username[0..visible_chars];
                let visible_end = &username[username.len() - visible_chars..];
                format!("{}****{}{}", visible_start, visible_end, domain)
            }
        } else {
            email.to_string()
        }
    }

    /// 格式化表格（简单的文本表格）
    pub fn format_table(headers: &[&str], rows: &[Vec<String>]) -> String {
        if headers.is_empty() || rows.is_empty() {
            return String::new();
        }

        // 计算每列的最大宽度
        let mut col_widths = headers.iter().map(|h| h.len()).collect::<Vec<_>>();

        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    col_widths[i] = col_widths[i].max(cell.len());
                }
            }
        }

        let mut result = String::new();

        // 表头
        result.push('|');
        for (i, header) in headers.iter().enumerate() {
            result.push_str(&format!(" {:width$} |", header, width = col_widths[i]));
        }
        result.push('\n');

        // 分隔线
        result.push('|');
        for &width in &col_widths {
            result.push_str(&format!(" {} |", "-".repeat(width)));
        }
        result.push('\n');

        // 数据行
        for row in rows {
            result.push('|');
            for (i, cell) in row.iter().enumerate() {
                let width = col_widths.get(i).unwrap_or(&0);
                result.push_str(&format!(" {:width$} |", cell, width = width));
            }
            result.push('\n');
        }

        result
    }

    /// 格式化 JSON（美化）
    pub fn format_json_pretty(json_str: &str) -> Result<String, serde_json::Error> {
        let value: serde_json::Value = serde_json::from_str(json_str)?;
        serde_json::to_string_pretty(&value)
    }

    /// 格式化键值对列表
    pub fn format_key_value_list(data: &HashMap<String, String>) -> String {
        let mut result = String::new();
        let max_key_length = data.keys().map(|k| k.len()).max().unwrap_or(0);

        for (key, value) in data {
            result.push_str(&format!(
                "{:width$}: {}\n",
                key,
                value,
                width = max_key_length
            ));
        }

        result
    }

    /// 格式化进度条
    pub fn format_progress_bar(
        current: usize,
        total: usize,
        width: usize,
        filled_char: char,
        empty_char: char,
    ) -> String {
        let percentage = if total == 0 {
            0.0
        } else {
            current as f64 / total as f64
        };
        let filled_width = (width as f64 * percentage) as usize;
        let empty_width = width - filled_width;

        let filled = filled_char.to_string().repeat(filled_width);
        let empty = empty_char.to_string().repeat(empty_width);
        let percent_text = format!(" {:.1}%", percentage * 100.0);

        format!(
            "[{}{}] {}/{}{}",
            filled, empty, current, total, percent_text
        )
    }

    /// 格式化列表（带缩进）
    pub fn format_indented_list(items: &[String], indent: usize, bullet: &str) -> String {
        let indent_str = " ".repeat(indent);
        items
            .iter()
            .map(|item| format!("{}{} {}", indent_str, bullet, item))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// 格式化代码块
    pub fn format_code_block(code: &str, language: &str) -> String {
        format!("```{}\n{}\n```", language, code)
    }

    /// 格式化引用块
    pub fn format_quote_block(text: &str) -> String {
        text.lines()
            .map(|line| format!("> {}", line))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// 格式化标题
    pub fn format_heading(text: &str, level: u8) -> String {
        let hashes = "#".repeat(level.min(6) as usize);
        format!("{} {}", hashes, text)
    }

    /// 格式化分隔线
    pub fn format_separator(char: char, length: usize) -> String {
        char.to_string().repeat(length)
    }

    /// 格式化框架文本
    pub fn format_boxed_text(text: &str, padding: usize) -> String {
        let lines: Vec<&str> = text.lines().collect();
        let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
        let box_width = max_width + 2 * padding + 2; // +2 for borders

        let mut result = String::new();

        // 顶部边框
        result.push_str(&format!("┌{}┐\n", "─".repeat(box_width - 2)));

        // 内容行
        for line in &lines {
            let padded_line = format!(
                "│{:padding$}{:width$}{:padding$}│\n",
                "",
                line,
                "",
                padding = padding,
                width = max_width
            );
            result.push_str(&padded_line);
        }

        // 底部边框
        result.push_str(&format!("└{}┘", "─".repeat(box_width - 2)));

        result
    }

    /// 格式化数字范围
    pub fn format_number_range(min: f64, max: f64, unit: &str) -> String {
        if (min - max).abs() < f64::EPSILON {
            format!("{} {}", min, unit)
        } else {
            format!("{} - {} {}", min, max, unit)
        }
    }

    /// 格式化颜色代码（ANSI）
    pub fn format_colored_text(text: &str, color: &str) -> String {
        let color_code = match color.to_lowercase().as_str() {
            "black" => "30",
            "red" => "31",
            "green" => "32",
            "yellow" => "33",
            "blue" => "34",
            "magenta" => "35",
            "cyan" => "36",
            "white" => "37",
            _ => "37", // 默认白色
        };

        format!("\x1b[{}m{}\x1b[0m", color_code, text)
    }

    /// 移除ANSI颜色代码
    pub fn strip_ansi_colors(text: &str) -> String {
        // 简化的ANSI序列移除
        let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
        re.replace_all(text, "").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_currency() {
        let formatted = FormatUtils::format_currency(1234.56, "$", 2);
        assert_eq!(formatted, "$1,234.56");
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(FormatUtils::format_file_size(1024), "1.00 KB");
        assert_eq!(FormatUtils::format_file_size(1048576), "1.00 MB");
        assert_eq!(FormatUtils::format_file_size(0), "0 B");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(FormatUtils::format_duration(3661), "1小时1分钟1秒");
        assert_eq!(
            FormatUtils::format_duration_en(3661),
            "1 hour, 1 minute, 1 second"
        );
    }

    #[test]
    fn test_format_phone() {
        assert_eq!(FormatUtils::format_phone_cn("13812345678"), "138-1234-5678");
    }

    #[test]
    fn test_format_masked() {
        assert_eq!(
            FormatUtils::format_email_masked("test@example.com"),
            "t****t@example.com"
        );

        assert_eq!(
            FormatUtils::format_bank_card_masked("1234567890123456"),
            "1234 **** 3456"
        );
    }

    #[test]
    fn test_format_percentage() {
        assert_eq!(FormatUtils::format_percentage(0.1234, 2), "12.34%");
    }

    #[test]
    fn test_format_progress_bar() {
        let bar = FormatUtils::format_progress_bar(50, 100, 20, '█', '░');
        assert!(bar.contains("50.0%"));
        assert!(bar.contains("50/100"));
    }

    #[test]
    fn test_format_table() {
        let headers = vec!["Name", "Age"];
        let rows = vec![
            vec!["Alice".to_string(), "25".to_string()],
            vec!["Bob".to_string(), "30".to_string()],
        ];

        let table = FormatUtils::format_table(&headers, &rows);
        assert!(table.contains("Name"));
        assert!(table.contains("Alice"));
    }

    #[test]
    fn test_format_boxed_text() {
        let boxed = FormatUtils::format_boxed_text("Hello\nWorld", 1);
        assert!(boxed.contains("┌"));
        assert!(boxed.contains("┐"));
        assert!(boxed.contains("└"));
        assert!(boxed.contains("┘"));
    }
}
