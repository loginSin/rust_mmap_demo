use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    create_build_info();
}

fn create_build_info() {
    // 每次构建都触发
    println!("cargo:rerun-if-changed=src/build_info.rs");

    let dest_path = Path::new("src").join("build_info.rs");
    let mut file = File::create(&dest_path).expect("Problem creating the build_info.rs");
    let version = env::var("CARGO_PKG_VERSION").unwrap();

    let mut write_line = |line: &str| {
        writeln!(file, "{}", line).expect("Could not write to file");
    };
    write_line(&format!("pub const RUST_SDK_VER: &str = \"{}\";", version));

    // 获取 Git commit ID
    let git_commit = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    write_line(&format!(
        "pub const RUST_SDK_COMMIT: &str = \"{}\";",
        git_commit
    ));

    // 获取构建时间（UTC）
    let build_time = chrono::Utc::now()
        .with_timezone(&chrono::FixedOffset::east_opt(8 * 3600).unwrap())
        .to_rfc3339();
    write_line(&format!(
        "pub const RUST_SDK_BUILD_TIME: &str = \"{}\";",
        build_time
    ));

    // 获取构建目标 triple，例如 aarch64-apple-ios
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    write_line(&format!(
        "pub const RUST_SDK_TARGET: &str = \"{}\";",
        target
    ));

    // 编译信息
    // 把编译信息写入 sdk，可以通过命令行直接查看 sdk 信息
    /// ```
    /// # 编译
    /// cargo build -p logger
    /// # 查看信息:
    /// strings target/debug/liblogger.a | grep my_version
    /// # 输出示例：
    /// {"my_version":"0.1.0","my_commit":"9c13add","my_build_time":"2025-05-27T14:59:22.667099+08:00"}
    /// ```
    let json_info = format!(
        r#"{{"my_version":"{version}","my_commit":"{commit}","my_build_time":"{time}","my_target":"{target}"}}"#,
        version = version,
        commit = git_commit,
        time = build_time,
        target = target,
    );

    write_line("#[used]");
    write_line(&format!(
        "static RUST_SDK_BUILD_INFO: &str = r#\"{}\"#;",
        json_info
    ));
}
