use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, TimeZone, Utc, Datelike, FixedOffset, Offset};
use chrono_tz::{Tz, Asia, America, Europe, Africa, Australia};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 时间格式常量
pub const DEFAULT_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub const DEFAULT_DATE_FORMAT: &str = "%Y-%m-%d";
pub const DEFAULT_TIME_FORMAT: &str = "%H:%M:%S";
pub const ISO8601_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";
pub const TIMESTAMP_FORMAT: &str = "%Y%m%d%H%M%S";

/// 时间工具结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeUtils;

impl TimeUtils {
    /// 获取当前 UTC 时间
    pub fn now_utc() -> DateTime<Utc> {
        Utc::now()
    }

    /// 获取当前本地时间
    pub fn now_local() -> DateTime<Local> {
        Local::now()
    }

    /// 获取当前时间戳（秒）
    pub fn timestamp() -> i64 {
        Utc::now().timestamp()
    }

    /// 获取当前时间戳（毫秒）
    pub fn timestamp_millis() -> i64 {
        Utc::now().timestamp_millis()
    }

    /// 从时间戳创建 DateTime
    pub fn from_timestamp(timestamp: i64) -> Option<DateTime<Utc>> {
        Utc.timestamp_opt(timestamp, 0).single()
    }

    /// 从毫秒时间戳创建 DateTime
    pub fn from_timestamp_millis(timestamp_millis: i64) -> Option<DateTime<Utc>> {
        Utc.timestamp_millis_opt(timestamp_millis).single()
    }

    /// 格式化时间为字符串
    pub fn format_datetime(datetime: &DateTime<Utc>, format: &str) -> String {
        datetime.format(format).to_string()
    }

    /// 格式化时间为默认格式
    pub fn format_default(datetime: &DateTime<Utc>) -> String {
        Self::format_datetime(datetime, DEFAULT_DATETIME_FORMAT)
    }

    /// 格式化时间为 ISO8601 格式
    pub fn format_iso8601(datetime: &DateTime<Utc>) -> String {
        Self::format_datetime(datetime, ISO8601_FORMAT)
    }

    /// 解析字符串为时间
    pub fn parse_datetime(datetime_str: &str, format: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        let naive_datetime = NaiveDateTime::parse_from_str(datetime_str, format)?;
        Ok(Utc.from_utc_datetime(&naive_datetime))
    }

    /// 解析默认格式的时间字符串
    pub fn parse_default(datetime_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        Self::parse_datetime(datetime_str, DEFAULT_DATETIME_FORMAT)
    }

    /// 解析 ISO8601 格式的时间字符串
    pub fn parse_iso8601(datetime_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        datetime_str.parse::<DateTime<Utc>>()
    }

    /// 时间加法
    pub fn add_duration(datetime: &DateTime<Utc>, duration: Duration) -> DateTime<Utc> {
        *datetime + duration
    }

    /// 时间减法
    pub fn sub_duration(datetime: &DateTime<Utc>, duration: Duration) -> DateTime<Utc> {
        *datetime - duration
    }

    /// 添加天数
    pub fn add_days(datetime: &DateTime<Utc>, days: i64) -> DateTime<Utc> {
        Self::add_duration(datetime, Duration::days(days))
    }

    /// 添加小时
    pub fn add_hours(datetime: &DateTime<Utc>, hours: i64) -> DateTime<Utc> {
        Self::add_duration(datetime, Duration::hours(hours))
    }

    /// 添加分钟
    pub fn add_minutes(datetime: &DateTime<Utc>, minutes: i64) -> DateTime<Utc> {
        Self::add_duration(datetime, Duration::minutes(minutes))
    }

    /// 添加秒
    pub fn add_seconds(datetime: &DateTime<Utc>, seconds: i64) -> DateTime<Utc> {
        Self::add_duration(datetime, Duration::seconds(seconds))
    }

    /// 计算两个时间的差值
    pub fn diff(datetime1: &DateTime<Utc>, datetime2: &DateTime<Utc>) -> Duration {
        *datetime1 - *datetime2
    }

    /// 判断是否为同一天
    pub fn is_same_day(datetime1: &DateTime<Utc>, datetime2: &DateTime<Utc>) -> bool {
        datetime1.date_naive() == datetime2.date_naive()
    }

    /// 获取一天的开始时间（00:00:00）
    pub fn start_of_day(datetime: &DateTime<Utc>) -> DateTime<Utc> {
        let date = datetime.date_naive();
        Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
    }

    /// 获取一天的结束时间（23:59:59）
    pub fn end_of_day(datetime: &DateTime<Utc>) -> DateTime<Utc> {
        let date = datetime.date_naive();
        Utc.from_utc_datetime(&date.and_hms_opt(23, 59, 59).unwrap())
    }

    /// 获取周的开始时间（周一 00:00:00）
    pub fn start_of_week(datetime: &DateTime<Utc>) -> DateTime<Utc> {
        let weekday = datetime.weekday();
        let days_to_subtract = weekday.num_days_from_monday() as i64;
        let start_date = datetime.date_naive() - Duration::days(days_to_subtract);
        Utc.from_utc_datetime(&start_date.and_hms_opt(0, 0, 0).unwrap())
    }

    /// 获取月的开始时间
    pub fn start_of_month(datetime: &DateTime<Utc>) -> DateTime<Utc> {
        let year = datetime.year();
        let month = datetime.month();
        let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        Utc.from_utc_datetime(&first_day.and_hms_opt(0, 0, 0).unwrap())
    }

    /// 获取年的开始时间
    pub fn start_of_year(datetime: &DateTime<Utc>) -> DateTime<Utc> {
        let year = datetime.year();
        let first_day = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        Utc.from_utc_datetime(&first_day.and_hms_opt(0, 0, 0).unwrap())
    }

    /// 判断是否为闰年
    pub fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    // ========== 时区相关功能 ==========

    /// 将 UTC 时间转换到指定时区
    pub fn to_timezone(datetime: &DateTime<Utc>, timezone: Tz) -> DateTime<Tz> {
        datetime.with_timezone(&timezone)
    }

    /// 将时区时间转换为 UTC
    pub fn to_utc<Tz: TimeZone>(datetime: &DateTime<Tz>) -> DateTime<Utc> {
        datetime.with_timezone(&Utc)
    }

    /// 从时区偏移创建 DateTime
    pub fn from_offset(datetime: &DateTime<Utc>, offset_hours: i32) -> DateTime<FixedOffset> {
        let offset = FixedOffset::east_opt(offset_hours * 3600).unwrap_or(FixedOffset::east_opt(0).unwrap());
        datetime.with_timezone(&offset)
    }

    /// 获取时区偏移（相对于 UTC 的小时数）
    pub fn get_timezone_offset<T: TimeZone>(datetime: &DateTime<T>) -> i32 
    where
        T::Offset: Offset,
    {
        datetime.offset().fix().local_minus_utc() / 3600
    }

    /// 将时间从一个时区转换到另一个时区
    pub fn convert_timezone(datetime: &DateTime<Tz>, _from_tz: Tz, to_tz: Tz) -> DateTime<Tz> {
        let utc_time = datetime.with_timezone(&Utc);
        utc_time.with_timezone(&to_tz)
    }

    /// 获取当前时间在指定时区的表示
    pub fn now_in_timezone(timezone: Tz) -> DateTime<Tz> {
        Utc::now().with_timezone(&timezone)
    }

    /// 在指定时区解析时间字符串
    pub fn parse_in_timezone(
        datetime_str: &str,
        format: &str,
        timezone: Tz,
    ) -> Result<DateTime<Tz>, Box<dyn std::error::Error>> {
        let naive_datetime = NaiveDateTime::parse_from_str(datetime_str, format)?;
        timezone.from_local_datetime(&naive_datetime).single()
           .ok_or_else(|| "Failed to parse datetime in timezone".into())
    }

    /// 获取常用时区列表
    pub fn get_common_timezones() -> HashMap<&'static str, Tz> {
        let mut timezones = HashMap::new();
        
        // 亚洲时区
        timezones.insert("北京", Asia::Shanghai);
        timezones.insert("东京", Asia::Tokyo);
        timezones.insert("首尔", Asia::Seoul);
        timezones.insert("新加坡", Asia::Singapore);
        timezones.insert("孟买", Asia::Kolkata);
        timezones.insert("迪拜", Asia::Dubai);
        
        // 欧洲时区
        timezones.insert("伦敦", Europe::London);
        timezones.insert("巴黎", Europe::Paris);
        timezones.insert("柏林", Europe::Berlin);
        timezones.insert("莫斯科", Europe::Moscow);
        timezones.insert("罗马", Europe::Rome);
        
        // 美洲时区
        timezones.insert("纽约", America::New_York);
        timezones.insert("洛杉矶", America::Los_Angeles);
        timezones.insert("芝加哥", America::Chicago);
        timezones.insert("丹佛", America::Denver);
        timezones.insert("圣保罗", America::Sao_Paulo);
        
        // 大洋洲时区
        timezones.insert("悉尼", Australia::Sydney);
        timezones.insert("墨尔本", Australia::Melbourne);
        
        // 非洲时区
        timezones.insert("开罗", Africa::Cairo);
        
        timezones
    }

    /// 根据时区名称获取时区
    pub fn get_timezone_by_name(name: &str) -> Option<Tz> {
        let timezones = Self::get_common_timezones();
        timezones.get(name).copied()
    }

    /// 获取时区的显示名称
    pub fn get_timezone_display_name(timezone: Tz) -> String {
        match timezone {
            Asia::Shanghai => "中国标准时间 (CST)".to_string(),
            Asia::Tokyo => "日本标准时间 (JST)".to_string(),
            Asia::Seoul => "韩国标准时间 (KST)".to_string(),
            Asia::Singapore => "新加坡标准时间 (SGT)".to_string(),
            Asia::Kolkata => "印度标准时间 (IST)".to_string(),
            Asia::Dubai => "阿联酋标准时间 (GST)".to_string(),
            Europe::London => "格林威治标准时间 (GMT)".to_string(),
            Europe::Paris => "中欧时间 (CET)".to_string(),
            Europe::Berlin => "中欧时间 (CET)".to_string(),
            Europe::Moscow => "莫斯科标准时间 (MSK)".to_string(),
            Europe::Rome => "中欧时间 (CET)".to_string(),
            America::New_York => "美国东部时间 (EST/EDT)".to_string(),
            America::Los_Angeles => "美国太平洋时间 (PST/PDT)".to_string(),
            America::Chicago => "美国中部时间 (CST/CDT)".to_string(),
            America::Denver => "美国山地时间 (MST/MDT)".to_string(),
            America::Sao_Paulo => "巴西时间 (BRT)".to_string(),
            Australia::Sydney => "澳大利亚东部时间 (AEST/AEDT)".to_string(),
            Australia::Melbourne => "澳大利亚东部时间 (AEST/AEDT)".to_string(),
            Africa::Cairo => "东欧时间 (EET)".to_string(),
            _ => format!("{}", timezone),
        }
    }

    /// 检查两个时间在不同时区下是否为同一天
    pub fn is_same_day_in_timezone(
        datetime1: &DateTime<Utc>,
        datetime2: &DateTime<Utc>,
        timezone: Tz,
    ) -> bool {
        let tz_datetime1 = datetime1.with_timezone(&timezone);
        let tz_datetime2 = datetime2.with_timezone(&timezone);
        tz_datetime1.date_naive() == tz_datetime2.date_naive()
    }

    /// 获取时区当前的夏令时状态
    pub fn is_dst_active(timezone: Tz, datetime: Option<DateTime<Utc>>) -> bool {
        let dt = datetime.unwrap_or_else(|| Utc::now());
        let tz_datetime = dt.with_timezone(&timezone);
        let offset = tz_datetime.offset();
        
        // 简化的夏令时检测：比较当前偏移与标准偏移
        // 这里使用一个简单的启发式方法
        let jan_dt = timezone.with_ymd_and_hms(dt.year(), 1, 1, 12, 0, 0).single()
            .unwrap_or_else(|| timezone.with_ymd_and_hms(dt.year(), 1, 2, 12, 0, 0).unwrap());
        let jul_dt = timezone.with_ymd_and_hms(dt.year(), 7, 1, 12, 0, 0).single()
            .unwrap_or_else(|| timezone.with_ymd_and_hms(dt.year(), 7, 2, 12, 0, 0).unwrap());
        
        let jan_offset = jan_dt.offset();
        let jul_offset = jul_dt.offset();
        
        if jan_offset.fix().local_minus_utc() != jul_offset.fix().local_minus_utc() {
            // 有夏令时变化
            let jan_seconds = jan_offset.fix().local_minus_utc();
            let jul_seconds = jul_offset.fix().local_minus_utc();
            let current_seconds = offset.fix().local_minus_utc();
            current_seconds != jan_seconds.min(jul_seconds)
        } else {
            false
        }
    }

    /// 获取时区的标准偏移和夏令时偏移
    pub fn get_timezone_offsets(timezone: Tz, year: i32) -> (i32, i32) {
        let jan_dt = timezone.with_ymd_and_hms(year, 1, 1, 12, 0, 0).single()
            .unwrap_or_else(|| timezone.with_ymd_and_hms(year, 1, 2, 12, 0, 0).unwrap());
        let jul_dt = timezone.with_ymd_and_hms(year, 7, 1, 12, 0, 0).single()
            .unwrap_or_else(|| timezone.with_ymd_and_hms(year, 7, 2, 12, 0, 0).unwrap());
        
        let jan_offset = jan_dt.offset().fix().local_minus_utc() / 3600;
        let jul_offset = jul_dt.offset().fix().local_minus_utc() / 3600;
        
        if jan_offset == jul_offset {
            // 无夏令时
            (jan_offset, jan_offset)
        } else {
            // 有夏令时，较小的是标准时间
            (jan_offset.min(jul_offset), jan_offset.max(jul_offset))
        }
    }

    /// 计算两个时区之间的时差
    pub fn timezone_difference(tz1: Tz, tz2: Tz, datetime: Option<DateTime<Utc>>) -> i32 {
        let dt = datetime.unwrap_or_else(|| Utc::now());
        let dt1 = dt.with_timezone(&tz1);
        let dt2 = dt.with_timezone(&tz2);
        
        let offset1 = dt1.offset().fix().local_minus_utc() / 3600;
        let offset2 = dt2.offset().fix().local_minus_utc() / 3600;
        
        offset1 - offset2
    }

    /// 获取世界时钟 - 显示多个时区的当前时间
    pub fn world_clock(timezones: &[(&str, Tz)]) -> Vec<WorldClockEntry> {
        let now = Utc::now();
        
        timezones.iter().map(|(name, tz)| {
            let local_time = now.with_timezone(tz);
            WorldClockEntry {
                city_name: name.to_string(),
                timezone: *tz,
                local_time,
                utc_offset: Self::get_timezone_offset(&local_time),
                is_dst: Self::is_dst_active(*tz, Some(now)),
            }
        }).collect()
    }

    /// 查找与给定时间最匹配的时区
    pub fn find_timezone_by_offset(offset_hours: i32) -> Vec<Tz> {
        let test_time = Utc::now();
        let mut matching_timezones = Vec::new();
        
        for (_, timezone) in Self::get_common_timezones() {
            let tz_time = test_time.with_timezone(&timezone);
            let tz_offset = Self::get_timezone_offset(&tz_time);
            
            if tz_offset == offset_hours {
                matching_timezones.push(timezone);
            }
        }
        
        matching_timezones
    }

    /// 获取时间的相对描述
    pub fn relative_time(datetime: &DateTime<Utc>) -> String {
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
}

/// 时间范围结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// 世界时钟条目
#[derive(Debug, Clone)]
pub struct WorldClockEntry {
    pub city_name: String,
    pub timezone: Tz,
    pub local_time: DateTime<Tz>,
    pub utc_offset: i32,
    pub is_dst: bool,
}

/// 时区转换器
#[derive(Debug, Clone)]
pub struct TimezoneConverter {
    pub source_timezone: Tz,
    pub target_timezone: Tz,
}

impl TimezoneConverter {
    /// 创建新的时区转换器
    pub fn new(source_timezone: Tz, target_timezone: Tz) -> Self {
        Self {
            source_timezone,
            target_timezone,
        }
    }

    /// 转换时间
    pub fn convert(&self, datetime: &DateTime<Tz>) -> DateTime<Tz> {
        TimeUtils::convert_timezone(datetime, self.source_timezone, self.target_timezone)
    }

    /// 转换当前时间
    pub fn convert_now(&self) -> DateTime<Tz> {
        let now_source = TimeUtils::now_in_timezone(self.source_timezone);
        self.convert(&now_source)
    }

    /// 批量转换时间
    pub fn convert_batch(&self, datetimes: &[DateTime<Tz>]) -> Vec<DateTime<Tz>> {
        datetimes.iter().map(|dt| self.convert(dt)).collect()
    }

    /// 获取时差
    pub fn get_time_difference(&self) -> i32 {
        TimeUtils::timezone_difference(self.source_timezone, self.target_timezone, None)
    }
}

impl TimeRange {
    /// 创建新的时间范围
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self { start, end }
    }

    /// 判断时间是否在范围内
    pub fn contains(&self, datetime: &DateTime<Utc>) -> bool {
        *datetime >= self.start && *datetime <= self.end
    }

    /// 获取时间范围的持续时间
    pub fn duration(&self) -> Duration {
        self.end - self.start
    }

    /// 判断两个时间范围是否重叠
    pub fn overlaps(&self, other: &TimeRange) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        let now = TimeUtils::now_utc();
        let timestamp = TimeUtils::timestamp();
        let from_timestamp = TimeUtils::from_timestamp(timestamp).unwrap();
        
        // 允许 1 秒的误差
        assert!((now.timestamp() - from_timestamp.timestamp()).abs() <= 1);
    }

    #[test]
    fn test_format_and_parse() {
        let now = TimeUtils::now_utc();
        let formatted = TimeUtils::format_default(&now);
        let parsed = TimeUtils::parse_default(&formatted).unwrap();
        
        // 由于格式化会丢失毫秒，所以比较到秒级
        assert_eq!(now.timestamp(), parsed.timestamp());
    }

    #[test]
    fn test_time_range() {
        let start = TimeUtils::now_utc();
        let end = TimeUtils::add_hours(&start, 1);
        let middle = TimeUtils::add_minutes(&start, 30);
        
        let range = TimeRange::new(start, end);
        assert!(range.contains(&middle));
        assert_eq!(range.duration(), Duration::hours(1));
    }

    #[test]
    fn test_timezone_conversion() {
        let utc_time = TimeUtils::now_utc();
        let beijing_time = TimeUtils::to_timezone(&utc_time, Asia::Shanghai);
        let back_to_utc = TimeUtils::to_utc(&beijing_time);
        
        // 允许毫秒级误差
        assert!((utc_time.timestamp() - back_to_utc.timestamp()).abs() <= 1);
    }

    #[test]
    fn test_timezone_offset() {
        let utc_time = TimeUtils::now_utc();
        let beijing_time = TimeUtils::to_timezone(&utc_time, Asia::Shanghai);
        let offset = TimeUtils::get_timezone_offset(&beijing_time);
        
        // 北京时间通常是 UTC+8
        assert_eq!(offset, 8);
    }

    #[test]
    fn test_timezone_by_name() {
        let beijing_tz = TimeUtils::get_timezone_by_name("北京");
        assert!(beijing_tz.is_some());
        assert_eq!(beijing_tz.unwrap(), Asia::Shanghai);
        
        let invalid_tz = TimeUtils::get_timezone_by_name("无效时区");
        assert!(invalid_tz.is_none());
    }

    #[test]
    fn test_world_clock() {
        let timezones = vec![
            ("北京", Asia::Shanghai),
            ("纽约", America::New_York),
            ("伦敦", Europe::London),
        ];
        
        let world_clock = TimeUtils::world_clock(&timezones);
        assert_eq!(world_clock.len(), 3);
        
        for entry in world_clock {
            assert!(!entry.city_name.is_empty());
            assert!(entry.utc_offset >= -12 && entry.utc_offset <= 14);
        }
    }

    #[test]
    fn test_timezone_converter() {
        let converter = TimezoneConverter::new(Asia::Shanghai, America::New_York);
        let time_diff = converter.get_time_difference();
        
        // 北京和纽约的时差应该在 12-13 小时之间（取决于夏令时）
        assert!(time_diff >= 12 && time_diff <= 13);
    }

    #[test]
    fn test_find_timezone_by_offset() {
        let timezones = TimeUtils::find_timezone_by_offset(8);
        assert!(!timezones.is_empty());
        
        // 应该包含亚洲的一些时区
        let contains_asia = timezones.iter().any(|&tz| {
            matches!(tz, Asia::Shanghai | Asia::Singapore)
        });
        assert!(contains_asia);
    }
}
