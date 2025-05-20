use crate::mmap_config::MmapConfig;
use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Ecb};
use chrono::{DateTime, Datelike, FixedOffset, Local, TimeZone, Timelike};
use hex;
use md5;
use memmap2::MmapMut;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::io::{BufWriter, Read};
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

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

pub struct MmapWriter {
    base_dir: PathBuf,
    config: MmapConfig,
    current_mmap: Option<MmapMut>, // 缓存当前 mmap
    current_file: Option<PathBuf>, // 当前日志文件路径
    buffer: Vec<u8>,               // 写入缓冲区
    buffer_size: usize,            // 缓冲区大小
    last_flush_time: Instant,      // 上次刷新时间
    flush_interval: Duration,      // 刷新间隔
}

const BUFFER_SIZE: usize = 128 * 1024; // 128 KB , 每次扩展的 buffer 大小
const FLUSH_SIZE_THRESHOLD: usize = 16 * 1024; // KB
const FLUSH_TIME_THRESHOLD: u64 = 5; // seconds

const CHUNK_SIZE: usize = 64 * 1024; // 64KB // todo 优化

impl MmapWriter {
    pub fn new(base_dir: &PathBuf, config: MmapConfig) -> Self {
        Self {
            base_dir: base_dir.clone(),
            config,
            current_mmap: None,
            current_file: None,
            buffer: Vec::with_capacity(BUFFER_SIZE), // 缓冲区
            buffer_size: 0,
            last_flush_time: Instant::now(),
            flush_interval: Duration::from_secs(FLUSH_TIME_THRESHOLD), // 刷新间隔
        }
    }

    // 写入日志
    pub fn write(&mut self, message: &str) -> io::Result<()> {
        let msg = if self.config.is_encrypt() {
            let encrypt_msg =
                encrypt_line(self.config.get_app_key(), message).unwrap_or(message.to_string());
            format!("{}\n", encrypt_msg)
        } else {
            format!("{}\n", message)
        };

        let data = msg.as_bytes();

        // 如果缓冲区已满，先刷新
        if self.buffer_size + data.len() > self.buffer.capacity() {
            self.flush()?;
        }

        // 将数据添加到缓冲区
        self.buffer.extend_from_slice(data);
        self.buffer_size += data.len();

        // 检查是否需要刷新：
        // 1. 缓冲区超过 FLUSH_SIZE_THRESHOLD KB
        // 2. 距离上次刷新超过 FLUSH_TIME_THRESHOLD 秒
        if self.buffer_size >= FLUSH_SIZE_THRESHOLD
            || self.last_flush_time.elapsed() >= self.flush_interval
        {
            self.flush()?;
        }

        Ok(())
    }

    // 刷新缓冲区到磁盘
    pub fn flush(&mut self) -> io::Result<()> {
        if self.buffer_size == 0 {
            return Ok(());
        }

        self.flush_to_disk()?;
        self.last_flush_time = Instant::now();
        Ok(())
    }

    /// 将指定时间范围的日志导出日志到指定路径
    pub fn export_logs(&self, start_ms: i64, end_ms: i64, output: &PathBuf) -> io::Result<()> {
        let tz = FixedOffset::east_opt(8 * 3600).unwrap();
        let start = tz.timestamp_millis_opt(start_ms).unwrap();
        let end = tz.timestamp_millis_opt(end_ms).unwrap();

        let mut out_buf = BufWriter::new(File::create(output)?);

        let mut current = start;
        while current <= end {
            self.export_log_by_file(&current, &mut out_buf)?;
            current = current + chrono::Duration::hours(1);
        }
        Ok(())
    }

    fn export_log_by_file(
        &self,
        current: &DateTime<FixedOffset>,
        out_buf: &mut BufWriter<File>,
    ) -> io::Result<()> {
        let y = current.year();
        let m = current.month();
        let d = current.day();
        let h = current.hour();
        let dir = self.base_dir.join(format!("{:04}{:02}{:02}", y, m, d));
        let encrypt_str = if self.config.is_encrypt() {
            "encrypt"
        } else {
            "plain"
        };
        let filename = format!("{:04}{:02}{:02}_{:02}_{}.log", y, m, d, h, encrypt_str);
        let filepath = dir.join(&filename);
        if !filepath.exists() {
            return Ok(());
        }

        let total_length = Self::get_file_size(&filepath)?;

        let mut src_file = File::open(&filepath)?;
        let mut buffer = vec![0u8; total_length as usize];
        src_file.read_exact(&mut buffer)?;

        for bytes in buffer.split(|&b| b == b'\n') {
            if bytes.is_empty() {
                continue;
            }
            let msg = if self.config.is_encrypt() {
                let encrypted_text = String::from_utf8(bytes.to_vec()).unwrap_or("".to_string());
                decrypt_line(self.config.get_app_key(), encrypted_text.as_str())
                    .unwrap_or("".to_string())
            } else {
                String::from_utf8(bytes.to_vec()).unwrap_or("".to_string())
            };
            writeln!(out_buf, "{}", msg)?;
        }

        Ok(())
    }

    // mmap 为填充完成，会拼接 0x00 ，把日志导出来的时候，需要把文末的 0x00 都去掉
    // 文件每 64 KB 倒查 0x00 的位置
    fn get_file_size(path: &PathBuf) -> io::Result<u64> {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let file_size = file.metadata()?.len();

        if file_size == 0 {
            return Ok(file_size);
        }

        let mut pos = file_size;
        let mut buffer = vec![0u8; CHUNK_SIZE];

        // 记录最后一个非 0x00 字节的绝对位置
        let mut last_non_zero_pos = None;

        while pos > 0 {
            // 计算本次读取的实际大小
            let read_size = if pos >= CHUNK_SIZE as u64 {
                CHUNK_SIZE as u64
            } else {
                pos
            };

            // 移动文件指针到当前块起始位置
            file.seek(SeekFrom::Start(pos - read_size))?;

            // 读取数据
            file.read_exact(&mut buffer[..read_size as usize])?;

            // 从块尾部向前查找非 0x00
            if let Some(i) = buffer[..read_size as usize].iter().rposition(|&b| b != 0) {
                last_non_zero_pos = Some(pos - read_size + i as u64 + 1);
                break;
            }

            pos -= read_size;
        }

        // 需要截断文件位置
        let new_len = last_non_zero_pos.unwrap_or(file_size);
        Ok(new_len)
    }
}

impl MmapWriter {
    // 初始化 mmap 映射
    fn init_mmap(&mut self, path: &Path) -> io::Result<()> {
        // 创建或打开文件
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        // 每次给文件扩展 BUFFER_SIZE 大小，确保文件足够大
        let file_size = file.metadata()?.len();
        file.set_len(file_size + BUFFER_SIZE as u64)?;

        // 创建内存映射
        let mmap = unsafe { MmapMut::map_mut(&file)? };
        self.current_mmap = Some(mmap);
        self.current_file = Some(path.to_path_buf());

        Ok(())
    }

    // 实际的磁盘写入逻辑
    fn flush_to_disk(&mut self) -> io::Result<()> {
        let log_path = self.current_log_path()?;

        // 检查是否需要切换文件
        if self.current_file.as_ref() != Some(&log_path) {
            self.init_mmap(&log_path)?;
            self.current_file = Some(log_path.clone());
        }

        // 获取当前 mmap
        let mmap = self.current_mmap.as_mut().unwrap();

        // warning：mmap 申请空间会扩容，且全部填充 0，所以拼接数据的时候，需要找到第一 0 的位置

        // 正序查找可用空间，文件越大，速度越慢
        // let pos = mmap.iter().position(|&b| b == 0).unwrap_or(mmap.len());

        // 倒序查找可用空间，速度快
        let mut pos = mmap.len();
        while pos > 0 && mmap[pos - 1] == 0 {
            pos -= 1;
        }

        if pos + self.buffer_size > mmap.len() {
            // 重新映射
            self.init_mmap(&log_path)?;
            return self.flush_to_disk(); // 递归重试
        }

        // 写入数据
        mmap[pos..pos + self.buffer_size].copy_from_slice(&self.buffer[..self.buffer_size]);
        mmap.flush()?;

        // 清空缓冲区
        self.buffer_size = 0;
        self.buffer.clear();

        Ok(())
    }
}

impl MmapWriter {
    // 获取当前时间的年月日小时格式
    fn current_time(&self) -> (i32, u32, u32, u32) {
        let now = Local::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
        (now.year(), now.month(), now.day(), now.hour())
    }

    // 创建目录结构
    fn ensure_directory(&self, year: i32, month: u32, day: u32) -> io::Result<PathBuf> {
        let name = format!("{}{:02}{:02}", year, month, day);
        let dir_path = self.base_dir.join(name.to_string());

        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)?;
        }
        Ok(dir_path)
    }

    // 获取当前日志文件路径
    fn current_log_path(&mut self) -> io::Result<PathBuf> {
        let (year, month, day, hour) = self.current_time();
        let dir = self.ensure_directory(year, month, day)?;

        let encrypt_str = if self.config.is_encrypt() {
            "encrypt"
        } else {
            "plain"
        };

        Ok(dir.join(format!(
            "{:04}{:02}{:02}_{:02}_{}.log",
            year, month, day, hour, encrypt_str
        )))
    }
}
