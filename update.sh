#!/bin/bash
# cnmsb 更新脚本

set -e

echo "开始更新 cnmsb..."

# 加载 Rust 环境
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
    echo "已加载 Rust 环境"
else
    export PATH="$HOME/.cargo/bin:$PATH"
    echo "使用 PATH 中的 Rust"
fi

# 检查 cargo 是否可用
if ! command -v cargo &> /dev/null; then
    echo "错误: 未找到 cargo，请先安装 Rust"
    echo "运行: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "找到 cargo: $(which cargo)"
echo "Rust 版本: $(rustc --version)"

# 检查编译依赖
echo ""
echo "检查编译依赖..."

# 检测系统类型
if [ -f /etc/debian_version ]; then
    # Debian/Ubuntu
    echo "检测到 Debian/Ubuntu 系统"
    
    # 检查并安装必要的依赖
    MISSING_DEPS=()
    
    if ! dpkg -l | grep -q "^ii.*build-essential"; then
        MISSING_DEPS+=("build-essential")
    fi
    
    if ! dpkg -l | grep -q "^ii.*pkg-config"; then
        MISSING_DEPS+=("pkg-config")
    fi
    
    if ! dpkg -l | grep -q "^ii.*libssl-dev"; then
        MISSING_DEPS+=("libssl-dev")
    fi
    
    if ! dpkg -l | grep -q "^ii.*zlib1g-dev"; then
        MISSING_DEPS+=("zlib1g-dev")
    fi
    
    if [ ${#MISSING_DEPS[@]} -gt 0 ]; then
        echo "发现缺失的依赖: ${MISSING_DEPS[*]}"
        echo "正在安装..."
        sudo apt-get update
        sudo apt-get install -y "${MISSING_DEPS[@]}"
    else
        echo "所有编译依赖已安装"
    fi
elif [ -f /etc/redhat-release ]; then
    # Red Hat/CentOS/Fedora
    echo "检测到 Red Hat/CentOS/Fedora 系统"
    # 可以添加相应的依赖检查
fi

echo "编译依赖检查完成"
echo ""

# 清理旧的编译缓存（解决权限问题）
echo "清理旧的编译缓存..."
cargo clean 2>/dev/null || true

# 编译项目
echo "编译项目..."
# 在 WSL 中，如果项目在 /mnt/c/ 下，建议使用 release 模式并设置环境变量
if [[ "$PWD" == /mnt/c/* ]]; then
    echo "检测到 Windows 文件系统，使用特殊编译选项..."
    export CARGO_TARGET_DIR="$HOME/.cargo/cnmsb-build"
    mkdir -p "$CARGO_TARGET_DIR"
    cargo build --release --target-dir "$CARGO_TARGET_DIR"
    BINARY_PATH="$CARGO_TARGET_DIR/release/cnmsb"
else
    cargo build --release
    BINARY_PATH="$SCRIPT_DIR/target/release/cnmsb"
fi

if [ $? -ne 0 ]; then
    echo "编译失败！"
    exit 1
fi

echo "编译成功！"
echo ""

# 运行测试
echo "运行测试..."
cargo test --lib

if [ $? -ne 0 ]; then
    echo "警告: 部分测试失败，但继续安装..."
fi

echo ""

# 安装新版本
echo "安装新版本..."

# 获取项目目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# 确定二进制文件路径
if [[ "$PWD" == /mnt/c/* ]] && [ -n "$CARGO_TARGET_DIR" ]; then
    BINARY_PATH="$CARGO_TARGET_DIR/release/cnmsb"
else
    BINARY_PATH="$SCRIPT_DIR/target/release/cnmsb"
fi

if [ ! -f "$BINARY_PATH" ]; then
    echo "错误: 未找到编译后的二进制文件: $BINARY_PATH"
    exit 1
fi

# 备份旧版本
if [ -f "/usr/local/bin/cnmsb" ]; then
    echo "备份旧版本到 /usr/local/bin/cnmsb.bak"
    sudo cp /usr/local/bin/cnmsb /usr/local/bin/cnmsb.bak
fi

# 安装新版本
echo "复制新版本到 /usr/local/bin/cnmsb"
sudo cp "$BINARY_PATH" /usr/local/bin/cnmsb
sudo chmod +x /usr/local/bin/cnmsb

# 创建符号链接
echo "创建 cnmsb-sql 符号链接..."
sudo ln -sf /usr/local/bin/cnmsb /usr/local/bin/cnmsb-sql

echo "创建 cntmd 符号链接..."
sudo ln -sf /usr/local/bin/cnmsb /usr/local/bin/cntmd

# 更新 shell 集成脚本
echo "更新 shell 集成脚本..."

# 检测当前 shell
CURRENT_SHELL=$(basename "$SHELL")

if [ "$CURRENT_SHELL" = "zsh" ]; then
    SHELL_RC="$HOME/.zshrc"
    SHELL_SCRIPT="$SCRIPT_DIR/shell/cnmsb.zsh"
elif [ "$CURRENT_SHELL" = "bash" ]; then
    SHELL_RC="$HOME/.bashrc"
    SHELL_SCRIPT="$SCRIPT_DIR/shell/cnmsb.zsh"  # 使用 zsh 脚本（因为只支持 zsh）
else
    echo "警告: 未识别的 shell: $CURRENT_SHELL"
    SHELL_RC="$HOME/.zshrc"
    SHELL_SCRIPT="$SCRIPT_DIR/shell/cnmsb.zsh"
fi

# 确保 shell 配置文件存在
if [ ! -f "$SHELL_RC" ]; then
    echo "创建 $SHELL_RC"
    touch "$SHELL_RC"
fi

# 检查是否已存在 cnmsb 配置
if ! grep -q "source.*cnmsb.zsh" "$SHELL_RC"; then
    echo "添加 cnmsb 配置到 $SHELL_RC"
    echo "" >> "$SHELL_RC"
    echo "# cnmsb 智能补全" >> "$SHELL_RC"
    echo "source $SHELL_SCRIPT" >> "$SHELL_RC"
else
    echo "cnmsb 配置已存在于 $SHELL_RC"
fi

echo ""
echo "更新完成！"
echo ""
echo "当前版本:"
cnmsb --version 2>/dev/null || echo "cnmsb $(cargo metadata --format-version 1 | grep -o '"version":"[^"]*' | head -1 | cut -d'"' -f4)"
echo ""
echo "请重新加载 shell 配置:"
echo "  source $SHELL_RC"
echo "或者重新打开终端"

