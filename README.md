# cnmsb - 操你妈傻逼

妈的，Linux 命令行补全工具，让你敲命令不用再他妈的翻那些狗屁文档。

## 这鸡巴玩意是干嘛的

你他妈敲命令的时候是不是老忘记参数？`tar` 到底是 `-xvf` 还是 `-zxvf`？`docker` 那一坨狗屎命令谁他妈记得住？每次都要去百度，烦不烦？

cnmsb 就是解决这个傻逼问题的。装上这玩意之后，你敲命令它就会像 IDE 一样在后面给你灰色提示，按 Tab 就补全了，省得你去他妈的查来查去。

## 特性

- 输入的时候直接显示灰色预测，不用你按什么狗屁快捷键
- Tab 或者右箭头接受建议
- 上下方向键切换建议，想选哪个选哪个
- 支持 50 多个常用命令，基本够用了
- 文件路径也能补全
- 还能匹配你之前敲过的命令

## 安装

### Debian/Ubuntu

```bash
# 下载 deb 包，别问我其他发行版怎么装，自己想办法
wget https://github.com/violettoolssite/cnmsb/releases/latest/download/cnmsb_0.1.0_amd64.deb

# 装上
sudo dpkg -i cnmsb_0.1.0_amd64.deb
```

装完会自动把你的 shell 换成 zsh，重新登录就行了。别他妈问我为什么用 zsh，bash 那破玩意实现不了这效果。

### 手动安装

不想用 deb 包？行，自己编译去：

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

没装 Rust？自己去装，我懒得写教程。

## 怎么用

装好就完事了，不用配置什么狗屁东西，直接敲命令就行。

敲的时候看到：
- **灰色的字** = 它猜你想输的
- **Tab / 右箭头** = 就是这个，给我补上
- **上下箭头** = 不对，换一个
- **Esc** = 滚，不要你的建议

举个鸡巴例子：

```
$ git c[灰色: ommit]     # 按 Tab，变成 git commit
$ tar -[灰色: xvf]       # 按 Tab，变成 tar -xvf
$ docker r[灰色: un]     # 按 Tab，变成 docker run
```

就这么简单，傻逼都会用。

## 支持哪些命令

- **版本控制**: git（这玩意参数最他妈多）
- **容器**: docker, docker-compose, kubectl
- **包管理**: apt, apt-get, dpkg, snap, pip, npm, cargo
- **文件操作**: ls, cp, mv, rm, mkdir, chmod, chown, find
- **文本处理**: grep, sed, awk, cat, head, tail, less, sort, uniq
- **网络**: curl, wget, ssh, scp, rsync, netstat, ss, ping
- **系统**: systemctl, journalctl, ps, top, htop, kill, df, du, free
- **压缩**: tar（终于不用记那些傻逼参数了）, zip, unzip, gzip, gunzip

不够用？自己加，命令定义在 `src/database/commands/` 目录下，都是 YAML 文件，看看就会了。不会？看 CONTRIBUTING.md。

## 为什么叫 cnmsb

因为每次忘记命令参数的时候都想骂人，操你妈傻逼，这参数谁设计的？

## 依赖

- Rust 1.75+（编译用的，装完就不需要了）
- zsh（必须的，bash 那破烂玩意不支持这功能）

## 协议

MIT，爱用用，不用滚，出了问题别他妈来找我，我也不一定能修。

## 有问题？

开 issue，但是别就写一句"不能用"就完了，那我怎么帮你？写清楚你用的什么系统、什么 shell、怎么复现。不写清楚的一律关闭，懒得猜。

## 项目地址

https://github.com/violettoolssite/cnmsb

想贡献代码？看 CONTRIBUTING.md，别瞎提 PR。
