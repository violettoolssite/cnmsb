#!/bin/bash
# cnmsb - 操你妈傻逼 Bash 集成脚本
# 提供智能命令补全体验

# 检查 cnmsb 是否可用
if ! command -v cnmsb &> /dev/null; then
    return 0
fi

# ================== 智能 Tab 补全 ==================

# cnmsb 补全函数
_cnmsb_completions() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local line="${COMP_LINE}"
    local cursor="${COMP_POINT}"
    
    # 调用 cnmsb 获取补全
    local completions
    completions=$(cnmsb complete --line "$line" --cursor "$cursor" --shell bash 2>/dev/null)
    
    if [[ -z "$completions" ]]; then
        return 0
    fi
    
    # 解析补全结果
    local IFS=$'\n'
    local items=($completions)
    
    COMPREPLY=()
    for item in "${items[@]}"; do
        local text="${item%%$'\t'*}"
        local desc="${item#*$'\t'}"
        if [[ -n "$text" ]]; then
            # 格式化显示：补全文本 (描述)
            if [[ "$desc" != "$text" && -n "$desc" ]]; then
                COMPREPLY+=("$text")
            else
                COMPREPLY+=("$text")
            fi
        fi
    done
    
    return 0
}

# 为常用命令注册补全
_cnmsb_register_commands() {
    local commands=(
        # 版本控制
        git
        # 容器
        docker kubectl
        # 包管理
        apt apt-get apt-cache dpkg yum dnf pacman snap flatpak
        npm cargo pip pip3
        # 系统管理
        systemctl service journalctl
        sudo su
        ps top htop kill killall pkill
        # 文件操作
        ls cp mv rm mkdir rmdir touch ln chmod chown
        find locate tree
        # 文本处理
        cat less more head tail grep sed awk sort uniq wc cut tr diff
        # 编辑器
        vim vi nano emacs code
        # 网络
        ping curl wget ssh scp rsync netstat ss ip
        nslookup dig host traceroute nc nmap
        # 压缩
        tar gzip gunzip zip unzip xz
        # 开发
        make cmake gcc g++ python python3 node java javac mvn gradle rustc go
        # 其他
        man history which whereis type xargs env export
    )
    
    for cmd in "${commands[@]}"; do
        complete -F _cnmsb_completions "$cmd" 2>/dev/null
    done
}

_cnmsb_register_commands

# 可选：设置更好的补全体验
if [[ -n "$BASH_VERSION" ]] && [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
    bind 'set show-all-if-ambiguous on' 2>/dev/null
    bind 'set show-all-if-unmodified on' 2>/dev/null
    bind 'set completion-ignore-case on' 2>/dev/null
    bind 'set menu-complete-display-prefix on' 2>/dev/null
    # Tab 循环补全选项
    bind 'TAB:menu-complete' 2>/dev/null
    bind '"\e[Z":menu-complete-backward' 2>/dev/null  # Shift+Tab 反向
fi

# 提示信息
echo "cnmsb (操你妈傻逼) 已加载 - 按 Tab 获取智能补全"
