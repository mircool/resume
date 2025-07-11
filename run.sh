#!/bin/bash

echo "正在启动个人简历网站..."
echo

# 检查是否安装了 Rust
if ! command -v cargo &> /dev/null; then
    echo "错误: 未找到 Cargo。请先安装 Rust。"
    echo "访问 https://rustup.rs/ 下载安装 Rust"
    exit 1
fi

echo "检查依赖..."
if ! cargo check; then
    echo "错误: 依赖检查失败"
    exit 1
fi

echo
echo "启动应用程序..."
echo "应用程序将在 http://localhost:8000 运行"
echo "按 Ctrl+C 停止服务器"
echo

cargo run
