#!/bin/zsh
# cnmsb - Zsh 智能补全
# https://github.com/cnmsb/cnmsb

(( $+commands[cnmsb] )) || return 0

autoload -Uz compinit && compinit -u -C

# ================== 配置 ==================

PS1='%F{208}%n@%m%f:%F{blue}%~%f%F{208}%%%f '
typeset -g zle_highlight=(default:fg=11,bold)

# ================== 建议系统 ==================

typeset -ga _clist=() _cdesc=() _csuff=()
typeset -g _cidx=0 _csug=""

_cfetch() {
    _clist=() _cdesc=() _csuff=() _cidx=0 _csug=""
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

_cshow() {
    POSTDISPLAY="" region_highlight=() _csug=""
    [[ ${#_clist[@]} -eq 0 || $_cidx -eq 0 ]] && return
    
    local suf="${_csuff[$_cidx]}" desc="${_cdesc[$_cidx]}"
    _csug="$suf"
    
    local disp="$suf"
    [[ ${#_clist[@]} -gt 1 ]] && disp+="  [${_cidx}/${#_clist[@]}]"
    [[ -n "$desc" ]] && disp+=" $desc"
    
    POSTDISPLAY="$disp"
    region_highlight+=("${#BUFFER} $((${#BUFFER}+${#POSTDISPLAY})) fg=240")
}

_cupd() { _cfetch "$BUFFER"; _cshow; }

# ================== 操作函数 ==================

cnmsb-prev() {
    [[ ${#_clist[@]} -gt 1 ]] && { ((_cidx--)); [[ $_cidx -lt 1 ]] && _cidx=${#_clist[@]}; _cshow; } || zle .up-line-or-history
}

cnmsb-next() {
    [[ ${#_clist[@]} -gt 1 ]] && { ((_cidx++)); [[ $_cidx -gt ${#_clist[@]} ]] && _cidx=1; _cshow; } || zle .down-line-or-history
}

cnmsb-accept() {
    [[ -n "$_csug" ]] && { BUFFER+="$_csug"; CURSOR=${#BUFFER}; _clist=() _cdesc=() _csuff=(); _cidx=0 _csug=""; POSTDISPLAY="" region_highlight=(); _cupd; } || zle expand-or-complete
}

cnmsb-insert() { zle .self-insert; _cupd; }
cnmsb-delete() { zle .backward-delete-char; _cupd; }
cnmsb-delword() { zle .backward-kill-word; _cupd; }
cnmsb-run() { POSTDISPLAY="" region_highlight=(); _clist=() _cdesc=() _csuff=(); _cidx=0 _csug=""; zle .accept-line; }
cnmsb-cancel() { POSTDISPLAY="" region_highlight=(); _clist=() _cdesc=() _csuff=(); _cidx=0 _csug=""; BUFFER=""; zle .redisplay; }
cnmsb-escape() { POSTDISPLAY="" region_highlight=(); _clist=() _cdesc=() _csuff=(); _cidx=0 _csug=""; zle .redisplay; }

# ================== 注册 ==================

zle -N cnmsb-prev
zle -N cnmsb-next
zle -N cnmsb-accept
zle -N cnmsb-insert
zle -N cnmsb-delete
zle -N cnmsb-delword
zle -N cnmsb-run
zle -N cnmsb-cancel
zle -N cnmsb-escape

# ================== 按键绑定 ==================

bindkey '^[[A' cnmsb-prev      # ↑
bindkey '^[[B' cnmsb-next      # ↓
bindkey '^I' cnmsb-accept      # Tab
bindkey '^[[C' cnmsb-accept    # →
bindkey '^M' cnmsb-run         # Enter
bindkey '^?' cnmsb-delete      # Backspace
bindkey '^H' cnmsb-delete
bindkey '^W' cnmsb-delword     # Ctrl+W
bindkey '^C' cnmsb-cancel      # Ctrl+C
bindkey '^[' cnmsb-escape      # Esc

# 字符输入
local c; for c in {a..z} {A..Z} {0..9} ' ' '-' '_' '.' '/' ':' '=' '+' '@' '~' '"' "'" ',' ';' '!' '?' '*' '&' '|' '<' '>' '(' ')' '[' ']' '{' '}' '$' '#' '%' '^' '\\' '`'; do
    bindkey "$c" cnmsb-insert 2>/dev/null
done

# ================== 完成 ==================

print -P "%F{208}cnmsb%f 已加载  %F{green}↑↓%f切换 %F{green}Tab%f接受"
