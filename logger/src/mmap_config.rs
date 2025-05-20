pub struct MmapConfig {
    /// 应用密钥，加密则必须设置该字段
    app_key: String,
    /// 是否加密
    is_encrypt: bool,
    /// 每次扩展的 buffer 大小，默认 128 KB
    buffer_size: usize,
    /// 刷新尺寸，buffer 超过该大小则立即写入，默认 16 KB
    flush_size: usize,
    /// 刷新间隔，单位秒，最多该时间段内强制写入，默认 5 秒
    flush_interval: usize,
}

impl MmapConfig {
    pub fn new(app_key: &str, is_encrypt: bool) -> Self {
        Self {
            app_key: app_key.to_string(),
            is_encrypt,
            buffer_size: 128 * 1024,
            flush_size: 16 * 1024,
            flush_interval: 5,
        }
    }

    /// 获取 `app_key`
    pub fn get_app_key(&self) -> &str {
        &self.app_key
    }

    /// 设置 `app_key`
    pub fn set_app_key(&mut self, app_key: &str) {
        self.app_key = app_key.to_string();
    }

    /// 获取 `is_encrypt`
    pub fn is_encrypt(&self) -> bool {
        self.is_encrypt
    }

    /// 设置 `is_encrypt`
    pub fn set_is_encrypt(&mut self, is_encrypt: bool) {
        self.is_encrypt = is_encrypt;
    }

    /// 获取 `buffer_size`
    pub fn get_buffer_size(&self) -> usize {
        self.buffer_size
    }

    /// 设置 `buffer_size` , 必须 >= 1024，否则不生效，将会使用默认值
    pub fn set_buffer_size(&mut self, buffer_size: usize) {
        if buffer_size >= 1024 {
            self.buffer_size = buffer_size;
        }
    }

    /// 获取 `flush_size`
    pub fn get_flush_size(&self) -> usize {
        self.flush_size
    }

    /// 设置 `flush_size`, 必须 >= 1024 且 <= buffer_size 否则不生效，将会使用默认值
    pub fn set_flush_size(&mut self, flush_size: usize) {
        if flush_size >= 1024 && flush_size <= self.buffer_size {
            self.flush_size = flush_size;
        }
    }

    /// 获取 `flush_interval`
    pub fn get_flush_interval(&self) -> usize {
        self.flush_interval
    }

    /// 设置 `flush_interval`，必须 > 0，否则不生效，将会使用默认值
    pub fn set_flush_interval(&mut self, flush_interval: usize) {
        if flush_interval > 0 {
            self.flush_interval = flush_interval;
        }
    }
}
