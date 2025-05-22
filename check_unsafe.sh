#!/bin/bash
set -e

# 指定要扫描的目录（可通过环境变量传入，默认为当前目录）
TARGET_DIR="logger/src"

# 要查找的关键字
KEYWORDS="unwrap\\(|expect\\(|unreachable!|panic!"

# 查找 .rs 文件中包含这些关键字的行
MATCHES=$(grep -rnE "$KEYWORDS" "$TARGET_DIR" --include="*.rs")

if [ -n "$MATCHES" ]; then
  echo "❌ 检测到以下不安全调用（unwrap/expect/unreachable!/panic!）："
  echo "$MATCHES"
  exit 1
else
  echo "✅ 未检测到 unwrap / expect / panic / unreachable"
fi
