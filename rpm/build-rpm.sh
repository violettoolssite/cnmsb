#!/bin/bash
# 构建 RPM 包的脚本

set -e

VERSION="0.1.0"
PACKAGE="cnmsb"

echo "====================================="
echo "  构建 cnmsb RPM 包"
echo "====================================="

# 检查依赖
check_deps() {
    local missing=()
    
    command -v cargo >/dev/null 2>&1 || missing+=("cargo/rust")
    command -v rpmbuild >/dev/null 2>&1 || missing+=("rpm-build")
    
    if [ ${#missing[@]} -gt 0 ]; then
        echo "缺少依赖: ${missing[*]}"
        echo ""
        echo "安装方法:"
        echo "  Fedora: sudo dnf install rust cargo rpm-build"
        echo "  RHEL/CentOS: sudo dnf install rust cargo rpm-build"
        exit 1
    fi
}

check_deps

# 获取脚本所在目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

echo "1. 编译 Rust 项目..."
cargo build --release

echo "2. 准备源码包..."
# 创建临时目录
TMPDIR=$(mktemp -d)
SRCDIR="$TMPDIR/$PACKAGE-$VERSION"
mkdir -p "$SRCDIR"

# 复制必要文件
cp -r src "$SRCDIR/"
cp -r shell "$SRCDIR/"
cp Cargo.toml Cargo.lock "$SRCDIR/"
cp LICENSE README.md "$SRCDIR/" 2>/dev/null || true
mkdir -p "$SRCDIR/target/release"
cp target/release/cnmsb "$SRCDIR/target/release/"

# 创建 tarball
cd "$TMPDIR"
tar czvf "$PACKAGE-$VERSION.tar.gz" "$PACKAGE-$VERSION"

echo "3. 设置 rpmbuild 目录..."
mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
cp "$PACKAGE-$VERSION.tar.gz" ~/rpmbuild/SOURCES/
cp "$PROJECT_DIR/rpm/cnmsb.spec" ~/rpmbuild/SPECS/

echo "4. 构建 RPM..."
cd ~/rpmbuild/SPECS
rpmbuild -ba cnmsb.spec

echo "5. 清理..."
rm -rf "$TMPDIR"

echo ""
echo "====================================="
echo "  构建完成！"
echo "====================================="
echo ""
echo "RPM 包位置:"
echo "  ~/rpmbuild/RPMS/x86_64/cnmsb-$VERSION-1*.rpm"
echo ""
echo "SRPM 包位置:"
echo "  ~/rpmbuild/SRPMS/cnmsb-$VERSION-1*.src.rpm"
echo ""
echo "安装方法:"
echo "  sudo dnf install ~/rpmbuild/RPMS/x86_64/cnmsb-$VERSION-1*.rpm"

