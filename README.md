# rust mmap Demo

这是个 Rust 使用 mmap 写的日志工具类

按照 yyMMdd/yyMMdd-hh.log 创建日志

## 运行单测

```shell
cargo test -- --test-threads=1
```

## 覆盖率

> 安装
```shell
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview
```

> 检测覆盖率
```shell
cargo llvm-cov --package logger --html --output-dir target/llvm-cov-logger -- --test-threads=1
```

> 查看覆盖率
```shell
open target/llvm-cov-logger/html/index.html
```

## 查看圈复杂度

> 安装
```shell
pip install lizard
```

> 检测圈复杂度
```shell
lizard -l rust logger/ -x"logger/tests/*" --html > ./target/lizard_logger.html
```

> 查看
```shell
open ./target/lizard_logger.html
```

# 解密工具

## 日志加密

```rust
fn main() {
    /// 秘钥
    let app_key = "testAppKey";
    /// 加密
    let is_encrypt = true;
    /// 日志路径
    let base_dir = PathBuf::from("./target/tmp_log/");
    /// 配置
    let config = MmapConfig::new(app_key, is_encrypt);
    /// 创建
    let mut encrypt_writer = MmapWriter::new(&base_dir, config);
}
```

## 日志解密

> 编译解密工具
```shell
cargo build -p decrypt_log --release
```

> 执行解密
```shell
./target/release/decrypt_log --app-key "testAppKey" --input "./target/tmp_log"
```

# TODO
```text
// todo 测试多线程
// todo 测试日志写入跨小时
// todo bench 跑性能测试
// todo 更新 README，详细讲述设计思路和实现逻辑
```