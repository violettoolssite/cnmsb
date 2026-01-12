#!/bin/bash
# cnmsb APT 源一键安装脚本
# 使用方法: curl -fsSL https://violettoolssite.github.io/cnmsb/apt/install.sh | sudo bash

set -e

echo "============================================"
echo "  cnmsb - Linux 命令行智能补全工具"
echo "  一键安装脚本"
echo "============================================"
echo ""

# 检查是否为 root
if [ "$EUID" -ne 0 ]; then
    echo "错误: 请使用 sudo 运行此脚本"
    exit 1
fi

# 检查系统
if ! command -v apt &> /dev/null; then
    echo "错误: 此脚本仅支持基于 Debian/Ubuntu 的系统"
    exit 1
fi

echo "[1/4] 添加 cnmsb APT 源..."

# 创建 sources.list.d 文件
cat > /etc/apt/sources.list.d/cnmsb.list << 'EOF'
deb [trusted=yes] https://violettoolssite.github.io/cnmsb/apt stable main
EOF

echo "[2/4] 更新软件包列表..."
apt update

echo "[3/4] 安装 cnmsb..."
apt install -y cnmsb

echo "[4/4] 安装完成!"
echo ""
echo "============================================"
echo "  安装成功!"
echo ""
echo "  重启终端或执行以下命令立即生效:"
echo "    source /etc/profile.d/cnmsb.sh"
echo ""
echo "  使用方法:"
echo "    - Tab: 打开补全菜单 / 确认选择"
echo "    - ↑↓: 切换选项"
echo "    - ?: 查看帮助"
echo "    - Alt+H: 历史命令"
echo "    - →: 接受内联建议"
echo ""
echo "  项目地址: https://github.com/violettoolssite/cnmsb"
echo "============================================"

