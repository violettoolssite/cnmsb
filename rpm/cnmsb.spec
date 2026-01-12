Name:           cnmsb
Version:        0.1.0
Release:        1%{?dist}
Summary:        Linux 命令行智能补全工具

License:        MIT
URL:            https://github.com/violettoolssite/cnmsb
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust >= 1.70
BuildRequires:  cargo
BuildRequires:  gcc

Requires:       zsh

%description
cnmsb 是一个为 Linux 命令行提供类似 IDE 编辑器补全体验的工具。

主要功能：
- 智能内联建议
- 交互式补全选择器
- 问号帮助模式
- 组合参数补全
- 智能路径补全
- 历史命令集成
- 实时更新
- 颜色区分

支持 300+ 常用命令，仅支持 Zsh shell。

%prep
%autosetup -n %{name}-%{version}

%build
export CARGO_HOME=%{_builddir}/cargo
cargo build --release

%install
rm -rf %{buildroot}

# 安装二进制文件
mkdir -p %{buildroot}%{_bindir}
install -m 755 target/release/cnmsb %{buildroot}%{_bindir}/cnmsb

# 创建 cnmsb-sql 符号链接
ln -sf cnmsb %{buildroot}%{_bindir}/cnmsb-sql

# 安装 shell 脚本
mkdir -p %{buildroot}%{_datadir}/cnmsb
install -m 644 shell/cnmsb.zsh %{buildroot}%{_datadir}/cnmsb/
install -m 644 shell/cnmsb.bash %{buildroot}%{_datadir}/cnmsb/

# 安装 profile.d 脚本（登录时自动加载）
mkdir -p %{buildroot}%{_sysconfdir}/profile.d
cat > %{buildroot}%{_sysconfdir}/profile.d/cnmsb.sh << 'EOF'
# cnmsb - Linux 命令行智能补全工具
# 仅支持 Zsh

if [ -n "$ZSH_VERSION" ]; then
    [ -f /usr/share/cnmsb/cnmsb.zsh ] && source /usr/share/cnmsb/cnmsb.zsh
fi
EOF
chmod 644 %{buildroot}%{_sysconfdir}/profile.d/cnmsb.sh

%files
%license LICENSE
%doc README.md
%{_bindir}/cnmsb
%{_bindir}/cnmsb-sql
%{_datadir}/cnmsb/
%config(noreplace) %{_sysconfdir}/profile.d/cnmsb.sh

%post
echo ""
echo "====================================="
echo "  cnmsb 安装成功！"
echo "====================================="
echo ""
echo "使用方法："
echo "  1. 重新登录或执行以下命令立即启用："
echo ""
echo "     source /usr/share/cnmsb/cnmsb.zsh"
echo ""
echo "  2. 或者将以下内容添加到 ~/.zshrc："
echo ""
echo "     source /usr/share/cnmsb/cnmsb.zsh"
echo ""
echo "开始享受智能补全体验吧！"
echo ""

%changelog
* Sun Jan 12 2026 violet <violetqqcom@qq.com> - 0.1.0-1
- 初始版本
- 支持 300+ 常用命令
- 智能内联建议
- 交互式补全选择器
- 问号帮助模式

