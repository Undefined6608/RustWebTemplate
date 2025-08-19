use hello_rust::utils::*;

fn main() {
    println!("=== Rust 工具库演示 ===\n");

    // 时间工具演示
    println!("📅 时间工具演示:");
    let now = TimeUtils::now_utc();
    println!("当前时间: {}", TimeUtils::format_default(&now));
    println!("时间戳: {}", TimeUtils::timestamp());
    println!("相对时间: {}", TimeUtils::relative_time(&TimeUtils::add_hours(&now, -2)));
    
    let yesterday = TimeUtils::add_days(&now, -1);
    println!("昨天: {}", TimeUtils::format_default(&yesterday));
    println!();

    // 字符串工具演示
    println!("🔤 字符串工具演示:");
    println!("驼峰转下划线: {}", StringUtils::camel_to_snake("camelCaseString"));
    println!("下划线转驼峰: {}", StringUtils::snake_to_camel("snake_case_string"));
    println!("截断文本: {}", StringUtils::truncate_with_ellipsis("这是一个很长的文本内容", 10));
    println!("首字母大写: {}", StringUtils::capitalize("hello world"));
    println!("邮箱验证: {}", StringUtils::is_valid_email("test@example.com"));
    println!("随机字符串: {}", StringUtils::random_string(10));
    println!();

    // 数字工具演示
    println!("🔢 数字工具演示:");
    println!("是否为质数(17): {}", NumberUtils::is_prime(17));
    println!("最大公约数(12, 8): {}", NumberUtils::gcd(12, 8));
    println!("斐波那契(10): {}", NumberUtils::fibonacci(10));
    println!("四舍五入(3.14159, 2位): {}", NumberUtils::round_to_decimal_places(3.14159, 2));
    println!("百分比(0.25): {}", NumberUtils::percentage(25.0, 100.0));
    println!("进制转换(255 -> 16进制): {}", NumberUtils::to_base(255, 16));
    println!("千分位格式: {}", NumberUtils::format_with_commas(1234567));
    
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    println!("平均值: {:?}", NumberUtils::average(&numbers));
    println!();

    // 集合工具演示
    println!("📦 集合工具演示:");
    let arr1 = vec![1, 2, 3, 4];
    let arr2 = vec![3, 4, 5, 6];
    println!("数组1: {:?}", arr1);
    println!("数组2: {:?}", arr2);
    println!("交集: {:?}", CollectionUtils::intersection(&arr1, &arr2));
    println!("并集: {:?}", CollectionUtils::union(&arr1, &arr2));
    println!("差集: {:?}", CollectionUtils::difference(&arr1, &arr2));
    
    let mixed_arr = vec![1, 2, 2, 3, 3, 3, 4];
    println!("去重: {:?}", CollectionUtils::unique(&mixed_arr));
    println!("分块: {:?}", CollectionUtils::chunk(&mixed_arr, 3));
    
    let freq = CollectionUtils::frequency(&mixed_arr);
    println!("频率统计: {:?}", freq);
    println!();

    // 加密工具演示
    println!("🔐 加密工具演示:");
    let data = "Hello, World!";
    let encoded = CryptoUtils::base64_encode(data.as_bytes());
    println!("Base64 编码: {}", encoded);
    println!("Base64 解码: {:?}", String::from_utf8(CryptoUtils::base64_decode(&encoded).unwrap()));
    
    let hex_data = CryptoUtils::hex_encode(data.as_bytes());
    println!("十六进制编码: {}", hex_data);
    
    println!("随机 UUID: {}", CryptoUtils::generate_uuid());
    println!("ROT13 编码: {}", CryptoUtils::rot13("Hello"));
    
    let password = CryptoUtils::generate_secure_password(12);
    println!("生成密码: {}", password);
    let strength = CryptoUtils::check_password_strength(&password);
    println!("密码强度: {:?}", strength.level);
    println!();

    // 类型转换工具演示
    println!("🔄 类型转换工具演示:");
    println!("字符串转数字: {:?}", ConvertUtils::str_to_i32("123"));
    println!("字符串转布尔: {:?}", ConvertUtils::str_to_bool("true"));
    
    let colors = vec!["红", "绿", "蓝"];
    let csv = ConvertUtils::array_to_delimited_string(&colors, ",");
    println!("数组转CSV: {}", csv);
    println!("CSV转数组: {:?}", ConvertUtils::delimited_string_to_array(&csv, ","));
    
    println!("文件大小转换: {}", ConvertUtils::bytes_to_human_readable(1048576));
    println!("颜色转换 RGB->HEX: {}", ConvertUtils::rgb_to_hex(255, 0, 0));
    println!("温度转换 摄氏->华氏: {:.1}°F", ConvertUtils::celsius_to_fahrenheit(25.0));
    println!();

    // 格式化工具演示
    println!("🎨 格式化工具演示:");
    println!("货币格式: {}", FormatUtils::format_currency(1234.56, "¥", 2));
    println!("文件大小: {}", FormatUtils::format_file_size(1048576));
    println!("持续时间: {}", FormatUtils::format_duration(3661));
    println!("百分比: {}", FormatUtils::format_percentage(0.1234, 2));
    
    println!("脱敏电话: {}", FormatUtils::format_phone_cn("13812345678"));
    println!("脱敏邮箱: {}", FormatUtils::format_email_masked("test@example.com"));
    
    println!("进度条: {}", FormatUtils::format_progress_bar(75, 100, 20, '█', '░'));
    
    // 表格演示
    let headers = vec!["姓名", "年龄", "城市"];
    let rows = vec![
        vec!["张三".to_string(), "25".to_string(), "北京".to_string()],
        vec!["李四".to_string(), "30".to_string(), "上海".to_string()],
        vec!["王五".to_string(), "28".to_string(), "广州".to_string()],
    ];
    println!("\n表格格式:");
    println!("{}", FormatUtils::format_table(&headers, &rows));
    
    // 彩色文本（如果终端支持）
    println!("彩色文本: {}", FormatUtils::format_colored_text("这是红色文本", "red"));
    
    // 框架文本
    println!("\n框架文本:");
    println!("{}", FormatUtils::format_boxed_text("欢迎使用\nRust 工具库", 2));
    
    println!("\n=== 演示完成 ===");
}
