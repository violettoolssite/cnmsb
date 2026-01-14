#!/bin/bash
# 强制更新 WSL 中的 cnmsb 版本

set -e

echo "开始更新 cnmsb..."

# 获取脚本所在目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# 加载 Rust 环境（如果存在）
if [ -f "$HOME/.cargo/env" ]; then
    echo "加载 Rust 环境..."
    source "$HOME/.cargo/env"
fi

# 尝试多个可能的 cargo 路径
if ! command -v cargo &> /dev/null; then
    # 尝试直接使用 cargo 路径
    if [ -f "$HOME/.cargo/bin/cargo" ]; then
        export PATH="$HOME/.cargo/bin:$PATH"
    elif [ -f "/usr/local/cargo/bin/cargo" ]; then
        export PATH="/usr/local/cargo/bin:$PATH"
    fi
fi

# 检查 cargo 是否可用
if ! command -v cargo &> /dev/null; then
    echo "错误: 找不到 cargo 命令"
    echo ""
    echo "请先安装 Rust 或加载 Rust 环境:"
    echo "  1. 安装 Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "  2. 或者运行: source ~/.cargo/env"
    echo "  3. 或者手动添加: export PATH=\"\$HOME/.cargo/bin:\$PATH\""
    exit 1
fi

echo "找到 cargo: $(which cargo)"
echo "Rust 版本: $(rustc --version 2>/dev/null || echo '未知')"

# 检查并安装必要的编译依赖
echo "检查编译依赖..."
if ! command -v cc &> /dev/null && ! command -v gcc &> /dev/null; then
    echo "缺少 C 编译器，正在安装..."
    
    # 检测 Linux 发行版并安装相应的包
    if command -v apt-get &> /dev/null; then
        # Debian/Ubuntu
        sudo apt-get update
        sudo apt-get install -y build-essential pkg-config libssl-dev zlib1g-dev
    elif command -v yum &> /dev/null; then
        # CentOS/RHEL
        sudo yum groupinstall -y "Development Tools"
        sudo yum install -y openssl-devel pkgconfig zlib-devel
    elif command -v dnf &> /dev/null; then
        # Fedora
        sudo dnf groupinstall -y "Development Tools"
        sudo dnf install -y openssl-devel pkgconfig zlib-devel
    elif command -v pacman &> /dev/null; then
        # Arch Linux
        sudo pacman -S --noconfirm base-devel openssl pkgconf zlib
    elif command -v zypper &> /dev/null; then
        # openSUSE
        sudo zypper install -y -t pattern devel_basis
        sudo zypper install -y libopenssl-devel pkg-config zlib-devel
    else
        echo "警告: 无法自动检测 Linux 发行版，请手动安装编译工具:"
        echo "  - Debian/Ubuntu: sudo apt-get install build-essential pkg-config libssl-dev"
        echo "  - CentOS/RHEL: sudo yum groupinstall 'Development Tools' && sudo yum install openssl-devel"
        echo "  - Fedora: sudo dnf groupinstall 'Development Tools' && sudo dnf install openssl-devel"
        echo "  - Arch: sudo pacman -S base-devel openssl pkgconf"
        read -p "按 Enter 继续（如果已手动安装）或 Ctrl+C 取消..."
    fi
fi

# 再次检查
if ! command -v cc &> /dev/null && ! command -v gcc &> /dev/null; then
    echo "错误: 仍然找不到 C 编译器，请手动安装后重试"
    exit 1
fi

echo "编译依赖检查完成"

# 编译项目
echo "编译项目..."
cargo build --release

# 检查编译是否成功
if [ ! -f "target/release/cnmsb" ]; then
    echo "错误: 编译失败，找不到可执行文件"
    exit 1
fi

# 备份旧版本（如果存在）
if [ -f "/usr/bin/cnmsb" ]; then
    echo "备份旧版本..."
    sudo cp /usr/bin/cnmsb /usr/bin/cnmsb.backup.$(date +%Y%m%d_%H%M%S) 2>/dev/null || true
fi

# 安装新版本
echo "安装新版本..."
sudo cp target/release/cnmsb /usr/bin/
sudo chmod +x /usr/bin/cnmsb

# 创建符号链接（如果不存在）
if [ ! -L /usr/bin/cnmsb-sql ]; then
    echo "创建 cnmsb-sql 符号链接..."
    sudo ln -sf /usr/bin/cnmsb /usr/bin/cnmsb-sql
fi

if [ ! -L /usr/bin/cntmd ]; then
    echo "创建 cntmd 符号链接..."
    sudo ln -sf /usr/bin/cnmsb /usr/bin/cntmd
fi

# 更新 shell 集成脚本
echo "更新 shell 集成脚本..."
sudo mkdir -p /usr/share/cnmsb
sudo cp shell/cnmsb.zsh /usr/share/cnmsb/
sudo cp shell/cnmsb.bash /usr/share/cnmsb/

# 检查版本
echo ""
echo "更新完成！"
echo "当前版本:"
/usr/bin/cnmsb --version 2>/dev/null || /usr/bin/cnmsb 2>/dev/null | head -3 || echo "无法获取版本信息"

echo ""
echo "检查并安装 zsh..."

# 检查 zsh 是否已安装
if ! command -v zsh &> /dev/null; then
    echo "zsh 未安装，正在安装..."
    
    # 检测 Linux 发行版并安装 zsh
    if command -v apt-get &> /dev/null; then
        # Debian/Ubuntu
        sudo apt-get update
        sudo apt-get install -y zsh
    elif command -v yum &> /dev/null; then
        # CentOS/RHEL
        sudo yum install -y zsh
    elif command -v dnf &> /dev/null; then
        # Fedora
        sudo dnf install -y zsh
    elif command -v pacman &> /dev/null; then
        # Arch Linux
        sudo pacman -S --noconfirm zsh
    elif command -v zypper &> /dev/null; then
        # openSUSE
        sudo zypper install -y zsh
    else
        echo "警告: 无法自动检测 Linux 发行版，请手动安装 zsh"
        exit 1
    fi
    
    echo "✓ zsh 安装完成"
else
    echo "✓ zsh 已安装"
fi

# 检查 zsh 是否已设置为默认 shell
CURRENT_SHELL=$(basename "$SHELL" 2>/dev/null || echo "bash")
if [ -z "$CURRENT_SHELL" ] || [ "$CURRENT_SHELL" = "/" ]; then
    # 如果无法从 $SHELL 获取，尝试从进程获取
    CURRENT_SHELL=$(ps -p $$ -o comm= | sed 's/^-//')
fi

# 获取 zsh 路径
ZSH_PATH=$(which zsh)
if [ -z "$ZSH_PATH" ]; then
    ZSH_PATH="/usr/bin/zsh"
fi

# 如果当前 shell 不是 zsh，设置为默认
if [ "$CURRENT_SHELL" != "zsh" ]; then
    echo "将 zsh 设置为默认 shell..."
    # 检查 zsh 是否在 /etc/shells 中
    if ! grep -q "$ZSH_PATH" /etc/shells 2>/dev/null; then
        echo "$ZSH_PATH" | sudo tee -a /etc/shells
    fi
    # 设置默认 shell
    sudo chsh -s "$ZSH_PATH" "$USER" 2>/dev/null || {
        echo "注意: 无法自动更改默认 shell，请手动运行:"
        echo "  sudo chsh -s $ZSH_PATH $USER"
        echo "然后重新登录"
    }
    echo "✓ zsh 已设置为默认 shell（需要重新登录生效）"
else
    echo "✓ zsh 已经是默认 shell"
fi

echo ""
echo "配置 shell 集成..."

# 配置 zsh（总是配置，因为 zsh 是必需的）
if [ -f "$HOME/.zshrc" ]; then
    if ! grep -q "cnmsb.zsh" "$HOME/.zshrc"; then
        echo "配置 ~/.zshrc..."
        echo "" >> "$HOME/.zshrc"
        echo "# cnmsb - Linux 命令行智能补全工具" >> "$HOME/.zshrc"
        echo "[ -f /usr/share/cnmsb/cnmsb.zsh ] && source /usr/share/cnmsb/cnmsb.zsh" >> "$HOME/.zshrc"
        echo "✓ ~/.zshrc 已更新"
    else
        echo "✓ ~/.zshrc 已包含 cnmsb 配置"
    fi
else
    echo "创建 ~/.zshrc 并配置 cnmsb..."
    cat > "$HOME/.zshrc" << 'EOF'
# cnmsb - Linux 命令行智能补全工具
[ -f /usr/share/cnmsb/cnmsb.zsh ] && source /usr/share/cnmsb/cnmsb.zsh
EOF
    echo "✓ ~/.zshrc 已创建并配置"
fi

# 配置 bash
if [ -f "$HOME/.bashrc" ]; then
    if ! grep -q "cnmsb.bash" "$HOME/.bashrc"; then
        echo "配置 bash..."
        echo "" >> "$HOME/.bashrc"
        echo "# cnmsb - Linux 命令行智能补全工具" >> "$HOME/.bashrc"
        echo "[ -f /usr/share/cnmsb/cnmsb.bash ] && source /usr/share/cnmsb/cnmsb.bash" >> "$HOME/.bashrc"
    fi
    echo "✓ bash 配置已更新"
elif [ "$CURRENT_SHELL" = "bash" ]; then
    echo "创建 ~/.bashrc 并配置 cnmsb..."
    cat > "$HOME/.bashrc" << 'EOF'
# cnmsb - Linux 命令行智能补全工具
[ -f /usr/share/cnmsb/cnmsb.bash ] && source /usr/share/cnmsb/cnmsb.bash
EOF
    echo "✓ ~/.bashrc 已创建并配置"
fi

echo ""
echo "=========================================="
echo "安装完成！"
echo "=========================================="
echo ""
echo "重要提示:"
echo "  1. zsh 已安装并设置为默认 shell"
echo "  2. 请重新登录或运行以下命令切换到 zsh:"
echo "     exec zsh"
echo ""
echo "  3. 如果当前在 bash 中，可以立即切换到 zsh:"
echo "     zsh"
echo ""
echo "  4. 验证安装:"
echo "     cnmsb"
echo "     echo \$SHELL  # 应该显示 /usr/bin/zsh 或类似路径"
echo ""

