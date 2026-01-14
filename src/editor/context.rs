//! 编辑器上下文感应补全
//!
//! 分析当前文件内容，提供智能上下文补全建议
//! 
//! 改进方案：
//! 1. 多行上下文分析（不仅仅是当前行）
//! 2. 语义理解（识别函数、变量、命令模式）
//! 3. 智能缓存（避免重复分析）
//! 4. 模式匹配（支持多种 shell 脚本模式）

use regex::Regex;
use std::collections::{HashMap, HashSet};
use super::nlp::{NLPAnalyzer, UserIntent};

/// 编辑器上下文分析器
pub struct EditorContext {
    /// 环境变量映射
    env_vars: HashMap<String, String>,
    /// 已定义的变量名（包括普通变量）
    defined_vars: Vec<String>,
    /// 函数名列表
    functions: HashSet<String>,
    /// 当前行的上下文
    current_line_context: LineContext,
    /// export 命令解析器
    export_regex: Regex,
    /// 变量赋值解析器（VAR=value）
    var_assign_regex: Regex,
    /// 函数定义解析器
    function_regex: Regex,
    /// 变量引用解析器（$VAR 或 ${VAR}）
    var_ref_regex: Regex,
    /// 缓存：文件内容的哈希（用于判断是否需要重新分析）
    file_hash: u64,
    /// 自然语言分析器
    nlp_analyzer: NLPAnalyzer,
}

/// 行上下文信息
#[derive(Debug, Clone)]
pub struct LineContext {
    /// 是否是 export 命令
    pub is_export: bool,
    /// 变量名（如果是 export VAR=...）
    pub var_name: Option<String>,
    /// 当前输入的前缀
    pub prefix: String,
    /// 行号
    pub line_number: usize,
}

impl EditorContext {
    /// 创建新的上下文分析器
    pub fn new() -> Self {
        // 支持大小写混合的变量名（如 java_home, JAVA_HOME, Path 等）
        let export_regex = Regex::new(r"^\s*export\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*(.*)$").unwrap();
        let var_assign_regex = Regex::new(r"^\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*(.*)$").unwrap();
        let function_regex = Regex::new(r"^\s*function\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(|^\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\(\s*\)\s*\{").unwrap();
        let var_ref_regex = Regex::new(r"\$\{?([a-zA-Z_][a-zA-Z0-9_]*)\}?").unwrap();
        
        EditorContext {
            env_vars: HashMap::new(),
            defined_vars: Vec::new(),
            functions: HashSet::new(),
            current_line_context: LineContext {
                is_export: false,
                var_name: None,
                prefix: String::new(),
                line_number: 0,
            },
            export_regex,
            var_assign_regex,
            function_regex,
            var_ref_regex,
            file_hash: 0,
            nlp_analyzer: NLPAnalyzer::new(),
        }
    }

    /// 分析文件内容，提取上下文信息（带缓存优化）
    pub fn analyze_file(&mut self, lines: &[String], current_line: usize, current_col: usize) {
        // 计算文件内容的简单哈希（用于缓存）
        let new_hash = self.compute_file_hash(lines);
        
        // 如果文件内容没有变化，只更新当前行上下文
        if new_hash == self.file_hash && !lines.is_empty() {
            if current_line < lines.len() {
                let line = &lines[current_line];
                self.analyze_current_line(line, current_col, current_line);
            }
            return;
        }
        
        // 文件内容变化，重新分析
        self.file_hash = new_hash;
        self.env_vars.clear();
        self.defined_vars.clear();
        self.functions.clear();
        
        // 分析所有行，提取上下文信息
        for (_i, line) in lines.iter().enumerate() {
            // 提取 export 变量
            if let Some((var_name, var_value)) = self.parse_export_line(line) {
                self.env_vars.insert(var_name.clone(), var_value);
                if !self.defined_vars.contains(&var_name) {
                    self.defined_vars.push(var_name);
                }
            }
            
            // 提取普通变量赋值（VAR=value）
            if let Some((var_name, _)) = self.parse_var_assign(line) {
                if !self.defined_vars.contains(&var_name) {
                    self.defined_vars.push(var_name);
                }
            }
            
            // 提取函数定义
            if let Some(func_name) = self.parse_function(line) {
                self.functions.insert(func_name);
            }
        }
        
        // 分析当前行
        if current_line < lines.len() {
            let line = &lines[current_line];
            self.analyze_current_line(line, current_col, current_line);
        }
    }
    
    /// 计算文件内容的简单哈希
    fn compute_file_hash(&self, lines: &[String]) -> u64 {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        
        let mut hasher = DefaultHasher::new();
        for line in lines {
            line.hash(&mut hasher);
        }
        hasher.finish()
    }
    
    /// 解析普通变量赋值（VAR=value）
    fn parse_var_assign(&self, line: &str) -> Option<(String, String)> {
        // 跳过注释和 export 命令（已处理）
        if line.trim_start().starts_with('#') || line.trim_start().starts_with("export") {
            return None;
        }
        
        if let Some(captures) = self.var_assign_regex.captures(line) {
            let var_name = captures.get(1)?.as_str().to_string();
            let var_value = captures.get(2)?.as_str().trim().to_string();
            
            // 移除引号
            let var_value = var_value
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .or_else(|| var_value.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
                .map(|s| s.to_string())
                .unwrap_or(var_value);
            
            Some((var_name, var_value))
        } else {
            None
        }
    }
    
    /// 解析函数定义
    fn parse_function(&self, line: &str) -> Option<String> {
        if let Some(captures) = self.function_regex.captures(line) {
            // 尝试第一个捕获组（function name()）
            if let Some(func_name) = captures.get(1) {
                return Some(func_name.as_str().to_string());
            }
            // 尝试第二个捕获组（name()）
            if let Some(func_name) = captures.get(2) {
                return Some(func_name.as_str().to_string());
            }
        }
        None
    }

    /// 解析 export 行
    fn parse_export_line(&self, line: &str) -> Option<(String, String)> {
        if let Some(captures) = self.export_regex.captures(line) {
            let var_name = captures.get(1)?.as_str().to_string();
            let var_value = captures.get(2)?.as_str().trim().to_string();
            
            // 移除引号
            let var_value = var_value
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .or_else(|| var_value.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
                .map(|s| s.to_string())
                .unwrap_or(var_value);
            
            Some((var_name, var_value))
        } else {
            None
        }
    }

    /// 分析当前行
    fn analyze_current_line(&mut self, line: &str, col: usize, line_number: usize) {
        let prefix: String = line.chars().take(col).collect();
        
        // 检查是否是 export 命令
        let is_export = prefix.trim_start().starts_with("export");
        
        // 提取变量名（如果正在输入 export VAR=...）
        let var_name = if is_export {
            // 尝试提取变量名
            if let Some(equals_pos) = prefix.find('=') {
                let before_equals = &prefix[..equals_pos];
                let words: Vec<&str> = before_equals.split_whitespace().collect();
                if words.len() >= 2 && words[0] == "export" {
                    Some(words[1].to_string())
                } else {
                    None
                }
            } else {
                // 还没有 =，尝试提取变量名
                let words: Vec<&str> = prefix.split_whitespace().collect();
                if words.len() >= 2 && words[0] == "export" {
                    Some(words[1].to_string())
                } else {
                    None
                }
            }
        } else {
            None
        };
        
        self.current_line_context = LineContext {
            is_export,
            var_name,
            prefix,
            line_number,
        };
    }

    /// 获取上下文补全建议（增强版，集成 NLP）
    pub fn get_contextual_suggestion(&self, current_word: &str) -> Option<String> {
        // 使用 NLP 分析当前行的完整上下文
        let full_context = &self.current_line_context.prefix;
        
        // 1. 使用 NLP 分析用户意图
        let intent = self.nlp_analyzer.analyze_intent(full_context);
        
        // 2. 基于意图生成智能建议
        match &intent {
            UserIntent::SetEnvVar { var_name: _, var_type: _ } => {
                // 如果正在输入变量值（在 = 之后）
                if full_context.contains('=') {
                    let suggestions = self.nlp_analyzer.generate_suggestions(&intent, &self.env_vars);
                    
                    // 获取当前输入的值部分（= 之后的内容）
                    let after_equals = full_context.split('=').last().unwrap_or("").trim();
                    
                    // 如果找到了建议的路径
                    for suggestion in &suggestions {
                        // 如果当前输入为空，直接返回第一个建议
                        if after_equals.is_empty() {
                            return Some(suggestion.clone());
                        }
                        
                        // 如果建议的路径以当前输入开头，返回剩余部分
                        if suggestion.starts_with(after_equals) {
                            if let Some(remaining) = suggestion.strip_prefix(after_equals) {
                                if !remaining.is_empty() {
                                    return Some(remaining.to_string());
                                }
                            }
                        }
                        
                        // 如果当前输入是建议路径的前缀（不区分大小写）
                        if suggestion.to_lowercase().starts_with(&after_equals.to_lowercase()) {
                            if suggestion.len() > after_equals.len() {
                                return Some(suggestion[after_equals.len()..].to_string());
                            }
                        }
                    }
                    
                    // 如果没有完全匹配，但找到了路径，返回第一个
                    if !suggestions.is_empty() {
                        return Some(suggestions[0].clone());
                    }
                }
            }
            UserIntent::ConfigurePath { .. } => {
                // 处理 PATH 配置
                if self.current_line_context.is_export {
                    if let Some(ref var_name) = self.current_line_context.var_name {
                        if var_name.to_uppercase() == "PATH" {
                            return self.suggest_path_value();
                        }
                    } else if current_word.to_uppercase() == "PATH" {
                        return self.suggest_path_value();
                    }
                }
            }
            _ => {}
        }
        
        // 3. 处理 export PATH= 的情况（不区分大小写）
        if self.current_line_context.is_export {
            if let Some(ref var_name) = self.current_line_context.var_name {
                if var_name.to_uppercase() == "PATH" {
                    return self.suggest_path_value();
                }
            } else if current_word.to_uppercase() == "PATH" {
                return self.suggest_path_value();
            }
        }
        
        // 4. 检查是否在变量引用中（$VAR 或 ${VAR}）
        if current_word.starts_with('$') {
            let var_prefix = current_word.trim_start_matches('$').trim_start_matches('{').trim_end_matches('}');
            
            // 特殊处理：如果输入 $path（小写），应该建议 PATH（大写）
            if var_prefix.to_uppercase() == "PATH" && var_prefix != "PATH" {
                // 用户输入的是小写 path，建议补全为大写 PATH
                let matched = var_prefix.len().min(4); // PATH 是 4 个字符
                if matched < 4 {
                    return Some("PATH"[matched..].to_string());
                }
            }
            
            return self.suggest_variable_name(var_prefix);
        }
        
        // 5. 检查是否应该建议已定义的变量（不带 $）
        if !current_word.is_empty() && current_word.len() >= 1 {
            if let Some(suggestion) = self.suggest_variable_name(current_word) {
                return Some(suggestion);
            }
        }
        
        // 6. 检查是否应该建议函数名
        if !current_word.is_empty() && current_word.len() >= 1 {
            for func_name in &self.functions {
                if func_name.to_lowercase().starts_with(&current_word.to_lowercase()) {
                    if func_name.to_lowercase() != current_word.to_lowercase() {
                        let suffix = &func_name[current_word.len()..];
                        return Some(suffix.to_string());
                    }
                }
            }
        }
        
        None
    }
    
    /// 建议变量名（智能大小写匹配）
    fn suggest_variable_name(&self, prefix: &str) -> Option<String> {
        if prefix.is_empty() {
            return None;
        }
        
        let prefix_lower = prefix.to_lowercase();
        
        // 标准环境变量列表（应该使用大写）
        let standard_vars = ["PATH", "HOME", "USER", "SHELL", "PWD", "TERM", "LANG", "LC_ALL", "EDITOR"];
        
        // 1. 检查是否是标准环境变量（不区分大小写匹配，但返回大写）
        for std_var in &standard_vars {
            if std_var.to_lowercase().starts_with(&prefix_lower) {
                if std_var.to_lowercase() != prefix_lower {
                    // 计算需要补全的部分（保持标准变量的大写）
                    let matched_len = prefix_lower.len().min(std_var.len());
                    let suffix = &std_var[matched_len..];
                    return Some(suffix.to_string());
                }
            }
        }
        
        // 2. 匹配已定义的变量（保持原定义的大小写）
        for var_name in &self.defined_vars {
            let var_lower = var_name.to_lowercase();
            if var_lower.starts_with(&prefix_lower) {
                if var_lower != prefix_lower {
                    // 计算匹配的长度（基于小写比较）
                    // 但返回时使用原始变量名的大小写
                    let matched_len = if prefix.len() <= var_name.len() {
                        // 找到实际匹配的字符数（考虑大小写）
                        let mut matched = 0;
                        let prefix_chars: Vec<char> = prefix.chars().collect();
                        let var_chars: Vec<char> = var_name.chars().collect();
                        
                        for i in 0..prefix_chars.len().min(var_chars.len()) {
                            if prefix_chars[i].to_lowercase().next() == 
                               var_chars[i].to_lowercase().next() {
                                matched += var_chars[i].len_utf8();
                            } else {
                                break;
                            }
                        }
                        matched
                    } else {
                        prefix.len()
                    };
                    
                    if matched_len < var_name.len() {
                        let suffix = &var_name[matched_len..];
                        return Some(suffix.to_string());
                    }
                }
            }
        }
        
        None
    }

    /// 为 PATH 变量生成智能建议（增强版）
    fn suggest_path_value(&self) -> Option<String> {
        // 查找所有 *_HOME 变量（按定义顺序）
        let home_vars: Vec<_> = self.defined_vars
            .iter()
            .filter(|k| k.ends_with("_HOME"))
            .cloned()
            .collect();
        
        if home_vars.is_empty() {
            return None;
        }
        
        // 获取现有的 PATH 值（如果有，不区分大小写查找）
        let existing_path = self.env_vars.iter()
            .find(|(k, _)| k.to_uppercase() == "PATH")
            .map(|(_, v)| v.as_str())
            .unwrap_or("$PATH");
        
        // 为最新的 *_HOME 变量生成建议（优先）
        if let Some(latest_home) = home_vars.last() {
            // 保持变量名的大小写（使用原定义的大小写）
            let base = format!("${}", latest_home);
            
            // 确保 existing_path 中的 PATH 使用大写
            let existing_path_fixed = if existing_path.contains("$path") {
                existing_path.replace("$path", "$PATH")
            } else {
                existing_path.to_string()
            };
            
            // 生成多个建议选项（按优先级）
            // 1. 包含 bin 和 sbin
            let suggestion = format!("{}:{}/bin:{}/sbin", existing_path_fixed, base, base);
            return Some(suggestion);
        }
        
        None
    }

    /// 获取所有环境变量
    pub fn get_env_vars(&self) -> &HashMap<String, String> {
        &self.env_vars
    }

    /// 获取已定义的变量名列表
    pub fn get_defined_vars(&self) -> &[String] {
        &self.defined_vars
    }
}

impl Default for EditorContext {
    fn default() -> Self {
        Self::new()
    }
}

