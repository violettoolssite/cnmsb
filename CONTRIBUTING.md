# 贡献指南

想给这项目添砖加瓦？行，看看怎么搞。别他妈瞎改，改坏了老子不负责。

> **不想看脏话？** [点这里看正常版本](CONTRIBUTING.normal.md)

---

## 项目结构

```
cnmsb-tool/
├── src/
│   ├── main.rs              # 程序入口
│   ├── lib.rs               # 库入口
│   ├── engine.rs            # 补全引擎核心（模糊匹配、排序）
│   ├── parser.rs            # 命令行解析器（支持前缀命令）
│   ├── shell.rs             # 交互式 Shell（已废弃，仅保留接口）
│   ├── completions/         # 补全实现
│   │   ├── mod.rs
│   │   ├── commands.rs      # 命令补全
│   │   ├── args.rs          # 参数补全（支持组合参数）
│   │   ├── files.rs         # 文件路径补全
│   │   ├── history.rs       # 历史命令补全
│   │   └── context.rs       # 上下文感知补全（环境变量、路径查找）
│   ├── database/            # 命令数据库
│   │   ├── mod.rs           # 数据库加载
│   │   └── commands/        # 命令定义文件（YAML 格式）
│   ├── sql/                 # SQL 补全模块
│   │   ├── mod.rs
│   │   ├── connection.rs    # 数据库连接（SQLite/MySQL/PostgreSQL）
│   │   ├── database.rs      # 数据库类型和配置
│   │   ├── engine.rs        # SQL 补全引擎
│   │   ├── shell.rs         # SQL 交互 Shell（使用 rustyline）
│   │   └── syntax/          # SQL 语法定义
│   │       ├── mod.rs
│   │       ├── common.rs    # 通用 SQL 语法
│   │       ├── mysql.rs     # MySQL 语法
│   │       ├── postgresql.rs # PostgreSQL 语法
│   │       └── sqlite.rs    # SQLite 语法
│   ├── editor/              # 文本编辑器模块（cntmd）
│   │   ├── mod.rs           # 编辑器主逻辑
│   │   ├── buffer.rs        # 文本缓冲区
│   │   ├── cursor.rs        # 光标控制
│   │   ├── mode.rs          # 编辑模式（Normal/Insert/Command）
│   │   ├── render.rs        # 渲染器
│   │   ├── input.rs         # 输入处理
│   │   ├── history.rs       # 历史管理
│   │   ├── completion.rs    # 基于历史的补全（Trie 结构）
│   │   ├── context.rs       # 上下文感知补全（环境变量、PATH 建议）
│   │   └── nlp.rs           # 自然语言理解和路径查找
│   └── ai/                  # AI 智能补全模块
│       ├── mod.rs           # 模块入口
│       ├── config.rs        # AI 配置管理（~/.config/cnmsb/ai.conf）
│       └── completer.rs     # AI 补全器（调用大语言模型 API）
├── shell/
│   ├── cnmsb.zsh            # Zsh 集成脚本（内联补全、选择器菜单）
│   └── cnmsb.bash           # Bash 集成脚本（已废弃，仅 Zsh 支持）
├── debian/                  # Debian 打包配置
├── build-deb.sh             # deb 包构建脚本
├── install-universal.sh     # 通用安装脚本
└── Cargo.toml               # Rust 项目配置
```

## 添加新命令

最简单的贡献方式就是添加命令定义。操，这都不会就别他妈贡献了。

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
  combinable_options:            # 可选：预定义的组合参数
    - "-abc"
    - "-xyz"
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

### 组合参数

很多命令支持短选项组合使用（如 `rm -rf`、`ls -la`、`tar -xzvf`）。

为了提供更好的补全体验，可以在命令定义中添加 `combinable_options` 字段，列出常用的参数组合：

```yaml
rm:
  name: rm
  description: 删除文件或目录
  combinable_options:
    - "-rf"      # 强制递归删除（最常用）
    - "-rv"      # 递归删除并显示详情
    - "-rfv"     # 强制递归删除并显示详情
    - "-ri"      # 递归删除，每个文件确认
    - "-rI"      # 递归删除，删除多个前确认
  options:
    - short: "-r"
      long: "--recursive"
      description: 递归删除目录
    # ...
```

#### 已支持组合参数的命令

| 命令 | 常用组合 | 说明 |
|------|----------|------|
| `rm` | `-rf`, `-rv`, `-rfv` | 强制递归删除 |
| `ls` | `-la`, `-lah`, `-ltr`, `-latr` | 列表显示 |
| `cp` | `-rv`, `-rpv`, `-a`, `-av` | 复制保留属性 |
| `mv` | `-vf`, `-vi` | 移动文件 |
| `chmod` | `-Rv`, `-Rc` | 递归修改权限 |
| `chown` | `-Rv`, `-Rc` | 递归修改所有者 |
| `mkdir` | `-p`, `-pv` | 创建父目录 |
| `grep` | `-rn`, `-rni`, `-rnw`, `-rE` | 递归搜索 |
| `ps` | `aux`, `auxf`, `-ef` | 进程列表 |
| `df` | `-h`, `-hT` | 磁盘使用 |
| `du` | `-sh`, `-shc` | 目录大小 |
| `tar` | `-xvf`, `-xzvf`, `-czvf` | 压缩解压 |
| `pacman` | `-Syu`, `-Syyu`, `-Rs`, `-Rns` | Arch 包管理 |
| `rsync` | `-av`, `-avz`, `-avzP` | 同步文件 |
| `curl` | `-sSL`, `-fsSL`, `-LO` | HTTP 请求 |
| `scp` | `-rv`, `-rpv`, `-rC` | 安全复制 |

#### 添加组合参数的原则

1. **只添加常用组合**：不要把所有可能的组合都加进去，只加实际工作中常用的。别他妈加一堆没用的
2. **保持简洁**：一般 5-10 个组合就够了。加多了也没人用，浪费空间
3. **最常用的放前面**：分数相同时，靠前的会优先显示。把最常用的放前面，别他妈乱排

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

- `parse()` - 解析命令行输入，支持前缀命令（sudo, time, env 等）
- `has_subcommands()` - 判断命令是否有子命令（从 YAML 动态加载）
- 自动识别前缀命令，跳过它们以正确补全实际命令

### Zsh 集成

Zsh 脚本在 `shell/cnmsb.zsh`，主要功能：

- `_cnmsb_fetch` - 获取补全建议
- `_cnmsb_tab` - Tab 键处理（第一次显示菜单，第二次确认选择）
- `_cnmsb_show_menu` - 显示选择器菜单
- `_cnmsb_show_inline` - 显示内联灰色建议
- `_cnmsb_history_menu` - 历史命令模式（Alt+H）
- `_cnmsb_ai_complete` - AI 智能补全（Alt+L）
- `_cnmsb_show_ai_menu` - 显示 AI 补全菜单（带 [AI 智能补全] 标题）
- `_cnmsb_question` - 帮助模式（? 键）
- 禁用 `sudo` 等前缀命令的默认补全，确保使用我们的补全系统

### SQL Shell

SQL 交互式 Shell 在 `src/sql/shell.rs`：

- 使用 `rustyline` 库提供稳定的行编辑体验
- `SqlHelper` - 实现 `Completer`、`Hinter`、`Highlighter` traits
- 支持 SQLite、MySQL、PostgreSQL 数据库连接
- 自动加载 Schema 信息用于表名和列名补全
- 支持特殊命令（`.help`, `.tables`, `.desc`, `.schema`, `.clear`）

### 文本编辑器

文本编辑器在 `src/editor/` 目录：

- `Editor` - 主编辑器结构
- `Buffer` - 文本缓冲区管理
- `Cursor` - 光标位置控制
- `Mode` - 编辑模式（Normal/Insert/Command）
- `Renderer` - 终端渲染（使用 crossterm）
- `Completer` - 基于历史的补全（Trie 数据结构）
- 支持文件头自动插入、欢迎屏幕、历史持久化

### AI 补全模块

AI 补全模块在 `src/ai/` 目录：

- `config.rs` - 配置管理（加载/保存 `~/.config/cnmsb/ai.conf`）
- `completer.rs` - AI 补全器（构建 prompt、调用 API、解析响应）

**添加新的 AI 提供商：**

1. 修改 `config.rs` 添加新的配置项
2. 修改 `completer.rs` 的 `complete()` 方法支持新的 API 格式
3. 测试：`cnmsb ai-complete --line "git co" --cursor 6`

### 构建和测试

```bash
# 编译
cargo build --release

# 测试命令补全
./target/release/cnmsb complete --line "git " --cursor 4 --shell zsh

# 测试子命令补全
./target/release/cnmsb complete --line "apt ins" --cursor 6 --shell zsh

# 测试前缀命令补全
./target/release/cnmsb complete --line "sudo ap" --cursor 7 --shell zsh

# 测试模糊匹配
./target/release/cnmsb complete --line "ar" --cursor 2 --shell zsh

# 测试帮助模式
./target/release/cnmsb complete --line "tar ?" --cursor 5 --shell zsh

# 测试组合参数补全
./target/release/cnmsb complete --line "tar -z" --cursor 6 --shell zsh

# 测试 AI 补全
./target/release/cnmsb ai-complete --line "git co" --cursor 6

# 测试 AI 配置
./target/release/cnmsb ai-config show

# 测试 SQL Shell
./target/release/cnmsb sql

# 测试编辑器
./target/release/cnmsb edit test.txt
# 或
./target/release/cntmd test.txt
```

### 构建 deb 包

```bash
./build-deb.sh
```

生成 `cnmsb_0.1.0_amd64.deb`

## 提交 PR

1. Fork 这个仓库（不会？百度去）
2. 创建你的分支: `git checkout -b feature/xxx`（别他妈用 master）
3. 提交改动: `git commit -m '添加了xxx'`（写清楚点，别他妈写"更新"）
4. 推送: `git push origin feature/xxx`（不会推送？别他妈贡献了）
5. 提交 Pull Request（写清楚改了什么，别他妈就写"修复bug"）

### 提交信息格式

说清楚你干了啥就行，别他妈写"更新"、"修复"这种废话：

```
feat: 添加 htop 命令定义
fix: 修复 tar 参数补全问题
perf: 优化模糊匹配性能
docs: 更新 README
```

写不清楚的，老子直接给你拒了。

### PR 检查清单

- [ ] YAML 格式正确，能正常解析
- [ ] `cargo build --release` 编译通过
- [ ] 新命令的选项和子命令完整
- [ ] 描述清晰准确

## 问题反馈

有 bug 或者建议直接开 issue，描述清楚。别他妈就写"不好使"，老子不是算命的：

1. 你用的什么系统（Ubuntu 22.04、Debian 12 等），别他妈写"Linux"
2. 什么 shell（zsh 版本），别他妈写"终端"
3. 怎么复现问题（具体输入什么命令），别他妈写"用不了"
4. 期望的行为是什么，别他妈写"应该能用"
5. 实际发生了什么，别他妈写"报错了"

写不清楚的，老子直接给你关了。别他妈抱怨。

## 核心功能说明

### 模糊匹配

补全引擎支持多种匹配方式（按优先级排序）：

1. **精确匹配**（大小写不敏感）- 分数 300
2. **前缀匹配**（大小写不敏感）- 分数 200+
3. **包含匹配**（大小写不敏感）- 分数 150+
4. **缩写匹配**（如 `ar` -> `tar`）- 分数 100+
5. **模糊匹配**（使用 `fuzzy-matcher`）- 分数 50+
6. **子序列匹配**（如 `ar` -> `tar`）- 分数 30+

### 前缀命令支持

系统自动识别以下前缀命令，并正确补全后面的实际命令：

- `sudo` - 以管理员权限执行
- `time` - 测量执行时间
- `env` - 设置环境变量执行
- `nice` - 调整进程优先级
- `nohup` - 后台运行
- `strace` - 系统调用跟踪
- `gdb` - 调试器
- `valgrind` - 内存检查

### 组合参数补全

支持短选项组合补全，例如：

- `tar -z` -> 建议 `-zx`, `-zv`, `-zf` 等
- `rm -r` -> 建议 `-rf`, `-rv`, `-ri` 等
- `ls -l` -> 建议 `-la`, `-lah`, `-ltr` 等

### SQL 补全特性

- **上下文感知**：根据 SQL 上下文（SELECT、FROM、WHERE 等）提供相应补全
- **Schema 感知**：自动加载数据库表名和列名
- **别名解析**：支持 `SELECT u.id FROM users u` 中的别名补全
- **大小写保持**：根据用户输入的大小写风格调整补全
- **表.列格式**：支持 `table.column` 格式的补全

### 编辑器特性

- **历史补全**：基于编辑历史和预加载常用词的 Trie 补全
- **上下文感知补全**：自动分析文件内容，识别环境变量定义，提供智能建议
- **自然语言理解**：理解用户意图（如 `export JAVA_HOME=`），自动查找系统路径
- **PATH 智能建议**：基于已定义的 `*_HOME` 变量，自动生成 PATH 配置建议
- **变量引用补全**：输入 `$VAR` 时自动匹配已定义的变量（不区分大小写）
- **模式切换**：Normal/Insert/Command 三种模式
- **文件头自动插入**：根据文件扩展名自动添加合适的文件头
- **欢迎屏幕**：新文件时显示帮助信息
- **历史持久化**：保存编辑历史用于后续补全
- **右方向键接受**：按右方向键（->）接受上下文补全建议

### 上下文感知补全

#### 编辑器中的上下文补全

编辑器会自动分析文件内容，提取环境变量定义，并提供智能补全：

- **环境变量识别**：识别 `export VAR=value` 和 `VAR=value` 格式
- **路径自动查找**：输入 `export JAVA_HOME=` 时，自动在系统中查找 Java 安装路径
- **PATH 智能建议**：输入 `export PATH=` 时，基于已定义的 `*_HOME` 变量生成建议
- **变量引用补全**：输入 `$VAR` 时，自动匹配已定义的变量

#### 命令行中的上下文补全

命令行补全也支持上下文感知：

- **环境变量补全**：在 `export` 命令中提供环境变量名和值的补全
- **路径查找**：自动查找 Java、Hadoop、Maven 等工具的安装路径
- **PATH 建议**：智能生成 PATH 配置建议

#### 路径查找功能

系统会自动在以下位置查找安装路径：

- **Java**：`/usr/lib/jvm`、`/opt/jdk`、`/opt/java` 等
- **Hadoop**：`/opt/hadoop`、`/usr/local/hadoop` 等
- **Maven**：`/opt/maven`、`/opt/apache-maven` 等
- **Python/Node.js**：使用 `which` 命令查找
- **通用查找**：在 `/opt`、`/usr/local` 中搜索

#### 自然语言理解

系统能够理解以下意图：

- **设置环境变量**：识别 `export VAR=` 的意图，根据变量类型查找路径
- **配置 PATH**：识别 `export PATH=` 的意图，生成智能建议
- **查找路径**：从文本中提取关键词，查找相关路径

## 代码规范

- Rust 代码用 `cargo fmt` 格式化
- Rust 代码用 `cargo clippy` 检查
- Shell 脚本用 2 空格缩进
- YAML 文件用 2 空格缩进
- 描述用中文
- 提交前确保 `cargo build --release` 编译通过

## 常见问题

### Q: 如何添加新的前缀命令？

A: 在 `src/parser.rs` 的 `prefix_commands` 数组中添加，同时在 `shell/cnmsb.zsh` 中使用 `compdef -d` 禁用默认补全。

### Q: 如何添加新的 SQL 数据库类型？

A: 在 `src/sql/database.rs` 的 `DatabaseType` 枚举中添加，实现对应的语法文件（`src/sql/syntax/`），并在 `src/sql/connection.rs` 中实现连接逻辑。

### Q: 组合参数补全不工作？

A: 检查 YAML 文件中是否定义了 `combinable_options` 字段，并且选项的 `short` 字段格式正确（如 `"-r"` 而不是 `"r"`）。

### Q: 子命令补全显示文件而不是子命令？

A: 检查 `src/engine.rs` 中的文件补全逻辑，确保在有子命令补全时跳过文件补全。

### Q: AI 补全不工作？

A: 检查以下几点：
1. API 密钥是否正确配置：`cnmsb ai-config show`
2. 网络是否能访问 API 地址：`curl https://api-inference.modelscope.cn/v1/`
3. 是否启用了 AI 补全：`cnmsb ai-config get enabled`

### Q: 如何添加新的 AI 提供商？

A: 修改 `src/ai/completer.rs`：
1. 添加新的 API 请求格式
2. 实现响应解析逻辑
3. 在 `config.rs` 中添加相关配置项

### Q: AI 补全响应太慢？

A: 这取决于 API 服务的响应速度。可以尝试：
1. 使用本地部署的模型
2. 选择响应更快的 API 服务
3. 修改 `completer.rs` 中的超时设置

## 许可证

贡献的代码会采用 MIT 协议，提交即表示同意。
