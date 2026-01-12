#!/bin/zsh
# cnmsb - Zsh 智能补全
# https://github.com/violettoolssite/cnmsb

(( $+commands[cnmsb] )) || return 0

autoload -Uz compinit && compinit -u -C

# 禁用 ? 作为通配符（避免 "no matches found" 错误）
setopt nonomatch

# ================== 配置 ==================

PS1='%F{208}%n@%m%f:%F{51}%~%f%F{208}%%%f '
typeset -g zle_highlight=(default:fg=226,bold)

# ================== 状态 ==================

typeset -ga _cnmsb_list=() _cnmsb_desc=() _cnmsb_suff=()
typeset -g _cnmsb_idx=0 _cnmsb_menu=0 _cnmsb_lastbuf="" _cnmsb_skip=0 _cnmsb_hist_mode=0

# ================== 核心函数 ==================

# 获取补全（默认过滤历史命令）
_cnmsb_fetch() {
    _cnmsb_list=() _cnmsb_desc=() _cnmsb_suff=() _cnmsb_idx=0
    [[ -z "$1" ]] && return
    
    local comps curword count=0
    # 过滤掉历史命令
    comps=$(cnmsb complete --line "$1" --cursor ${#1} --shell bash 2>/dev/null | grep -v "历史")
    [[ -z "$comps" ]] && return
    
    local words=(${(z)1})
    [[ "$1" != *" " && ${#words[@]} -gt 0 ]] && curword="${words[-1]}"
    
    local text desc suf
    while IFS=$'\t' read -r text desc; do
        [[ -z "$text" ]] && continue
        ((count++))
        [[ $count -gt 10 ]] && break
        
        if [[ -n "$curword" && "$text" == "$curword"* ]]; then
            suf="${text#$curword}"
        else
            suf="$text"
        fi
        _cnmsb_list+=("$text") _cnmsb_desc+=("$desc") _cnmsb_suff+=("$suf")
    done <<< "$comps"
    
    [[ ${#_cnmsb_list[@]} -gt 0 ]] && _cnmsb_idx=1
}

# 获取历史命令（仅历史，按输入前缀过滤）
_cnmsb_fetch_history() {
    _cnmsb_list=() _cnmsb_desc=() _cnmsb_suff=() _cnmsb_idx=0
    
    local count=0
    local prefix="$BUFFER"
    local -a hist_array seen_cmds
    local hist_cmd suf
    
    # 获取历史命令到数组（最近的在前）
    hist_array=("${(@f)$(fc -l -n -r 1 2>/dev/null)}")
    
    for hist_cmd in "${hist_array[@]}"; do
        # 去掉前导空格
        hist_cmd="${hist_cmd#"${hist_cmd%%[![:space:]]*}"}"
        [[ -z "$hist_cmd" ]] && continue
        
        # 如果有输入，只显示以输入开头的历史
        if [[ -n "$prefix" ]]; then
            [[ "$hist_cmd" != "$prefix"* ]] && continue
        fi
        
        # 去重
        [[ " ${seen_cmds[*]} " == *" $hist_cmd "* ]] && continue
        seen_cmds+=("$hist_cmd")
        
        ((count++))
        [[ $count -gt 15 ]] && break
        
        if [[ -n "$prefix" && "$hist_cmd" == "$prefix"* ]]; then
            suf="${hist_cmd#$prefix}"
        else
            suf="$hist_cmd"
        fi
        _cnmsb_list+=("$hist_cmd") _cnmsb_desc+=("历史") _cnmsb_suff+=("$suf")
    done
    
    [[ ${#_cnmsb_list[@]} -gt 0 ]] && _cnmsb_idx=1
}

_cnmsb_clear() {
    POSTDISPLAY=""
    region_highlight=()
}

_cnmsb_show_inline() {
    _cnmsb_clear
    [[ ${#_cnmsb_list[@]} -eq 0 || $_cnmsb_idx -eq 0 ]] && return
    
    local suf="${_cnmsb_suff[$_cnmsb_idx]}"
    [[ -z "$suf" ]] && return
    
    POSTDISPLAY="$suf"
    region_highlight+=("${#BUFFER} $((${#BUFFER}+${#suf})) fg=245")
}

_cnmsb_show_menu() {
    _cnmsb_clear
    [[ ${#_cnmsb_list[@]} -eq 0 ]] && return
    
    local disp=$'\n'
    local i item desc
    
    for ((i=1; i<=${#_cnmsb_list[@]}; i++)); do
        item="${_cnmsb_list[$i]}"
        desc="${_cnmsb_desc[$i]}"
        
        [[ ${#item} -gt 50 ]] && item="${item:0:47}..."
        [[ ${#desc} -gt 30 ]] && desc="${desc:0:27}..."
        
        if [[ $i -eq $_cnmsb_idx ]]; then
            disp+="  > $item"
        else
            disp+="    $item"
        fi
        [[ -n "$desc" ]] && disp+="  ($desc)"
        disp+=$'\n'
    done
    
    disp+=$'\n'"  [Tab=确认  Ctrl+P/N或方向键=选择  Esc=取消]"
    POSTDISPLAY="$disp"
}

_cnmsb_reset() {
    _cnmsb_clear
    _cnmsb_list=() _cnmsb_desc=() _cnmsb_suff=()
    _cnmsb_idx=0 _cnmsb_menu=0 _cnmsb_hist_mode=0 _cnmsb_lastbuf=""
}

# ================== 钩子 ==================

_cnmsb_line_pre_redraw() {
    [[ $_cnmsb_skip -eq 1 ]] && { _cnmsb_skip=0; return; }
    
    if [[ "$BUFFER" != "$_cnmsb_lastbuf" ]]; then
        _cnmsb_lastbuf="$BUFFER"
        
        if [[ $_cnmsb_hist_mode -eq 1 ]]; then
            # 历史模式：实时更新历史菜单
            _cnmsb_fetch_history
            if [[ ${#_cnmsb_list[@]} -eq 0 ]]; then
                # 无匹配结果，退出历史模式
                _cnmsb_hist_mode=0
                _cnmsb_menu=0
                _cnmsb_clear
            else
                _cnmsb_show_history_menu
            fi
        elif [[ $_cnmsb_menu -eq 1 ]]; then
            # 命令菜单模式：实时更新命令菜单
            _cnmsb_fetch "$BUFFER"
            if [[ ${#_cnmsb_list[@]} -eq 0 ]]; then
                # 无匹配结果，退出菜单模式
                _cnmsb_menu=0
                _cnmsb_clear
            else
                _cnmsb_show_menu
            fi
        else
            # 普通模式：显示内联建议
            _cnmsb_fetch "$BUFFER"
            _cnmsb_show_inline
        fi
    fi
}

zle -N zle-line-pre-redraw _cnmsb_line_pre_redraw

# ================== Widget 函数 ==================

_cnmsb_prev() {
    _cnmsb_skip=1
    if [[ ($_cnmsb_menu -eq 1 || $_cnmsb_hist_mode -eq 1) && ${#_cnmsb_list[@]} -gt 0 ]]; then
        ((_cnmsb_idx--))
        [[ $_cnmsb_idx -lt 1 ]] && _cnmsb_idx=${#_cnmsb_list[@]}
        if [[ $_cnmsb_hist_mode -eq 1 ]]; then
            _cnmsb_show_history_menu
        else
            _cnmsb_show_menu
        fi
        zle -R
    else
        zle .up-line-or-history
    fi
}

_cnmsb_next() {
    _cnmsb_skip=1
    if [[ ($_cnmsb_menu -eq 1 || $_cnmsb_hist_mode -eq 1) && ${#_cnmsb_list[@]} -gt 0 ]]; then
        ((_cnmsb_idx++))
        [[ $_cnmsb_idx -gt ${#_cnmsb_list[@]} ]] && _cnmsb_idx=1
        if [[ $_cnmsb_hist_mode -eq 1 ]]; then
            _cnmsb_show_history_menu
        else
            _cnmsb_show_menu
        fi
        zle -R
    else
        zle .down-line-or-history
    fi
}

_cnmsb_tab() {
    _cnmsb_skip=1
    if [[ $_cnmsb_menu -eq 1 || $_cnmsb_hist_mode -eq 1 ]]; then
        if [[ ${#_cnmsb_list[@]} -gt 0 && $_cnmsb_idx -gt 0 ]]; then
            local suf="${_cnmsb_suff[$_cnmsb_idx]}"
            [[ -n "$suf" ]] && { BUFFER+="$suf"; CURSOR=${#BUFFER}; }
        fi
        _cnmsb_reset
        _cnmsb_lastbuf="$BUFFER"
        _cnmsb_fetch "$BUFFER"
        _cnmsb_show_inline
    else
        _cnmsb_fetch "$BUFFER"
        _cnmsb_lastbuf="$BUFFER"
        if [[ ${#_cnmsb_list[@]} -gt 0 ]]; then
            _cnmsb_menu=1
            _cnmsb_show_menu
        else
            zle expand-or-complete
        fi
    fi
    zle -R
}

_cnmsb_accept() {
    _cnmsb_skip=1
    if [[ ($_cnmsb_menu -eq 1 || $_cnmsb_hist_mode -eq 1 || ${#_cnmsb_list[@]} -gt 0) && $_cnmsb_idx -gt 0 ]]; then
        local suf="${_cnmsb_suff[$_cnmsb_idx]}"
        [[ -n "$suf" ]] && { BUFFER+="$suf"; CURSOR=${#BUFFER}; }
        _cnmsb_reset
        _cnmsb_lastbuf="$BUFFER"
        _cnmsb_fetch "$BUFFER"
        _cnmsb_show_inline
    else
        zle .forward-char
    fi
    zle -R
}

_cnmsb_run() {
    POSTDISPLAY=""
    region_highlight=()
    _cnmsb_list=() _cnmsb_desc=() _cnmsb_suff=()
    _cnmsb_idx=0 _cnmsb_menu=0 _cnmsb_lastbuf=""
    zle -R
    zle .accept-line
}

_cnmsb_escape() {
    _cnmsb_skip=1
    if [[ $_cnmsb_menu -eq 1 || $_cnmsb_hist_mode -eq 1 ]]; then
        _cnmsb_menu=0
        _cnmsb_hist_mode=0
        _cnmsb_fetch "$BUFFER"
        _cnmsb_show_inline
    else
        _cnmsb_reset
    fi
    zle -R
}

# 显示历史命令菜单（仅显示部分）
_cnmsb_show_history_menu() {
    _cnmsb_clear
    
    if [[ ${#_cnmsb_list[@]} -gt 0 ]]; then
        local disp=$'\n'
        disp+="  === 历史命令 ==="$'\n'
        
        local i item start_pos
        start_pos=${#BUFFER}
        for ((i=1; i<=${#_cnmsb_list[@]}; i++)); do
            item="${_cnmsb_list[$i]}"
            [[ ${#item} -gt 60 ]] && item="${item:0:57}..."
            
            if [[ $i -eq $_cnmsb_idx ]]; then
                disp+="  > ${item}"
            else
                disp+="    ${item}"
            fi
            disp+=$'\n'
        done
        
        disp+=$'\n'"  [Tab=确认  ↑↓=选择  Esc=取消]"
        POSTDISPLAY="$disp"
        region_highlight+=("$start_pos $((start_pos + ${#disp})) fg=245")
    else
        POSTDISPLAY=$'\n'"  (无匹配的历史命令)"$'\n'
        region_highlight+=("${#BUFFER} $((${#BUFFER} + 25)) fg=245")
    fi
}

# 打开历史命令菜单
_cnmsb_history_menu() {
    _cnmsb_skip=1
    _cnmsb_hist_mode=1
    _cnmsb_menu=1
    _cnmsb_fetch_history
    _cnmsb_show_history_menu
    zle -R
}

# ================== 注册 ==================

zle -N _cnmsb_prev
zle -N _cnmsb_next
zle -N _cnmsb_tab
zle -N _cnmsb_accept
zle -N _cnmsb_run
zle -N _cnmsb_escape
zle -N _cnmsb_history_menu

# ================== ? 帮助功能 ==================

_cnmsb_show_help() {
    _cnmsb_skip=1
    _cnmsb_clear
    
    local prefix="$BUFFER"
    
    echo ""
    
    if [[ -z "$prefix" ]]; then
        # 只输入了 ?，显示所有命令（过滤历史）
        echo "\033[1;38;5;226m可用命令:\033[0m"
        echo ""
        cnmsb complete --line "" --cursor 0 --shell bash 2>/dev/null | grep -v "历史" | head -20 | while IFS=$'\t' read -r cmd desc; do
            printf "  \033[32m%-20s\033[0m %s\n" "$cmd" "$desc"
        done
    elif [[ "$prefix" == *" -"* || "$prefix" == *" --"* ]]; then
        # 参数帮助，如 tar -zx?（过滤历史）
        local cmd="${prefix%% *}"
        echo "\033[1;38;5;226m$cmd 可用选项:\033[0m"
        echo ""
        cnmsb complete --line "$prefix" --cursor ${#prefix} --shell bash 2>/dev/null | grep -v "历史" | head -20 | while IFS=$'\t' read -r opt desc; do
            printf "  \033[38;5;226m%-20s\033[0m %s\n" "$opt" "$desc"
        done
    elif [[ "$prefix" == *" "* ]]; then
        # 子命令帮助，如 git ?（过滤历史）
        local cmd="${prefix%% *}"
        echo "\033[1;38;5;226m$cmd 子命令/选项:\033[0m"
        echo ""
        cnmsb complete --line "$prefix" --cursor ${#prefix} --shell bash 2>/dev/null | grep -v "历史" | head -20 | while IFS=$'\t' read -r sub desc; do
            printf "  \033[36m%-20s\033[0m %s\n" "$sub" "$desc"
        done
    else
        # 命令前缀帮助，如 gi?（过滤历史）
        echo "\033[1;38;5;226m匹配 '$prefix' 的命令:\033[0m"
        echo ""
        cnmsb complete --line "$prefix" --cursor ${#prefix} --shell bash 2>/dev/null | grep -v "历史" | head -20 | while IFS=$'\t' read -r cmd desc; do
            printf "  \033[32m%-20s\033[0m %s\n" "$cmd" "$desc"
        done
    fi
    
    echo ""
    zle reset-prompt
}

_cnmsb_question() {
    _cnmsb_skip=1
    _cnmsb_show_help
    # 不添加 ? 到 BUFFER
}

zle -N _cnmsb_question
zle -N _cnmsb_show_help

# ================== 按键绑定 ==================

# 方向键
bindkey '^[[A' _cnmsb_prev      # Up (CSI)
bindkey '^[[B' _cnmsb_next      # Down (CSI)
bindkey '^[OA' _cnmsb_prev      # Up (SS3)
bindkey '^[OB' _cnmsb_next      # Down (SS3)

# Ctrl+P/N
bindkey '^P' _cnmsb_prev
bindkey '^N' _cnmsb_next

bindkey '^I' _cnmsb_tab         # Tab
bindkey '^[[C' _cnmsb_accept    # Right (CSI)
bindkey '^[OC' _cnmsb_accept    # Right (SS3)
bindkey '^M' _cnmsb_run         # Enter
bindkey '^[' _cnmsb_escape      # Esc

# ? 帮助键
bindkey '?' _cnmsb_question

# Alt+H 历史命令
bindkey '^[h' _cnmsb_history_menu
bindkey '^[H' _cnmsb_history_menu

# ================== 别名 ==================

alias 操你妈傻逼='cnmsb'
alias 草泥马傻逼='cnmsb'
alias caonimashabi='cnmsb'

# ================== 完成 ==================

print -P "%F{208}cnmsb%f 已加载 (输入 \x1b[38;5;226m操你妈傻逼\x1b[0m 或 \x1b[38;5;226mcnmsb\x1b[0m 查看帮助)"
print -P "  %F{226}Tab%f=选择  %F{46}↑↓%f=切换  %F{51}→%f=接受  %F{201}?%f=帮助  %F{245}Alt+H%f=历史  %F{196}Esc%f=取消"
