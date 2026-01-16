<div align="center">

# cnmsb - 操你妈傻逼

**Linux 命令行智能补全工具，提供类似 IDE 的补全体验**

<p>
  <a href="https://github.com/violettoolssite/cnmsb/stargazers">
    <img src="https://img.shields.io/github/stars/violettoolssite/cnmsb?style=for-the-badge&logo=github&logoColor=white&labelColor=1a1a2e&color=d4ff00" alt="Stars">
  </a>
  <a href="https://github.com/violettoolssite/cnmsb/network/members">
    <img src="https://img.shields.io/github/forks/violettoolssite/cnmsb?style=for-the-badge&logo=github&logoColor=white&labelColor=1a1a2e&color=00d9ff" alt="Forks">
  </a>
  <a href="https://github.com/violettoolssite/cnmsb/issues">
    <img src="https://img.shields.io/github/issues/violettoolssite/cnmsb?style=for-the-badge&logo=github&logoColor=white&labelColor=1a1a2e&color=ff6b6b" alt="Issues">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge&labelColor=1a1a2e&color=a855f7" alt="License">
  </a>
</p>

<p>
  <a href="https://github.com/violettoolssite/cnmsb/releases">
    <img src="https://img.shields.io/github/v/release/violettoolssite/cnmsb?style=for-the-badge&logo=semantic-release&logoColor=white&labelColor=1a1a2e&color=22c55e" alt="Release">
  </a>
  <a href="https://github.com/violettoolssite/cnmsb">
    <img src="https://img.shields.io/github/languages/top/violettoolssite/cnmsb?style=for-the-badge&logo=rust&logoColor=white&labelColor=1a1a2e&color=dea584" alt="Language">
  </a>
  <a href="https://github.com/violettoolssite/cnmsb/commits/main">
    <img src="https://img.shields.io/github/last-commit/violettoolssite/cnmsb?style=for-the-badge&logo=git&logoColor=white&labelColor=1a1a2e&color=f97316" alt="Last Commit">
  </a>
</p>

<br>

### ⭐ Star 增长趋势

<a href="https://star-history.com/#violettoolssite/cnmsb&Date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=violettoolssite/cnmsb&type=Date&theme=dark" />
    <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=violettoolssite/cnmsb&type=Date" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=violettoolssite/cnmsb&type=Date" width="600" />
  </picture>
</a>

<br>

</div>

---

<div align="center">

### **300+ 命令支持 | 智能内联补全 | 交互式选择器 | 历史命令模式 | 内置编辑器 | SQL Shell | AI 级别上下文感知**

**史上最暴躁的命令行补全工具，用脏话命名，用实力说话**

**操你妈的，Linux 命令行补全工具，让你敲命令不用再他妈的翻那些狗屁文档。每次都要查参数烦不烦？装了这个傻逼工具，直接给你补全，省得你他妈到处找。**

> **为什么叫 cnmsb？** 因为作者写代码的时候经常骂这个，干脆就叫这个了。  
> **不想看脏话？** [点这里看正常版本](README.normal.md)  
> **还在犹豫？** 看看下面那些被参数折磨的可怜人，你就知道为什么需要这个工具了。

</div>

---

## 使用演示

![使用教程](demo.gif)

---

## 这鸡巴玩意是干嘛的

你他妈敲命令的时候是不是老忘记参数？`tar` 到底是 `-xvf` 还是 `-zxvf`？`docker` 那一坨狗屎命令谁他妈记得住？每次都要去百度，烦不烦？操，老子也烦。

**cnmsb 就是解决这个傻逼问题的。** 装上这玩意之后，你敲命令它就会像 IDE 一样在后面给你灰色提示，按 Tab 就补全了，省得你去他妈的查来查去。不用再他妈翻 man 手册，不用再百度，直接补全，爽不爽？

### 为什么这个工具这么特别？

- **用脏话命名**：作者写代码的时候经常骂这个，干脆就叫这个了。够直接，够真实。
- **AI 级别智能**：不是简单的关键词匹配，是真的理解你的意图。输入 `export JAVA_HOME=` 它就知道你要找 Java 路径。
- **学习你的习惯**：用久了它会记住你常用的命令组合，越用越顺手。
- **内置编辑器**：自带 `cntmd`（操你他妈的编辑器），不用再装那些垃圾插件。
- **300+ 命令支持**：基本上 Linux 能用的都有，不够用？自己加。

### 对比一下

| 功能 | 其他补全工具 | cnmsb |
|------|------------|-------|
| 智能程度 | 关键词匹配 | **AI 级别理解** |
| 上下文感知 | 不支持 | **自动查找路径** |
| 学习能力 | 不支持 | **记住你的习惯** |
| 内置编辑器 | 不支持 | **cntmd 编辑器** |
| 命名风格 | 正经 | **暴躁但真实** |
| 使用体验 | 一般 | **爽到飞起** |

## 功能列表（都他妈很实用，而且比那些垃圾工具强多了）

> **提示**：这些功能不是吹的，是真的好用。不信？装一个试试，保证你用了就回不去了。

### 智能内联建议（最实用的功能）

敲命令的时候直接显示灰色预测文字，不用按什么狗屁快捷键，它自己就冒出来了。按 Tab 或者右箭头就能接受建议。妈的，比那些垃圾补全工具强多了。

**为什么这个功能这么牛逼？**
- 不需要手动触发，边输入边看建议
- 像 IDE 一样智能，但比 IDE 更快
- 支持模糊匹配，打错了也能找到
- 实时更新，你删一个字它也跟着变

### 补全选择器
按一次 Tab 打开选择菜单，用上下箭头选你要的，再按 Tab 确认。不想要了按 Esc 滚蛋。操，简单粗暴，比那些花里胡哨的强。

```
$ git c
  > commit          提交更改
    checkout        切换分支
    clone           克隆仓库
  [Tab=确认  ↑↓=选择  Esc=取消]
```

### 问号帮助模式
在命令后面打个 `?`，它会把所有可用的选项都给你列出来，按类别分好了。不用再他妈翻 man 手册，不用再百度，直接看，爽不爽？

```
$ tar ?
╭─ tar 帮助 ─────────────────────────────────╮
│ 操作选项:                                   │
│   -c, --create     创建新归档               │
│   -x, --extract    解压归档                 │
│   -t, --list       列出内容                 │
│ 压缩选项:                                   │
│   -z, --gzip       使用 gzip 压缩           │
│   -j, --bzip2      使用 bzip2 压缩          │
╰─────────────────────────────────────────────╯
```

### 组合参数补全
输入 `tar -z` 然后按 Tab，它会建议你继续加参数变成 `-zx`、`-zv`、`-zf` 等组合，不用一个一个想了。

### 智能路径补全
文件路径自动补全，目录用**亮青色**显示，普通文件用绿色，一眼就能分辨。

### 历史命令模式
按 **Alt+H** 打开专门的历史命令选择器，只显示匹配当前输入的历史命令，敲一个字它就实时筛选。

### 实时更新
不管是普通补全还是历史命令，你打字或者删除的时候列表都会实时更新，不用重新按 Tab。

### 智能上下文感应补全（AI 级别，这个功能最牛逼）

基于自然语言理解，自动识别你的意图并查找系统路径。操，这功能太他妈智能了，其他工具都没有：

```bash
export JAVA_HOME=  # 自动查找系统中的 Java 安装路径并建议
export HADOOP_HOME=  # 自动查找 Hadoop 安装路径
export PATH=$PATH:$JAVA_HOME/bin  # 智能建议 PATH 配置
```

**为什么这个功能这么牛逼？**
- **自然语言理解**：理解 `export JAVA_HOME=` 的意图，自动查找 Java 路径，不用你他妈手动找。其他工具能做到？做不到。
- **自动路径查找**：在 `/opt`、`/usr/local`、`/usr/lib/jvm` 等常见位置查找，省得你到处翻。它会自动扫描系统，找到所有可能的安装位置。
- **智能建议**：根据已定义的 `*_HOME` 变量，自动生成 PATH 配置建议，不用一个个敲。比如你定义了 `JAVA_HOME`，它会自动建议 `$PATH:$JAVA_HOME/bin`。
- **大小写智能匹配**：输入 `$path` 自动建议 `$PATH`，保持变量名一致性，不会搞错。这个细节其他工具都没有。

**实际使用场景：**
- 写脚本配置环境变量？不用再手动找路径了
- 配置 Hadoop 集群？自动找到所有相关路径
- 设置开发环境？一键生成所有 PATH 配置

### 颜色区分
- **亮黄色**: 选项和命令
- **亮青色**: 目录
- **绿色**: 文件、子命令
- **灰色**: 描述文字
- **白色**: 你敲的命令

### 编辑器智能补全（cntmd）
在编辑器中输入时，自动分析上下文并提供智能建议：

- **环境变量补全**：输入 `export VAR=` 时，根据变量类型自动查找路径
- **PATH 智能建议**：输入 `export PATH=` 时，基于已定义的 `*_HOME` 变量生成建议
- **变量引用补全**：输入 `$var` 时，自动匹配已定义的变量（不区分大小写）
- **右方向键接受**：按 `->`（右方向键）`tab` 或接受上下文补全建议

---

## cntmd - 操你他妈的编辑器（内置彩蛋）

cnmsb 还自带一个文本编辑器 **cntmd**（操你他妈的），类似 vim 但是带智能补全，而且补全是基于你的输入历史的。妈的，比 vim 好用多了，不用装那些垃圾插件。

**为什么叫 cntmd？** 因为 `cnmsb` 是"操你妈傻逼"，那编辑器就叫"操你他妈的"（cntmd）。够直接，够暴躁。

**这个编辑器有什么特别？**
- 类 vim 操作，但比 vim 更智能
- 自动学习你的输入习惯
- 预装 100+ 常用词汇
- 上下文感知补全（其他编辑器都没有）
- 自动查找系统路径（写脚本时超实用）

### 打开编辑器

两种方式都行：

```bash
# 方式一：直接用 cntmd 命令
cntmd myfile.txt

# 方式二：用 cnmsb edit 子命令
cnmsb edit myfile.txt
```

### 编辑器功能

- **类 vim 操作**：`i` 进入插入模式，`Esc` 返回普通模式，`:w` 保存，`:q` 退出
- **智能补全**：输入的时候自动显示灰色建议，按 Tab 或右箭头接受
- **实时学习**：你输入的词它会记住，下次输入同样的词就会建议
- **预装常用词**：100+ 个 shell 命令和编程关键词开箱即用

### 编辑器快捷键

| 模式 | 按键 | 功能 |
|------|------|------|
| Normal | `i` | 进入插入模式 |
| Normal | `a` | 在光标后插入 |
| Normal | `o` | 下方新建一行 |
| Normal | `h/j/k/l` | 光标移动 |
| Normal | `:w` | 保存文件 |
| Normal | `:q` | 退出（没保存会提示） |
| Normal | `:wq` | 保存并退出 |
| Insert | `Tab` | 接受补全建议 |
| Insert | `→` | 接受补全建议 |
| Insert | `Esc` | 返回 Normal 模式 |

### 补全示例

```
输入 "ex" → 自动建议 "port"（因为 export 是常用命令）
输入 "fun" → 自动建议 "ction"
输入你之前打过的词 → 直接建议补全
```

---

## 支持哪些命令

操，支持 **300+ 命令**，基本上 Linux 能用的都有。不够用？自己加，别他妈抱怨。

**为什么支持这么多命令？**
- 作者自己也在用，所以把常用的都加进去了
- 持续更新，新命令会不断添加
- 支持自定义，你可以自己加命令定义

**最受欢迎的命令（按使用频率排序）：**
1. `git` - 这玩意参数最他妈多，必须支持
2. `docker` - 容器命令太多，记不住
3. `tar` - 参数组合太多，容易搞混
4. `systemctl` - 系统管理必备
5. `kubectl` - K8s 命令太复杂

| 分类 | 命令（部分） |
|------|-------------|
| **版本控制** | git（这玩意参数最他妈多） |
| **容器** | docker, docker-compose, kubectl, podman, helm |
| **包管理** | apt, dpkg, snap, pip, npm, yarn, cargo, go |
| **文件操作** | ls, cp, mv, rm, mkdir, chmod, chown, find, locate, tree, ln |
| **文本处理** | grep, sed, awk, cat, head, tail, less, sort, uniq, wc, cut, tr |
| **网络** | curl, wget, ssh, scp, rsync, netstat, ss, ping, traceroute, nmap, tcpdump, dig |
| **系统** | systemctl, journalctl, ps, top, htop, kill, df, du, free, uname, who |
| **压缩** | tar, zip, unzip, gzip, gunzip, bzip2, xz, 7z |
| **编辑器** | vim, vi, nano, emacs |
| **Shell** | echo, printf, read, test, source, eval, xargs, watch, nohup, screen, tmux, tee |
| **硬件** | lscpu, lsmem, lspci, lsusb, lsblk, hdparm, smartctl |
| **安全** | sudo, su, passwd, ssh-keygen, gpg, openssl, ufw, iptables |
| **开发工具** | make, cmake, gcc, g++, python, node, java, go, rustc, cargo |
| **数据库** | mysql, psql, sqlite3, redis-cli, mongo |
| **云服务** | aws, gcloud, az, terraform, ansible |
| **多媒体** | ffmpeg, ffprobe, convert, sox, mpv, yt-dlp, vlc |
| **虚拟化** | qemu, virsh, vagrant, VBoxManage, multipass |
| **监控** | sar, iostat, vmstat, dstat, glances, iftop, iotop, nethogs |
| **消息队列** | kafka, rabbitmq, mosquitto, nats |
| **备份** | borg, restic, rclone, duplicity, dd |

## 安装（简单到爆）

### 一键安装（所有 Linux 发行版通用）

操，不管你是 Ubuntu、Debian、CentOS、Fedora、Arch 还是什么鬼发行版，一行命令搞定。别他妈问能不能装，能装就装，不能装就滚。

**为什么选择一键安装？**
- 自动检测系统类型
- 自动安装依赖
- 自动配置环境
- 装完就能用，不用折腾

**安装时间：** 通常 2-5 分钟（取决于网络和编译速度）

```bash
curl -sSL https://raw.githubusercontent.com/violettoolssite/cnmsb/main/install-universal.sh | bash
```

这个脚本会自动：
- 检测你的系统类型
- 安装必要的依赖（zsh、rust）
- 下载源码并编译
- 配置好 zsh

装完重新登录就能用了。

### Debian/Ubuntu 用 deb 包安装

如果你是 Debian、Ubuntu、Mint 这些系统，可以直接用 deb 包，更快：

```bash
# 下载 deb 包
wget https://github.com/violettoolssite/cnmsb/releases/latest/download/cnmsb_0.1.0_amd64.deb

# 安装
sudo dpkg -i cnmsb_0.1.0_amd64.deb

# 如果有依赖问题
sudo apt-get install -f
```

### 手动安装

如果你他妈不信任一键脚本（我理解，有些人就是疑神疑鬼），可以手动装。别他妈装完不会用又来问。

```bash
# 克隆项目
git clone https://github.com/violettoolssite/cnmsb.git
cd cnmsb/cnmsb-tool

# 编译
cargo build --release

# 安装
sudo mkdir -p /usr/bin /usr/share/cnmsb
sudo cp target/release/cnmsb /usr/bin/
sudo cp shell/cnmsb.zsh /usr/share/cnmsb/
```

然后在 `~/.zshrc` 里加一行：

```bash
source /usr/share/cnmsb/cnmsb.zsh
```

## 快捷键

| 按键 | 作用 |
|------|------|
| **Tab** | 接受建议 / 打开选择器 / 确认选择 |
| **右箭头** | 接受内联建议 |
| **上下箭头** | 在选择器中切换选项 |
| **Alt+H** | 打开历史命令选择器 |
| **Alt+L** | 🤖 AI 智能补全（需配置 API） |
| **?** | 查看命令帮助 |
| **Esc** | 关闭选择器 |

## 🤖 AI 智能补全（可选功能）

使用大语言模型（默认 Qwen2.5-Coder-32B）生成智能命令补全建议。这是一个**可选功能**，需要配置 API 密钥才能使用。

### 配置 AI 补全

```bash
# 初始化配置文件
cnmsb ai-config init

# 设置 API 密钥（使用 ModelScope）
cnmsb ai-config set api_key <your_api_key>

# 查看当前配置
cnmsb ai-config show
```

### 使用方式

1. 输入命令的一部分
2. 按 **Alt+F4** 触发 AI 补全
3. 选择 AI 建议的命令

### 自定义 API

```bash
# 使用自定义 API 地址
cnmsb ai-config set base_url https://your-api-endpoint/v1/

# 使用其他模型
cnmsb ai-config set model gpt-4

# 禁用 AI 补全
cnmsb ai-config set enabled false
```

配置文件位置：`~/.config/cnmsb/ai.conf`

## 运行 cnmsb 命令（彩蛋时间）

直接运行 `cnmsb` 不带参数会显示这个工具的信息：

```bash
$ cnmsb
╔══════════════════════════════════════════════════════════════╗
║                    cnmsb - 操你妈傻逼                         ║
║              Linux 命令行智能补全，让你少查文档                ║
╚══════════════════════════════════════════════════════════════╝
```

**彩蛋：** 也可以用中文别名（这个功能其他工具都没有）：
```bash
$ 操你妈傻逼
$ 草泥马傻逼
```

**为什么支持中文别名？**
- 因为作者觉得好玩
- 因为够直接，够真实
- 因为其他工具都不敢这么干

## 添加自定义命令

命令定义在 `src/database/commands/` 目录下，都是 YAML 文件：

```yaml
mycommand:
  name: mycommand
  description: 我的命令
  options:
    - short: "-v"
      long: "--verbose"
      description: 详细输出
  subcommands:
    sub1:
      name: sub1
      description: 子命令1
```

详细贡献指南见 [CONTRIBUTING.md](CONTRIBUTING.md)

## 依赖

- Rust 1.70+（编译用的）
- zsh（必须的，bash 实现不了这效果）

## 协议

MIT，爱用用，不用滚。别他妈问能不能商用，能，随便用，别烦我。

## 最后的话

如果你觉得这个工具有用，给个 Star 吧。如果你觉得不好用，也别他妈抱怨，自己改代码去。

**这个工具的特点：**
- 用脏话命名，但功能不垃圾
- AI 级别智能，不是吹的
- 持续更新，作者自己也在用
- 开源免费，随便用
- 够直接，够真实，不装逼

**还在犹豫？** 装一个试试，保证你用了就回不去了。如果不好用，欢迎提 issue，但别他妈就写"不好使"，写清楚问题。

## 项目地址

https://github.com/violettoolssite/cnmsb
