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
    /// 匹配位置（用于高亮）
    pub match_indices: Vec<usize>,
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
            CompletionKind::Option => "\x1b[38;5;226m",  // 鲜艳黄色 - 选项
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
            matcher: SkimMatcherV2::default().ignore_case(),
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

            // 2. 检查是否有子命令补全
            let has_subcommand_completion = completions.iter().any(|c| matches!(c.kind, CompletionKind::Subcommand));
            
            // 3. 文件补全：只在以下情况显示
            //    - 没有子命令补全，或者
            //    - 用户明确输入了路径（以 /、.、~ 开头或包含 /），或者
            //    - 已经在子命令之后（current_word_index > 1 或已有 subcommand）
            let should_complete_files = if has_subcommand_completion {
                // 如果有子命令补全，只在用户明确输入路径时才补全文件
                parsed.current_word.starts_with('/')
                    || parsed.current_word.starts_with('.')
                    || parsed.current_word.starts_with('~')
                    || parsed.current_word.contains('/')
            } else {
                // 没有子命令补全，正常补全文件
                parsed.current_word.is_empty() 
                    || parsed.current_word.starts_with('/')
                    || parsed.current_word.starts_with('.')
                    || parsed.current_word.starts_with('~')
                    || (!parsed.current_word.starts_with('-') && (parsed.current_word_index > 1 || parsed.subcommand.is_some()))
            };
            
            if should_complete_files {
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
        let pattern_lower = pattern.to_lowercase();
        
        let mut scored: Vec<Completion> = completions
            .into_iter()
            .filter_map(|mut c| {
                let text_lower = c.text.to_lowercase();
                
                // 1. 精确匹配（大小写不敏感）
                if text_lower == pattern_lower {
                    c.score = 300;
                    c.match_indices = (0..c.text.len()).collect();
                    return Some(c);
                }
                
                // 2. 前缀匹配（大小写不敏感）
                if text_lower.starts_with(&pattern_lower) {
                    c.score = 200 + (100 - c.text.len() as i64).max(0);
                    c.match_indices = (0..pattern.len()).collect();
                    return Some(c);
                }
                
                // 3. 包含匹配（大小写不敏感）
                if let Some(pos) = text_lower.find(&pattern_lower) {
                    c.score = 150 + (50 - pos as i64).max(0);
                    c.match_indices = (pos..pos + pattern.len()).collect();
                    return Some(c);
                }
                
                // 4. 缩写匹配（首字母匹配，如 gco -> git checkout）
                if let Some((score, indices)) = self.abbreviation_match(&c.text, pattern) {
                    c.score = score;
                    c.match_indices = indices;
                    return Some(c);
                }
                
                // 5. 模糊匹配
                if let Some((score, indices)) = self.matcher.fuzzy_indices(&c.text, pattern) {
                    c.score = score;
                    c.match_indices = indices;
                    return Some(c);
                }
                
                // 6. 跳跃匹配（字符可以跳过，但顺序必须一致）
                if let Some((score, indices)) = self.subsequence_match(&c.text, pattern) {
                    c.score = score;
                    c.match_indices = indices;
                    return Some(c);
                }
                
                None
            })
            .collect();

        scored.sort_by(|a, b| b.score.cmp(&a.score));
        scored
    }

    /// 缩写匹配：检查 pattern 是否是 text 中单词首字母的缩写
    /// 例如：gco 匹配 "git checkout", sc 匹配 "systemctl"
    fn abbreviation_match(&self, text: &str, pattern: &str) -> Option<(i64, Vec<usize>)> {
        let pattern_chars: Vec<char> = pattern.to_lowercase().chars().collect();
        let text_chars: Vec<char> = text.chars().collect();
        
        if pattern_chars.is_empty() {
            return None;
        }
        
        let mut indices = Vec::new();
        let mut pattern_idx = 0;
        let mut prev_was_separator = true;
        
        for (i, &ch) in text_chars.iter().enumerate() {
            if pattern_idx >= pattern_chars.len() {
                break;
            }
            
            let is_separator = ch == '-' || ch == '_' || ch == ' ' || ch == '/';
            
            // 匹配首字母或分隔符后的第一个字符
            if prev_was_separator || ch.is_uppercase() {
                if ch.to_lowercase().next() == Some(pattern_chars[pattern_idx]) {
                    indices.push(i);
                    pattern_idx += 1;
                }
            }
            
            prev_was_separator = is_separator;
        }
        
        if pattern_idx == pattern_chars.len() {
            // 缩写匹配成功，给予较高分数
            let score = 120 + (pattern_chars.len() as i64 * 10);
            Some((score, indices))
        } else {
            None
        }
    }

    /// 子序列匹配：pattern 中的字符按顺序出现在 text 中
    fn subsequence_match(&self, text: &str, pattern: &str) -> Option<(i64, Vec<usize>)> {
        let pattern_chars: Vec<char> = pattern.to_lowercase().chars().collect();
        let text_chars: Vec<char> = text.to_lowercase().chars().collect();
        
        if pattern_chars.is_empty() {
            return None;
        }
        
        let mut indices = Vec::new();
        let mut pattern_idx = 0;
        
        for (i, &ch) in text_chars.iter().enumerate() {
            if pattern_idx >= pattern_chars.len() {
                break;
            }
            
            if ch == pattern_chars[pattern_idx] {
                indices.push(i);
                pattern_idx += 1;
            }
        }
        
        if pattern_idx == pattern_chars.len() {
            // 计算分数：匹配的字符越连续，分数越高
            let mut score = 50i64;
            let mut consecutive_bonus = 0i64;
            
            for i in 1..indices.len() {
                if indices[i] == indices[i - 1] + 1 {
                    consecutive_bonus += 10;
                }
            }
            
            // 匹配位置越靠前越好
            if !indices.is_empty() {
                score += (20 - indices[0] as i64).max(0);
            }
            
            score += consecutive_bonus;
            Some((score, indices))
        } else {
            None
        }
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
    
    /// 获取带高亮的补全文本
    pub fn highlight_match(text: &str, indices: &[usize], highlight_color: &str, reset_color: &str) -> String {
        if indices.is_empty() {
            return text.to_string();
        }
        
        let chars: Vec<char> = text.chars().collect();
        let mut result = String::new();
        let mut in_highlight = false;
        
        for (i, &ch) in chars.iter().enumerate() {
            let should_highlight = indices.contains(&i);
            
            if should_highlight && !in_highlight {
                result.push_str(highlight_color);
                in_highlight = true;
            } else if !should_highlight && in_highlight {
                result.push_str(reset_color);
                in_highlight = false;
            }
            
            result.push(ch);
        }
        
        if in_highlight {
            result.push_str(reset_color);
        }
        
        result
    }
}

impl Default for CompletionEngine {
    fn default() -> Self {
        Self::new()
    }
}
