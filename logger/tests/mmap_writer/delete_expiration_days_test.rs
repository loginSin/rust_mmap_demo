#[cfg(test)]
pub mod delete_expiration_days_test {
    use crate::base::base_test::BaseTest;
    use chrono::{Datelike, FixedOffset, Local, Timelike};
    use logger::mmap_writer::delete_expired_directories;
    use std::fs;
    use std::fs::File;
    use std::path::PathBuf;

    #[test]
    pub fn test_base_dir() {
        let base_dir = PathBuf::from("../target1234/tmp_log");
        delete_expired_directories(&base_dir, 7).unwrap();
    }

    #[test]
    pub fn test() {
        let app_key = "12345";
        let is_encrypt = true;
        let base_dir = PathBuf::from("../target/tmp_log");
        let base_test = BaseTest::new(app_key, &base_dir, is_encrypt, true);

        // 创建超过 7 天的目录和文件
        create_subdir_and_file(&base_dir, "20220501", "test.log").unwrap();
        create_subdir_and_file(&base_dir, "20220502", "test.log").unwrap();
        create_subdir_and_file(&base_dir, "20220503", "test.log").unwrap();
        create_subdir_and_file(&base_dir, "20220504", "test.log").unwrap();
        create_subdir_and_file(&base_dir, "20220505", "test.log").unwrap();
        create_subdir_and_file(&base_dir, "20220506", "test.log").unwrap();
        create_subdir_and_file(&base_dir, "20220507", "test.log").unwrap();

        // 创建当天的目录和文件
        let (year, month, day, hour) = current_time();
        let today_dir = format!("{:04}{:02}{:02}", year, month, day);
        create_subdir_and_file(&base_dir, today_dir.as_str(), "test.log").unwrap();

        // 检查创建了 8 个目录
        let mut all_dir_count = 0;
        for entry in fs::read_dir(&base_dir).unwrap() {
            all_dir_count += 1;
        }
        assert_eq!(all_dir_count, 8);

        // 删除过期目录
        delete_expired_directories(&base_dir, 7).unwrap();

        // 检查删除结果，确定值保留了一个 today_dir
        let mut left_dir_count = 0;
        for entry in fs::read_dir(&base_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|os| os.to_str()) {
                assert!(name.eq(today_dir.as_str()));
            }

            left_dir_count += 1;
        }
        assert_eq!(left_dir_count, 1);
    }

    fn current_time() -> (i32, u32, u32, u32) {
        let now = Local::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
        (now.year(), now.month(), now.day(), now.hour())
    }

    fn create_subdir_and_file(
        base_dir: &PathBuf,
        subdir_name: &str,
        file_name: &str,
    ) -> std::io::Result<()> {
        // 1. 创建子目录路径
        let sub_dir_path = base_dir.join(subdir_name);

        // 2. 创建子目录（如果不存在）
        if !sub_dir_path.exists() {
            std::fs::create_dir_all(&sub_dir_path)?;
        }

        // 3. 创建文件路径
        let file_path = sub_dir_path.join(file_name);

        // 4. 创建空白文件（如果已存在则会覆盖）
        File::create(file_path)?;

        Ok(())
    }
}
