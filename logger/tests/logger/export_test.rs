#[cfg(test)]
pub mod export_test {
    use crate::base::base_test::BaseTest;
    use crate::base::random_tool::string_by_length;
    use chrono::{Duration, FixedOffset, Local, NaiveTime, TimeZone};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;

    #[test]
    fn test_export_plain_log() {
        let count = 1 * 100;
        let length = 100;
        let app_key = "12345";
        let is_encrypt = false;
        let base_dir = PathBuf::from("../target/tmp_log");
        let base_test = BaseTest::new(app_key, base_dir, is_encrypt, true);

        let arc_writer = base_test.get_mmap_writer();
        let mut writer = arc_writer.borrow_mut();
        for _ in 0..1 * count {
            let _ = writer.write(string_by_length(length).as_str());
        }
        writer.flush().unwrap();

        let (start_millis, end_millis) = get_start_end_timestamp();
        let output = PathBuf::from("../target/tmp_log/plain_log.log");
        writer
            .export_logs(start_millis, end_millis, &output)
            .unwrap();

        // 检查是否是固定的开头结尾
        let file = File::open(&output).unwrap();
        let reader = BufReader::new(file);

        let mut line_count = 0;

        for line_result in reader.lines() {
            let line = line_result.unwrap();
            line_count += 1;

            if !line.starts_with("start") {
                assert!(false);
            }
            if !line.ends_with("end") {
                assert!(false);
            }
        }

        assert_eq!(line_count, 100)
    }

    #[test]
    fn test_export_encrypt_log() {
        let count = 1 * 100;
        let length = 100;
        let app_key = "12345";
        let is_encrypt = true;
        let base_dir = PathBuf::from("../target/tmp_log");
        let base_test = BaseTest::new(app_key, base_dir, is_encrypt, true);

        let arc_writer = base_test.get_mmap_writer();
        let mut writer = arc_writer.borrow_mut();
        for _ in 0..1 * count {
            let _ = writer.write(string_by_length(length).as_str());
        }
        writer.flush().unwrap();

        let (start_millis, end_millis) = get_start_end_timestamp();
        let output = PathBuf::from("../target/tmp_log/encrypt_log.log");
        writer
            .export_logs(start_millis, end_millis, &output)
            .unwrap();

        // 检查是否是固定的开头结尾
        let file = File::open(&output).unwrap();
        let reader = BufReader::new(file);

        let mut line_count = 0;

        for line_result in reader.lines() {
            let line = line_result.unwrap();
            line_count += 1;

            if !line.starts_with("start") {
                assert!(false);
            }
            if !line.ends_with("end") {
                assert!(false);
            }
        }

        assert_eq!(line_count, 100)
    }

    fn get_start_end_timestamp() -> (i64, i64) {
        // 北京时间（东八区）
        let beijing_tz = FixedOffset::east_opt(8 * 3600).unwrap();

        // 获取当前本地时间
        let now = Local::now();
        let date = now.date_naive();

        // 构造当天 00:00:00
        let start_naive = date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        let start = beijing_tz.from_local_datetime(&start_naive).unwrap();
        let start_millis = start.timestamp_millis();

        // 构造次日 00:00:00 - 1 毫秒 = 当天 23:59:59.999
        let end = start + Duration::days(1) - Duration::milliseconds(1);
        let end_millis = end.timestamp_millis();
        (start_millis, end_millis)
    }
}
