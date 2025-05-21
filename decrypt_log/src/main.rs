use clap::Parser;
use logger::encrypt_util::decrypt_line;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::{BufWriter, Read};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(name = "decrypt_log")]
#[command(about = "加密日志的解密工具", long_about = None)]
struct Args {
    /// 加密秘钥
    #[arg(short, long)]
    app_key: String,

    /// 输入文件路径，支持目录和文件
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    if args.app_key.is_empty() {
        println!("请输入有效的 app_key");
        exit(1);
    }

    if args.input.is_empty() {
        println!("请输入有效的 input");
        exit(1);
    }

    println!("app_key: {}", args.app_key);
    println!("输入路径: {}", args.input);
    decrypt_log(args.app_key, args.input);
}

fn decrypt_log(app_key: String, input: String) {
    let path = Path::new(input.as_str());
    let mut log_files = Vec::new();
    if path.is_dir() {
        traverse_directory(path, &mut log_files);
    } else if path.is_file() && path.extension().map_or(false, |ext| ext == "log") {
        log_files.push(path.to_path_buf());
    }

    println!("找到的全部日志文件:");
    for file in &log_files {
        println!("{:?}", file);
    }

    let mut encrypt_files = Vec::new();
    for file in &log_files {
        if is_encrypt_file(file) {
            encrypt_files.push(file.to_path_buf());
            continue;
        }
    }

    if encrypt_files.is_empty() {
        println!("没有找到加密日志文件");
        return;
    }

    for file in &encrypt_files {
        println!("开始解密: {:?}", file);
        decrypt_file(&app_key, file).expect("解密失败");
    }
}

fn is_encrypt_file(file: &PathBuf) -> bool {
    if let Some(file_name) = file.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            if file_name_str.contains("encrypt") {
                return true;
            }
        }
    }
    false
}

fn decrypt_file(app_key: &str, encrypt_file: &PathBuf) -> io::Result<()> {
    let decrypt_file = append_to_filename(encrypt_file, "_decrypt");
    let mut out_buf = BufWriter::new(File::create(&decrypt_file).expect("创建解密文件失败"));

    let total_length = get_first_zero_pos(&encrypt_file).expect("倒查 0x00 第一个位置 失败");
    let mut src_file = File::open(encrypt_file).expect("打开加密文件失败");
    let mut buffer = vec![0u8; total_length as usize];
    src_file.read_exact(&mut buffer).expect("读取加密文件失败");

    for bytes in buffer.split(|&b| b == b'\n') {
        if bytes.is_empty() {
            continue;
        }
        let encrypted_text = String::from_utf8(bytes.to_vec()).unwrap_or("".to_string());
        let msg = decrypt_line(app_key, encrypted_text.as_str()).unwrap_or("".to_string());
        writeln!(out_buf, "{}", msg)?;
    }
    println!("解密成功: {:?}", decrypt_file);
    Ok(())
}

fn get_first_zero_pos(path: &PathBuf) -> io::Result<u64> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let total_length = file.metadata()?.len();

    let mut buffer = vec![0u8; total_length as usize];
    file.read_exact(&mut buffer)?;

    let mut pos = total_length as usize;
    while pos > 0 && buffer[pos - 1] == 0 {
        pos -= 1;
    }
    Ok(pos as u64)
}

fn append_to_filename(path: &PathBuf, suffix: &str) -> PathBuf {
    let parent = path.parent().expect("获取父目录失败");
    let file_stem = path.file_stem().expect("获取文件名失败");
    let extension = path.extension();

    // 构建新文件名
    let mut new_file_name = file_stem.to_os_string();
    new_file_name.push(suffix);

    if let Some(ext) = extension {
        new_file_name.push(".");
        new_file_name.push(ext);
    }

    parent.join(new_file_name)
}

fn traverse_directory(dir: &Path, log_files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    traverse_directory(&path, log_files);
                } else if path.extension().map_or(false, |ext| ext == "log") {
                    log_files.push(path);
                }
            }
        }
    }
}
