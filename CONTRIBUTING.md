# 贡献指南

想给这项目添砖加瓦？行，看看怎么搞。

## 添加新命令

最简单的贡献方式就是添加命令定义。

命令定义文件在 `src/database/commands/` 目录下，按类别分成了多个 YAML 文件：

```
commands/
├── git.yaml         # Git 相关
├── docker.yaml      # Docker 相关
├── kubernetes.yaml  # K8s 相关
├── files.yaml       # 文件操作
├── text.yaml        # 文本处理
├── network.yaml     # 网络工具
├── system.yaml      # 系统管理
├── package.yaml     # 包管理器
└── archive.yaml     # 压缩归档
```

### YAML 格式

```yaml
command_name:
  description: "命令简介"
  subcommands:
    - name: "子命令名"
      description: "子命令说明"
  options:
    - name: "--option"
      short: "-o"
      description: "选项说明"
      takes_value: true  # 是否需要参数值
```

### 示例

给 `htop` 添加定义：

```yaml
htop:
  description: "交互式进程查看器"
  options:
    - name: "--delay"
      short: "-d"
      description: "更新延迟秒数"
      takes_value: true
    - name: "--sort-key"
      short: "-s"
      description: "排序列"
      takes_value: true
    - name: "--user"
      short: "-u"
      description: "只显示指定用户进程"
      takes_value: true
    - name: "--tree"
      short: "-t"
      description: "树状显示"
    - name: "--help"
      short: "-h"
      description: "显示帮助"
    - name: "--version"
      short: "-V"
      description: "显示版本"
```

### 添加新分类

如果你要加的命令不属于现有分类，直接在 `commands/` 目录下新建一个 `.yaml` 文件就行，程序会自动加载。

## 修改核心代码

核心代码在 `src/` 目录：

- `main.rs` - 入口
- `engine.rs` - 补全引擎
- `parser.rs` - 命令行解析
- `completions/` - 各类补全实现
- `database/` - 命令数据库加载

### 构建

```bash
cargo build --release
```

### 测试

```bash
# 测试补全
./target/release/cnmsb complete --line "git " --cursor 4 --shell zsh

# 测试交互模式
./target/release/cnmsb
```

### 构建 deb 包

```bash
./build-deb.sh
```

## 提交 PR

1. Fork 这个仓库
2. 创建你的分支: `git checkout -b feature/xxx`
3. 提交改动: `git commit -m '添加了xxx'`
4. 推送: `git push origin feature/xxx`
5. 提交 Pull Request

### 提交信息格式

别整那些花里胡哨的 commit message 格式，说清楚你干了啥就行：

```
添加 htop 命令定义
修复 tar 参数补全问题
优化模糊匹配性能
```

## 问题反馈

有 bug 或者建议直接开 issue，描述清楚：

1. 你用的什么系统
2. 什么 shell
3. 怎么复现问题
4. 期望的行为是什么

别就扔一句"不好使"就完了，那我也帮不了你。

## 代码规范

- Rust 代码用 `cargo fmt` 格式化
- Shell 脚本别写太复杂，能跑就行
- YAML 文件用 2 空格缩进

## 许可证

贡献的代码会采用 MIT 协议，提交即表示同意。

