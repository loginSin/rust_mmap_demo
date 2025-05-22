use crate::encrypt_util::{decrypt_line, encrypt_line};
use crate::mmap_config::MmapConfig;
use block_modes::BlockMode;
use chrono::{DateTime, Datelike, LocalResult, NaiveDate, TimeZone, Timelike, Utc};
use chrono_tz::Asia::Shanghai;
use chrono_tz::Tz;
use hex;
use md5;
use memmap2::MmapMut;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::io::Write;
use std::io::{BufWriter, Read};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

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

impl MmapWriter {
    pub fn try_new(base_dir: &PathBuf, config: MmapConfig) -> io::Result<Self> {
        delete_expired_directories(base_dir, config.get_expiration_days())?;
        let buf_size = config.get_buffer_size();
        let flush_interval = config.get_flush_interval();

        let writer = MmapWriter {
            base_dir: base_dir.clone(),
            config,
            current_mmap: None,
            current_file: None,
            buffer: Vec::with_capacity(buf_size), // 缓冲区
            buffer_size: 0,
            last_flush_time: Instant::now(),
            flush_interval: Duration::from_secs(flush_interval as u64), // 刷新间隔
        };
        Ok(writer)
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
        // 1. 缓冲区超过 flush_size KB
        // 2. 距离上次刷新超过 flush_interval 秒
        if self.buffer_size >= self.config.get_flush_size()
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
        // 将毫秒时间戳转为 UTC 时间
        let start_utc = Utc.timestamp_millis_opt(start_ms).single().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Invalid start timestamp")
        })?;

        let end_utc = Utc
            .timestamp_millis_opt(end_ms)
            .single()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid end timestamp"))?;

        // 转换为北京时间
        let start: DateTime<Tz> = start_utc.with_timezone(&Shanghai);
        let end: DateTime<Tz> = end_utc.with_timezone(&Shanghai);

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
        current: &DateTime<Tz>,
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

        let total_length = Self::get_first_zero_pos(&filepath)?;

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
    // 倒查 0x00 第一个位置
    fn get_first_zero_pos(path: &PathBuf) -> io::Result<u64> {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut total_length = file.metadata()?.len();

        let mut buffer = vec![0u8; total_length as usize];
        file.read_exact(&mut buffer)?;

        let mut pos = total_length as usize;
        while pos > 0 && buffer[pos - 1] == 0 {
            pos -= 1;
        }
        Ok(pos as u64)
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
        file.set_len(file_size + self.config.get_buffer_size() as u64)?;

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
        let mmap = self.current_mmap.as_mut().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Can't flush to disk by mmap")
        })?;

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
        let now = Utc::now().with_timezone(&Shanghai);
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

/// 删除 base_dir 下超过 7 天的子目录（目录名格式为 yyyymmdd）
pub fn delete_expired_directories(
    base_dir: &PathBuf,
    expiration_days: usize,
) -> Result<(), io::Error> {
    if !base_dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        if let Some(name) = path.file_name().and_then(|os| os.to_str()) {
            if let Ok(date) = NaiveDate::parse_from_str(name, "%Y%m%d") {
                // 直接构造北京时间当天的 00:00:00
                if let LocalResult::Single(dir_datetime) =
                    Shanghai.with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0)
                {
                    let now = Utc::now().with_timezone(&Shanghai);
                    let seven_days_ago = now - chrono::Duration::days(expiration_days as i64);

                    if dir_datetime < seven_days_ago {
                        println!("删除过期目录: {:?}", path);
                        fs::remove_dir_all(&path)?;
                    }
                }
            }
        }
    }

    Ok(())
}
