#!/bin/bash
# 构建发布包脚本
# 生成 deb 包、预编译二进制 tarball 和 APT 仓库文件

set -e

VERSION="0.1.1"
ARCH="amd64"

echo "======================================"
echo "  构建 cnmsb v${VERSION} 发布包"
echo "======================================"

# 1. 编译
echo ""
echo "1. 编译 Rust 项目..."
cargo build --release

# 2. 创建预编译二进制 tarball
echo ""
echo "2. 创建预编译二进制包..."
TARBALL_DIR="cnmsb-${VERSION}-linux-${ARCH}"
rm -rf "$TARBALL_DIR"
mkdir -p "$TARBALL_DIR"

cp target/release/cnmsb "$TARBALL_DIR/"
cp shell/cnmsb.zsh "$TARBALL_DIR/"
cp README.md "$TARBALL_DIR/"
cp LICENSE "$TARBALL_DIR/" 2>/dev/null || echo "LICENSE 文件不存在，跳过"

# 创建安装说明
cat > "$TARBALL_DIR/INSTALL.txt" << 'EOF'
cnmsb 安装说明
==============

1. 复制二进制文件：
   sudo cp cnmsb /usr/local/bin/

2. 复制 zsh 集成脚本：
   sudo mkdir -p /usr/share/cnmsb
   sudo cp cnmsb.zsh /usr/share/cnmsb/

3. 添加到 ~/.zshrc：
   echo 'source /usr/share/cnmsb/cnmsb.zsh' >> ~/.zshrc

4. 重新登录或执行：
   source ~/.zshrc

完成！输入命令后按 Tab 即可体验智能补全。
EOF

tar -czf "cnmsb-linux-${ARCH}.tar.gz" "$TARBALL_DIR"
rm -rf "$TARBALL_DIR"
echo "   创建完成: cnmsb-linux-${ARCH}.tar.gz"

# 3. 构建 deb 包（如果 build-deb.sh 存在）
if [[ -f "build-deb.sh" ]]; then
    echo ""
    echo "3. 构建 deb 包..."
    bash build-deb.sh
fi

# 4. 显示结果
echo ""
echo "======================================"
echo "  构建完成！"
echo "======================================"
echo ""
echo "生成的文件："
ls -lh cnmsb-linux-*.tar.gz 2>/dev/null || true
ls -lh cnmsb_*.deb 2>/dev/null || true
echo ""
echo "上传这些文件到 GitHub Release 即可。"

