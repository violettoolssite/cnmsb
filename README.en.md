<div align="center">

**🌍 Language / 语言：** [简体中文](README.md) | [繁體中文](README.zh-TW.md) | [日本語](README.ja.md) | [English](README.en.md)

---

# cnmsb - Smart Shell Completion

**Linux Command Line Smart Completion Tool with IDE-like Experience**

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

> **"Programming is no longer about typing speed, but about clear expression."**
> 
> *—— Michael Truell, Cursor Co-founder*

</div>

---

<div align="center">

### **Ready to Use | 300+ Commands | Smart Inline Completion | Interactive Selector | History | Built-in Editor | AI Completion [Optional]**

**A brutally honest command line completion tool that speaks your language**

> **Why the name cnmsb?** It's a Chinese slang the author often mutters while coding. Blunt but real.  
> **Want a cleaner version?** [Click here for normal version](README.normal.md)

</div>

---

## Get Started in 3 Steps

```
1. Type command    →  Smart completion shows gray suggestions
2. Press Tab       →  Open selector, ↑↓ to choose, Tab to confirm
3. Natural language →  Describe what you want, Alt+L for AI suggestions (optional)
```

**Multi-language natural input support:**
- 🇨🇳 Chinese: `查找大于100M的文件` → `find . -size +100M`
- 🇺🇸 English: `list running containers` → `docker ps`
- 🇯🇵 Japanese: `メモリ使用量を確認` → `free -h`

---

## What is This?

Tired of forgetting command parameters? Is it `tar -xvf` or `-zxvf`? Who can remember all those Docker commands?

**cnmsb solves this problem.** Once installed, it shows IDE-like gray suggestions while you type. Press Tab to complete. No more man pages, no more Google searches.

### Why This Tool?

- **Works out of the box**: No configuration needed, based on local 300+ command database
- **AI-level intelligence**: Not just keyword matching, actually understands your intent
- **AI completion [Optional]**: Press Alt+L for LLM-powered suggestions (requires API setup)
- **Learns your habits**: Gets smarter the more you use it
- **Built-in editor**: Comes with `cntmd` editor with smart completion

### Comparison

| Feature | Other Tools | cnmsb |
|---------|-------------|-------|
| Ready to use | Needs config | **Works immediately** |
| Intelligence | Keyword matching | **AI-level understanding** |
| Context aware | Not supported | **Auto-find paths** |
| Learning | Not supported | **Remembers your habits** |
| AI completion | Not supported | **Optional (Alt+L)** |
| Built-in editor | Not supported | **cntmd editor** |

---

## Two Completion Modes

cnmsb provides **two completion modes** for different scenarios:

| Mode | Trigger | Needs API | Description |
|------|---------|-----------|-------------|
| **Normal Completion** | Tab / Auto | No | Core feature, works offline, based on local database |
| **AI Completion** | Alt+L | Yes | Optional enhancement, uses LLM for understanding |

> **Important**: Normal completion is the core feature, **works immediately after installation, no configuration needed**. AI completion is an optional enhancement.

---

## Supported Commands

Supports **300+ commands**, covering most common Linux commands:

| Category | Commands (partial) |
|----------|-------------------|
| **Version Control** | git |
| **Containers** | docker, docker-compose, kubectl, podman, helm |
| **Package Management** | apt, dpkg, snap, pip, npm, yarn, cargo, go |
| **File Operations** | ls, cp, mv, rm, mkdir, chmod, chown, find, locate, tree, ln |
| **Text Processing** | grep, sed, awk, cat, head, tail, less, sort, uniq, wc, cut, tr |
| **Network** | curl, wget, ssh, scp, rsync, netstat, ss, ping, traceroute, nmap |
| **System** | systemctl, journalctl, ps, top, htop, kill, df, du, free, uname |
| **Compression** | tar, zip, unzip, gzip, gunzip, bzip2, xz, 7z |

---

## Installation

### One-click Install (Universal)

Works on Ubuntu, Debian, CentOS, Fedora, Arch, and more:

```bash
curl -sSL https://raw.githubusercontent.com/violettoolssite/cnmsb/main/install-universal.sh | bash
```

### Debian/Ubuntu APT Repository

```bash
# Add GPG key
curl -fsSL https://cnmsb.kami666.xyz/gpg.key | sudo gpg --dearmor -o /usr/share/keyrings/cnmsb-archive-keyring.gpg

# Add source
echo "deb [signed-by=/usr/share/keyrings/cnmsb-archive-keyring.gpg] https://cnmsb.kami666.xyz/apt stable main" | sudo tee /etc/apt/sources.list.d/cnmsb.list

# Install
sudo apt update
sudo apt install cnmsb
```

### CentOS/RHEL/Fedora YUM Repository

```bash
# Add yum source
sudo tee /etc/yum.repos.d/cnmsb.repo << EOF
[cnmsb]
name=cnmsb
baseurl=https://cnmsb.kami666.xyz/yum
enabled=1
gpgcheck=1
gpgkey=https://cnmsb.kami666.xyz/gpg.key
EOF

# Install
sudo yum install cnmsb   # CentOS/RHEL
sudo dnf install cnmsb   # Fedora
```

### Manual Install

```bash
# Clone repository
git clone https://github.com/violettoolssite/cnmsb.git
cd cnmsb/cnmsb-tool

# Build
cargo build --release

# Install
sudo mkdir -p /usr/bin /usr/share/cnmsb
sudo cp target/release/cnmsb /usr/bin/
sudo cp shell/cnmsb.zsh /usr/share/cnmsb/
```

Add to `~/.zshrc`:

```bash
source /usr/share/cnmsb/cnmsb.zsh
```

---

## Keyboard Shortcuts

### Normal Completion (Core, No API needed)

| Key | Action |
|-----|--------|
| **Tab** | Accept suggestion / Open selector / Confirm |
| **Right Arrow** | Accept inline suggestion |
| **Up/Down** | Navigate options |
| **Alt+H** | Open history selector |
| **?** | View command help |
| **Esc** | Close selector |

### AI Completion (Optional, Needs API)

| Key | Action |
|-----|--------|
| **Alt+L** | Trigger AI completion |
| **Up/Down** | Select AI suggestion |
| **Tab** | Confirm selection |
| **Esc** | Cancel |

---

## AI Smart Completion (Optional)

> **Note**: This is an **optional enhancement**. Normal completion (Tab) works without any API.

Uses large language models (default Qwen2.5-Coder-32B) to generate smart command suggestions. Press **Alt+L** to trigger.

### Get ModelScope API Key

1. Visit [modelscope.cn](https://modelscope.cn)
2. Login and go to Account Settings → Access Token
3. Create new token and copy it
4. Configure cnmsb:

```bash
cnmsb ai-config set api_key "your-api-key"
```

### Usage

1. Type part of a command (e.g., `git co`)
2. Press **Alt+L** to trigger AI completion
3. Use ↑↓ to select, Tab to confirm

```
$ git co
  [AI Smart Completion]
  > git checkout  (switch branches or restore files)
    git commit    (commit changes)
    git config    (get and set configuration)
  [Tab=confirm  ↑↓=select  Esc=cancel]
```

---

## cntmd - Built-in Editor

cnmsb includes a text editor **cntmd** with smart completion based on your input history.

### Open Editor

```bash
cntmd myfile.txt
# or
cnmsb edit myfile.txt
```

### Features

- Vim-like keybindings: `i` for insert, `Esc` for normal, `:w` save, `:q` quit
- Smart completion: Shows gray suggestions while typing
- Learning: Remembers words you type

---

## License

MIT - Use it however you want.

## Project

https://github.com/violettoolssite/cnmsb

