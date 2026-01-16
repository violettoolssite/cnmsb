#!/bin/bash
# cnmsb 通用安装脚本
# 支持所有主流 Linux 发行版

set -e

VERSION="0.1.0"
REPO_URL="https://github.com/violettoolssite/cnmsb"

echo ""
echo "====================================="
echo "  cnmsb - Linux 命令行智能补全工具"
echo "====================================="
echo ""

# 检测发行版
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO="$ID"
        DISTRO_FAMILY="$ID_LIKE"
    elif [ -f /etc/redhat-release ]; then
        DISTRO="rhel"
        DISTRO_FAMILY="rhel fedora"
    elif [ -f /etc/debian_version ]; then
        DISTRO="debian"
        DISTRO_FAMILY="debian"
    elif [ -f /etc/arch-release ]; then
        DISTRO="arch"
        DISTRO_FAMILY="arch"
    elif [ -f /etc/SuSE-release ] || [ -f /etc/SUSE-brand ]; then
        DISTRO="opensuse"
        DISTRO_FAMILY="suse"
    else
        DISTRO="unknown"
        DISTRO_FAMILY=""
    fi
}

# 检查是否为 Debian 系列
is_debian_based() {
    [[ "$DISTRO" == "debian" || "$DISTRO" == "ubuntu" || "$DISTRO" == "linuxmint" || "$DISTRO" == "pop" || "$DISTRO" == "elementary" || "$DISTRO" == "kali" ]] || \
    [[ "$DISTRO_FAMILY" == *"debian"* || "$DISTRO_FAMILY" == *"ubuntu"* ]]
}

# 检查是否为 Red Hat 系列
is_redhat_based() {
    [[ "$DISTRO" == "fedora" || "$DISTRO" == "rhel" || "$DISTRO" == "centos" || \
       "$DISTRO" == "rocky" || "$DISTRO" == "almalinux" || "$DISTRO" == "ol" ]] || \
    [[ "$DISTRO_FAMILY" == *"rhel"* || "$DISTRO_FAMILY" == *"fedora"* ]]
}

# 检查是否为 Arch 系列
is_arch_based() {
    [[ "$DISTRO" == "arch" || "$DISTRO" == "manjaro" || "$DISTRO" == "endeavouros" || "$DISTRO" == "garuda" ]] || \
    [[ "$DISTRO_FAMILY" == *"arch"* ]]
}

# 检查是否为 SUSE 系列
is_suse_based() {
    [[ "$DISTRO" == "opensuse"* || "$DISTRO" == "sles" || "$DISTRO" == "opensuse-leap" || "$DISTRO" == "opensuse-tumbleweed" ]] || \
    [[ "$DISTRO_FAMILY" == *"suse"* ]]
}

# 安装依赖
install_deps() {
    echo "检查依赖..."
    
    # 安装基本工具
    if ! command -v curl >/dev/null 2>&1; then
        echo "安装 curl..."
        install_package curl
    fi
    
    # 安装 git（如果没有）
    if ! command -v git >/dev/null 2>&1; then
        echo "安装 git..."
        install_package git
    fi
    
    # 安装 zsh
    if ! command -v zsh >/dev/null 2>&1; then
        echo "安装 zsh..."
        install_package zsh
    fi
    
    # 安装/更新 Rust（需要 1.82+）
    if ! command -v cargo >/dev/null 2>&1; then
        echo "安装 Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    else
        # 检查 Rust 版本，如果太旧则更新
        # 使用兼容的方式提取版本号（避免 grep -P）
        RUST_VER=$(rustc --version | sed 's/rustc \([0-9]*\.[0-9]*\).*/\1/')
        RUST_MAJOR=$(echo "$RUST_VER" | cut -d. -f1)
        RUST_MINOR=$(echo "$RUST_VER" | cut -d. -f2)
        if [ "$RUST_MAJOR" -eq 1 ] && [ "$RUST_MINOR" -lt 82 ]; then
            echo "Rust 版本 $RUST_VER 太旧，需要 1.82+，正在更新..."
            if command -v rustup >/dev/null 2>&1; then
                rustup update stable
            else
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            fi
            source "$HOME/.cargo/env"
        fi
    fi
    
    # 安装编译依赖
    echo "安装编译依赖..."
    if is_debian_based; then
        sudo apt-get update
        sudo apt-get install -y build-essential pkg-config libssl-dev
    elif is_redhat_based; then
        sudo dnf groupinstall -y "Development Tools" 2>/dev/null || sudo yum groupinstall -y "Development Tools"
        sudo dnf install -y openssl-devel 2>/dev/null || sudo yum install -y openssl-devel
    elif is_arch_based; then
        sudo pacman -Sy --noconfirm base-devel openssl
    elif is_suse_based; then
        sudo zypper install -y -t pattern devel_basis
        sudo zypper install -y libopenssl-devel
    fi
}

# 通用包安装函数
install_package() {
    local pkg=$1
    
    if is_debian_based; then
        sudo apt-get update
        sudo apt-get install -y "$pkg"
    elif is_redhat_based; then
        sudo dnf install -y "$pkg" 2>/dev/null || sudo yum install -y "$pkg"
    elif is_arch_based; then
        sudo pacman -Sy --noconfirm "$pkg"
    elif is_suse_based; then
        sudo zypper install -y "$pkg"
    else
        echo "警告: 未知的包管理器，请手动安装 $pkg"
    fi
}

# 从源码编译安装
install_from_source() {
    echo "从源码编译安装..."
    
    # 确保 cargo 在 PATH 中
    [ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env"
    
    # 创建临时目录（确保在 Linux 原生文件系统中）
    # 在 WSL 中使用 $HOME 下的目录避免权限问题
    if grep -qi microsoft /proc/version 2>/dev/null; then
        echo "检测到 WSL 环境，使用 Linux 原生文件系统编译..."
        TMPDIR="$HOME/.cnmsb-build-tmp"
        rm -rf "$TMPDIR"
        mkdir -p "$TMPDIR"
    else
    TMPDIR=$(mktemp -d)
    fi
    cd "$TMPDIR"
    
    # 下载源码
    echo "下载源码..."
    if command -v git >/dev/null 2>&1; then
        git clone --depth 1 "$REPO_URL.git" cnmsb
    else
        curl -sL "$REPO_URL/archive/refs/heads/main.tar.gz" | tar xz
        mv cnmsb-main cnmsb
    fi
    
    cd cnmsb
    
    # 编译（WSL 中设置 target 目录到 Linux 原生文件系统）
    echo "编译中（可能需要几分钟）..."
    if grep -qi microsoft /proc/version 2>/dev/null; then
        export CARGO_TARGET_DIR="$HOME/.cargo/cnmsb-build"
        cargo build --release
    else
    cargo build --release
    fi
    
    # 安装
    echo "安装..."
    sudo mkdir -p /usr/bin
    sudo mkdir -p /usr/share/cnmsb
    sudo mkdir -p /etc/profile.d
    
    # 确定二进制文件路径（WSL 使用自定义 target 目录）
    if [ -n "$CARGO_TARGET_DIR" ] && [ -f "$CARGO_TARGET_DIR/release/cnmsb" ]; then
        BINARY_PATH="$CARGO_TARGET_DIR/release/cnmsb"
    else
        BINARY_PATH="target/release/cnmsb"
    fi
    
    sudo cp "$BINARY_PATH" /usr/bin/
    sudo chmod +x /usr/bin/cnmsb
    
    # 如果 /usr/local/bin/cnmsb 存在，也更新它（避免 PATH 优先级问题）
    if [ -f /usr/local/bin/cnmsb ]; then
        sudo cp "$BINARY_PATH" /usr/local/bin/cnmsb
        sudo chmod +x /usr/local/bin/cnmsb
        echo "已更新 /usr/local/bin/cnmsb"
    fi
    
    # 创建命令别名
    sudo ln -sf /usr/bin/cnmsb /usr/bin/cnmsb-sql
    sudo ln -sf /usr/bin/cnmsb /usr/bin/cntmd
    
    # 复制 shell 集成脚本
    sudo cp shell/cnmsb.zsh /usr/share/cnmsb/
    sudo cp shell/cnmsb.bash /usr/share/cnmsb/
    
    # 创建 profile.d 脚本
    sudo tee /etc/profile.d/cnmsb.sh > /dev/null << 'EOF'
# cnmsb - Linux 命令行智能补全工具
if [ -n "$ZSH_VERSION" ]; then
    [ -f /usr/share/cnmsb/cnmsb.zsh ] && source /usr/share/cnmsb/cnmsb.zsh
fi
EOF
    
    # 清理
    cd /
    rm -rf "$TMPDIR"
    
    echo ""
    echo "====================================="
    echo "  安装成功！"
    echo "====================================="
}

# 配置 Zsh
setup_zsh() {
    echo ""
    echo "配置 Zsh..."
    
    # 添加到 .zshrc
    if [ -f "$HOME/.zshrc" ]; then
        if ! grep -q "cnmsb.zsh" "$HOME/.zshrc"; then
            echo "" >> "$HOME/.zshrc"
            echo "# cnmsb 智能补全" >> "$HOME/.zshrc"
            echo "[ -f /usr/share/cnmsb/cnmsb.zsh ] && source /usr/share/cnmsb/cnmsb.zsh" >> "$HOME/.zshrc"
            echo "已添加到 ~/.zshrc"
        else
            echo "~/.zshrc 已配置"
        fi
    else
        echo "# cnmsb 智能补全" > "$HOME/.zshrc"
        echo "[ -f /usr/share/cnmsb/cnmsb.zsh ] && source /usr/share/cnmsb/cnmsb.zsh" >> "$HOME/.zshrc"
        echo "已创建 ~/.zshrc"
    fi
    
    # 询问是否将默认 shell 改为 zsh
    ZSH_PATH=$(which zsh 2>/dev/null)
    CURRENT_SHELL=$(getent passwd "$USER" 2>/dev/null | cut -d: -f7)
    
    if [ -n "$ZSH_PATH" ] && [ "$CURRENT_SHELL" != "$ZSH_PATH" ]; then
        echo ""
        echo "当前默认 shell: $CURRENT_SHELL"
        echo -n "是否将默认 shell 改为 zsh? [y/N]: "
        read -r change_shell
        
        if [ "$change_shell" = "y" ] || [ "$change_shell" = "Y" ]; then
            chsh -s "$ZSH_PATH" 2>/dev/null && echo "默认 shell 已更改为 zsh" || echo "更改失败，请手动执行: chsh -s $ZSH_PATH"
        fi
    fi
}

# 显示系统信息
show_system_info() {
    echo "检测到发行版: $DISTRO"
    
    if is_debian_based; then
        echo "系统类型: Debian/Ubuntu 系列"
    elif is_redhat_based; then
        echo "系统类型: Red Hat/Fedora/CentOS 系列"
    elif is_arch_based; then
        echo "系统类型: Arch Linux 系列"
    elif is_suse_based; then
        echo "系统类型: openSUSE/SLES 系列"
    else
        echo "系统类型: 其他 Linux（将尝试通用安装）"
    fi
    
    echo ""
}

# 主函数
main() {
    detect_distro
    show_system_info
    install_deps
    install_from_source
    setup_zsh
    
    echo ""
    echo "使用方法："
    echo "  1. 重新登录"
    echo "  2. 或执行: source /usr/share/cnmsb/cnmsb.zsh"
    echo ""
    echo "如果当前 shell 不是 zsh，请执行："
    echo "  chsh -s \$(which zsh)"
    echo ""
    echo "可用命令："
    echo "  cnmsb        - 命令行智能补全（自动生效）"
    echo "  cnmsb edit   - 打开智能补全编辑器"
    echo "  cntmd        - 打开智能补全编辑器（快捷命令）"
    echo "  cnmsb-sql    - SQL 智能补全客户端"
    echo ""
    echo "编辑器示例："
    echo "  cntmd myfile.txt"
    echo "  cnmsb edit myfile.txt"
    echo ""
    echo "支持的发行版："
    echo "  - Debian/Ubuntu/Mint/Pop!_OS/Kali"
    echo "  - Fedora/RHEL/CentOS/Rocky/Alma"
    echo "  - Arch/Manjaro/EndeavourOS"
    echo "  - openSUSE/SLES"
    echo ""
}

main "$@"
