#!/bin/bash
# cnmsb 通用安装脚本
# 支持 Debian/Ubuntu 和 RHEL/Fedora/CentOS

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
    else
        DISTRO="unknown"
        DISTRO_FAMILY=""
    fi
}

# 检查是否为 Debian 系列
is_debian_based() {
    [[ "$DISTRO" == "debian" || "$DISTRO" == "ubuntu" || "$DISTRO" == "linuxmint" ]] || \
    [[ "$DISTRO_FAMILY" == *"debian"* ]]
}

# 检查是否为 Red Hat 系列
is_redhat_based() {
    [[ "$DISTRO" == "fedora" || "$DISTRO" == "rhel" || "$DISTRO" == "centos" || \
       "$DISTRO" == "rocky" || "$DISTRO" == "almalinux" ]] || \
    [[ "$DISTRO_FAMILY" == *"rhel"* || "$DISTRO_FAMILY" == *"fedora"* ]]
}

# 安装依赖
install_deps() {
    echo "检查依赖..."
    
    if ! command -v zsh >/dev/null 2>&1; then
        echo "安装 zsh..."
        if is_debian_based; then
            sudo apt-get update
            sudo apt-get install -y zsh
        elif is_redhat_based; then
            sudo dnf install -y zsh || sudo yum install -y zsh
        fi
    fi
    
    if ! command -v cargo >/dev/null 2>&1; then
        echo "安装 Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
}

# 从源码编译安装
install_from_source() {
    echo "从源码编译安装..."
    
    # 创建临时目录
    TMPDIR=$(mktemp -d)
    cd "$TMPDIR"
    
    # 下载源码
    echo "下载源码..."
    if command -v git >/dev/null 2>&1; then
        git clone --depth 1 "$REPO_URL.git" cnmsb
    else
        curl -sL "$REPO_URL/archive/refs/heads/main.tar.gz" | tar xz
        mv cnmsb-main cnmsb
    fi
    
    cd cnmsb/cnmsb-tool
    
    # 编译
    echo "编译中（可能需要几分钟）..."
    cargo build --release
    
    # 安装
    echo "安装..."
    sudo mkdir -p /usr/bin
    sudo mkdir -p /usr/share/cnmsb
    sudo mkdir -p /etc/profile.d
    
    sudo cp target/release/cnmsb /usr/bin/
    sudo ln -sf /usr/bin/cnmsb /usr/bin/cnmsb-sql
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
}

# 主函数
main() {
    detect_distro
    
    echo "检测到发行版: $DISTRO"
    
    if is_debian_based; then
        echo "系统类型: Debian/Ubuntu 系列"
    elif is_redhat_based; then
        echo "系统类型: Red Hat/Fedora 系列"
    else
        echo "系统类型: 其他 Linux"
    fi
    
    echo ""
    
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
}

main "$@"

