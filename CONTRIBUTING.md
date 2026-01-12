# 贡献指南

想给这项目添砖加瓦？行，看看怎么搞。

> **不想看脏话？** [点这里看正常版本](CONTRIBUTING.normal.md)

---

## 项目结构

```
cnmsb-tool/
├── src/
│   ├── main.rs              # 程序入口
│   ├── lib.rs               # 库入口
│   ├── engine.rs            # 补全引擎核心
│   ├── parser.rs            # 命令行解析器
│   ├── shell.rs             # 交互式 Shell
│   ├── completions/         # 补全实现
│   │   ├── mod.rs
│   │   ├── commands.rs      # 命令补全
│   │   ├── args.rs          # 参数补全
│   │   ├── files.rs         # 文件路径补全
│   │   └── history.rs       # 历史命令补全
│   ├── database/            # 命令数据库
│   │   ├── mod.rs           # 数据库加载
│   │   └── commands/        # 命令定义文件
│   └── sql/                 # SQL 补全模块
│       ├── mod.rs
│       ├── connection.rs    # 数据库连接
│       ├── engine.rs        # SQL 补全引擎
│       ├── shell.rs         # SQL 交互 Shell
│       └── syntax/          # SQL 语法定义
├── shell/
│   ├── cnmsb.zsh            # Zsh 集成脚本
│   └── cnmsb.bash           # Bash 集成脚本
├── debian/                  # Debian 打包配置
├── build-deb.sh             # deb 包构建脚本
└── Cargo.toml               # Rust 项目配置
```

## 添加新命令

最简单的贡献方式就是添加命令定义。

命令定义文件在 `src/database/commands/` 目录下，按类别分成了多个 YAML 文件：

```
commands/
├── git.yaml           # Git 版本控制
├── docker.yaml        # Docker 容器
├── kubernetes.yaml    # Kubernetes
├── files.yaml         # 文件操作 (ls, cp, mv, rm, find, chmod, chown...)
├── text.yaml          # 文本处理 (grep, sed, awk, cat, head, tail, sort...)
├── network.yaml       # 网络工具 (curl, wget, ssh, scp, ping, netstat, nmap...)
├── system.yaml        # 系统管理 (systemctl, journalctl, ps, top, kill, df, du...)
├── package.yaml       # 包管理器 (apt, dpkg, snap, pip, npm, cargo...)
├── archive.yaml       # 压缩归档 (tar, zip, gzip, bzip2, xz, 7z...)
├── devtools.yaml      # 开发工具 (make, cmake, gcc, python, node, go, rustc...)
├── cloud.yaml         # 云服务 (aws, gcloud, az, terraform, ansible...)
├── database.yaml      # 数据库客户端 (mysql, psql, sqlite3, redis-cli, mongo...)
├── editors.yaml       # 编辑器 (vim, vi, nano, emacs...)
├── shell.yaml         # Shell 工具 (echo, printf, xargs, watch, screen, tmux...)
├── hardware.yaml      # 硬件信息 (lscpu, lsmem, lspci, lsusb, lsblk...)
├── security.yaml      # 安全工具 (sudo, passwd, ssh-keygen, gpg, openssl, ufw...)
├── info.yaml          # 系统信息 (uname, hostname, uptime, who, w...)
├── kernel.yaml        # 内核工具 (dmesg, sysctl, modprobe, lsmod...)
├── multimedia.yaml    # 多媒体 (ffmpeg, ffprobe, convert, sox, mpv, yt-dlp...)
├── virtualization.yaml # 虚拟化 (qemu, virsh, vagrant, VBoxManage, multipass...)
├── monitoring.yaml    # 监控工具 (sar, iostat, vmstat, dstat, glances, iftop...)
├── messaging.yaml     # 消息队列 (kafka, rabbitmq, mosquitto, nats...)
├── backup.yaml        # 备份工具 (borg, restic, rclone, duplicity, dd...)
└── cnmsb.yaml         # cnmsb 自身命令
```

### YAML 格式

```yaml
command_name:
  name: command_name
  description: 命令简介
  options:
    - short: "-o"
      long: "--option"
      description: 选项说明
      takes_value: true          # 是否需要参数值
      values: ["val1", "val2"]   # 可选：预定义的可选值
  subcommands:
    subcommand_name:
      name: subcommand_name
      description: 子命令说明
      options:
        - short: "-x"
          long: "--example"
          description: 子命令选项
```

### 示例：添加简单命令

```yaml
htop:
  name: htop
  description: 交互式进程查看器
  options:
    - short: "-d"
      long: "--delay"
      description: 更新延迟秒数
      takes_value: true
    - short: "-s"
      long: "--sort-key"
      description: 排序列
      takes_value: true
      values: ["PID", "USER", "CPU", "MEM", "TIME", "COMMAND"]
    - short: "-u"
      long: "--user"
      description: 只显示指定用户进程
      takes_value: true
    - short: "-t"
      long: "--tree"
      description: 树状显示
    - short: "-h"
      long: "--help"
      description: 显示帮助
  subcommands: {}
```

### 示例：添加带子命令的命令

```yaml
mycli:
  name: mycli
  description: 示例 CLI 工具
  options:
    - short: "-v"
      long: "--verbose"
      description: 详细输出
    - short: "-c"
      long: "--config"
      description: 配置文件
      takes_value: true
  subcommands:
    init:
      name: init
      description: 初始化项目
      options:
        - short: "-f"
          long: "--force"
          description: 强制初始化
        - short: "-t"
          long: "--template"
          description: 模板名称
          takes_value: true
          values: ["default", "minimal", "full"]
    build:
      name: build
      description: 构建项目
      options:
        - short: "-r"
          long: "--release"
          description: 发布构建
        - short: "-j"
          long: "--jobs"
          description: 并行任务数
          takes_value: true
```

### 添加新分类

如果你要加的命令不属于现有分类：

1. 在 `commands/` 目录下新建一个 `.yaml` 文件
2. 在 `src/database/mod.rs` 的 `load_all_commands` 函数中添加 `include_str!`

```rust
let files = [
    // ... 现有文件
    include_str!("commands/your_new_file.yaml"),
];
```

## 修改核心代码

### 补全引擎

补全逻辑在 `src/engine.rs`，主要函数：

- `complete()` - 主补全入口
- `filter_and_rank()` - 过滤和排序补全结果
- `get_completions_for_*()` - 各类补全获取

### 命令解析

解析逻辑在 `src/parser.rs`：

- `parse()` - 解析命令行输入
- `has_subcommands()` - 判断命令是否有子命令（从 YAML 动态加载）

### Zsh 集成

Zsh 脚本在 `shell/cnmsb.zsh`，主要功能：

- `_cnmsb_predict` - 内联预测
- `_cnmsb_complete` - Tab 补全
- `_cnmsb_show_selector` - 选择器菜单
- `_cnmsb_history_mode` - 历史命令模式

### 构建和测试

```bash
# 编译
cargo build --release

# 测试补全输出
./target/release/cnmsb complete --line "git " --cursor 4 --shell zsh

# 测试帮助模式
./target/release/cnmsb complete --line "tar ?" --cursor 5 --shell zsh

# 测试交互模式
./target/release/cnmsb shell

# 测试 SQL 模式
./target/release/cnmsb sql
```

### 构建 deb 包

```bash
./build-deb.sh
```

生成 `cnmsb_0.1.0_amd64.deb`

## 提交 PR

1. Fork 这个仓库
2. 创建你的分支: `git checkout -b feature/xxx`
3. 提交改动: `git commit -m '添加了xxx'`
4. 推送: `git push origin feature/xxx`
5. 提交 Pull Request

### 提交信息格式

说清楚你干了啥就行：

```
feat: 添加 htop 命令定义
fix: 修复 tar 参数补全问题
perf: 优化模糊匹配性能
docs: 更新 README
```

### PR 检查清单

- [ ] YAML 格式正确，能正常解析
- [ ] `cargo build --release` 编译通过
- [ ] 新命令的选项和子命令完整
- [ ] 描述清晰准确

## 问题反馈

有 bug 或者建议直接开 issue，描述清楚：

1. 你用的什么系统（Ubuntu 22.04、Debian 12 等）
2. 什么 shell（zsh 版本）
3. 怎么复现问题（具体输入什么命令）
4. 期望的行为是什么
5. 实际发生了什么

别就扔一句"不好使"就完了，那我也帮不了你。

## 代码规范

- Rust 代码用 `cargo fmt` 格式化
- Rust 代码用 `cargo clippy` 检查
- Shell 脚本用 2 空格缩进
- YAML 文件用 2 空格缩进
- 描述用中文

## 许可证

贡献的代码会采用 MIT 协议，提交即表示同意。
