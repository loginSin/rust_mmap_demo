/// 测试读取跨天跨小时的日志
#[cfg(test)]
pub mod export_over_hour_test {
    use crate::base::base_test::{create_subdir_and_file, BaseTest};
    use chrono::{Datelike, Duration, TimeZone, Timelike};
    use chrono_tz::Asia::Shanghai;
    use logger::encrypt_util::encrypt_line;
    use std::fs::{File, OpenOptions};
    use std::io::{BufRead, BufReader, Write};
    use std::path::PathBuf;

    fn get_today() -> (i32, u32, u32, u32) {
        let now_utc = chrono::Utc::now();
        let now_bj = now_utc.with_timezone(&Shanghai);
        (now_bj.year(), now_bj.month(), now_bj.day(), now_bj.hour())
    }

    fn get_yesterday() -> (i32, u32, u32, u32) {
        let now_utc = chrono::Utc::now();
        let now_bj = now_utc.with_timezone(&Shanghai);
        let yesterday_bj = now_bj - Duration::hours(24);
        (
            yesterday_bj.year(),
            yesterday_bj.month(),
            yesterday_bj.day(),
            yesterday_bj.hour(),
        )
    }

    fn create_file_and_insert(
        base_dir: &PathBuf,
        subdir_name: &str,
        file_name: &str,
        content: &str,
    ) {
        // 创建目录和文件
        create_subdir_and_file(&base_dir, &subdir_name, &file_name).unwrap();

        // 写入内容
        let file_path = base_dir.join(&subdir_name).join(&file_name);
        let mut file = OpenOptions::new()
            .create(true) // 文件不存在就创建
            .append(true) // 在文件末尾追加
            .open(file_path)
            .unwrap();
        writeln!(file, "{}", content).unwrap(); // 写入并自动添加换行符
    }

    fn create_file_over_days(base_dir: &PathBuf, app_key: &str, is_encrypt: bool) -> Vec<String> {
        // 创建昨天 22 ~ 23 点的目录和日志
        let (yesterday_year, yesterday_month, yesterday_day, yesterday_hour) = get_yesterday();
        let yesterday = format!(
            "{:04}{:02}{:02}",
            yesterday_year, yesterday_month, yesterday_day
        );

        let encrypt_str = if is_encrypt { "encrypt" } else { "plain" };

        let yesterday_22 = format!("{}_{:02}_{}.log", yesterday, 22, encrypt_str);
        if !is_encrypt {
            create_file_and_insert(&base_dir, &yesterday, &yesterday_22, &yesterday_22);
        } else {
            let encrypt_content = encrypt_line(app_key, &yesterday_22).unwrap();
            create_file_and_insert(&base_dir, &yesterday, &yesterday_22, &encrypt_content);
        }

        let yesterday_23 = format!("{}_{:02}_{}.log", yesterday, 23, encrypt_str);
        if !is_encrypt {
            create_file_and_insert(&base_dir, &yesterday, &yesterday_23, &yesterday_23);
        } else {
            let encrypt_content = encrypt_line(app_key, &yesterday_23).unwrap();
            create_file_and_insert(&base_dir, &yesterday, &yesterday_23, &encrypt_content);
        }

        // 创建今天 0 ~ 8 点的目录和日志
        let (now_year, now_month, now_day, now_hour) = get_today();
        let today = format!("{:04}{:02}{:02}", now_year, now_month, now_day);

        let today_0 = format!("{}_{:02}_{}.log", today, 0, encrypt_str);
        if !is_encrypt {
            create_file_and_insert(&base_dir, &today, &today_0, &today_0);
        } else {
            let encrypt_content = encrypt_line(app_key, &today_0).unwrap();
            create_file_and_insert(&base_dir, &today, &today_0, &encrypt_content);
        }

        let today_1 = format!("{}_{:02}_{}.log", today, 1, encrypt_str);
        if !is_encrypt {
            create_file_and_insert(&base_dir, &today, &today_1, &today_1);
        } else {
            let encrypt_content = encrypt_line(app_key, &today_1).unwrap();
            create_file_and_insert(&base_dir, &today, &today_1, &encrypt_content);
        }

        let today_2 = format!("{}_{:02}_{}.log", today, 2, encrypt_str);
        if !is_encrypt {
            create_file_and_insert(&base_dir, &today, &today_2, &today_2);
        } else {
            let encrypt_content = encrypt_line(app_key, &today_2).unwrap();
            create_file_and_insert(&base_dir, &today, &today_2, &encrypt_content);
        }

        let today_3 = format!("{}_{:02}_{}.log", today, 3, encrypt_str);
        if !is_encrypt {
            create_file_and_insert(&base_dir, &today, &today_3, &today_3);
        } else {
            let encrypt_content = encrypt_line(app_key, &today_3).unwrap();
            create_file_and_insert(&base_dir, &today, &today_3, &encrypt_content);
        }

        let today_4 = format!("{}_{:02}_{}.log", today, 4, encrypt_str);
        if !is_encrypt {
            create_file_and_insert(&base_dir, &today, &today_4, &today_4);
        } else {
            let encrypt_content = encrypt_line(app_key, &today_4).unwrap();
            create_file_and_insert(&base_dir, &today, &today_4, &encrypt_content);
        }

        let today_5 = format!("{}_{:02}_{}.log", today, 5, encrypt_str);
        if !is_encrypt {
            create_file_and_insert(&base_dir, &today, &today_5, &today_5);
        } else {
            let encrypt_content = encrypt_line(app_key, &today_5).unwrap();
            create_file_and_insert(&base_dir, &today, &today_5, &encrypt_content);
        }

        let today_6 = format!("{}_{:02}_{}.log", today, 6, encrypt_str);
        if !is_encrypt {
            create_file_and_insert(&base_dir, &today, &today_6, &today_6);
        } else {
            let encrypt_content = encrypt_line(app_key, &today_6).unwrap();
            create_file_and_insert(&base_dir, &today, &today_6, &encrypt_content);
        }

        let today_7 = format!("{}_{:02}_{}.log", today, 7, encrypt_str);
        if !is_encrypt {
            create_file_and_insert(&base_dir, &today, &today_7, &today_7);
        } else {
            let encrypt_content = encrypt_line(app_key, &today_7).unwrap();
            create_file_and_insert(&base_dir, &today, &today_7, &encrypt_content);
        }

        let today_8 = format!("{}_{:02}_{}.log", today, 8, encrypt_str);
        if !is_encrypt {
            create_file_and_insert(&base_dir, &today, &today_8, &today_8);
        } else {
            let encrypt_content = encrypt_line(app_key, &today_8).unwrap();
            create_file_and_insert(&base_dir, &today, &today_8, &encrypt_content);
        }

        let mut content_vec = vec![];
        content_vec.push(yesterday_22);
        content_vec.push(yesterday_23);
        content_vec.push(today_0);
        content_vec.push(today_1);
        content_vec.push(today_2);
        content_vec.push(today_3);
        content_vec.push(today_4);
        content_vec.push(today_5);
        content_vec.push(today_6);
        content_vec.push(today_7);
        content_vec.push(today_8);
        content_vec
    }

    fn check_file_over_days(
        base_test: &BaseTest,
        app_key: &str,
        is_encrypt: bool,
        content_vec: Vec<String>,
    ) {
        // 导出昨天 22 ~ 今天 8 点的日志
        let (start_millis, end_millis) = get_start_end_timestamp();
        let output = PathBuf::from("../target/tmp_log/encrypt_log.log");

        let arc_writer = base_test.get_mmap_writer();
        let mut writer = arc_writer.borrow_mut();
        writer
            .export_logs(start_millis, end_millis, &output)
            .unwrap();

        // 检查内容
        let file = File::open(&output).unwrap();
        let reader = BufReader::new(file);

        let mut line_count = 0;
        for line_result in reader.lines() {
            let line = line_result.unwrap();
            let content = content_vec.get(line_count).unwrap();
            assert_eq!(line, content.to_string());
            line_count += 1;
        }

        assert_eq!(line_count, 11)
    }

    #[test]
    fn test_export_over_hour_plain() {
        let count = 1 * 100;
        let length = 100;
        let app_key = "12345";
        let is_encrypt = false;
        let base_dir = PathBuf::from("../target/tmp_log");
        let base_test = BaseTest::new(app_key, &base_dir, is_encrypt, true);

        let content_vec = create_file_over_days(&base_dir, app_key, is_encrypt);

        check_file_over_days(&base_test, app_key, is_encrypt, content_vec);
    }

    #[test]
    fn test_export_over_hour_encrypt() {
        let count = 1 * 100;
        let length = 100;
        let app_key = "12345";
        let is_encrypt = true;
        let base_dir = PathBuf::from("../target/tmp_log");
        let base_test = BaseTest::new(app_key, &base_dir, is_encrypt, true);

        let content_vec = create_file_over_days(&base_dir, app_key, is_encrypt);

        check_file_over_days(&base_test, app_key, is_encrypt, content_vec);
    }

    fn get_start_end_timestamp() -> (i64, i64) {
        let now_bj = chrono::Utc::now().with_timezone(&Shanghai);

        // 今天的年月日
        let year = now_bj.year();
        let month = now_bj.month();
        let day = now_bj.day();

        // 构造今天中午 12:00
        let today_noon = Shanghai
            .with_ymd_and_hms(year, month, day, 12, 0, 0)
            .single()
            .expect("Invalid today noon time");

        // 昨天上午 8:00 = 今天 12:00 - 1 天 - 4 小时
        let yesterday_morning = today_noon - Duration::days(1) - Duration::hours(4);

        // 转成毫秒时间戳
        let start_ts_ms = yesterday_morning.timestamp_millis();
        let end_ts_ms = today_noon.timestamp_millis();
        (start_ts_ms, end_ts_ms)
    }
}
