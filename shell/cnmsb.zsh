#!/bin/zsh
# cnmsb - Zsh 智能补全
# https://github.com/violettoolssite/cnmsb

(( $+commands[cnmsb] )) || return 0

autoload -Uz compinit && compinit -u -C

# 禁用 ? 作为通配符（避免 "no matches found" 错误）
setopt nonomatch

# 禁用特定命令的默认补全，让 cnmsb 接管
# sudo 及其它前缀命令
compdef -d sudo
compdef -d time
compdef -d env
compdef -d nice
compdef -d nohup
compdef -d strace
compdef -d gdb
compdef -d valgrind

# ================== 配置 ==================

# 主题设置控制
# 设置 CNMSB_NO_THEME=1 可禁用 cnmsb 主题
# 设置 CNMSB_FORCE_THEME=1 可强制使用 cnmsb 主题
_cnmsb_setup_theme() {
    # 如果用户明确禁用主题，直接返回
    [[ "$CNMSB_NO_THEME" == "1" ]] && return
    
    # 如果用户强制使用主题，直接设置
    if [[ "$CNMSB_FORCE_THEME" == "1" ]]; then
        PS1='%F{208}%n@%m%f:%F{51}%~%f%F{208}%%%f '
        return
    fi
    
    # 检测用户是否有自定义主题
    local has_custom_theme=0
    
    # 检测 oh-my-zsh
    [[ -n "$ZSH_THEME" ]] && has_custom_theme=1
    
    # 检测 powerlevel10k
    [[ -n "$POWERLEVEL9K_MODE" || -f ~/.p10k.zsh ]] && has_custom_theme=1
    
    # 检测 starship
    command -v starship >/dev/null 2>&1 && [[ "$PROMPT" == *"starship"* || -f ~/.config/starship.toml ]] && has_custom_theme=1
    
    # 检测 prezto
    [[ -n "$ZPREZTODIR" ]] && has_custom_theme=1
    
    # 检测 PS1/PROMPT 是否已被自定义（非默认值）
    if [[ -n "$PS1" && "$PS1" != "%m%# " && "$PS1" != "%n@%m %~ %# " ]]; then
        has_custom_theme=1
    fi
    
    if [[ $has_custom_theme -eq 1 ]]; then
        # 检查是否已经询问过用户（保存在文件中）
        local config_file="$HOME/.config/cnmsb/theme_choice"
        
        if [[ -f "$config_file" ]]; then
            local choice=$(cat "$config_file" 2>/dev/null)
            if [[ "$choice" == "yes" ]]; then
                PS1='%F{208}%n@%m%f:%F{51}%~%f%F{208}%%%f '
            fi
            # 如果是 "no" 则保留原主题
            return
        fi
        
        # 首次检测到自定义主题，询问用户
        echo ""
        echo -e "\033[38;5;208m[cnmsb]\033[0m 检测到您已有自定义 zsh 主题。"
        echo -e "是否要使用 cnmsb 主题覆盖? (y=覆盖 / n=保留原主题 / a=总是保留 / f=总是覆盖)"
        echo -n "请选择 [y/n/a/f]: "
        
        # 读取用户输入
        local user_choice
        read -r user_choice
        
        # 创建配置目录
        mkdir -p "$HOME/.config/cnmsb"
        
        case "$user_choice" in
            [Yy])
                PS1='%F{208}%n@%m%f:%F{51}%~%f%F{208}%%%f '
                ;;
            [Nn])
                # 保留原主题，不做任何事
                ;;
            [Aa])
                # 总是保留原主题
                echo "no" > "$config_file"
                echo -e "\033[32m已保存选择：保留原主题\033[0m"
                echo -e "提示：可通过删除 ~/.config/cnmsb/theme_choice 重置此选择"
                ;;
            [Ff])
                # 总是使用 cnmsb 主题
                echo "yes" > "$config_file"
                PS1='%F{208}%n@%m%f:%F{51}%~%f%F{208}%%%f '
                echo -e "\033[32m已保存选择：使用 cnmsb 主题\033[0m"
                echo -e "提示：可通过删除 ~/.config/cnmsb/theme_choice 重置此选择"
                ;;
            *)
                # 默认保留原主题
                echo -e "\033[33m未识别的选择，保留原主题\033[0m"
                ;;
        esac
    else
        # 没有自定义主题，使用 cnmsb 默认主题
        PS1='%F{208}%n@%m%f:%F{51}%~%f%F{208}%%%f '
    fi
}

# 执行主题设置
_cnmsb_setup_theme

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
        
        # 计算后缀（用于内联显示）
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
        
        # 检测中文输入（可能是意图描述），自动触发语义匹配
        local input="$BUFFER"
        if [[ -n "$input" ]]; then
            local words=(${(z)input})
            local first_word="${words[1]}"
            
            # 如果第一个词不是有效命令，尝试语义匹配
            if [[ -n "$first_word" ]] && ! command -v "$first_word" >/dev/null 2>&1; then
                # 检查是否包含非ASCII字符（可能是中文或其他语言）
                # 使用字符范围检测：ASCII 可打印字符范围是 0x20-0x7E
                if [[ "$first_word" == *[^$'\x20'-$'\x7e']* ]]; then
                    # 包含非ASCII字符，可能是意图描述，自动显示建议菜单
                    _cnmsb_fetch "$input"
                    if [[ ${#_cnmsb_list[@]} -gt 0 ]]; then
                        # 有语义匹配建议，自动显示菜单
                        _cnmsb_menu=1
                        _cnmsb_hist_mode=0
                        _cnmsb_idx=1
                        _cnmsb_show_menu
                        return
                    fi
                fi
            fi
        fi
        
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

# 新行初始化 - 清除所有显示
_cnmsb_line_init() {
    POSTDISPLAY=""
    region_highlight=()
    _cnmsb_list=() _cnmsb_desc=() _cnmsb_suff=()
    _cnmsb_idx=0 _cnmsb_menu=0 _cnmsb_hist_mode=0 _cnmsb_lastbuf="" _cnmsb_skip=0
}

zle -N zle-line-init _cnmsb_line_init

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
            local selected="${_cnmsb_list[$_cnmsb_idx]}"
            
            # 历史模式：直接替换整个 BUFFER（因为历史命令是完整的命令）
            if [[ $_cnmsb_hist_mode -eq 1 ]]; then
                BUFFER="$selected"
                CURSOR=${#BUFFER}
            else
                # 普通补全模式：智能追加/替换
                # 获取当前词和位置
                local words=(${(z)BUFFER})
                local curword=""
                local curword_start=0
                
                if [[ "$BUFFER" != *" " && ${#words[@]} -gt 0 ]]; then
                    curword="${words[-1]}"
                    curword_start=$((${#BUFFER} - ${#curword}))
                fi
                
                if [[ -n "$curword" ]]; then
                    if [[ "$selected" == "$curword"* ]]; then
                        # 前缀匹配：追加后缀部分
                        BUFFER+="${selected#$curword}"
                    else
                        # 模糊匹配：用子串方式替换
                        BUFFER="${BUFFER[1,$curword_start]}${selected}"
                    fi
                else
                    # 没有当前词，直接追加
                    BUFFER+="$selected"
                fi
                CURSOR=${#BUFFER}
            fi
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
            # 检查是否是前缀命令（sudo, time, env 等）
            local words=(${(z)BUFFER})
            local first_word="${words[1]}"
            local prefix_commands=("sudo" "time" "env" "nice" "nohup" "strace" "gdb" "valgrind")
            
            # 如果是前缀命令，不调用默认补全，直接返回
            if [[ -n "$first_word" && " ${prefix_commands[@]} " =~ " $first_word " ]]; then
                # 前缀命令，使用我们的补全系统（即使没有结果也不调用默认补全）
                zle -R
                return
            fi
            
            # 其他情况，如果没有补全结果，使用默认补全
            zle expand-or-complete
        fi
    fi
    zle -R
}

_cnmsb_accept() {
    _cnmsb_skip=1
    if [[ ($_cnmsb_menu -eq 1 || $_cnmsb_hist_mode -eq 1 || ${#_cnmsb_list[@]} -gt 0) && $_cnmsb_idx -gt 0 ]]; then
        local selected="${_cnmsb_list[$_cnmsb_idx]}"
        
        # 历史模式：直接替换整个 BUFFER（因为历史命令是完整的命令）
        if [[ $_cnmsb_hist_mode -eq 1 ]]; then
            BUFFER="$selected"
            CURSOR=${#BUFFER}
        else
            # 普通补全模式：智能追加/替换
            # 获取当前词和位置
            local words=(${(z)BUFFER})
            local curword=""
            local curword_start=0
            
            if [[ "$BUFFER" != *" " && ${#words[@]} -gt 0 ]]; then
                curword="${words[-1]}"
                curword_start=$((${#BUFFER} - ${#curword}))
            fi
            
            if [[ -n "$curword" ]]; then
                if [[ "$selected" == "$curword"* ]]; then
                    # 前缀匹配：追加后缀部分
                    BUFFER+="${selected#$curword}"
                else
                    # 模糊匹配：用子串方式替换
                    BUFFER="${BUFFER[1,$curword_start]}${selected}"
                fi
            else
                # 没有当前词，直接追加
                BUFFER+="$selected"
            fi
            CURSOR=${#BUFFER}
        fi
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
    # 检查输入是否是意图描述（不是有效命令）
    local input="$BUFFER"
    
    # 检查是否是有效的命令（第一个词在 PATH 中）
    local words=(${(z)input})
    local first_word="${words[1]}"
    
    # 如果第一个词不是有效命令，尝试语义匹配
    if [[ -n "$first_word" ]] && ! command -v "$first_word" >/dev/null 2>&1; then
        # 检查是否包含非ASCII字符（可能是中文或其他语言）
        # 使用字符范围检测：ASCII 可打印字符范围是 0x20-0x7E
        if [[ "$first_word" == *[^$'\x20'-$'\x7e']* ]]; then
            # 包含非ASCII字符，可能是意图描述
            # 触发补全建议，而不是直接执行
            _cnmsb_fetch "$input"
            if [[ ${#_cnmsb_list[@]} -gt 0 ]]; then
                # 有建议，显示菜单让用户选择
                _cnmsb_menu=1
                _cnmsb_idx=1
                _cnmsb_show_menu
                zle -R
                return
            fi
        fi
    fi
    
    # 清除所有显示和状态
    _cnmsb_list=() _cnmsb_desc=() _cnmsb_suff=()
    _cnmsb_idx=0 _cnmsb_menu=0 _cnmsb_hist_mode=0 _cnmsb_lastbuf=""
    
    # 清除 POSTDISPLAY（建议文字）
    POSTDISPLAY=""
    region_highlight=()
    
    # 方法：清除当前行并重新打印干净的命令
    local cmd="$BUFFER"
    
    # 清除当前行（回到行首，清除到行尾）
    print -rn -- $'\r\e[K'
    
    # 重新打印提示符（使用 -P 展开 % 格式化）和命令（不带建议）
    print -Prn -- "${PS1}"
    print -rn -- "${cmd}"
    
    # 清除光标后的任何残留
    print -rn -- $'\e[K\e[J'
    
    # 执行命令
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

# ================== 命令记录（用于 NLP 预测） ==================

# 记录命令执行（用于学习命令序列）
_cnmsb_record_command() {
    local cmd="$1"
    [[ -z "$cmd" ]] && return
    
    # 调用 cnmsb 记录命令（后台执行，不阻塞）
    (cnmsb record "$cmd" 2>/dev/null &)
}

# 在命令执行前记录（preexec 钩子）
preexec_functions+=(_cnmsb_record_command)

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
