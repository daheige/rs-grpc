#!/usr/bin/env bash
root_dir=$(cd "$(dirname "$0")"; cd ..; pwd)

# 启动二进制，其实就是/app/main
RUST_LOG=info $root_dir/main
