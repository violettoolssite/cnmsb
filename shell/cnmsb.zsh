#!/bin/zsh
# cnmsb - Zsh 智能补全
# https://github.com/violettoolssite/cnmsb

(( $+commands[cnmsb] )) || return 0

autoload -Uz compinit && compinit -u -C

# ================== 配置 ==================

PS1='%F{208}%n@%m%f:%F{blue}%~%f%F{208}%%%f '
typeset -g zle_highlight=(default:fg=11,bold)

# ================== 建议系统 ==================

typeset -ga _clist=() _cdesc=() _csuff=()
typeset -g _cidx=0 _cmenu=0

_cfetch() {
    _clist=() _cdesc=() _csuff=() _cidx=0
    [[ -z "$1" ]] && return
    
    local comps curword
    comps=$(cnmsb complete --line "$1" --cursor ${#1} --shell bash 2>/dev/null)
    [[ -z "$comps" ]] && return
    
    local words=(${(z)1})
    [[ "$1" != *" " && ${#words[@]} -gt 0 ]] && curword="${words[-1]}"
    
    local text desc suf
    while IFS=$'\t' read -r text desc; do
        [[ -z "$text" ]] && continue
        if [[ -n "$curword" && "$text" == "$curword"* ]]; then
            suf="${text#$curword}"
        elif [[ -z "$curword" || "$1" == *" " ]]; then
            suf="$text"
        else
            suf="$text"
        fi
        _clist+=("$text") _cdesc+=("$desc") _csuff+=("$suf")
    done <<< "$comps"
    
    [[ ${#_clist[@]} -gt 0 ]] && _cidx=1
}

# 清除显示
_cclear() {
    POSTDISPLAY=""
    region_highlight=()
}

# 显示内联预测（灰色后缀，不主动显示）
_cshow_inline() {
    _cclear
    [[ ${#_clist[@]} -eq 0 || $_cidx -eq 0 ]] && return
    
    local suf="${_csuff[$_cidx]}"
    [[ -z "$suf" ]] && return
    
    POSTDISPLAY="$suf"
    region_highlight+=("${#BUFFER} $((${#BUFFER}+${#suf})) fg=240")
}

# 显示选择菜单
_cshow_menu() {
    _cclear
    [[ ${#_clist[@]} -eq 0 ]] && return
    
    local disp=$'\n'
    local i item desc
    
    for ((i=1; i<=${#_clist[@]}; i++)); do
        item="${_clist[$i]}"
        desc="${_cdesc[$i]}"
        
        if [[ $i -eq $_cidx ]]; then
            disp+="  > $item"
        else
            disp+="    $item"
        fi
        
        [[ -n "$desc" ]] && disp+="  ($desc)"
        disp+=$'\n'
    done
    
    disp+=$'\n'"  [Tab=确认 ↑↓=选择 Esc=取消]"
    
    POSTDISPLAY="$disp"
}

_creset() {
    _cclear
    _clist=() _cdesc=() _csuff=()
    _cidx=0 _cmenu=0
}

# ================== 操作函数 ==================

cnmsb-prev() {
    if [[ $_cmenu -eq 1 && ${#_clist[@]} -gt 0 ]]; then
        ((_cidx--))
        [[ $_cidx -lt 1 ]] && _cidx=${#_clist[@]}
        _cshow_menu
    else
        zle .up-line-or-history
    fi
}

cnmsb-next() {
    if [[ $_cmenu -eq 1 && ${#_clist[@]} -gt 0 ]]; then
        ((_cidx++))
        [[ $_cidx -gt ${#_clist[@]} ]] && _cidx=1
        _cshow_menu
    else
        zle .down-line-or-history
    fi
}

cnmsb-tab() {
    if [[ $_cmenu -eq 1 ]]; then
        # 菜单已打开，第二次 Tab = 接受
        if [[ ${#_clist[@]} -gt 0 && $_cidx -gt 0 ]]; then
            local suf="${_csuff[$_cidx]}"
            [[ -n "$suf" ]] && { BUFFER+="$suf"; CURSOR=${#BUFFER}; }
        fi
        _creset
        # 接受后重新获取建议（为下一次输入准备）
        _cfetch "$BUFFER"
    else
        # 第一次 Tab = 弹出选择器
        _cfetch "$BUFFER"
        if [[ ${#_clist[@]} -gt 0 ]]; then
            _cmenu=1
            _cshow_menu
        else
            zle expand-or-complete
        fi
    fi
}

cnmsb-accept-arrow() {
    # 右箭头直接接受当前建议
    if [[ $_cmenu -eq 1 && ${#_clist[@]} -gt 0 && $_cidx -gt 0 ]]; then
        local suf="${_csuff[$_cidx]}"
        [[ -n "$suf" ]] && { BUFFER+="$suf"; CURSOR=${#BUFFER}; }
        _creset
        _cfetch "$BUFFER"
    elif [[ ${#_clist[@]} -gt 0 && $_cidx -gt 0 ]]; then
        # 内联模式也可以用右箭头接受
        local suf="${_csuff[$_cidx]}"
        [[ -n "$suf" ]] && { BUFFER+="$suf"; CURSOR=${#BUFFER}; }
        _creset
        _cfetch "$BUFFER"
    else
        zle .forward-char
    fi
}

cnmsb-insert() { 
    zle .self-insert
    _cmenu=0
    _cclear
    # 输入时获取建议并显示内联预测
    _cfetch "$BUFFER"
    _cshow_inline
}

cnmsb-delete() { 
    zle .backward-delete-char
    _cmenu=0
    _cclear
    _cfetch "$BUFFER"
    _cshow_inline
}

cnmsb-delword() { 
    zle .backward-kill-word
    _cmenu=0
    _cclear
    _cfetch "$BUFFER"
    _cshow_inline
}

cnmsb-run() { 
    _creset
    zle .accept-line
}

cnmsb-cancel() { 
    _creset
    BUFFER=""
    zle .redisplay
}

cnmsb-escape() { 
    if [[ $_cmenu -eq 1 ]]; then
        _cmenu=0
        _cshow_inline
    else
        _creset
        zle .redisplay
    fi
}

# ================== 注册 ==================

zle -N cnmsb-prev
zle -N cnmsb-next
zle -N cnmsb-tab
zle -N cnmsb-accept-arrow
zle -N cnmsb-insert
zle -N cnmsb-delete
zle -N cnmsb-delword
zle -N cnmsb-run
zle -N cnmsb-cancel
zle -N cnmsb-escape

# ================== 按键绑定 ==================

bindkey '^[[A' cnmsb-prev         # ↑
bindkey '^[[B' cnmsb-next         # ↓
bindkey '^I' cnmsb-tab            # Tab
bindkey '^[[C' cnmsb-accept-arrow # →
bindkey '^M' cnmsb-run            # Enter
bindkey '^?' cnmsb-delete         # Backspace
bindkey '^H' cnmsb-delete
bindkey '^W' cnmsb-delword        # Ctrl+W
bindkey '^C' cnmsb-cancel         # Ctrl+C
bindkey '^[' cnmsb-escape         # Esc

# 字符输入
local c; for c in {a..z} {A..Z} {0..9} ' ' '-' '_' '.' '/' ':' '=' '+' '@' '~' '"' "'" ',' ';' '!' '?' '*' '&' '|' '<' '>' '(' ')' '[' ']' '{' '}' '$' '#' '%' '^' '\\' '`'; do
    bindkey "$c" cnmsb-insert 2>/dev/null
done

# ================== 完成 ==================

print -P "%F{208}cnmsb%f 已加载"
print -P "  %F{11}Tab%f=弹出选择  %F{11}Tab Tab%f=确认  %F{green}↑↓%f=切换  %F{green}→%f=直接接受"
