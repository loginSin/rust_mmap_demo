use logger::mmap_config::MmapConfig;
use logger::mmap_writer::MmapWriter;
use std::cell::RefCell;
use std::fs::{remove_dir_all, File};
use std::path::PathBuf;
use std::sync::Arc;

pub struct BaseTest {
    mmap_writer: Arc<RefCell<MmapWriter>>,
}

impl BaseTest {
    pub fn new(app_key: &str, base_dir: &PathBuf, is_encrypt: bool, clean: bool) -> Self {
        Self::clean_old_file(base_dir.clone(), clean);
        let config = MmapConfig::new(app_key, is_encrypt);
        let mmap_writer = MmapWriter::try_new(&base_dir, config).unwrap();
        Self {
            mmap_writer: Arc::new(RefCell::new(mmap_writer)),
        }
    }

    pub fn get_mmap_writer(&self) -> Arc<RefCell<MmapWriter>> {
        self.mmap_writer.clone()
    }

    fn clean_old_file(base_dir: PathBuf, clean: bool) {
        if clean {
            let _ = remove_dir_all(base_dir);
        }
    }
}

pub fn create_subdir_and_file(
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
