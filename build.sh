#!/bin/bash
# cnmsb 构建脚本

set -e

echo "======================================="
echo "  cnmsb (操你妈傻逼) 构建脚本"
echo "======================================="
echo

# 检查 Rust 环境
if ! command -v cargo &> /dev/null; then
    echo "错误: 未找到 cargo，请先安装 Rust"
    echo "安装命令: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "Rust 版本: $(rustc --version)"
echo "Cargo 版本: $(cargo --version)"
echo

# 选择构建模式
BUILD_MODE="${1:-release}"

case "$BUILD_MODE" in
    release)
        echo "构建发布版本..."
        cargo build --release
        BINARY_PATH="target/release/cnmsb"
        ;;
    debug)
        echo "构建调试版本..."
        cargo build
        BINARY_PATH="target/debug/cnmsb"
        ;;
    deb)
        echo "构建 Debian 包..."
        if ! command -v dpkg-buildpackage &> /dev/null; then
            echo "错误: 未找到 dpkg-buildpackage"
            echo "安装命令: sudo apt-get install debhelper"
            exit 1
        fi
        dpkg-buildpackage -us -uc -b
        echo
        echo "Debian 包已生成在上级目录"
        ls -la ../cnmsb*.deb 2>/dev/null || echo "未找到 .deb 文件"
        exit 0
        ;;
    *)
        echo "用法: $0 [release|debug|deb]"
        echo "  release - 构建发布版本（默认）"
        echo "  debug   - 构建调试版本"
        echo "  deb     - 构建 Debian 包"
        exit 1
        ;;
esac

echo
echo "构建完成！"
echo "二进制文件: $BINARY_PATH"
echo

# 显示安装说明
echo "======================================="
echo "  安装说明"
echo "======================================="
echo
echo "1. 复制二进制文件到系统路径:"
echo "   sudo cp $BINARY_PATH /usr/local/bin/"
echo
echo "2. 复制 shell 脚本:"
echo "   sudo mkdir -p /usr/share/cnmsb"
echo "   sudo cp shell/cnmsb.bash /usr/share/cnmsb/"
echo "   sudo cp shell/cnmsb.zsh /usr/share/cnmsb/"
echo
echo "3. 添加到 shell 配置文件:"
echo
echo "   Bash (~/.bashrc):"
echo "     eval \"\$(cnmsb init bash)\""
echo
echo "   Zsh (~/.zshrc):"
echo "     eval \"\$(cnmsb init zsh)\""
echo
echo "4. 重新加载 shell 或重新登录"
echo

