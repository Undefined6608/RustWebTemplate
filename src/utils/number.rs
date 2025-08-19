

/// 数字工具结构体
pub struct NumberUtils;

impl NumberUtils {
    /// 安全地将字符串转换为数字
    pub fn parse_i32(s: &str) -> Option<i32> {
        s.trim().parse().ok()
    }

    /// 安全地将字符串转换为 i64
    pub fn parse_i64(s: &str) -> Option<i64> {
        s.trim().parse().ok()
    }

    /// 安全地将字符串转换为 f64
    pub fn parse_f64(s: &str) -> Option<f64> {
        s.trim().parse().ok()
    }

    /// 判断数字是否为偶数
    pub fn is_even(n: i64) -> bool {
        n % 2 == 0
    }

    /// 判断数字是否为奇数
    pub fn is_odd(n: i64) -> bool {
        n % 2 != 0
    }

    /// 判断数字是否为质数
    pub fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        
        let sqrt_n = (n as f64).sqrt() as u64;
        for i in (3..=sqrt_n).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    /// 计算最大公约数
    pub fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    /// 计算最小公倍数
    pub fn lcm(a: u64, b: u64) -> u64 {
        if a == 0 || b == 0 {
            0
        } else {
            (a * b) / Self::gcd(a, b)
        }
    }

    /// 计算阶乘
    pub fn factorial(n: u64) -> u64 {
        if n <= 1 {
            1
        } else {
            n * Self::factorial(n - 1)
        }
    }

    /// 计算斐波那契数列第 n 项
    pub fn fibonacci(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let mut a = 0;
                let mut b = 1;
                for _ in 2..=n {
                    let temp = a + b;
                    a = b;
                    b = temp;
                }
                b
            }
        }
    }

    /// 数字四舍五入到指定小数位
    pub fn round_to_decimal_places(n: f64, places: u32) -> f64 {
        let multiplier = 10_f64.powi(places as i32);
        (n * multiplier).round() / multiplier
    }

    /// 数字向上取整到指定小数位
    pub fn ceil_to_decimal_places(n: f64, places: u32) -> f64 {
        let multiplier = 10_f64.powi(places as i32);
        (n * multiplier).ceil() / multiplier
    }

    /// 数字向下取整到指定小数位
    pub fn floor_to_decimal_places(n: f64, places: u32) -> f64 {
        let multiplier = 10_f64.powi(places as i32);
        (n * multiplier).floor() / multiplier
    }

    /// 将数字限制在指定范围内
    pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }

    /// 线性插值
    pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
        start + (end - start) * t
    }

    /// 将值从一个范围映射到另一个范围
    pub fn map_range(value: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
        let normalized = (value - from_min) / (from_max - from_min);
        Self::lerp(to_min, to_max, normalized)
    }

    /// 计算百分比
    pub fn percentage(part: f64, total: f64) -> f64 {
        if total == 0.0 {
            0.0
        } else {
            (part / total) * 100.0
        }
    }

    /// 计算增长率
    pub fn growth_rate(old_value: f64, new_value: f64) -> f64 {
        if old_value == 0.0 {
            0.0
        } else {
            ((new_value - old_value) / old_value) * 100.0
        }
    }

    /// 生成指定范围内的随机整数
    pub fn random_int(min: i32, max: i32) -> i32 {
        use rand::Rng;
        rand::thread_rng().gen_range(min..=max)
    }

    /// 生成指定范围内的随机浮点数
    pub fn random_float(min: f64, max: f64) -> f64 {
        use rand::Rng;
        rand::thread_rng().gen_range(min..=max)
    }

    /// 计算平均值
    pub fn average(numbers: &[f64]) -> Option<f64> {
        if numbers.is_empty() {
            None
        } else {
            Some(numbers.iter().sum::<f64>() / numbers.len() as f64)
        }
    }

    /// 计算中位数
    pub fn median(numbers: &[f64]) -> Option<f64> {
        if numbers.is_empty() {
            return None;
        }
        
        let mut sorted = numbers.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let len = sorted.len();
        if len % 2 == 0 {
            Some((sorted[len / 2 - 1] + sorted[len / 2]) / 2.0)
        } else {
            Some(sorted[len / 2])
        }
    }

    /// 计算众数
    pub fn mode(numbers: &[i32]) -> Option<i32> {
        use std::collections::HashMap;
        
        if numbers.is_empty() {
            return None;
        }
        
        let mut frequency = HashMap::new();
        for &num in numbers {
            *frequency.entry(num).or_insert(0) += 1;
        }
        
        frequency.into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(num, _)| num)
    }

    /// 计算标准差
    pub fn standard_deviation(numbers: &[f64]) -> Option<f64> {
        if numbers.len() < 2 {
            return None;
        }
        
        let mean = Self::average(numbers)?;
        let variance = numbers.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / (numbers.len() - 1) as f64;
        
        Some(variance.sqrt())
    }

    /// 数字转换为进制字符串
    pub fn to_base(num: u64, base: u32) -> String {
        if base < 2 || base > 36 {
            return "0".to_string();
        }
        
        if num == 0 {
            return "0".to_string();
        }
        
        let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut result = String::new();
        let mut n = num;
        
        while n > 0 {
            let remainder = (n % base as u64) as usize;
            result.insert(0, digits.chars().nth(remainder).unwrap());
            n /= base as u64;
        }
        
        result
    }

    /// 进制字符串转换为数字
    pub fn from_base(s: &str, base: u32) -> Option<u64> {
        if base < 2 || base > 36 {
            return None;
        }
        
        let mut result = 0u64;
        let mut power = 1u64;
        
        for c in s.chars().rev() {
            let digit = match c {
                '0'..='9' => c as u32 - '0' as u32,
                'A'..='Z' => c as u32 - 'A' as u32 + 10,
                'a'..='z' => c as u32 - 'a' as u32 + 10,
                _ => return None,
            };
            
            if digit >= base {
                return None;
            }
            
            result += digit as u64 * power;
            power *= base as u64;
        }
        
        Some(result)
    }

    /// 数字格式化为货币字符串
    pub fn format_currency(amount: f64, currency_symbol: &str, decimal_places: u32) -> String {
        let formatted_amount = format!("{:.1$}", amount, decimal_places as usize);
        format!("{}{}", currency_symbol, formatted_amount)
    }

    /// 数字格式化为千分位字符串
    pub fn format_with_commas(num: i64) -> String {
        let num_str = num.to_string();
        let mut result = String::new();
        
        for (i, c) in num_str.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                result.insert(0, ',');
            }
            result.insert(0, c);
        }
        
        result
    }

    /// 计算两点之间的距离
    pub fn distance_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }

    /// 计算三点之间的距离
    pub fn distance_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
        ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt()
    }

    /// 角度转弧度
    pub fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * std::f64::consts::PI / 180.0
    }

    /// 弧度转角度
    pub fn radians_to_degrees(radians: f64) -> f64 {
        radians * 180.0 / std::f64::consts::PI
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(NumberUtils::is_prime(2));
        assert!(NumberUtils::is_prime(17));
        assert!(!NumberUtils::is_prime(4));
        assert!(!NumberUtils::is_prime(1));
    }

    #[test]
    fn test_gcd_lcm() {
        assert_eq!(NumberUtils::gcd(12, 8), 4);
        assert_eq!(NumberUtils::lcm(12, 8), 24);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(NumberUtils::fibonacci(0), 0);
        assert_eq!(NumberUtils::fibonacci(1), 1);
        assert_eq!(NumberUtils::fibonacci(10), 55);
    }

    #[test]
    fn test_percentage() {
        assert_eq!(NumberUtils::percentage(25.0, 100.0), 25.0);
        assert_eq!(NumberUtils::percentage(0.0, 100.0), 0.0);
    }

    #[test]
    fn test_base_conversion() {
        assert_eq!(NumberUtils::to_base(255, 16), "FF");
        assert_eq!(NumberUtils::from_base("FF", 16), Some(255));
    }

    #[test]
    fn test_statistics() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(NumberUtils::average(&numbers), Some(3.0));
        assert_eq!(NumberUtils::median(&numbers), Some(3.0));
    }
}
