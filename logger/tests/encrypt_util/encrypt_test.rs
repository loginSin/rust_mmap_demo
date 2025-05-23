#[cfg(test)]
pub mod encrypt_test {
    use logger::encrypt_util::{decrypt_line, encrypt_line};

    #[test]
    pub fn test_alphabet_number() {
        let key = "123321";

        let text0 = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let encrypted_text0 = encrypt_line(key, text0).unwrap();
        assert_ne!(text0, encrypted_text0);
        let decrypted_text0 = decrypt_line(key, &encrypted_text0).unwrap();
        assert_eq!(text0, decrypted_text0);
    }

    #[test]
    pub fn test_chinese() {
        let key = "123321";

        let text0 = "甲乙丙丁戊";
        let encrypted_text0 = encrypt_line(key, text0).unwrap();
        assert_ne!(text0, encrypted_text0);
        let decrypted_text0 = decrypt_line(key, &encrypted_text0).unwrap();
        assert_eq!(text0, decrypted_text0);

        let text0 = "一万年太久，只争朝夕!";
        let encrypted_text0 = encrypt_line(key, text0).unwrap();
        assert_ne!(text0, encrypted_text0);
        let decrypted_text0 = decrypt_line(key, &encrypted_text0).unwrap();
        assert_eq!(text0, decrypted_text0);
    }

    #[test]
    pub fn test_special_char() {
        let key = "123321";

        // 英文字符
        let text0 = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
        let encrypted_text0 = encrypt_line(key, text0).unwrap();
        assert_ne!(text0, encrypted_text0);
        let decrypted_text0 = decrypt_line(key, &encrypted_text0).unwrap();
        assert_eq!(text0, decrypted_text0);

        // 中文字符
        let text0 = "，。、《》？；‘：“【】「」~！@#￥%……&*（）——+·-=||、";
        let encrypted_text0 = encrypt_line(key, text0).unwrap();
        assert_ne!(text0, encrypted_text0);
        let decrypted_text0 = decrypt_line(key, &encrypted_text0).unwrap();
        assert_eq!(text0, decrypted_text0);
    }
}
