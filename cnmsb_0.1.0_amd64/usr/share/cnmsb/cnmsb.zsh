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
typeset -g _cidx=0 _csug="" _cmenu=0 _clastbuf=""

_cfetch() {
    _clist=() _cdesc=() _csuff=() _cidx=0 _csug="" _cmenu=0
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

# 显示内联预测（灰色后缀）
_cshow_inline() {
    POSTDISPLAY="" region_highlight=() _csug=""
    [[ ${#_clist[@]} -eq 0 || $_cidx -eq 0 ]] && return
    
    local suf="${_csuff[$_cidx]}"
    _csug="$suf"
    
    POSTDISPLAY="$suf"
    [[ -n "$suf" ]] && region_highlight+=("${#BUFFER} $((${#BUFFER}+${#suf})) fg=240")
}

# 显示选择菜单（全部选项）
_cshow_menu() {
    POSTDISPLAY="" region_highlight=()
    [[ ${#_clist[@]} -eq 0 ]] && return
    
    local disp="\n"
    local i item desc marker
    
    for ((i=1; i<=${#_clist[@]}; i++)); do
        item="${_clist[$i]}"
        desc="${_cdesc[$i]}"
        
        if [[ $i -eq $_cidx ]]; then
            marker="▸ "
            disp+="  $marker\e[1;33m$item\e[0m"
        else
            marker="  "
            disp+="  $marker\e[37m$item\e[0m"
        fi
        
        [[ -n "$desc" ]] && disp+="  \e[90m$desc\e[0m"
        disp+="\n"
    done
    
    disp+="\n  \e[90m[↑↓选择 Tab确认 Esc取消]\e[0m"
    
    POSTDISPLAY="$disp"
    _csug="${_csuff[$_cidx]}"
}

_cupd() { 
    _cfetch "$BUFFER"
    _clastbuf="$BUFFER"
    if [[ $_cmenu -eq 1 ]]; then
        _cshow_menu
    else
        _cshow_inline
    fi
}

_creset() {
    POSTDISPLAY="" region_highlight=()
    _clist=() _cdesc=() _csuff=()
    _cidx=0 _csug="" _cmenu=0 _clastbuf=""
}

# ================== 操作函数 ==================

cnmsb-prev() {
    if [[ $_cmenu -eq 1 && ${#_clist[@]} -gt 0 ]]; then
        ((_cidx--))
        [[ $_cidx -lt 1 ]] && _cidx=${#_clist[@]}
        _csug="${_csuff[$_cidx]}"
        _cshow_menu
    else
        zle .up-line-or-history
    fi
}

cnmsb-next() {
    if [[ $_cmenu -eq 1 && ${#_clist[@]} -gt 0 ]]; then
        ((_cidx++))
        [[ $_cidx -gt ${#_clist[@]} ]] && _cidx=1
        _csug="${_csuff[$_cidx]}"
        _cshow_menu
    else
        zle .down-line-or-history
    fi
}

cnmsb-tab() {
    if [[ ${#_clist[@]} -eq 0 ]]; then
        # 没有建议，获取建议
        _cfetch "$BUFFER"
        _clastbuf="$BUFFER"
        if [[ ${#_clist[@]} -gt 0 ]]; then
            _cmenu=1
            _cshow_menu
        else
            zle expand-or-complete
        fi
    elif [[ $_cmenu -eq 0 ]]; then
        # 有建议但菜单未打开，打开菜单
        _cmenu=1
        _cshow_menu
    else
        # 菜单已打开，接受当前选择
        if [[ -n "$_csug" ]]; then
            BUFFER+="$_csug"
            CURSOR=${#BUFFER}
            _creset
            _cupd
        fi
    fi
}

cnmsb-accept-arrow() {
    # 右箭头直接接受，不管菜单状态
    if [[ -n "$_csug" ]]; then
        BUFFER+="$_csug"
        CURSOR=${#BUFFER}
        _creset
        _cupd
    else
        zle .forward-char
    fi
}

cnmsb-insert() { 
    zle .self-insert
    _cmenu=0  # 输入字符时关闭菜单，回到内联模式
    _cupd
}

cnmsb-delete() { 
    zle .backward-delete-char
    _cmenu=0
    _cupd
}

cnmsb-delword() { 
    zle .backward-kill-word
    _cmenu=0
    _cupd
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
        # 菜单打开时按 Esc 关闭菜单，回到内联模式
        _cmenu=0
        _cshow_inline
    else
        # 内联模式按 Esc 清除建议
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
print -P "  %F{11}Tab%f=弹出选择器  %F{11}Tab Tab%f=接受  %F{green}↑↓%f=切换  %F{green}→%f=直接接受  %F{red}Esc%f=关闭"
