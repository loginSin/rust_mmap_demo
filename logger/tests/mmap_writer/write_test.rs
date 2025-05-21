#[cfg(test)]
pub mod write_test {
    use crate::base::base_test::BaseTest;
    use crate::base::random_tool::string_by_length;
    use std::path::PathBuf;
    use std::time::Instant;

    #[test]
    fn test_write_plain_log() {
        let count = 1 * 10000;
        let length = 100;
        let app_key = "12345";
        let is_encrypt = false;
        let base_dir = PathBuf::from("../target/tmp_log");
        let base_test = BaseTest::new(app_key, &base_dir, is_encrypt, true);

        let rc_writer = base_test.get_mmap_writer();
        let mut writer = rc_writer.borrow_mut();
        // 添加计时开始点
        let start = Instant::now();
        for _ in 0..1 * count {
            let _ = writer.write(string_by_length(length).as_str());
        }
        // 计算总耗时
        let duration = start.elapsed();
        println!(
            "write_log Total time: {} ms，is_encrypt {}",
            duration.as_millis(),
            is_encrypt
        );
    }

    #[test]
    fn test_write_encrypt_log() {
        let count = 1 * 10000;
        let length = 100;
        let app_key = "12345";
        let is_encrypt = true;
        let base_dir = PathBuf::from("../target/tmp_log");
        let base_test = BaseTest::new(app_key, &base_dir, is_encrypt, true);

        let rc_writer = base_test.get_mmap_writer();
        let mut writer = rc_writer.borrow_mut();
        // 添加计时开始点
        let start = Instant::now();
        for _ in 0..1 * count {
            let _ = writer.write(string_by_length(length).as_str());
        }
        // 计算总耗时
        let duration = start.elapsed();
        println!(
            "write_encrypt_log Total time: {} ms，is_encrypt {}",
            duration.as_millis(),
            is_encrypt
        );
    }
}
