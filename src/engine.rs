//! 补全引擎核心

use crate::completions::{args::ArgsCompleter, commands::CommandCompleter, files::FileCompleter, history::HistoryCompleter};
use crate::parser::CommandParser;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

/// 补全建议
#[derive(Debug, Clone)]
pub struct Completion {
    /// 补全文本
    pub text: String,
    /// 描述信息
    pub description: String,
    /// 匹配分数（用于排序）
    pub score: i64,
    /// 补全类型
    pub kind: CompletionKind,
}

/// 补全类型
#[derive(Debug, Clone, PartialEq)]
pub enum CompletionKind {
    Command,
    Subcommand,
    Option,
    Argument,
    File,
    Directory,
    History,
}

impl CompletionKind {
    /// 获取类型对应的颜色代码
    pub fn color(&self) -> &'static str {
        match self {
            CompletionKind::Command => "\x1b[32m",      // 绿色 - 命令
            CompletionKind::Subcommand => "\x1b[36m",   // 青色 - 子命令
            CompletionKind::Option => "\x1b[33m",       // 黄色 - 选项
            CompletionKind::Argument => "\x1b[35m",     // 紫色 - 参数值
            CompletionKind::File => "\x1b[0m",          // 默认 - 文件
            CompletionKind::Directory => "\x1b[34;1m", // 蓝色粗体 - 目录
            CompletionKind::History => "\x1b[38;5;245m", // 灰色 - 历史
        }
    }
    
    /// 获取类型标签
    pub fn label(&self) -> &'static str {
        match self {
            CompletionKind::Command => "命令",
            CompletionKind::Subcommand => "子命令",
            CompletionKind::Option => "选项",
            CompletionKind::Argument => "参数",
            CompletionKind::File => "文件",
            CompletionKind::Directory => "目录",
            CompletionKind::History => "历史",
        }
    }
}

/// 补全引擎
pub struct CompletionEngine {
    parser: CommandParser,
    command_completer: CommandCompleter,
    file_completer: FileCompleter,
    history_completer: HistoryCompleter,
    args_completer: ArgsCompleter,
    matcher: SkimMatcherV2,
}

impl CompletionEngine {
    pub fn new() -> Self {
        CompletionEngine {
            parser: CommandParser::new(),
            command_completer: CommandCompleter::new(),
            file_completer: FileCompleter::new(),
            history_completer: HistoryCompleter::new(),
            args_completer: ArgsCompleter::new(),
            matcher: SkimMatcherV2::default(),
        }
    }

    /// 获取补全建议
    pub fn complete(&self, line: &str, cursor: usize) -> Vec<Completion> {
        let parsed = self.parser.parse(line, cursor);
        let mut completions = Vec::new();

        // 如果还没输入命令，补全命令名和历史命令
        if parsed.current_word_index == 0 {
            completions.extend(self.command_completer.complete(&parsed.current_word));
            // 只在命令位置显示历史命令
            completions.extend(self.history_completer.complete(&parsed.current_word));
        } else {
            // 已有命令，根据上下文补全参数/选项
            
            // 1. 参数/选项补全（优先）
            completions.extend(self.args_completer.complete(&parsed));

            // 2. 如果当前词看起来像路径，补全文件
            if parsed.current_word.is_empty() 
                || parsed.current_word.starts_with('/')
                || parsed.current_word.starts_with('.')
                || parsed.current_word.starts_with('~')
                || !parsed.current_word.starts_with('-') {
                completions.extend(self.file_completer.complete(&parsed.current_word));
            }
            
            // 不在参数位置显示历史命令（太干扰了）
        }

        // 模糊匹配过滤和排序
        if !parsed.current_word.is_empty() {
            completions = self.filter_and_rank(completions, &parsed.current_word);
        }

        // 去重并限制数量
        self.deduplicate_and_limit(completions, 20)
    }

    /// 模糊匹配过滤和排序
    fn filter_and_rank(&self, completions: Vec<Completion>, pattern: &str) -> Vec<Completion> {
        let mut scored: Vec<Completion> = completions
            .into_iter()
            .filter_map(|mut c| {
                // 前缀匹配优先（确保组合参数如 -zc 能匹配 -z）
                if c.text.starts_with(pattern) {
                    // 精确匹配得最高分
                    if c.text == pattern {
                        c.score = 200;
                    } else {
                        // 前缀匹配
                        c.score = 150;
                    }
                    Some(c)
                } else if let Some(score) = self.matcher.fuzzy_match(&c.text, pattern) {
                    c.score = score;
                    Some(c)
                } else {
                    None
                }
            })
            .collect();

        scored.sort_by(|a, b| b.score.cmp(&a.score));
        scored
    }

    /// 去重并限制数量
    fn deduplicate_and_limit(&self, completions: Vec<Completion>, limit: usize) -> Vec<Completion> {
        let mut seen = std::collections::HashSet::new();
        completions
            .into_iter()
            .filter(|c| seen.insert(c.text.clone()))
            .take(limit)
            .collect()
    }
}

impl Default for CompletionEngine {
    fn default() -> Self {
        Self::new()
    }
}

