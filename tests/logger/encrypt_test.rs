#[cfg(test)]
pub mod encrypt_test {

    #[test]
    pub fn test() {
        let key = "123321";

        let text0 = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let encrypted_text = encrypt_line(key, text0).unwrap();
    }
}