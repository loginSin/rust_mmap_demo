use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Ecb};

// 定义类型
type Aes128Ecb = Ecb<Aes128, Pkcs7>;

// 生成 128 位密钥
fn generate_key(app_key: &str) -> [u8; 16] {
    let digest = md5::compute(app_key);
    let mut key = [0u8; 16];
    key.copy_from_slice(&digest[..16]);
    key
}

// 加密一行日志
pub fn encrypt_line(app_key: &str, plain: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = generate_key(app_key);
    let cipher = Aes128Ecb::new_from_slices(&key, &[])?;
    let encrypted = cipher.encrypt_vec(plain.as_bytes());
    Ok(hex::encode(encrypted)) // 将二进制加密数据转为十六进制写入
}

// 解密一行日志
pub fn decrypt_line(
    app_key: &str,
    encrypted_hex: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let key = generate_key(app_key);
    let cipher = Aes128Ecb::new_from_slices(&key, &[])?;
    let encrypted = hex::decode(encrypted_hex)?;
    let decrypted = cipher.decrypt_vec(&encrypted)?;
    Ok(String::from_utf8(decrypted)?)
}
