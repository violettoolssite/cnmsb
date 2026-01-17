<div align="center">

**🌍 Language / 言語：** [简体中文](README.md) | [繁體中文](README.zh-TW.md) | [日本語](README.ja.md) | [English](README.en.md)

---

# cnmsb - スマートシェル補完ツール

**Linux コマンドラインスマート補完ツール、IDE のような補完体験を提供**

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

> **「プログラミングはもはやタイピング速度ではなく、明確な表現力だ。」**
> 
> *—— Michael Truell, Cursor 共同創設者*

</div>

---

<div align="center">

### **すぐに使える | 300+ コマンド | スマートインライン補完 | インタラクティブセレクター | 履歴 | 内蔵エディタ | AI 補完 [オプション]**

**最も直接的なコマンドライン補完ツール**

> **中国語版を見たい？** [こちらをクリック](README.md)  
> **通常版を見たい？** [こちらをクリック](README.normal.md)

</div>

---

## 3ステップで始める

```
1. コマンド入力    →  スマート補完がグレーの提案を自動表示
2. Tab を押す      →  セレクターを開く、↑↓ で選択、Tab で確認
3. 自然言語        →  説明を入力、Alt+L で AI コマンド提案（オプション）
```

**多言語自然入力対応：**
- 🇨🇳 中国語：`查找大于100M的文件` → `find . -size +100M`
- 🇺🇸 English：`list running containers` → `docker ps`
- 🇯🇵 日本語：`メモリ使用量を確認` → `free -h`

---

## これは何？

コマンドを打つときにパラメータを忘れていませんか？`tar` は `-xvf` なのか `-zxvf` なのか？Docker のあのコマンドを誰が覚えていますか？毎回 Google で検索するのは面倒ですよね？

**cnmsb はこの問題を解決します。** インストールすると、コマンドを入力するとき IDE のようにグレーの提案が表示され、Tab を押すと補完されます。もう man ページを見る必要もなく、検索する必要もありません。

### なぜこのツールが特別なのか？

- **すぐに使える**：設定不要、ローカルの 300+ コマンドデータベースに基づく
- **AI レベルのスマート**：単なるキーワードマッチングではなく、意図を理解
- **AI 補完 [オプション]**：Alt+L で LLM による提案（API 設定が必要）
- **習慣を学習**：使えば使うほどスマートになる
- **内蔵エディタ**：スマート補完付きの `cntmd` エディタ

### 比較

| 機能 | 他のツール | cnmsb |
|------|----------|-------|
| すぐに使える | 設定が必要 | **すぐに動作** |
| スマート度 | キーワードマッチング | **AI レベルの理解** |
| コンテキスト認識 | 非対応 | **パスを自動検索** |
| 学習機能 | 非対応 | **習慣を記憶** |
| AI 補完 | 非対応 | **オプション（Alt+L）** |
| 内蔵エディタ | 非対応 | **cntmd エディタ** |

---

## 2つの補完モード

cnmsb は **2つの補完モード** を提供し、さまざまなシナリオに対応：

| モード | トリガー | API 必要 | 説明 |
|--------|---------|---------|------|
| **通常補完** | Tab / 自動 | 不要 | コア機能、オフラインで動作、ローカルデータベースに基づく |
| **AI 補完** | Alt+L | 必要 | オプション拡張、意図を理解するために LLM を使用 |

> **重要**：通常補完はコア機能で、**インストール後すぐに使え、設定不要**です。AI 補完はオプションの拡張機能です。

---

## 対応コマンド

**300+** のコマンドに対応、一般的な Linux コマンドのほとんどをカバー：

| カテゴリ | コマンド（一部） |
|---------|-----------------|
| **バージョン管理** | git |
| **コンテナ** | docker, docker-compose, kubectl, podman, helm |
| **パッケージ管理** | apt, dpkg, snap, pip, npm, yarn, cargo, go |
| **ファイル操作** | ls, cp, mv, rm, mkdir, chmod, chown, find, locate, tree, ln |
| **テキスト処理** | grep, sed, awk, cat, head, tail, less, sort, uniq, wc, cut, tr |
| **ネットワーク** | curl, wget, ssh, scp, rsync, netstat, ss, ping, traceroute, nmap |
| **システム** | systemctl, journalctl, ps, top, htop, kill, df, du, free, uname |
| **圧縮** | tar, zip, unzip, gzip, gunzip, bzip2, xz, 7z |

---

## インストール

### ワンクリックインストール（ユニバーサル）

Ubuntu、Debian、CentOS、Fedora、Arch などで動作：

```bash
curl -sSL https://raw.githubusercontent.com/violettoolssite/cnmsb/main/install-universal.sh | bash
```

### Debian/Ubuntu APT リポジトリ

```bash
# GPG キーを追加
curl -fsSL https://cnmsb.kami666.xyz/gpg.key | sudo gpg --dearmor -o /usr/share/keyrings/cnmsb-archive-keyring.gpg

# ソースを追加
echo "deb [signed-by=/usr/share/keyrings/cnmsb-archive-keyring.gpg] https://cnmsb.kami666.xyz/apt stable main" | sudo tee /etc/apt/sources.list.d/cnmsb.list

# インストール
sudo apt update
sudo apt install cnmsb
```

### CentOS/RHEL/Fedora YUM リポジトリ

```bash
# yum ソースを追加
sudo tee /etc/yum.repos.d/cnmsb.repo << EOF
[cnmsb]
name=cnmsb
baseurl=https://cnmsb.kami666.xyz/yum
enabled=1
gpgcheck=1
gpgkey=https://cnmsb.kami666.xyz/gpg.key
EOF

# インストール
sudo yum install cnmsb   # CentOS/RHEL
sudo dnf install cnmsb   # Fedora
```

### 手動インストール

```bash
# リポジトリをクローン
git clone https://github.com/violettoolssite/cnmsb.git
cd cnmsb/cnmsb-tool

# ビルド
cargo build --release

# インストール
sudo mkdir -p /usr/bin /usr/share/cnmsb
sudo cp target/release/cnmsb /usr/bin/
sudo cp shell/cnmsb.zsh /usr/share/cnmsb/
```

`~/.zshrc` に追加：

```bash
source /usr/share/cnmsb/cnmsb.zsh
```

---

## キーボードショートカット

### 通常補完（コア、API 不要）

| キー | 動作 |
|------|------|
| **Tab** | 提案を受け入れる / セレクターを開く / 確認 |
| **右矢印** | インライン提案を受け入れる |
| **上下矢印** | オプションを移動 |
| **Alt+H** | 履歴セレクターを開く |
| **?** | コマンドヘルプを表示 |
| **Esc** | セレクターを閉じる |

### AI 補完（オプション、API 必要）

| キー | 動作 |
|------|------|
| **Alt+L** | AI 補完をトリガー |
| **上下矢印** | AI 提案を選択 |
| **Tab** | 選択を確認 |
| **Esc** | キャンセル |

---

## AI スマート補完（オプション）

> **注意**：これは**オプションの拡張機能**です。通常補完（Tab）は API なしで動作します。

大規模言語モデル（デフォルト Qwen2.5-Coder-32B）を使用してスマートなコマンド補完提案を生成します。**Alt+L** でトリガー。

### ModelScope API キーの取得

1. [modelscope.cn](https://modelscope.cn) にアクセス
2. ログイン後、「アカウント設定 → アクセストークン」に移動
3. 新しいトークンを作成してコピー
4. cnmsb を設定：

```bash
cnmsb ai-config set api_key "あなたのAPIキー"
```

### 使い方

1. コマンドの一部を入力（例：`git co`）
2. **Alt+L** を押して AI 補完をトリガー
3. ↑↓ で選択、Tab で確認

```
$ git co
  [AI スマート補完]
  > git checkout  (ブランチを切り替えるかファイルを復元)
    git commit    (変更をコミット)
    git config    (設定を取得・設定)
  [Tab=確認  ↑↓=選択  Esc=キャンセル]
```

---

## cntmd - 内蔵エディタ

cnmsb にはスマート補完付きのテキストエディタ **cntmd** が含まれています。

### エディタを開く

```bash
cntmd myfile.txt
# または
cnmsb edit myfile.txt
```

### 機能

- Vim ライクなキーバインド：`i` で挿入、`Esc` でノーマル、`:w` で保存、`:q` で終了
- スマート補完：入力中にグレーの提案を表示
- 学習機能：入力した単語を記憶

---

## ライセンス

MIT - 自由に使用してください。

## プロジェクト

https://github.com/violettoolssite/cnmsb

