#!/bin/bash
# 构建 RPM 包脚本
# 需要在 CentOS/RHEL/Fedora 系统上运行

set -e

VERSION="0.1.1"
RELEASE="1"
NAME="cnmsb"

echo "======================================"
echo "  构建 cnmsb RPM 包"
echo "======================================"

# 检查依赖
if ! command -v rpmbuild &> /dev/null; then
    echo "安装 rpm-build..."
    sudo yum install -y rpm-build || sudo dnf install -y rpm-build
fi

# 创建 RPM 构建目录
mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# 编译
echo "1. 编译项目..."
cargo build --release

# 创建源码包
echo "2. 创建源码包..."
SRCDIR="${NAME}-${VERSION}"
mkdir -p "$SRCDIR"
cp target/release/cnmsb "$SRCDIR/"
cp shell/cnmsb.zsh "$SRCDIR/"
cp README.md "$SRCDIR/"
tar -czf ~/rpmbuild/SOURCES/${NAME}-${VERSION}.tar.gz "$SRCDIR"
rm -rf "$SRCDIR"

# 创建 spec 文件
echo "3. 创建 spec 文件..."
cat > ~/rpmbuild/SPECS/${NAME}.spec << EOF
Name:           ${NAME}
Version:        ${VERSION}
Release:        ${RELEASE}%{?dist}
Summary:        Linux 命令行智能补全工具

License:        MIT
URL:            https://github.com/violettoolssite/cnmsb
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  zsh
Requires:       zsh

%description
cnmsb - 操你妈傻逼，Linux 命令行智能补全工具，提供类似 IDE 的补全体验。

%prep
%setup -q

%install
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/share/cnmsb
cp cnmsb %{buildroot}/usr/bin/
cp cnmsb.zsh %{buildroot}/usr/share/cnmsb/
ln -sf /usr/bin/cnmsb %{buildroot}/usr/bin/cntmd

%post
# 配置 zsh
REAL_USER=\${SUDO_USER:-\$USER}
REAL_HOME=\$(getent passwd "\$REAL_USER" | cut -d: -f6)
ZSHRC="\$REAL_HOME/.zshrc"

if [ ! -f "\$ZSHRC" ]; then
    touch "\$ZSHRC"
    chown "\$REAL_USER:\$REAL_USER" "\$ZSHRC"
fi

if ! grep -q "cnmsb" "\$ZSHRC" 2>/dev/null; then
    echo "" >> "\$ZSHRC"
    echo "# cnmsb 智能命令补全" >> "\$ZSHRC"
    echo "source /usr/share/cnmsb/cnmsb.zsh" >> "\$ZSHRC"
fi

echo ""
echo "====================================="
echo "  cnmsb 安装成功！"
echo "====================================="
echo ""
echo "请重新登录或运行 'zsh' 开始使用"

%files
/usr/bin/cnmsb
/usr/bin/cntmd
/usr/share/cnmsb/cnmsb.zsh

%changelog
* $(date "+%a %b %d %Y") cnmsb <violettools.site@gmail.com> - ${VERSION}-${RELEASE}
- Initial RPM release
EOF

# 构建 RPM
echo "4. 构建 RPM..."
rpmbuild -bb ~/rpmbuild/SPECS/${NAME}.spec

# 复制结果
echo "5. 复制 RPM 包..."
cp ~/rpmbuild/RPMS/x86_64/${NAME}-${VERSION}-${RELEASE}*.rpm ./ 2>/dev/null || \
cp ~/rpmbuild/RPMS/noarch/${NAME}-${VERSION}-${RELEASE}*.rpm ./ 2>/dev/null || \
echo "RPM 包在 ~/rpmbuild/RPMS/ 目录"

echo ""
echo "======================================"
echo "  构建完成！"
echo "======================================"
ls -la *.rpm 2>/dev/null || ls -la ~/rpmbuild/RPMS/*/*.rpm

