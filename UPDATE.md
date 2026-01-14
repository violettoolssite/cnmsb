# 更新 WSL 中的 cnmsb 版本

## 方法 1: 使用更新脚本（推荐）

更新脚本会自动：
- ✅ 安装编译依赖（build-essential, zlib1g-dev 等）
- ✅ 编译项目
- ✅ 安装新版本
- ✅ **自动安装 zsh（如果未安装）**
- ✅ **自动将 zsh 设置为默认 shell**
- ✅ 自动配置 ~/.zshrc

```bash
cd /mnt/c/Users/chen/Desktop/cnmsb/cnmsb-tool
source ~/.cargo/env  # 如果 cargo 不在 PATH 中
./update-wsl.sh
```

**安装完成后，请重新登录或运行 `exec zsh` 切换到 zsh。**

## 方法 2: 手动更新

如果脚本无法运行，可以手动执行以下命令：

```bash
# 1. 进入项目目录
cd /mnt/c/Users/chen/Desktop/cnmsb/cnmsb-tool

# 2. 加载 Rust 环境（如果需要）
source ~/.cargo/env
# 或者
export PATH="$HOME/.cargo/bin:$PATH"

# 3. 编译项目
cargo build --release

# 4. 安装新版本
sudo cp target/release/cnmsb /usr/bin/
sudo chmod +x /usr/bin/cnmsb

# 5. 创建符号链接
sudo ln -sf /usr/bin/cnmsb /usr/bin/cnmsb-sql
sudo ln -sf /usr/bin/cnmsb /usr/bin/cntmd

# 6. 更新 shell 集成脚本
sudo mkdir -p /usr/share/cnmsb
sudo cp shell/cnmsb.zsh /usr/share/cnmsb/
sudo cp shell/cnmsb.bash /usr/share/cnmsb/

# 7. 重新加载 shell 配置
source ~/.zshrc
```

## 方法 3: 一行命令更新

```bash
cd /mnt/c/Users/chen/Desktop/cnmsb/cnmsb-tool && \
source ~/.cargo/env 2>/dev/null || export PATH="$HOME/.cargo/bin:$PATH" && \
cargo build --release && \
sudo cp target/release/cnmsb /usr/bin/ && \
sudo chmod +x /usr/bin/cnmsb && \
sudo ln -sf /usr/bin/cnmsb /usr/bin/cnmsb-sql && \
sudo ln -sf /usr/bin/cnmsb /usr/bin/cntmd && \
echo "更新完成！请运行: source ~/.zshrc"
```

## 验证更新

```bash
# 检查版本
cnmsb --version

# 或者
cnmsb

# 检查符号链接
ls -l /usr/bin/cnmsb*
ls -l /usr/bin/cntmd

# 检查默认 shell（应该是 zsh）
echo $SHELL

# 如果显示的不是 zsh，运行：
exec zsh
```

## 切换到 zsh

如果更新脚本已将 zsh 设置为默认 shell，但当前仍在 bash 中：

```bash
# 方法 1: 立即切换到 zsh
exec zsh

# 方法 2: 重新登录 WSL
# 关闭当前终端，重新打开

# 方法 3: 手动切换
zsh
```

## 安装编译依赖

在首次编译前，需要安装编译工具链：

### Debian/Ubuntu
```bash
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev zlib1g-dev
```

### CentOS/RHEL
```bash
sudo yum groupinstall -y "Development Tools"
sudo yum install -y openssl-devel pkgconfig zlib-devel
```

### Fedora
```bash
sudo dnf groupinstall -y "Development Tools"
sudo dnf install -y openssl-devel pkgconfig zlib-devel
```

### Arch Linux
```bash
sudo pacman -S --noconfirm base-devel openssl pkgconf zlib
```

### openSUSE
```bash
sudo zypper install -y -t pattern devel_basis
sudo zypper install -y libopenssl-devel pkg-config zlib-devel
```

## 故障排除

### 问题: linker `cc` not found

**原因:** 缺少 C 编译器

**解决方案:**
```bash
# Debian/Ubuntu
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev zlib1g-dev

# 然后重新运行更新脚本
./update-wsl.sh
```

### 问题: unable to find library -lz

**原因:** 缺少 zlib 开发库

**解决方案:**
```bash
# Debian/Ubuntu
sudo apt-get install -y zlib1g-dev

# CentOS/RHEL/Fedora
sudo yum install -y zlib-devel
# 或
sudo dnf install -y zlib-devel

# Arch Linux
sudo pacman -S zlib

# 然后重新编译
cargo build --release
```

### 问题: cargo: command not found

**解决方案:**
```bash
# 方法 1: 加载 Rust 环境
source ~/.cargo/env

# 方法 2: 手动添加 PATH
export PATH="$HOME/.cargo/bin:$PATH"

# 方法 3: 检查 Rust 是否安装
which rustc
which cargo

# 如果未安装，安装 Rust:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 问题: 权限不足

**解决方案:**
```bash
# 确保使用 sudo
sudo cp target/release/cnmsb /usr/bin/
```

### 问题: 符号链接已存在

**解决方案:**
```bash
# 强制覆盖
sudo ln -sf /usr/bin/cnmsb /usr/bin/cnmsb-sql
sudo ln -sf /usr/bin/cnmsb /usr/bin/cntmd
```

