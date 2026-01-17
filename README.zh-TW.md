<div align="center">

**🌍 Language / 語言：** [简体中文](README.md) | [繁體中文](README.zh-TW.md) | [日本語](README.ja.md) | [English](README.en.md)

---

# cnmsb - 智慧命令列補全工具

**Linux 命令列智慧補全工具，提供類似 IDE 的補全體驗**

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

> **「程式設計不再靠打字速度，而靠清晰表達。」**
> 
> *—— Michael Truell, Cursor 聯合創始人*

</div>

---

<div align="center">

### **開箱即用 | 300+ 命令 | 智慧內聯補全 | 互動式選擇器 | 歷史命令 | 內建編輯器 | AI 補全 [可選]**

**最暴躁的命令列補全工具，用實力說話**

> **想看簡體中文版？** [點這裡](README.md)  
> **想看正常版本？** [點這裡看正常版本](README.normal.md)

</div>

---

## 三步上手

```
1. 輸入命令      →  智慧補全自動顯示灰色提示
2. 按 Tab 選擇   →  彈出選擇器，↑↓ 選擇，Tab 確認
3. 自然語言      →  輸入描述，Alt+L 獲取 AI 命令建議（可選）
```

**支援多語言自然輸入：**
- 🇨🇳 中文：`查找大於100M的檔案` → `find . -size +100M`
- 🇺🇸 English：`list running containers` → `docker ps`
- 🇯🇵 日本語：`メモリ使用量を確認` → `free -h`

---

## 這是什麼？

你敲命令的時候是不是老忘記參數？`tar` 到底是 `-xvf` 還是 `-zxvf`？`docker` 那一堆命令誰記得住？每次都要去 Google，煩不煩？

**cnmsb 就是解決這個問題的。** 裝上這玩意之後，你敲命令它就會像 IDE 一樣在後面給你灰色提示，按 Tab 就補全了。不用再翻 man 手冊，不用再搜尋，直接補全。

### 為什麼這個工具這麼特別？

- **開箱即用**：普通補全不需要任何設定，裝完就能用，基於本地 300+ 命令資料庫
- **AI 級別智慧**：不是簡單的關鍵詞匹配，是真的理解你的意圖
- **AI 大模型補全 [可選]**：按 Alt+L 觸發，接入大模型生成命令（需設定 API）
- **學習你的習慣**：用久了它會記住你常用的命令組合
- **內建編輯器**：自帶 `cntmd` 編輯器，帶智慧補全

### 對比一下

| 功能 | 其他補全工具 | cnmsb |
|------|------------|-------|
| 開箱即用 | 需要設定 | **裝完就能用** |
| 智慧程度 | 關鍵詞匹配 | **AI 級別理解** |
| 上下文感知 | 不支援 | **自動查找路徑** |
| 學習能力 | 不支援 | **記住你的習慣** |
| AI 大模型補全 | 不支援 | **可選增強（Alt+L）** |
| 內建編輯器 | 不支援 | **cntmd 編輯器** |

---

## 兩種補全模式

cnmsb 提供 **兩種補全模式**，滿足不同場景需求：

| 模式 | 觸發方式 | 需要 API | 說明 |
|------|---------|---------|------|
| **普通補全** | Tab / 自動 | 不需要 | 核心功能，開箱即用，基於本地命令資料庫 |
| **AI 補全** | Alt+L | 需要 | 可選增強，呼叫大模型理解意圖 |

> **重要**：普通補全是核心功能，**裝完就能用，不需要任何設定**。AI 補全是額外的增強功能。

---

## 支援哪些命令

支援 **300+ 命令**，基本上 Linux 能用的都有：

| 分類 | 命令（部分） |
|------|-------------|
| **版本控制** | git |
| **容器** | docker, docker-compose, kubectl, podman, helm |
| **套件管理** | apt, dpkg, snap, pip, npm, yarn, cargo, go |
| **檔案操作** | ls, cp, mv, rm, mkdir, chmod, chown, find, locate, tree, ln |
| **文字處理** | grep, sed, awk, cat, head, tail, less, sort, uniq, wc, cut, tr |
| **網路** | curl, wget, ssh, scp, rsync, netstat, ss, ping, traceroute, nmap |
| **系統** | systemctl, journalctl, ps, top, htop, kill, df, du, free, uname |
| **壓縮** | tar, zip, unzip, gzip, gunzip, bzip2, xz, 7z |

---

## 安裝

### 一鍵安裝（所有 Linux 發行版通用）

不管你是 Ubuntu、Debian、CentOS、Fedora、Arch 還是什麼發行版，一行命令搞定：

```bash
curl -sSL https://raw.githubusercontent.com/violettoolssite/cnmsb/main/install-universal.sh | bash
```

### Debian/Ubuntu 用 APT 倉庫安裝

```bash
# 添加 GPG 金鑰
curl -fsSL https://cnmsb.kami666.xyz/gpg.key | sudo gpg --dearmor -o /usr/share/keyrings/cnmsb-archive-keyring.gpg

# 添加軟體源
echo "deb [signed-by=/usr/share/keyrings/cnmsb-archive-keyring.gpg] https://cnmsb.kami666.xyz/apt stable main" | sudo tee /etc/apt/sources.list.d/cnmsb.list

# 安裝
sudo apt update
sudo apt install cnmsb
```

### CentOS/RHEL/Fedora 用 yum/dnf 安裝

```bash
# 添加 yum 源
sudo tee /etc/yum.repos.d/cnmsb.repo << EOF
[cnmsb]
name=cnmsb
baseurl=https://cnmsb.kami666.xyz/yum
enabled=1
gpgcheck=1
gpgkey=https://cnmsb.kami666.xyz/gpg.key
EOF

# 安裝
sudo yum install cnmsb   # CentOS/RHEL
sudo dnf install cnmsb   # Fedora
```

### 手動安裝

```bash
# 複製專案
git clone https://github.com/violettoolssite/cnmsb.git
cd cnmsb/cnmsb-tool

# 編譯
cargo build --release

# 安裝
sudo mkdir -p /usr/bin /usr/share/cnmsb
sudo cp target/release/cnmsb /usr/bin/
sudo cp shell/cnmsb.zsh /usr/share/cnmsb/
```

然後在 `~/.zshrc` 裡加一行：

```bash
source /usr/share/cnmsb/cnmsb.zsh
```

---

## 快捷鍵

### 普通補全（核心功能，不需要 API）

| 按鍵 | 作用 |
|------|------|
| **Tab** | 接受建議 / 打開選擇器 / 確認選擇 |
| **右方向鍵** | 接受內聯建議 |
| **上下方向鍵** | 在選擇器中切換選項 |
| **Alt+H** | 打開歷史命令選擇器 |
| **?** | 查看命令幫助 |
| **Esc** | 關閉選擇器 |

### AI 補全（可選增強，需要設定 API）

| 按鍵 | 作用 |
|------|------|
| **Alt+L** | 觸發 AI 智慧補全 |
| **上下方向鍵** | 選擇 AI 建議 |
| **Tab** | 確認選擇 |
| **Esc** | 取消 |

---

## AI 智慧補全（可選增強功能）

> **注意**：這是一個**可選的增強功能**，不設定也完全不影響普通補全的使用。

使用大語言模型（預設 Qwen2.5-Coder-32B）生成智慧命令補全建議。按 **Alt+L** 觸發。

### 獲取 ModelScope API 金鑰

1. 訪問 [modelscope.cn](https://modelscope.cn)
2. 登入後進入「帳號設定 → 訪問令牌」
3. 新建令牌並複製
4. 設定 cnmsb：

```bash
cnmsb ai-config set api_key "你的API金鑰"
```

### 使用方式

1. 輸入命令的一部分（如 `git co`）
2. 按 **Alt+L** 觸發 AI 補全
3. 使用 ↑↓ 選擇建議，按 Tab 確認

```
$ git co
  [AI 智慧補全]
  > git checkout  (切換分支或還原工作樹檔案)
    git commit    (提交更改)
    git config    (取得和設定設定變數)
  [Tab=確認  ↑↓=選擇  Esc=取消]
```

---

## cntmd - 內建編輯器

cnmsb 還自帶一個文字編輯器 **cntmd**，類似 vim 但是帶智慧補全。

### 打開編輯器

```bash
cntmd myfile.txt
# 或
cnmsb edit myfile.txt
```

### 編輯器功能

- 類 vim 操作：`i` 進入插入模式，`Esc` 返回普通模式，`:w` 儲存，`:q` 退出
- 智慧補全：輸入的時候自動顯示灰色建議
- 即時學習：你輸入的詞它會記住

---

## 協議

MIT，隨便用。

## 專案地址

https://github.com/violettoolssite/cnmsb

