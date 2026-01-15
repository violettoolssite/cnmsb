# 快速更新命令

## 解决 WSL 权限问题

如果在 WSL 中编译时遇到权限问题（特别是在 `/mnt/c/` 路径下），可以使用以下方法：

### 方法 1: 清理并重新编译（推荐）

```bash
cd cnmsb-tool

# 清理编译缓存
cargo clean

# 使用用户目录编译（避免权限问题）
export CARGO_TARGET_DIR="$HOME/.cargo/cnmsb-build"
mkdir -p "$CARGO_TARGET_DIR"

# 编译
cargo build --release --target-dir "$CARGO_TARGET_DIR"

# 安装
sudo cp "$CARGO_TARGET_DIR/release/cnmsb" /usr/local/bin/cnmsb
sudo chmod +x /usr/local/bin/cnmsb
```

### 方法 2: 将项目移到 Linux 文件系统

```bash
# 将项目复制到 Linux 文件系统
cp -r /mnt/c/Users/chen/Desktop/cnmsb ~/cnmsb
cd ~/cnmsb/cnmsb-tool

# 正常编译
cargo build --release

# 安装
sudo cp target/release/cnmsb /usr/local/bin/cnmsb
sudo chmod +x /usr/local/bin/cnmsb
```

### 方法 3: 修复 target 目录权限

```bash
cd cnmsb-tool

# 删除 target 目录
rm -rf target

# 重新编译
cargo build --release
```

### 方法 4: 使用更新脚本（已修复）

```bash
cd cnmsb-tool
./update.sh
```

脚本会自动检测 Windows 文件系统并使用用户目录编译。

## 常见问题

### Q: 为什么会有权限问题？

A: Windows 文件系统（`/mnt/c/`）和 Linux 文件系统的权限模型不同，某些操作（如创建硬链接、设置权限）在 Windows 文件系统上可能失败。

### Q: 如何避免这个问题？

A: 最佳实践是将 Rust 项目放在 Linux 文件系统中（如 `~/projects/`），而不是 Windows 文件系统（`/mnt/c/`）。

### Q: 编译成功后如何安装？

A: 
```bash
# 如果使用默认 target 目录
sudo cp target/release/cnmsb /usr/local/bin/cnmsb

# 如果使用自定义 target 目录
sudo cp ~/.cargo/cnmsb-build/release/cnmsb /usr/local/bin/cnmsb

# 创建符号链接
sudo ln -sf /usr/local/bin/cnmsb /usr/local/bin/cnmsb-sql
sudo ln -sf /usr/local/bin/cnmsb /usr/local/bin/cntmd
```

