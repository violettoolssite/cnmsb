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

        // 获取命令定义
        let cmd_def = if let Some(ref sub) = parsed.subcommand {
            // 先尝试获取子命令的定义
            self.database
                .get_subcommand(&parsed.command, sub)
                .or_else(|| self.database.get_command(&parsed.command))
        } else {
            self.database.get_command(&parsed.command)
        };

        if let Some(cmd) = cmd_def {
            // 如果正在输入选项（以 - 开头）
            if parsed.is_option || parsed.current_word.is_empty() {
                for opt in &cmd.options {
                    // 检查是否已经使用过这个选项
                    let already_used = parsed.args.iter().any(|a| {
                        a == &opt.short || a == &opt.long || a.starts_with(&format!("{}=", opt.long))
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
                        });
                    }

                    // 长选项
                    if !opt.long.is_empty() {
                        completions.push(Completion {
                            text: opt.long.clone(),
                            description: opt.description.clone(),
                            score: 80,
                            kind: CompletionKind::Option,
                        });
                    }
                }
            }

            // 如果前一个词是需要参数的选项
            if let Some(ref prev) = parsed.previous_word {
                if let Some(opt) = cmd.options.iter().find(|o| &o.short == prev || &o.long == prev)
                {
                    if let Some(ref values) = opt.values {
                        for value in values {
                            completions.push(Completion {
                                text: value.clone(),
                                description: format!("{} 的值", opt.long),
                                score: 90,
                                kind: CompletionKind::Argument,
                            });
                        }
                    }
                }
            }

            // 子命令补全（如果命令有子命令且当前是第一个参数）
            if parsed.subcommand.is_none() && parsed.current_word_index == 1 {
                if let Some(subcommands) = self.database.get_subcommands(&parsed.command) {
                    for (sub_name, sub_desc) in subcommands {
                        completions.push(Completion {
                            text: sub_name,
                            description: sub_desc,
                            score: 95,
                            kind: CompletionKind::Subcommand,
                        });
                    }
                }
            }
        }

        completions
    }
}

impl Default for ArgsCompleter {
    fn default() -> Self {
        Self::new()
    }
}

