# 发布 cnmsb 到公共 APT 源

## 方案一：Launchpad PPA（Ubuntu 官方）

### 前提条件
- Launchpad 账户：https://launchpad.net/
- GPG 密钥
- Ubuntu/Debian 构建环境

### 步骤

1. **生成 GPG 密钥**
```bash
gpg --full-generate-key
# 选择 RSA，4096 位
```

2. **上传密钥到 Ubuntu 服务器**
```bash
gpg --keyserver keyserver.ubuntu.com --send-keys YOUR_KEY_ID
```

3. **在 Launchpad 创建 PPA**
   - 访问 https://launchpad.net/~/+activate-ppa
   - 创建 PPA 名称，如 `cnmsb`

4. **构建源码包**
```bash
cd cnmsb-tool
debuild -S -sa -k YOUR_KEY_ID
```

5. **上传到 PPA**
```bash
dput ppa:violettoolssite/cnmsb ../cnmsb_0.1.0_source.changes
```

6. **用户安装**
```bash
sudo add-apt-repository ppa:violettoolssite/cnmsb
sudo apt update
sudo apt install cnmsb
```

---

## 方案二：PackageCloud（推荐，简单）

### 1. 注册 PackageCloud
访问 https://packagecloud.io/ 注册免费账户

### 2. 创建仓库
在 Dashboard 创建新仓库，如 `cnmsb`

### 3. 上传 deb 包
```bash
# 安装 packagecloud CLI
gem install package_cloud

# 登录
package_cloud login

# 上传（支持多个 Ubuntu/Debian 版本）
package_cloud push violettoolssite/cnmsb/ubuntu/jammy cnmsb_0.1.0_amd64.deb
package_cloud push violettoolssite/cnmsb/ubuntu/focal cnmsb_0.1.0_amd64.deb
package_cloud push violettoolssite/cnmsb/debian/bookworm cnmsb_0.1.0_amd64.deb
```

### 4. 用户安装
```bash
# 一键添加源
curl -s https://packagecloud.io/install/repositories/violettoolssite/cnmsb/script.deb.sh | sudo bash

# 安装
sudo apt install cnmsb
```

---

## 方案三：Gemfury（企业级）

```bash
# 安装 CLI
pip install gemfury

# 上传
fury push cnmsb_0.1.0_amd64.deb --as=violettoolssite
```

---

## 方案四：自建 APT 源（当前方案）

当前已通过 GitHub Pages 部署：

```bash
# 添加源
echo "deb [trusted=yes] https://violettoolssite.github.io/cnmsb/apt stable main" | sudo tee /etc/apt/sources.list.d/cnmsb.list

# 安装
sudo apt update && sudo apt install cnmsb
```

---

## 推荐

| 方案 | 优点 | 缺点 |
|------|------|------|
| Launchpad PPA | 官方支持，自动构建 | 配置复杂，仅限 Ubuntu |
| PackageCloud | 简单，支持多发行版 | 免费版有限制 |
| GitHub Pages | 完全免费 | 需要手动添加源 |

**建议**：先用 GitHub Pages 方案，积累用户后迁移到 PackageCloud 或 PPA。

