use chrono::{Datelike, FixedOffset, Local, Timelike};
use memmap2::MmapMut;
use std::fs::{self, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

// 日志配置结构体
pub struct Logger {
    app_key: String,
    base_dir: PathBuf,
    current_mmap: Option<MmapMut>, // 缓存当前 mmap
    current_file: Option<PathBuf>, // 当前日志文件路径
    buffer: Vec<u8>,               // 写入缓冲区
    buffer_size: usize,            // 缓冲区大小
    last_flush_time: Instant,      // 上次刷新时间
    flush_interval: Duration,      // 刷新间隔
}

const BUFFER_SIZE: usize = 128 * 1024; // 128 KB , 每次扩展的 buffer 大小
const FLUSH_SIZE_THRESHOLD: usize = 1024; // 1 KB
const FLUSH_TIME_THRESHOLD: u64 = 5; // 5 seconds

impl Logger {
    pub fn new<P: AsRef<Path>>(app_key: &str, base_dir: P) -> Self {
        Self {
            app_key: app_key.to_string(),
            base_dir: base_dir.as_ref().to_path_buf(),
            current_mmap: None,
            current_file: None,
            buffer: Vec::with_capacity(BUFFER_SIZE), // 1MB 缓冲区
            buffer_size: 0,
            last_flush_time: Instant::now(),
            flush_interval: Duration::from_secs(FLUSH_TIME_THRESHOLD), // 5秒刷新间隔
        }
    }

    // 写入日志（线程安全）
    pub fn log(&mut self, message: &str) -> io::Result<()> {
        let data = message.as_bytes();

        // 如果缓冲区已满，先刷新
        if self.buffer_size + data.len() > self.buffer.capacity() {
            self.flush()?;
        }

        // 将数据添加到缓冲区
        self.buffer.extend_from_slice(data);
        self.buffer_size += data.len();

        // 检查是否需要刷新：
        // 1. 缓冲区超过 1KB
        // 2. 距离上次刷新超过 5 秒
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
}

impl Logger {
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

        // 查找可用空间
        let pos = mmap.iter().position(|&b| b == 0).unwrap_or(mmap.len());

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

impl Logger {
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

        Ok(dir.join(format!(
            "{:04}-{:02}-{:02}_{:02}.log",
            year, month, day, hour
        )))
    }

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
}
