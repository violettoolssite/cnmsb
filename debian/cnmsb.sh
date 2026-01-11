#!/bin/sh
# cnmsb 自动加载脚本
# 此脚本在登录时自动执行，加载 cnmsb 智能补全

# 检测当前 shell 类型并加载对应脚本
if [ -n "$BASH_VERSION" ]; then
    # Bash
    if [ -f /usr/share/cnmsb/cnmsb.bash ]; then
        . /usr/share/cnmsb/cnmsb.bash
    fi
elif [ -n "$ZSH_VERSION" ]; then
    # Zsh
    if [ -f /usr/share/cnmsb/cnmsb.zsh ]; then
        . /usr/share/cnmsb/cnmsb.zsh
    fi
fi

