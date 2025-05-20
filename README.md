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