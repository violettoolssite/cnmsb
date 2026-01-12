#!/bin/bash
# 发布 cnmsb 到 PackageCloud
# 使用方法: ./scripts/publish-packagecloud.sh

set -e

VERSION="0.1.0"
PACKAGE="cnmsb_${VERSION}_amd64.deb"
REPO="violettoolssite/cnmsb"

echo "=========================================="
echo "  发布 cnmsb 到 PackageCloud"
echo "=========================================="

# 检查 deb 包是否存在
if [ ! -f "$PACKAGE" ]; then
    echo "错误: 找不到 $PACKAGE"
    echo "请先构建 deb 包: ./build-deb.sh"
    exit 1
fi

# 检查 package_cloud CLI
if ! command -v package_cloud &> /dev/null; then
    echo "安装 packagecloud CLI..."
    gem install package_cloud
fi

echo ""
echo "上传到以下发行版："
echo "  - Ubuntu 24.04 (noble)"
echo "  - Ubuntu 22.04 (jammy)"
echo "  - Ubuntu 20.04 (focal)"
echo "  - Debian 12 (bookworm)"
echo "  - Debian 11 (bullseye)"
echo ""

# 上传到各个发行版
package_cloud push $REPO/ubuntu/noble $PACKAGE
package_cloud push $REPO/ubuntu/jammy $PACKAGE
package_cloud push $REPO/ubuntu/focal $PACKAGE
package_cloud push $REPO/debian/bookworm $PACKAGE
package_cloud push $REPO/debian/bullseye $PACKAGE

echo ""
echo "=========================================="
echo "  发布完成!"
echo ""
echo "  用户安装方式:"
echo "  curl -s https://packagecloud.io/install/repositories/$REPO/script.deb.sh | sudo bash"
echo "  sudo apt install cnmsb"
echo "=========================================="

