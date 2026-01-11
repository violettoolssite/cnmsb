#!/bin/bash
# cnmsb 快速安装脚本

set -e

echo "======================================="
echo "  cnmsb (操你妈傻逼) 安装脚本"
echo "======================================="
echo

# 检查是否以 root 运行
if [[ $EUID -ne 0 ]]; then
   SUDO="sudo"
   echo "需要 sudo 权限来安装系统文件"
   echo
else
   SUDO=""
fi

# 检查是否已构建
if [[ ! -f "target/release/cnmsb" ]]; then
    echo "未找到构建文件，正在构建..."
    if ! command -v cargo &> /dev/null; then
        echo "错误: 未找到 cargo，请先安装 Rust"
        exit 1
    fi
    cargo build --release
fi

echo "正在安装 cnmsb..."
echo

# 安装二进制文件
echo "安装二进制文件到 /usr/local/bin/cnmsb"
$SUDO install -m 755 target/release/cnmsb /usr/local/bin/cnmsb

# 安装 shell 脚本
echo "安装 shell 脚本到 /usr/share/cnmsb/"
$SUDO mkdir -p /usr/share/cnmsb
$SUDO install -m 644 shell/cnmsb.bash /usr/share/cnmsb/
$SUDO install -m 644 shell/cnmsb.zsh /usr/share/cnmsb/

# 安装 profile.d 脚本（可选）
if [[ -d /etc/profile.d ]]; then
    echo "安装自动加载脚本到 /etc/profile.d/"
    $SUDO install -m 644 debian/cnmsb.sh /etc/profile.d/
fi

echo
echo "======================================="
echo "  安装完成！"
echo "======================================="
echo
echo "cnmsb 已安装到 /usr/local/bin/cnmsb"
echo
echo "验证安装:"
echo "  cnmsb version"
echo
echo "启用补全（选择一种方式）:"
echo
echo "方式 1 - 重新登录（推荐）"
echo "  系统会自动加载 cnmsb"
echo
echo "方式 2 - 手动加载"
echo "  Bash: source /usr/share/cnmsb/cnmsb.bash"
echo "  Zsh:  source /usr/share/cnmsb/cnmsb.zsh"
echo
echo "方式 3 - 添加到配置文件"
echo "  Bash: echo 'eval \"\$(cnmsb init bash)\"' >> ~/.bashrc"
echo "  Zsh:  echo 'eval \"\$(cnmsb init zsh)\"' >> ~/.zshrc"
echo

