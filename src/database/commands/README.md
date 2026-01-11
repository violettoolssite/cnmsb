# 命令定义文件

这个目录放的是命令参数定义，每个 YAML 文件是一个分类。

## 文件列表

| 文件 | 内容 |
|------|------|
| git.yaml | Git 版本控制 |
| docker.yaml | Docker 容器 |
| kubernetes.yaml | Kubernetes |
| files.yaml | 文件操作命令 |
| text.yaml | 文本处理命令 |
| network.yaml | 网络工具 |
| system.yaml | 系统管理 |
| package.yaml | 包管理器 |
| archive.yaml | 压缩归档 |

## 格式

```yaml
命令名:
  description: "说明"
  subcommands:
    - name: "子命令"
      description: "说明"
  options:
    - name: "--长选项"
      short: "-短选项"
      description: "说明"
      takes_value: true/false
```

想添加命令？看根目录的 CONTRIBUTING.md。
