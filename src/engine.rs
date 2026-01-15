//! 补全引擎核心

use crate::completions::{
    args::ArgsCompleter, 
    commands::CommandCompleter, 
    context::ContextAwareCompleter, 
    files::FileCompleter, 
    history::HistoryCompleter,
    prediction::CommandSequencePredictor,
    semantic::SemanticMatcher,
    context_analyzer::ContextAnalyzer,
    learning::LearningEngine,
    context_cache::ContextCache,
};
use crate::parser::{CommandParser, ParsedCommand};
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
    context_completer: ContextAwareCompleter,
    predictor: CommandSequencePredictor,
    semantic_matcher: SemanticMatcher,
    context_analyzer: ContextAnalyzer,
    learning_engine: LearningEngine,
    context_cache: ContextCache,
    matcher: SkimMatcherV2,
    /// 最近执行的命令（用于预测）
    recent_commands: Vec<String>,
}

impl CompletionEngine {
    pub fn new() -> Self {
        let mut engine = CompletionEngine {
            parser: CommandParser::new(),
            command_completer: CommandCompleter::new(),
            file_completer: FileCompleter::new(),
            history_completer: HistoryCompleter::new(),
            args_completer: ArgsCompleter::new(),
            context_completer: ContextAwareCompleter::new(),
            predictor: CommandSequencePredictor::new(),
            semantic_matcher: SemanticMatcher::new(),
            context_analyzer: ContextAnalyzer::new(),
            learning_engine: LearningEngine::new(),
            context_cache: ContextCache::new(),
            matcher: SkimMatcherV2::default().ignore_case(),
            recent_commands: Vec::new(),
        };
        
        // 从历史命令中提取环境变量
        let history = engine.history_completer.get_all_history();
        engine.context_completer.extract_env_vars(history);
        
        // 从历史命令中学习序列模式
        engine.predictor.learn_from_history(history);
        
        engine
    }

    /// 获取补全建议
    pub fn complete(&self, line: &str, cursor: usize) -> Vec<Completion> {
        let parsed = self.parser.parse(line, cursor);
        let mut completions = Vec::new();

        // 检查是否是命令补全位置
        // 1. 第一个词位置 (current_word_index == 0)
        // 2. 前缀命令后的第一个词位置 (如 "sudo ap" 中的 "ap")
        let words: Vec<&str> = line[..cursor.min(line.len())].split_whitespace().collect();
        let prefix_commands = ["sudo", "time", "env", "nice", "nohup", "strace", "gdb", "valgrind"];
        
        let is_command_position = if words.is_empty() {
            true  // 空输入，补全命令
        } else if words.len() == 1 {
            // 只有一个词，检查是否是前缀命令
            let first = words[0];
            prefix_commands.contains(&first) || parsed.current_word_index == 0
        } else if words.len() == 2 {
            // 两个词，检查第一个是否是前缀命令，第二个是否是命令位置
            let first = words[0];
            prefix_commands.contains(&first) && parsed.current_word_index == 1
        } else {
            parsed.current_word_index == 0
        };

        // 获取增强的上下文
        let context = self.context_analyzer.analyze_cwd_enhanced(self.recent_commands.clone());
        
        // 如果还没输入命令，或在前缀命令后补全命令，补全命令名和历史命令
        if is_command_position {
            // 1. 语义理解建议（增强版）
            if !parsed.current_word.is_empty() {
                let intent_matches = self.semantic_matcher.identify_intent_advanced(&parsed.current_word, Some(&context));
                for (i, intent_match) in intent_matches.iter().enumerate() {
                    for cmd in &intent_match.commands {
                        completions.push(Completion {
                            text: cmd.clone(),
                            description: format!("语义匹配: {} (分数: {})", intent_match.intent, intent_match.score),
                            score: 130 + (intent_match.score as i64) - (i as i64),
                            kind: CompletionKind::Command,
                            match_indices: Vec::new(),
                        });
                    }
                    
                    // 如果有参数建议，也添加
                    if intent_match.has_args {
                        let args_suggestions = self.semantic_matcher.infer_command_args(intent_match, Some(&context));
                        for (j, arg_cmd) in args_suggestions.iter().enumerate() {
                            completions.push(Completion {
                                text: arg_cmd.clone(),
                                description: format!("完整命令建议"),
                                score: 120 - (j as i64),
                                kind: CompletionKind::Command,
                                match_indices: Vec::new(),
                            });
                        }
                    }
                }
            }
            
            // 2. 预测建议（基于上一条命令）
            let predictions = self.get_predictions(&parsed);
            completions.extend(predictions);
            
            // 3. 上下文建议
            if parsed.current_word.is_empty() {
                let context_commands = self.context_analyzer.suggest_commands(&context);
                for (i, cmd) in context_commands.iter().enumerate() {
                    let cmd_name = cmd.split_whitespace().next().unwrap_or(cmd);
                    completions.push(Completion {
                        text: cmd_name.to_string(),
                        description: format!("上下文: {}", cmd),
                        score: 90 - i as i64,
                        kind: CompletionKind::Command,
                        match_indices: Vec::new(),
                    });
                }
            }
            
            // 4. 个性化建议（基于学习）
            let personalized = self.learning_engine.get_personalized_suggestions(&context);
            for (i, cmd) in personalized.iter().enumerate() {
                if parsed.current_word.is_empty() || cmd.to_lowercase().starts_with(&parsed.current_word.to_lowercase()) {
                    completions.push(Completion {
                        text: cmd.clone(),
                        description: "个性化建议".to_string(),
                        score: 85 - i as i64,
                        kind: CompletionKind::Command,
                        match_indices: Vec::new(),
                    });
                }
            }
            
            // 5. 标准命令补全
            completions.extend(self.command_completer.complete(&parsed.current_word));
            
            // 6. 历史命令补全
            if !parsed.current_word.is_empty() {
                completions.extend(self.history_completer.complete(&parsed.current_word));
            }
            
            // 7. 如果输入看起来像意图描述，添加语义匹配建议（保持向后兼容）
            if self.semantic_matcher.looks_like_intent(&parsed.current_word) {
                let semantic_commands = self.semantic_matcher.identify_intent(&parsed.current_word);
                for (i, cmd) in semantic_commands.iter().enumerate() {
                    // 检查是否已存在（避免重复）
                    if !completions.iter().any(|c| c.text == *cmd) {
                        completions.push(Completion {
                            text: cmd.clone(),
                            description: format!("语义匹配: {}", cmd),
                            score: 110 - i as i64,
                            kind: CompletionKind::Command,
                            match_indices: Vec::new(),
                        });
                    }
                }
            }
        } else {
            // 已有命令，根据上下文补全参数/选项
            
            // 0. 上下文感知补全（优先，特别是 export 命令）
            if self.context_completer.is_export_command(&parsed) {
                let context_completions = self.context_completer.complete_env_var(&parsed);
                completions.extend(context_completions);
            }
            
            // 检查是否是文件操作命令（需要文件补全的命令）
            let file_operation_commands = [
                "touch", "cat", "less", "head", "tail", "more", "bat",
                "ls", "cd", "cp", "mv", "rm", "mkdir", "rmdir",
                "chmod", "chown", "grep", "find", "locate",
                "vim", "vi", "nano", "emacs", "code",
                "tar", "zip", "unzip", "gzip", "gunzip",
                "python", "python3", "node", "bash", "sh",
            ];
            let is_file_operation = file_operation_commands.contains(&parsed.command.as_str());
            
            // 1. 参数/选项补全（如果不是文件操作命令或已输入路径）
            if !is_file_operation || parsed.current_word.starts_with('-') {
                completions.extend(self.args_completer.complete(&parsed));
            }

            // 2. 检查是否有子命令补全
            let has_subcommand_completion = completions.iter().any(|c| matches!(c.kind, CompletionKind::Subcommand));
            
            // 3. 文件补全：优先显示文件补全（特别是文件操作命令）
            let should_complete_files = if is_file_operation {
                // 文件操作命令：优先显示文件补全
                // 如果当前词是路径（包含 /、.、~ 开头），或者为空，或者不是以 - 开头
                parsed.current_word.is_empty()
                    || parsed.current_word.starts_with('/')
                    || parsed.current_word.starts_with('.')
                    || parsed.current_word.starts_with('~')
                    || parsed.current_word.contains('/')
                    || (!parsed.current_word.starts_with('-') && parsed.current_word_index >= 1)
            } else if has_subcommand_completion {
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
                let file_completions = self.file_completer.complete(&parsed.current_word);
                // 如果是文件操作命令，提高文件补全的优先级
                let file_completions: Vec<Completion> = if is_file_operation {
                    file_completions.into_iter().map(|mut c| {
                        c.score += 20; // 提高文件补全的优先级
                        c
                    }).collect()
                } else {
                    file_completions
                };
                completions.extend(file_completions);
            }
            
            // 不在参数位置显示历史命令（太干扰了）
        }

        // 模糊匹配过滤和排序
        if !parsed.current_word.is_empty() {
            completions = self.filter_and_rank(completions, &parsed.current_word);
        }
        
        // 个性化排序（基于学习引擎）
        let mut completions_mut = completions;
        let context = self.context_analyzer.analyze_cwd_enhanced(self.recent_commands.clone());
        self.learning_engine.personalize_ranking(&mut completions_mut, &context);

        // 去重并限制数量
        self.deduplicate_and_limit(completions_mut, 20)
    }

    /// 获取预测建议（增强版：基于上一条命令的上下文）
    fn get_predictions(&self, parsed: &ParsedCommand) -> Vec<Completion> {
        let mut predictions = Vec::new();
        
        // 获取上一条命令（从历史中）
        let last_command = self.get_last_command_from_history();
        
        // 1. 命令序列预测（基于上一条命令）- 使用智能过滤
        if let Some(ref last_cmd) = last_command {
            let next_commands = self.predictor.predict_next_filtered(last_cmd, &parsed.current_word);
            for (i, cmd) in next_commands.iter().enumerate() {
                predictions.push(Completion {
                    text: cmd.clone(),
                    description: format!("预测: {} 后常用", last_cmd),
                    score: 120 - i as i64, // 提高优先级
                    kind: CompletionKind::Command,
                    match_indices: Vec::new(),
                });
            }
        }
        
        // 2. 上下文预测（基于工作目录和项目类型）
        let context = self.context_analyzer.analyze_cwd();
        let context_commands = self.context_analyzer.suggest_commands(&context);
        for (i, cmd) in context_commands.iter().enumerate() {
            // 提取命令名（第一个词）
            let cmd_name = cmd.split_whitespace().next().unwrap_or(cmd);
            
            // 如果用户已输入字符，检查是否匹配
            if !parsed.current_word.is_empty() {
                if !cmd_name.to_lowercase().starts_with(&parsed.current_word.to_lowercase()) {
                    continue; // 不匹配用户输入，跳过
                }
            }
            
            predictions.push(Completion {
                text: cmd_name.to_string(),
                description: format!("上下文建议: {}", cmd),
                score: 110 - i as i64,
                kind: CompletionKind::Command,
                match_indices: Vec::new(),
            });
        }
        
        // 3. 基于工作目录的预测
        let cwd = context.cwd.clone();
        let recent: Vec<String> = self.recent_commands.iter().take(5).cloned().collect();
        let context_predictions = self.predictor.predict_from_context(&cwd, &recent);
        for (i, cmd) in context_predictions.iter().enumerate() {
            // 如果用户已输入字符，检查是否匹配
            if !parsed.current_word.is_empty() {
                if !cmd.to_lowercase().starts_with(&parsed.current_word.to_lowercase()) {
                    continue; // 不匹配用户输入，跳过
                }
            }
            
            predictions.push(Completion {
                text: cmd.clone(),
                description: "基于工作目录预测".to_string(),
                score: 100 - i as i64,
                kind: CompletionKind::Command,
                match_indices: Vec::new(),
            });
        }
        
        // 4. 如果用户已输入字符，添加语义匹配（结合上一条命令的上下文）
        if !parsed.current_word.is_empty() && last_command.is_some() {
            let semantic_commands = self.semantic_matcher.identify_intent(&parsed.current_word);
            for (i, cmd) in semantic_commands.iter().enumerate() {
                if cmd.to_lowercase().starts_with(&parsed.current_word.to_lowercase()) {
                    predictions.push(Completion {
                        text: cmd.clone(),
                        description: format!("语义匹配（基于上下文）"),
                        score: 115 - i as i64,
                        kind: CompletionKind::Command,
                        match_indices: Vec::new(),
                    });
                }
            }
        }
        
        predictions
    }
    
    /// 从历史中获取上一条命令
    fn get_last_command_from_history(&self) -> Option<String> {
        // 优先使用最近执行的命令（从 recent_commands）
        if let Some(last) = self.recent_commands.last() {
            return Some(last.clone());
        }
        
        // 如果没有，从历史命令中获取
        if let Some(last) = self.history_completer.get_last_command() {
            return Some(last);
        }
        
        None
    }

    /// 获取最后一个执行的命令
    fn get_last_command(&self) -> Option<String> {
        self.recent_commands.last().cloned()
    }

    /// 记录命令执行（用于学习）
    pub fn record_command(&mut self, command: &str) {
        let cmd = command.trim().to_string();
        if !cmd.is_empty() {
            // 学习命令序列（如果上一条命令存在）
            if let Some(last_cmd) = self.recent_commands.last() {
                self.predictor.learn_sequence(last_cmd, &cmd);
                
                // 学习命令模式
                let sequence = vec![last_cmd.clone(), cmd.clone()];
                self.learning_engine.learn_pattern(sequence);
            }
            
            // 添加到最近命令列表
            self.recent_commands.push(cmd.clone());
            // 限制最近命令数量
            if self.recent_commands.len() > 10 {
                self.recent_commands.remove(0);
            }
            
            // 记录到预测器（用于上下文学习）
            if let Ok(cwd) = std::env::current_dir() {
                let cwd_str = cwd.to_string_lossy().to_string();
                self.predictor.record_command_in_context(&cwd_str, &cmd);
                
                // 更新上下文缓存
                let context = self.context_analyzer.analyze_cwd_enhanced(self.recent_commands.clone());
                self.context_cache.set_context(&cwd_str, context);
            }
            
            // 学习用户选择（用于个性化）
            let context = self.context_analyzer.analyze_cwd();
            self.learning_engine.learn_from_selection(&cmd, &context);
            
            // 定期保存学习数据
            if self.recent_commands.len() % 5 == 0 {
                self.predictor.save_data();
            }
        }
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
