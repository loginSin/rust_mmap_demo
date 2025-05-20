use logger::mmap_writer::MmapWriter;
use std::cell::RefCell;
use std::fs::remove_dir_all;
use std::path::PathBuf;
use std::sync::Arc;

pub struct BaseTest {
    mmap_writer: Arc<RefCell<MmapWriter>>,
}

impl BaseTest {
    pub fn new(app_key: &str, base_dir: PathBuf, is_encrypt: bool, clean: bool) -> Self {
        Self::clean_old_file(base_dir.clone(), clean);
        let mmap_writer = MmapWriter::new(app_key, base_dir, is_encrypt);
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
