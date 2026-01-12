//! 命令参数智能补全

use crate::database::CommandDatabase;
use crate::engine::{Completion, CompletionKind};
use crate::parser::ParsedCommand;

/// 参数补全器
pub struct ArgsCompleter {
    database: CommandDatabase,
}

impl ArgsCompleter {
    pub fn new() -> Self {
        ArgsCompleter {
            database: CommandDatabase::new(),
        }
    }

    /// 获取参数补全
    pub fn complete(&self, parsed: &ParsedCommand) -> Vec<Completion> {
        let mut completions = Vec::new();

        // 获取命令定义（支持嵌套子命令）
        let cmd_def = self.get_command_definition(parsed);

        if let Some(cmd) = cmd_def {
            let current = &parsed.current_word;
            
            // 1. 组合短选项补全（如 -zxvf, -Syu）
            if self.is_combinable_option(current) {
                self.add_combined_options(&mut completions, &cmd, current, parsed);
            }
            
            // 2. 预定义的组合选项（如 pacman 的 -Syu）
            if current.starts_with('-') || current.is_empty() {
                self.add_predefined_combinations(&mut completions, &cmd, current);
            }
            
            // 3. 常规选项补全
            if parsed.is_option || current.is_empty() || current.starts_with('-') {
                self.add_regular_options(&mut completions, &cmd, parsed);
            }

            // 4. 选项值补全
            if let Some(ref prev) = parsed.previous_word {
                self.add_option_values(&mut completions, &cmd, prev);
            }

            // 5. 子命令补全
            self.add_subcommands(&mut completions, parsed);
        }

        completions
    }
    
    /// 获取命令定义（支持嵌套子命令）
    fn get_command_definition(&self, parsed: &ParsedCommand) -> Option<crate::database::CommandDef> {
        // 检查是否有子命令
        if let Some(ref sub) = parsed.subcommand {
            // 尝试获取子命令定义
            if let Some(sub_def) = self.database.get_subcommand(&parsed.command, sub) {
                // 检查是否还有更深层的子命令
                for arg in &parsed.args {
                    if !arg.starts_with('-') && arg != sub {
                        if let Some(nested) = sub_def.subcommands.get(arg) {
                            return Some(nested.clone());
                        }
                    }
                }
                return Some(sub_def.clone());
            }
        }
        self.database.get_command(&parsed.command).cloned()
    }
    
    /// 检查是否是可组合的短选项
    fn is_combinable_option(&self, current: &str) -> bool {
        current.starts_with('-') && 
        !current.starts_with("--") && 
        current.len() > 1
    }
    
    /// 添加组合选项补全
    fn add_combined_options(
        &self, 
        completions: &mut Vec<Completion>, 
        cmd: &crate::database::CommandDef,
        current: &str,
        parsed: &ParsedCommand
    ) {
        // 提取已有的选项字符（去掉 -）
        let existing_chars: Vec<char> = current[1..].chars().collect();
        
        // 收集所有单字符选项
        let mut available_options: Vec<(char, String)> = Vec::new();
        
        for opt in &cmd.options {
            if opt.short.len() == 2 && opt.short.starts_with('-') {
                if let Some(opt_char) = opt.short.chars().nth(1) {
                    // 如果这个选项字符还没使用
                    if !existing_chars.contains(&opt_char) {
                        // 检查是否已在命令行中使用
                        let already_used = parsed.args.iter().any(|a| {
                            a == &opt.short || a == &opt.long || 
                            (a.starts_with('-') && !a.starts_with("--") && a.contains(opt_char))
                        });
                        
                        if !already_used {
                            available_options.push((opt_char, opt.description.clone()));
                        }
                    }
                }
            }
        }
        
        // 为每个可追加的选项创建补全
        for (opt_char, desc) in available_options {
            let combined = format!("{}{}", current, opt_char);
            completions.push(Completion {
                text: combined.clone(),
                description: format!("组合选项 (-{} {})", opt_char, desc),
                score: 95,
                kind: CompletionKind::Option,
                match_indices: Vec::new(),
            });
        }
        
        // 特殊处理：为常见组合模式提供建议
        self.add_common_combinations(completions, cmd, current, &existing_chars);
    }
    
    /// 添加常见的组合模式
    fn add_common_combinations(
        &self,
        completions: &mut Vec<Completion>,
        cmd: &crate::database::CommandDef,
        current: &str,
        existing_chars: &[char]
    ) {
        // tar 的常见组合
        if cmd.name == "tar" {
            let tar_combos = [
                ("xvf", "解压并显示文件"),
                ("cvf", "创建归档并显示文件"),
                ("xzvf", "解压 gzip 并显示"),
                ("czvf", "创建 gzip 归档"),
                ("xjvf", "解压 bzip2 并显示"),
                ("cjvf", "创建 bzip2 归档"),
                ("xJvf", "解压 xz 并显示"),
                ("tvf", "列出归档内容"),
            ];
            
            for (combo, desc) in tar_combos {
                if combo.starts_with(&current[1..]) && combo != &current[1..] {
                    completions.push(Completion {
                        text: format!("-{}", combo),
                        description: desc.to_string(),
                        score: 100,
                        kind: CompletionKind::Option,
                        match_indices: Vec::new(),
                    });
                }
            }
        }
        
        // pacman 的常见组合
        if cmd.name == "pacman" || cmd.name == "yay" || cmd.name == "paru" {
            let pacman_combos = [
                ("Syu", "同步数据库并升级"),
                ("Syyu", "强制刷新并升级"),
                ("Ss", "搜索包"),
                ("Si", "包信息"),
                ("Sii", "详细包信息"),
                ("Sc", "清理缓存"),
                ("Scc", "完全清理缓存"),
                ("Sw", "仅下载"),
                ("Rs", "删除包及依赖"),
                ("Rns", "删除包、依赖和配置"),
                ("Rsc", "删除包及其依赖的包"),
                ("Qi", "查询已安装包信息"),
                ("Ql", "列出包文件"),
                ("Qe", "列出明确安装的包"),
                ("Qm", "列出外部包"),
                ("Qo", "查询文件所属包"),
                ("Qs", "搜索已安装包"),
            ];
            
            for (combo, desc) in pacman_combos {
                // 检查当前输入是否是这个组合的前缀
                let current_chars = &current[1..];
                if combo.starts_with(current_chars) && combo != current_chars {
                    completions.push(Completion {
                        text: format!("-{}", combo),
                        description: desc.to_string(),
                        score: 100,
                        kind: CompletionKind::Option,
                        match_indices: Vec::new(),
                    });
                }
            }
        }
    }
    
    /// 添加预定义的组合选项
    fn add_predefined_combinations(
        &self,
        completions: &mut Vec<Completion>,
        cmd: &crate::database::CommandDef,
        current: &str
    ) {
        // 如果命令有预定义的组合选项
        if let Some(ref combos) = cmd.combinable_options {
            for combo in combos {
                if combo.starts_with(current) || current.is_empty() {
                    completions.push(Completion {
                        text: combo.clone(),
                        description: self.get_combo_description(&cmd.name, combo),
                        score: 98,
                        kind: CompletionKind::Option,
                        match_indices: Vec::new(),
                    });
                }
            }
        }
    }
    
    /// 获取组合选项的描述
    fn get_combo_description(&self, cmd_name: &str, combo: &str) -> String {
        match (cmd_name, combo) {
            ("pacman", "-Syu") | ("yay", "-Syu") | ("paru", "-Syu") => "同步并升级系统".to_string(),
            ("pacman", "-Syyu") | ("yay", "-Syyu") | ("paru", "-Syyu") => "强制刷新并升级".to_string(),
            ("pacman", "-Ss") => "搜索包".to_string(),
            ("pacman", "-Rs") => "删除包及未使用的依赖".to_string(),
            ("pacman", "-Rns") => "删除包、依赖和配置".to_string(),
            ("tar", "-xvf") => "解压并显示文件".to_string(),
            ("tar", "-cvf") => "创建归档并显示".to_string(),
            ("tar", "-xzvf") => "解压 gzip 归档".to_string(),
            ("tar", "-czvf") => "创建 gzip 归档".to_string(),
            _ => format!("组合选项 {}", combo),
        }
    }
    
    /// 添加常规选项
    fn add_regular_options(
        &self,
        completions: &mut Vec<Completion>,
        cmd: &crate::database::CommandDef,
        parsed: &ParsedCommand
    ) {
        for opt in &cmd.options {
            // 检查是否已经使用过这个选项
            let already_used = parsed.args.iter().any(|a| {
                a == &opt.short || a == &opt.long || 
                a.starts_with(&format!("{}=", opt.long)) ||
                // 检查组合选项中是否包含
                (a.starts_with('-') && !a.starts_with("--") && 
                 opt.short.len() == 2 && 
                 a.contains(opt.short.chars().nth(1).unwrap_or(' ')))
            });

            if already_used {
                continue;
            }

            // 短选项
            if !opt.short.is_empty() {
                completions.push(Completion {
                    text: opt.short.clone(),
                    description: opt.description.clone(),
                    score: 85,
                    kind: CompletionKind::Option,
                    match_indices: Vec::new(),
                });
            }

            // 长选项
            if !opt.long.is_empty() {
                completions.push(Completion {
                    text: opt.long.clone(),
                    description: opt.description.clone(),
                    score: 80,
                    kind: CompletionKind::Option,
                    match_indices: Vec::new(),
                });
            }
        }
    }
    
    /// 添加选项值补全
    fn add_option_values(
        &self,
        completions: &mut Vec<Completion>,
        cmd: &crate::database::CommandDef,
        prev: &str
    ) {
        if let Some(opt) = cmd.options.iter().find(|o| &o.short == prev || &o.long == prev) {
            if let Some(ref values) = opt.values {
                for value in values {
                    completions.push(Completion {
                        text: value.clone(),
                        description: format!("{} 的可选值", 
                            if !opt.long.is_empty() { &opt.long } else { &opt.short }),
                        score: 90,
                        kind: CompletionKind::Argument,
                        match_indices: Vec::new(),
                    });
                }
            }
        }
    }
    
    /// 添加子命令补全
    fn add_subcommands(&self, completions: &mut Vec<Completion>, parsed: &ParsedCommand) {
        // 一级子命令
        if parsed.subcommand.is_none() && 
           (parsed.current_word_index == 1 || parsed.current_word.is_empty()) {
            if let Some(subcommands) = self.database.get_subcommands(&parsed.command) {
                for (sub_name, sub_desc) in subcommands {
                    completions.push(Completion {
                        text: sub_name,
                        description: sub_desc,
                        score: 95,
                        kind: CompletionKind::Subcommand,
                        match_indices: Vec::new(),
                    });
                }
            }
        } 
        // 二级子命令
        else if let Some(ref sub) = parsed.subcommand {
            if parsed.current_word_index == 2 || 
               (parsed.current_word_index > 1 && !parsed.current_word.starts_with('-')) {
                if let Some(sub_def) = self.database.get_subcommand(&parsed.command, sub) {
                    for (sub_name, nested_def) in &sub_def.subcommands {
                        completions.push(Completion {
                            text: sub_name.clone(),
                            description: nested_def.description.clone(),
                            score: 95,
                            kind: CompletionKind::Subcommand,
                            match_indices: Vec::new(),
                        });
                    }
                }
            }
        }
    }
}

impl Default for ArgsCompleter {
    fn default() -> Self {
        Self::new()
    }
}
