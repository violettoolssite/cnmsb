#!/bin/zsh
# cnmsb - Zsh 智能补全

(( $+commands[cnmsb] )) || return 0

autoload -Uz compinit && compinit -u -C

# ================== 提示符 ==================

PS1='%F{208}%n@%m%f:%F{blue}%~%f%F{208}%%%f '

# ================== 输入颜色 ==================

zle-line-init() { echoti smkx 2>/dev/null; zle reset-prompt; }
zle-line-finish() { echoti rmkx 2>/dev/null; }
zle -N zle-line-init
zle -N zle-line-finish

typeset -g zle_highlight=(default:fg=11,bold)

# ================== 建议列表 ==================

typeset -ga _clist=()      # 建议列表 (完整文本)
typeset -ga _cdesc=()      # 描述列表
typeset -ga _csuff=()      # 后缀列表 (要追加的部分)
typeset -g _cidx=0         # 当前选中索引
typeset -g _csug=""        # 当前要追加的后缀

# 获取所有建议
_cfetch() {
    _clist=()
    _cdesc=()
    _csuff=()
    _cidx=0
    _csug=""
    
    local buf="$1"
    [[ -z "$buf" ]] && return
    
    local comps
    comps=$(cnmsb complete --line "$buf" --cursor ${#buf} --shell bash 2>/dev/null)
    [[ -z "$comps" ]] && return
    
    # 获取当前词
    local words=(${(z)buf})
    local curword=""
    if [[ "$buf" != *" " && ${#words[@]} -gt 0 ]]; then
        curword="${words[-1]}"
    fi
    
    while IFS=$'\t' read -r text desc; do
        [[ -z "$text" ]] && continue
        
        # 计算后缀
        local suf=""
        if [[ -n "$curword" && "$text" == "$curword"* ]]; then
            # 补全当前词
            suf="${text#$curword}"
        elif [[ -z "$curword" || "$buf" == *" " ]]; then
            # 新词
            suf="$text"
        else
            # 替换当前词
            suf="$text"
        fi
        
        _clist+=("$text")
        _cdesc+=("$desc")
        _csuff+=("$suf")
    done <<< "$comps"
    
    [[ ${#_clist[@]} -gt 0 ]] && _cidx=1
}

# 显示当前建议
_cshow() {
    POSTDISPLAY=""
    region_highlight=()
    _csug=""
    
    if [[ ${#_clist[@]} -eq 0 || $_cidx -eq 0 ]]; then
        return
    fi
    
    local suf="${_csuff[$_cidx]}"
    local desc="${_cdesc[$_cidx]}"
    local comp="${_clist[$_cidx]}"
    
    _csug="$suf"
    
    # 显示: 后缀 [索引/总数] 描述
    local display="$suf"
    if [[ ${#_clist[@]} -gt 1 ]]; then
        display+="  [${_cidx}/${#_clist[@]}]"
    fi
    if [[ -n "$desc" ]]; then
        display+=" $desc"
    fi
    
    POSTDISPLAY="$display"
    local start=${#BUFFER}
    local end=$((start + ${#POSTDISPLAY}))
    region_highlight+=("$start $end fg=240")
}

# 更新建议
_cupd() {
    _cfetch "$BUFFER"
    _cshow
}

# 上一个建议
_cprev() {
    if [[ ${#_clist[@]} -gt 1 ]]; then
        (( _cidx-- ))
        [[ $_cidx -lt 1 ]] && _cidx=${#_clist[@]}
        _cshow
    elif [[ ${#_clist[@]} -eq 0 ]]; then
        zle .up-line-or-history
    fi
}

# 下一个建议
_cnext() {
    if [[ ${#_clist[@]} -gt 1 ]]; then
        (( _cidx++ ))
        [[ $_cidx -gt ${#_clist[@]} ]] && _cidx=1
        _cshow
    elif [[ ${#_clist[@]} -eq 0 ]]; then
        zle .down-line-or-history
    fi
}

# 接受当前建议
_cacc() {
    if [[ -n "$_csug" ]]; then
        BUFFER+="$_csug"
        CURSOR=${#BUFFER}
        _clist=()
        _cdesc=()
        _csuff=()
        _cidx=0
        _csug=""
        POSTDISPLAY=""
        region_highlight=()
        # 获取下一级建议
        _cupd
    else
        POSTDISPLAY=""
        region_highlight=()
        zle expand-or-complete
    fi
}

# 输入字符
_cins() { zle .self-insert; _cupd; }

# 删除
_cdel() { zle .backward-delete-char; _cupd; }
_cdelw() { zle .backward-kill-word; _cupd; }

# 执行
_crun() { 
    POSTDISPLAY=""
    region_highlight=()
    _clist=(); _cdesc=(); _csuff=(); _cidx=0; _csug=""
    zle .accept-line
}

# 取消
_cbrk() { 
    POSTDISPLAY=""
    region_highlight=()
    _clist=(); _cdesc=(); _csuff=(); _cidx=0; _csug=""
    BUFFER=""
    zle .redisplay
}

# 清除建议但保留输入
_cesc() {
    POSTDISPLAY=""
    region_highlight=()
    _clist=(); _cdesc=(); _csuff=(); _cidx=0; _csug=""
    zle .redisplay
}

# 注册小部件
zle -N cins _cins
zle -N cdel _cdel
zle -N cdelw _cdelw
zle -N cacc _cacc
zle -N crun _crun
zle -N cbrk _cbrk
zle -N cprev _cprev
zle -N cnext _cnext
zle -N cesc _cesc

# 按键绑定
bindkey '^I' cacc           # Tab: 接受
bindkey '^[[C' cacc         # 右箭头: 接受
bindkey '^[[A' cprev        # 上箭头: 上一个
bindkey '^[[B' cnext        # 下箭头: 下一个
bindkey '^M' crun           # Enter: 执行
bindkey '^?' cdel           # Backspace
bindkey '^H' cdel
bindkey '^C' cbrk           # Ctrl+C: 取消
bindkey '^W' cdelw          # Ctrl+W
bindkey '^[' cesc           # Esc: 关闭建议

# 字符输入
for c ({a..z} {A..Z} {0..9} ' ' '-' '_' '.' '/' ':' '=' '+' '@' '~'); do
    bindkey "$c" cins 2>/dev/null
done

# ================== 补全样式 ==================

zstyle ':completion:*' menu select
zstyle ':completion:*' list-colors 'ma=30;43'

print -P "%F{208}cnmsb%f 已加载"
print -P "  %F{green}↑↓%f 切换建议  %F{green}Tab/→%f 接受  %F{green}Esc%f 关闭"
