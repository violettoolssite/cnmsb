#!/bin/bash
# cnmsb deb 包构建脚本

set -e

VERSION="0.1.1"
ARCH="amd64"
PKG_NAME="cnmsb_${VERSION}_${ARCH}"
SRC_DIR="$(pwd)"
BUILD_DIR="/tmp/cnmsb-build-$$"

echo "======================================"
echo "  构建 cnmsb deb 包"
echo "======================================"
echo

# 构建 release 版本
echo "1. 编译 Rust 项目..."

# 检测 WSL 并设置编译目录（避免 Windows 文件系统权限问题）
if grep -qi microsoft /proc/version 2>/dev/null; then
    export CARGO_TARGET_DIR="$HOME/.cargo/cnmsb-build"
    echo "检测到 WSL 环境，使用 CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
fi

cargo build --release

# 确定二进制文件路径（WSL 使用自定义 target 目录）
if [ -n "$CARGO_TARGET_DIR" ] && [ -f "$CARGO_TARGET_DIR/release/cnmsb" ]; then
    BINARY_PATH="$CARGO_TARGET_DIR/release/cnmsb"
else
    BINARY_PATH="target/release/cnmsb"
fi

# 在 Linux 原生文件系统创建包目录
echo "2. 创建包目录结构..."
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR/$PKG_NAME/DEBIAN"
mkdir -p "$BUILD_DIR/$PKG_NAME/usr/bin"
mkdir -p "$BUILD_DIR/$PKG_NAME/usr/share/cnmsb"
mkdir -p "$BUILD_DIR/$PKG_NAME/etc/profile.d"

# 复制文件
echo "3. 复制文件..."
cp "$BINARY_PATH" "$BUILD_DIR/$PKG_NAME/usr/bin/cnmsb"
chmod 755 "$BUILD_DIR/$PKG_NAME/usr/bin/cnmsb"

# 创建 cntmd 符号链接
ln -sf cnmsb "$BUILD_DIR/$PKG_NAME/usr/bin/cntmd"

cp shell/cnmsb.bash "$BUILD_DIR/$PKG_NAME/usr/share/cnmsb/"
cp shell/cnmsb.zsh "$BUILD_DIR/$PKG_NAME/usr/share/cnmsb/"
chmod 644 "$BUILD_DIR/$PKG_NAME/usr/share/cnmsb/"*

cp debian/cnmsb.sh "$BUILD_DIR/$PKG_NAME/etc/profile.d/"
chmod 644 "$BUILD_DIR/$PKG_NAME/etc/profile.d/cnmsb.sh"

# 设置正确权限
chmod 755 "$BUILD_DIR/$PKG_NAME/DEBIAN"

# 创建 control 文件
echo "4. 创建控制文件..."
cat > "$BUILD_DIR/$PKG_NAME/DEBIAN/control" << EOF
Package: cnmsb
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Depends: zsh
Maintainer: cnmsb contributors <cnmsb@example.com>
Description: 操你妈傻逼 - Linux 命令行智能补全工具
 cnmsb 是一个为 Linux 命令行提供类似 IDE 编辑器补全体验的工具。
 让终端不再是程序员专属，零基础用户也能用自然语言生成命令。
 .
 主要功能：
  - 智能内联建议（灰色预测文字）
  - 交互式补全选择器（Tab 打开，上下选择）
  - 命令参数和选项补全（支持 300+ 常用命令）
  - 组合参数补全（如 tar -zxvf）
  - 问号帮助模式（输入 ? 查看所有选项）
  - 历史命令模式（Alt+H）
  - AI 智能补全（Alt+L）用自然语言生成命令
 .
 包含命令：
  - cnmsb: 主程序
  - cntmd: 智能补全编辑器（类 vim）
 .
 安装后自动配置 Zsh 为默认 shell。
EOF

# 创建 postinst 脚本
cat > "$BUILD_DIR/$PKG_NAME/DEBIAN/postinst" << 'POSTINST'
#!/bin/bash
set -e

# 获取当前实际用户（不是 root）
if [ -n "$SUDO_USER" ]; then
    REAL_USER="$SUDO_USER"
else
    REAL_USER="$(logname 2>/dev/null || echo $USER)"
fi

REAL_HOME=$(getent passwd "$REAL_USER" | cut -d: -f6)

echo ""
echo "====================================="
echo "  cnmsb (操你妈傻逼) 安装成功！"
echo "  包含: cnmsb, cntmd 编辑器"
echo "====================================="
echo ""

# 检查 zsh 是否安装
if ! command -v zsh &> /dev/null; then
    echo "正在安装 zsh..."
    apt-get update -qq
    apt-get install -y -qq zsh
fi

ZSH_PATH=$(which zsh)

# 配置 .zshrc
ZSHRC="$REAL_HOME/.zshrc"

echo "配置用户 $REAL_USER 的 zsh..."

# 创建 .zshrc 如果不存在
if [ ! -f "$ZSHRC" ]; then
    touch "$ZSHRC"
    chown "$REAL_USER:$REAL_USER" "$ZSHRC"
fi

# 检查是否已配置 cnmsb
if ! grep -q "cnmsb" "$ZSHRC" 2>/dev/null; then
    echo "" >> "$ZSHRC"
    echo "# cnmsb 智能命令补全" >> "$ZSHRC"
    echo "source /usr/share/cnmsb/cnmsb.zsh" >> "$ZSHRC"
    echo "已添加 cnmsb 到 $ZSHRC"
fi

# 提示用户如何改变默认 shell（不自动改变，避免覆盖用户设置）
CURRENT_SHELL=$(getent passwd "$REAL_USER" | cut -d: -f7)

if [ "$CURRENT_SHELL" != "$ZSH_PATH" ]; then
    echo ""
    echo "提示: 当前默认 shell 是 $CURRENT_SHELL"
    echo "如需将默认 shell 改为 zsh，请执行: chsh -s $ZSH_PATH"
fi

echo ""
echo "====================================="
echo "  配置完成！"
echo "====================================="
echo ""
echo "命令补全 (cnmsb)："
echo "  - 输入时自动显示建议 (灰色)"
echo "  - Tab 打开选择器 / 确认选择"
echo "  - ↑↓ 键切换不同建议"
echo "  - → 接受建议"
echo "  - ? 查看命令帮助"
echo ""
echo "智能编辑器 (cntmd)："
echo "  - cntmd <文件名> 打开编辑器"
echo "  - i 进入插入模式，Esc 返回普通模式"
echo "  - :w 保存，:q 退出，:q! 强制退出"
echo "  - Tab 接受补全建议"
echo ""
echo "请重新登录或运行 'zsh' 开始使用！"
echo ""

exit 0
POSTINST
chmod 755 "$BUILD_DIR/$PKG_NAME/DEBIAN/postinst"

# 创建 prerm 脚本
cat > "$BUILD_DIR/$PKG_NAME/DEBIAN/prerm" << 'EOF'
#!/bin/sh
set -e
echo "正在卸载 cnmsb..."
echo "提示: 你可能需要手动从 ~/.zshrc 中移除 cnmsb 相关配置"
exit 0
EOF
chmod 755 "$BUILD_DIR/$PKG_NAME/DEBIAN/prerm"

# 构建 deb 包
echo "5. 构建 deb 包..."
cd "$BUILD_DIR"
dpkg-deb --build "$PKG_NAME"

# 复制回源目录
cp "${PKG_NAME}.deb" "$SRC_DIR/"

# 清理
rm -rf "$BUILD_DIR"

echo ""
echo "======================================"
echo "  构建完成！"
echo "======================================"
echo ""
echo "deb 包: $SRC_DIR/${PKG_NAME}.deb"
echo ""
echo "安装命令: sudo dpkg -i ${PKG_NAME}.deb"
echo ""
echo "安装后会自动："
echo "  1. 安装 zsh (如果未安装)"
echo "  2. 将用户默认 shell 改为 zsh"
echo "  3. 配置 ~/.zshrc 加载 cnmsb"
echo ""

cd "$SRC_DIR"
ls -lh "${PKG_NAME}.deb"
