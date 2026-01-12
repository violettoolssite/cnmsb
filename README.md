# cnmsb - 操你妈傻逼

妈的，Linux 命令行补全工具，让你敲命令不用再他妈的翻那些狗屁文档。

> **不想看脏话？** [点这里看正常版本](README.normal.md)

---

## 这鸡巴玩意是干嘛的

你他妈敲命令的时候是不是老忘记参数？`tar` 到底是 `-xvf` 还是 `-zxvf`？`docker` 那一坨狗屎命令谁他妈记得住？每次都要去百度，烦不烦？

cnmsb 就是解决这个傻逼问题的。装上这玩意之后，你敲命令它就会像 IDE 一样在后面给你灰色提示，按 Tab 就补全了，省得你去他妈的查来查去。

## 功能列表（都他妈很实用）

### 智能内联建议
敲命令的时候直接显示灰色预测文字，不用按什么狗屁快捷键，它自己就冒出来了。按 Tab 或者右箭头就能接受建议。

### 补全选择器
按一次 Tab 打开选择菜单，用上下箭头选你要的，再按 Tab 确认。不想要了按 Esc 滚蛋。

```
$ git c
  > commit          提交更改
    checkout        切换分支
    clone           克隆仓库
  [Tab=确认  ↑↓=选择  Esc=取消]
```

### 问号帮助模式
在命令后面打个 `?`，它会把所有可用的选项都给你列出来，按类别分好了：

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

### 颜色区分
- **亮黄色**: 选项和命令
- **亮青色**: 目录
- **绿色**: 文件、子命令
- **灰色**: 描述文字
- **白色**: 你敲的命令

## 支持哪些命令

妈的支持 300+ 命令，基本上 Linux 能用的都有：

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

## 安装

### Debian/Ubuntu

```bash
# 下载 deb 包
wget https://github.com/violettoolssite/cnmsb/releases/latest/download/cnmsb_0.1.0_amd64.deb

# 装上
sudo dpkg -i cnmsb_0.1.0_amd64.deb
```

装完会自动把你的 shell 换成 zsh，重新登录就行了。

### 手动安装

```bash
git clone https://github.com/violettoolssite/cnmsb.git
cd cnmsb/cnmsb-tool
cargo build --release
sudo ./install.sh
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
| **?** | 查看命令帮助 |
| **Esc** | 关闭选择器 |

## 运行 cnmsb 命令

直接运行 `cnmsb` 不带参数会显示这个工具的信息：

```bash
$ cnmsb
╔══════════════════════════════════════════════════════════════╗
║                    cnmsb - 操你妈傻逼                         ║
║              Linux 命令行智能补全，让你少查文档                ║
╚══════════════════════════════════════════════════════════════╝
```

也可以用中文别名：
```bash
$ 操你妈傻逼
```

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

MIT，爱用用，不用滚。

## 项目地址

https://github.com/violettoolssite/cnmsb
