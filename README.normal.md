<div align="center">

# cnmsb - 命令行智能补全工具

**Linux 命令行智能补全工具，提供类似 IDE 的代码补全体验**

[![GitHub stars](https://img.shields.io/github/stars/violettoolssite/cnmsb?style=for-the-badge&logo=github)](https://github.com/violettoolssite/cnmsb/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/violettoolssite/cnmsb?style=for-the-badge&logo=github)](https://github.com/violettoolssite/cnmsb/network/members)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)](LICENSE)

[![Star History Chart](https://api.star-history.com/svg?repos=violettoolssite/cnmsb&type=Date)](https://star-history.com/#violettoolssite/cnmsb&Date)

</div>

---

<div align="center">

### 🚀 **300+ 命令支持 | 智能内联补全 | 交互式选择器 | 历史命令模式 | 内置编辑器 | SQL Shell**

**一款强大的 Linux 命令行智能补全工具，让命令行操作更高效、更智能。**

> [查看另一个版本](README.md)

</div>

---

## 使用演示

![使用教程](demo.gif)

---

## 功能特性

### 智能内联建议
在输入命令时自动显示灰色预测文本，无需按键即可看到建议，按 Tab 或右箭头键接受建议。

### 交互式补全选择器
- 首次按 Tab：打开补全选择菜单
- 使用上下箭头键选择选项
- 再次按 Tab：确认选择
- 按 Esc：取消选择

```
$ git c
  > commit          提交更改
    checkout        切换分支
    clone           克隆仓库
  [Tab=确认  ↑↓=选择  Esc=取消]
```

### 帮助模式
在命令后输入 `?` 可查看该命令的所有可用选项，按类别分组显示。

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
支持类似 `tar -zxvf` 的组合参数智能补全，逐步构建完整参数组合。

### 路径补全
自动补全文件和目录路径，通过颜色区分目录和文件。

### 历史命令集成
按 Alt+H 打开历史命令选择器，实时筛选匹配的历史命令。

### 实时更新
补全列表随输入实时更新，无需重复触发。

### 颜色区分
- **亮黄色**: 选项和命令
- **亮青色**: 目录
- **绿色**: 文件、子命令
- **灰色**: 描述文字
- **白色**: 输入内容

---

## cntmd - 内置文本编辑器

cnmsb 包含一个类 vim 文本编辑器 **cntmd**，具有基于历史的智能补全功能。

### 启动编辑器

```bash
# 方式一：使用 cntmd 命令
cntmd myfile.txt

# 方式二：使用 cnmsb edit 子命令
cnmsb edit myfile.txt
```

### 功能特点

- 类 vim 操作模式（Normal/Insert/Command）
- 智能补全：输入时自动显示灰色建议
- 实时学习：记忆用户输入的词汇
- 预装 100+ 常用词汇（shell 命令、编程关键词）

### 编辑器快捷键

| 模式 | 按键 | 功能 |
|------|------|------|
| Normal | i | 进入插入模式 |
| Normal | a | 在光标后插入 |
| Normal | o | 下方新建一行 |
| Normal | h/j/k/l | 光标移动 |
| Normal | :w | 保存文件 |
| Normal | :q | 退出 |
| Normal | :wq | 保存并退出 |
| Insert | Tab | 接受补全建议 |
| Insert | 右箭头 | 接受补全建议 |
| Insert | Esc | 返回 Normal 模式 |

---

## 支持命令

支持 300+ 常用 Linux 命令，涵盖：

| 分类 | 命令（部分） |
|------|-------------|
| **版本控制** | git |
| **容器管理** | docker, docker-compose, kubectl, podman, helm |
| **包管理** | apt, dpkg, snap, pip, npm, yarn, cargo, go |
| **文件操作** | ls, cp, mv, rm, mkdir, chmod, chown, find, locate, tree, ln |
| **文本处理** | grep, sed, awk, cat, head, tail, less, sort, uniq, wc, cut, tr |
| **网络工具** | curl, wget, ssh, scp, rsync, netstat, ss, ping, traceroute, nmap, tcpdump, dig |
| **系统管理** | systemctl, journalctl, ps, top, htop, kill, df, du, free, uname, who |
| **压缩归档** | tar, zip, unzip, gzip, gunzip, bzip2, xz, 7z |
| **编辑器** | vim, vi, nano, emacs |
| **Shell 工具** | echo, printf, read, test, source, eval, xargs, watch, nohup, screen, tmux, tee |
| **硬件信息** | lscpu, lsmem, lspci, lsusb, lsblk, hdparm, smartctl |
| **安全工具** | sudo, su, passwd, ssh-keygen, gpg, openssl, ufw, iptables |
| **开发工具** | make, cmake, gcc, g++, python, node, java, go, rustc, cargo |
| **数据库客户端** | mysql, psql, sqlite3, redis-cli, mongo |
| **云服务 CLI** | aws, gcloud, az, terraform, ansible |
| **多媒体** | ffmpeg, ffprobe, convert, sox, mpv, yt-dlp, vlc |
| **虚拟化** | qemu, virsh, vagrant, VBoxManage, multipass |
| **监控工具** | sar, iostat, vmstat, dstat, glances, iftop, iotop, nethogs |
| **消息队列** | kafka, rabbitmq, mosquitto, nats |
| **备份工具** | borg, restic, rclone, duplicity, dd |

## 安装方法

### 一键安装（所有 Linux 发行版通用）

支持 Ubuntu、Debian、CentOS、Fedora、Arch、openSUSE 等所有主流发行版：

```bash
curl -sSL https://raw.githubusercontent.com/violettoolssite/cnmsb/main/install-universal.sh | bash
```

安装脚本会自动：
- 检测系统类型
- 安装必要依赖（zsh、rust）
- 下载源码并编译
- 配置 zsh 环境

安装完成后重新登录即可使用。

### Debian/Ubuntu 使用 deb 包安装

适用于 Debian、Ubuntu、Linux Mint 等 Debian 系发行版：

```bash
# 下载 deb 包
wget https://github.com/violettoolssite/cnmsb/releases/latest/download/cnmsb_0.1.0_amd64.deb

# 安装
sudo dpkg -i cnmsb_0.1.0_amd64.deb

# 解决依赖（如有）
sudo apt-get install -f
```

### 手动安装

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

在 `~/.zshrc` 中添加：

```bash
source /usr/share/cnmsb/cnmsb.zsh
```

## 快捷键

| 按键 | 功能 |
|------|------|
| Tab | 接受建议/打开选择器/确认选择 |
| 右箭头 | 接受内联建议 |
| 上下箭头 | 切换选项 |
| Alt+H | 历史命令选择器 |
| ? | 查看帮助 |
| Esc | 取消 |

## 自定义命令

命令定义文件位于 `src/database/commands/` 目录，使用 YAML 格式：

```yaml
mycommand:
  name: mycommand
  description: 命令描述
  options:
    - short: "-v"
      long: "--verbose"
      description: 详细输出
  subcommands:
    sub1:
      name: sub1
      description: 子命令描述
```

详见 [CONTRIBUTING.normal.md](CONTRIBUTING.normal.md)

## 系统要求

- 操作系统：所有主流 Linux 发行版（Ubuntu、Debian、CentOS、Fedora、Arch、openSUSE 等）
- Shell：Zsh（必需，安装脚本会自动安装）
- 编译环境：Rust 1.70+（安装脚本会自动安装）

## 开源协议

MIT License

## 项目地址

https://github.com/violettoolssite/cnmsb

