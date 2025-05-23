use chrono::{FixedOffset, NaiveDateTime, TimeZone};
use logger::mmap_config::MmapConfig;
use logger::mmap_writer::MmapWriter;
use rand::seq::IndexedRandom;
use std::fs::remove_dir_all;
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    let _ = remove_dir_all("./target/tmp_log");
    let count = 1 * 10000;
    let length = 100;
    // write_log(count, length);
    // write_encrypt_log(count, length);
    write_all_log(count, length);

    // export_encrypt_log();
    // export_log();
    export_all_log();
}

fn export_all_log() {
    export_encrypt_log();
    export_log();
}

fn export_encrypt_log() {
    let start_ts = get_timestamp("2025-05-20 08:00:00");
    let end_ts = get_timestamp("2025-05-20 18:00:00");

    let app_key = "testAppKey";
    let is_encrypt = true;
    let base_dir = PathBuf::from("./target/tmp_log/");
    let config = MmapConfig::new(app_key, is_encrypt);
    let mut encrypt_writer = MmapWriter::try_new(&base_dir, config).unwrap();
    // 添加计时开始点
    let start = Instant::now();
    let output = PathBuf::from("./target/tmp_log/encrypt_log.log");
    let _ = encrypt_writer.export_logs(start_ts, end_ts, &output);
    // 获取 总耗时
    let duration = start.elapsed();
    println!(
        "Total time: {} ms, is_encrypt {}",
        duration.as_millis(),
        is_encrypt
    );
}

fn export_log() {
    let start_ts = get_timestamp("2025-05-20 08:00:00");
    let end_ts = get_timestamp("2025-05-20 18:00:00");

    let app_key = "testAppKey";
    let is_encrypt = false;
    let base_dir = PathBuf::from("./target/tmp_log/");
    let config = MmapConfig::new(app_key, is_encrypt);
    let mut encrypt_writer = MmapWriter::try_new(&base_dir, config).unwrap();
    // 添加计时开始点
    let start = Instant::now();
    let output = PathBuf::from("./target/tmp_log/plain_log.log");
    let _ = encrypt_writer.export_logs(start_ts, end_ts, &output);
    // 获取总耗时
    let duration = start.elapsed();
    println!(
        "Total time: {} ms, is_encrypt {}",
        duration.as_millis(),
        is_encrypt
    );
}

fn write_all_log(count: i32, length: i32) {
    let app_key = "testAppKey";
    let base_dir = PathBuf::from("./target/tmp_log/");
    let config0 = MmapConfig::new(app_key, false);
    let mut writer = MmapWriter::try_new(&base_dir, config0).unwrap();
    let config1 = MmapConfig::new(app_key, true);
    let mut encrypt_writer = MmapWriter::try_new(&base_dir, config1).unwrap();
    // 添加计时开始点
    let start = Instant::now();
    for _ in 0..1 * count {
        let text = string_by_length(length);
        let _ = writer.write(text.as_str());
        let _ = encrypt_writer.write(text.as_str());
    }
    // 获取总耗时
    let duration = start.elapsed();
    println!("Total time: {} ms", duration.as_millis(),);
}

// 写加密日志
fn write_encrypt_log(count: i32, length: i32) {
    let app_key = "testAppKey";
    let is_encrypt = true;
    let base_dir = PathBuf::from("./target/tmp_log/");
    let config = MmapConfig::new(app_key, is_encrypt);
    let mut writer = MmapWriter::try_new(&base_dir, config).unwrap();
    // 添加计时开始点
    let start = Instant::now();
    for _ in 0..1 * count {
        let _ = writer.write(string_by_length(length).as_str());
    }
    // 获取总耗时
    let duration = start.elapsed();
    println!(
        "Total time: {} ms，is_encrypt {}",
        duration.as_millis(),
        is_encrypt
    );
}

// 写普通日志
fn write_log(count: i32, length: i32) {
    let app_key = "testAppKey";
    let is_encrypt = false;
    let base_dir = PathBuf::from("./target/tmp_log/");
    let config = MmapConfig::new(app_key, is_encrypt);
    let mut writer = MmapWriter::try_new(&base_dir, config).unwrap();
    // 添加计时开始点
    let start = Instant::now();
    for _ in 0..1 * count {
        let _ = writer.write(string_by_length(length).as_str());
    }
    // 计算总耗时
    let duration = start.elapsed();
    println!(
        "Total time: {} ms，is_encrypt {}",
        duration.as_millis(),
        is_encrypt
    );
}

fn string_by_length(length: i32) -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let mut rng = rand::rng();
    let random_string = (0..length)
        .map(|_| *chars.choose(&mut rng).unwrap())
        .collect::<String>();
    format!("start-{}-end", random_string)
}

/// 获取毫秒时间戳
///
/// time:  时间字符串，格式为 %Y-%m-%d %H:%M:%S ， 示例 2025-05-29 09:10:00
fn get_timestamp(time: &str) -> i64 {
    // 定义北京时间偏移 +08:00
    let beijing_tz = FixedOffset::east_opt(8 * 3600).unwrap();

    // 构建一个 NaiveDateTime（不带时区）
    let naive_time =
        NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S").expect("时间格式不正确");

    // 加上东八区时区
    let datetime_with_tz = beijing_tz.from_local_datetime(&naive_time).unwrap();

    // 获取毫秒时间戳
    let timestamp_millis = datetime_with_tz.timestamp_millis();

    // println!("毫秒时间戳: {}", timestamp_millis);
    timestamp_millis
}
