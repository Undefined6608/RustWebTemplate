use hello_rust::utils::time::*;
use chrono_tz::{Asia, America, Europe};
use chrono::Datelike;

fn main() {
    println!("=== 时区工具演示 ===\n");

    // 基本时区转换
    println!("🌍 基本时区转换:");
    let utc_now = TimeUtils::now_utc();
    println!("UTC 时间: {}", TimeUtils::format_default(&utc_now));
    
    let beijing_time = TimeUtils::to_timezone(&utc_now, Asia::Shanghai);
    println!("北京时间: {}", TimeUtils::format_default(&TimeUtils::to_utc(&beijing_time)));
    println!("北京时区偏移: UTC+{} 小时", TimeUtils::get_timezone_offset(&beijing_time));
    
    let ny_time = TimeUtils::to_timezone(&utc_now, America::New_York);
    println!("纽约时间: {}", TimeUtils::format_default(&TimeUtils::to_utc(&ny_time)));
    println!("纽约时区偏移: UTC{:+} 小时", TimeUtils::get_timezone_offset(&ny_time));
    println!();

    // 时区名称和显示
    println!("🏷️  时区信息:");
    let common_timezones = TimeUtils::get_common_timezones();
    for (name, tz) in common_timezones.iter().take(5) {
        let display_name = TimeUtils::get_timezone_display_name(*tz);
        let current_time = TimeUtils::now_in_timezone(*tz);
        let offset = TimeUtils::get_timezone_offset(&current_time);
        println!("{}: {} (UTC{:+})", name, display_name, offset);
    }
    println!();

    // 世界时钟
    println!("🌐 世界时钟:");
    let major_cities = vec![
        ("北京", Asia::Shanghai),
        ("纽约", America::New_York),
        ("伦敦", Europe::London),
        ("东京", Asia::Tokyo),
        ("洛杉矶", America::Los_Angeles),
    ];
    
    let world_clock = TimeUtils::world_clock(&major_cities);
    for entry in world_clock {
        let time_str = entry.local_time.format("%Y-%m-%d %H:%M:%S").to_string();
        let dst_status = if entry.is_dst { " (夏令时)" } else { "" };
        println!("{}: {} UTC{:+}{}", 
                entry.city_name, 
                time_str, 
                entry.utc_offset,
                dst_status);
    }
    println!();

    // 时差计算
    println!("⏰ 时差计算:");
    let beijing_ny_diff = TimeUtils::timezone_difference(Asia::Shanghai, America::New_York, None);
    println!("北京与纽约时差: {} 小时", beijing_ny_diff);
    
    let london_tokyo_diff = TimeUtils::timezone_difference(Europe::London, Asia::Tokyo, None);
    println!("伦敦与东京时差: {} 小时", london_tokyo_diff);
    println!();

    // 时区转换器
    println!("🔄 时区转换器:");
    let converter = TimezoneConverter::new(Asia::Shanghai, America::New_York);
    println!("转换器: 北京 -> 纽约");
    println!("时差: {} 小时", converter.get_time_difference());
    
    let current_ny_time = converter.convert_now();
    println!("当前纽约时间: {}", current_ny_time.format("%Y-%m-%d %H:%M:%S"));
    println!();

    // 夏令时检测
    println!("☀️ 夏令时状态:");
    let dst_timezones = vec![
        ("纽约", America::New_York),
        ("伦敦", Europe::London),
        ("北京", Asia::Shanghai),
    ];
    
    for (name, tz) in dst_timezones {
        let is_dst = TimeUtils::is_dst_active(tz, None);
        let (std_offset, dst_offset) = TimeUtils::get_timezone_offsets(tz, utc_now.year());
        println!("{}: 当前{} (标准: UTC{:+}, 夏令时: UTC{:+})", 
                name, 
                if is_dst { "使用夏令时" } else { "使用标准时间" },
                std_offset,
                dst_offset);
    }
    println!();

    // 按偏移查找时区
    println!("🔍 按偏移查找时区:");
    let utc8_timezones = TimeUtils::find_timezone_by_offset(8);
    println!("UTC+8 时区 ({} 个):", utc8_timezones.len());
    for tz in utc8_timezones.iter().take(3) {
        println!("  - {}", TimeUtils::get_timezone_display_name(*tz));
    }
    println!();

    // 同一天检测
    println!("📅 同一天检测:");
    let time1 = utc_now;
    let time2 = TimeUtils::add_hours(&utc_now, 10);
    
    let is_same_utc = TimeUtils::is_same_day(&time1, &time2);
    let is_same_beijing = TimeUtils::is_same_day_in_timezone(&time1, &time2, Asia::Shanghai);
    let is_same_ny = TimeUtils::is_same_day_in_timezone(&time1, &time2, America::New_York);
    
    println!("时间1: {}", TimeUtils::format_default(&time1));
    println!("时间2: {}", TimeUtils::format_default(&time2));
    println!("UTC 同一天: {}", is_same_utc);
    println!("北京时区同一天: {}", is_same_beijing);
    println!("纽约时区同一天: {}", is_same_ny);
    println!();

    // 时区解析
    println!("📝 时区时间解析:");
    let time_str = "2024-03-15 14:30:00";
    
    if let Ok(beijing_parsed) = TimeUtils::parse_in_timezone(time_str, "%Y-%m-%d %H:%M:%S", Asia::Shanghai) {
        let utc_equiv = TimeUtils::to_utc(&beijing_parsed);
        println!("解析 '{}' 为北京时间", time_str);
        println!("对应 UTC 时间: {}", TimeUtils::format_default(&utc_equiv));
    }
    
    println!("\n=== 演示完成 ===");
}
