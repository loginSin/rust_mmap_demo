#[cfg(test)]
pub mod config_test {
    use logger::mmap_config::MmapConfig;

    #[test]
    fn test_new() {
        let app_key = "123321";
        let is_encrypt = true;
        let conf = MmapConfig::new(app_key, is_encrypt);

        assert_eq!(app_key, conf.get_app_key());
        assert_eq!(is_encrypt, conf.is_encrypt());
        assert_eq!(128 * 1024, conf.get_buffer_size());
        assert_eq!(16 * 1024, conf.get_flush_size());
        assert_eq!(5, conf.get_flush_interval());
        assert_eq!(7, conf.get_expiration_days());
    }

    #[test]
    fn test_getter_setter() {
        let app_key = "123321";
        let is_encrypt = true;
        let mut conf = MmapConfig::new(app_key, is_encrypt);

        let new_app_key = "abc";
        conf.set_app_key(new_app_key);
        assert_eq!(new_app_key, conf.get_app_key());

        let new_is_encrypt = false;
        conf.set_is_encrypt(new_is_encrypt);
        assert_eq!(new_is_encrypt, conf.is_encrypt());

        let new_buffer_size = 3333;
        conf.set_buffer_size(new_buffer_size);
        assert_eq!(new_buffer_size, conf.get_buffer_size());

        let new_flush_size = 2222;
        conf.set_flush_size(new_flush_size);
        assert_eq!(new_flush_size, conf.get_flush_size());

        let new_flush_interval = 11;
        conf.set_flush_interval(new_flush_interval);
        assert_eq!(new_flush_interval, conf.get_flush_interval());
    }

    #[test]
    fn test_set_buffer_size() {
        let app_key = "123321";
        let is_encrypt = true;
        let mut conf = MmapConfig::new(app_key, is_encrypt);

        // usize::MIN
        let buffer_size = usize::MIN;
        conf.set_buffer_size(buffer_size);
        assert_eq!(128 * 1024, conf.get_buffer_size());

        // < 1024
        let buffer_size = 1000;
        conf.set_buffer_size(buffer_size);
        assert_eq!(128 * 1024, conf.get_buffer_size());

        // = 1024
        let buffer_size = 1024;
        conf.set_buffer_size(buffer_size);
        assert_eq!(buffer_size, conf.get_buffer_size());

        // > 1024
        let new_buffer_size = 3333;
        conf.set_buffer_size(new_buffer_size);
        assert_eq!(new_buffer_size, conf.get_buffer_size());

        // usize::MAX
        let new_buffer_size = usize::MAX;
        conf.set_buffer_size(new_buffer_size);
        assert_eq!(new_buffer_size, conf.get_buffer_size());
    }

    #[test]
    fn test_set_flush_size() {
        let app_key = "123321";
        let is_encrypt = true;
        let mut conf = MmapConfig::new(app_key, is_encrypt);

        let buffer_size = 10 * 1024;
        conf.set_buffer_size(buffer_size);
        assert_eq!(buffer_size, conf.get_buffer_size());

        // usize::MIN
        let flush_size = usize::MIN;
        conf.set_flush_size(flush_size);
        assert_eq!(16 * 1024, conf.get_flush_size());

        // < 1024
        let flush_size = 1024 - 1;
        conf.set_flush_size(flush_size);
        assert_eq!(16 * 1024, conf.get_flush_size());

        // = 1024
        let flush_size = 1024;
        conf.set_flush_size(flush_size);
        assert_eq!(1024, conf.get_flush_size());

        // > 1024
        let flush_size = 1024 + 1;
        conf.set_flush_size(flush_size);
        assert_eq!(flush_size, conf.get_flush_size());
    }

    #[test]
    fn test_set_flush_size_border() {
        let app_key = "123321";
        let is_encrypt = true;
        let mut conf = MmapConfig::new(app_key, is_encrypt);

        // usize::MAX
        // > 1024 && > buffer_size
        let flush_size = usize::MAX;
        conf.set_flush_size(flush_size);
        assert_eq!(16 * 1024, conf.get_flush_size());
    }

    #[test]
    fn test_set_flush_interval() {
        let app_key = "123321";
        let is_encrypt = true;
        let mut conf = MmapConfig::new(app_key, is_encrypt);

        // usize::MIN
        let flush_interval = usize::MIN;
        conf.set_flush_interval(flush_interval);
        assert_eq!(5, conf.get_flush_interval());

        // <= 0
        let flush_interval = 0;
        conf.set_flush_interval(flush_interval);
        assert_eq!(5, conf.get_flush_interval());

        // > 0
        let flush_interval = 3333;
        conf.set_flush_interval(flush_interval);
        assert_eq!(flush_interval, conf.get_flush_interval());

        // usize::MAX
        let flush_interval = usize::MAX;
        conf.set_flush_interval(flush_interval);
        assert_eq!(flush_interval, conf.get_flush_interval());
    }

    #[test]
    fn test_expiration_days() {
        let app_key = "123321";
        let is_encrypt = true;
        let mut conf = MmapConfig::new(app_key, is_encrypt);

        assert_eq!(7, conf.get_expiration_days());

        let expiration_days = 0;
        conf.set_expiration_days(expiration_days);
        assert_eq!(7, conf.get_expiration_days());

        let expiration_days = 1;
        conf.set_expiration_days(expiration_days);
        assert_eq!(expiration_days, conf.get_expiration_days());

        let expiration_days = 999;
        conf.set_expiration_days(expiration_days);
        assert_eq!(expiration_days, conf.get_expiration_days());
    }
}
