//! 上下文感知补全
//!
//! 解析历史命令中的环境变量定义，为 PATH 等变量提供智能补全建议

use crate::engine::{Completion, CompletionKind};
use crate::parser::ParsedCommand;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

/// 上下文感知补全器
pub struct ContextAwareCompleter {
    /// 环境变量缓存（变量名 -> 值）
    env_vars: HashMap<String, String>,
    /// 历史命令解析器
    history_parser: HistoryParser,
    /// 路径查找器（用于自动查找系统路径）
    path_finder: PathFinder,
}

/// 历史命令解析器
struct HistoryParser {
    /// 匹配 export VAR=value 的正则表达式
    export_regex: Regex,
}

impl HistoryParser {
    fn new() -> Self {
        // 匹配: export VAR=value, export VAR="value", export VAR='value'
        let export_regex = Regex::new(r"^\s*export\s+([A-Z_][A-Z0-9_]*)\s*=\s*(.*)$").unwrap();
        
        HistoryParser {
            export_regex,
        }
    }

    /// 解析单条命令，提取环境变量
    /// 返回 (变量名, 变量值)
    fn parse_export_command(&self, line: &str) -> Option<(String, String)> {
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

    /// 从历史中提取所有环境变量
    fn extract_all_env_vars(&self, history: &[String]) -> HashMap<String, String> {
        let mut env_vars = HashMap::new();
        
        for line in history {
            if let Some((var_name, var_value)) = self.parse_export_command(line) {
                env_vars.insert(var_name, var_value);
            }
        }
        
        env_vars
    }
}

/// 路径查找器（用于命令行补全）
struct PathFinder;

impl PathFinder {
    fn new() -> Self {
        PathFinder
    }

    /// 查找 Java 安装路径
    fn find_java_paths(&self) -> Vec<String> {
        let mut paths = Vec::new();
        
        let common_paths = vec![
            "/usr/lib/jvm",
            "/opt/jdk",
            "/opt/java",
            "/usr/java",
            "/usr/local/java",
            "/opt/openjdk",
        ];
        
        for base in common_paths {
            let path = std::path::Path::new(base);
            if path.exists() {
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries.flatten() {
                        let entry_path = entry.path();
                        if entry_path.is_dir() {
                            let java_bin = entry_path.join("bin").join("java");
                            if java_bin.exists() {
                                paths.push(entry_path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
        
        paths
    }

    /// 查找 Hadoop 安装路径
    fn find_hadoop_paths(&self) -> Vec<String> {
        let mut paths = Vec::new();
        
        let common_paths = vec!["/opt/hadoop", "/usr/local/hadoop"];
        
        for base in common_paths {
            let path = std::path::Path::new(base);
            if path.exists() && path.is_dir() {
                let hadoop_bin = path.join("bin").join("hadoop");
                if hadoop_bin.exists() {
                    paths.push(path.to_string_lossy().to_string());
                }
            }
        }
        
        // 在 /opt 中查找 hadoop* 目录
        if let Ok(entries) = fs::read_dir("/opt") {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    if let Some(name) = entry_path.file_name().and_then(|n| n.to_str()) {
                        if name.starts_with("hadoop") {
                            let hadoop_bin = entry_path.join("bin").join("hadoop");
                            if hadoop_bin.exists() {
                                paths.push(entry_path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
        
        paths
    }

    /// 根据变量类型查找路径
    fn find_paths_by_var_type(&self, var_name: &str) -> Vec<String> {
        let var_lower = var_name.to_lowercase();
        
        if var_lower.contains("java") || var_lower.contains("jdk") {
            return self.find_java_paths();
        }
        
        if var_lower.contains("hadoop") {
            return self.find_hadoop_paths();
        }
        
        Vec::new()
    }
}

impl ContextAwareCompleter {
    /// 创建新的上下文感知补全器
    pub fn new() -> Self {
        ContextAwareCompleter {
            env_vars: HashMap::new(),
            history_parser: HistoryParser::new(),
            path_finder: PathFinder::new(),
        }
    }

    /// 从历史命令中提取环境变量
    pub fn extract_env_vars(&mut self, history: &[String]) {
        self.env_vars = self.history_parser.extract_all_env_vars(history);
    }

    /// 检测是否是 export 命令
    pub fn is_export_command(&self, parsed: &ParsedCommand) -> bool {
        parsed.command == "export" || 
        (parsed.command.is_empty() && (parsed.current_word.starts_with("export") || 
         parsed.current_word == "PATH" || parsed.current_word == "path"))
    }

    /// 获取环境变量补全建议（增强版，集成路径查找）
    pub fn complete_env_var(&self, parsed: &ParsedCommand) -> Vec<Completion> {
        let mut completions = Vec::new();
        
        // 检查是否是 export PATH= 的情况
        if parsed.command == "export" {
            // 检查前一个词是否是 PATH
            let is_path_var = parsed.previous_word.as_ref()
                .map(|w| w.to_uppercase() == "PATH")
                .unwrap_or(false);
            
            // 或者当前词是 PATH（在输入 export PATH 时）
            let is_path_input = parsed.current_word.to_uppercase() == "PATH";
            
            // 如果当前词为空且前一个词是 PATH，说明在 = 之后
            let is_after_equals = parsed.current_word.is_empty() && is_path_var;
            
            if is_path_input || is_after_equals {
                // 获取 PATH 变量的智能建议
                let path_suggestions = self.suggest_path_value();
                for (i, suggestion) in path_suggestions.iter().enumerate() {
                    let text = if is_after_equals {
                        suggestion.clone()
                    } else {
                        format!("={}", suggestion)
                    };
                    
                    completions.push(Completion {
                        text,
                        description: format!("PATH 变量建议 #{}", i + 1),
                        score: 100 - i as i64,
                        kind: CompletionKind::Argument,
                        match_indices: Vec::new(),
                    });
                }
            } else if parsed.current_word.is_empty() || parsed.current_word == "export" {
                // 建议已定义的环境变量名
                for (var_name, var_value) in &self.env_vars {
                    completions.push(Completion {
                        text: var_name.clone(),
                        description: format!("环境变量: {}", var_value),
                        score: 80,
                        kind: CompletionKind::Argument,
                        match_indices: Vec::new(),
                    });
                }
            } else if parsed.current_word.contains('=') {
                // 在 = 之后，可能是变量值，尝试查找路径
                let var_name = if let Some(ref prev) = parsed.previous_word {
                    Some(prev.clone())
                } else {
                    // 尝试从当前词中提取变量名（export VAR=value 的情况）
                    parsed.current_word.split('=').next()
                        .map(|s| s.to_string())
                };
                
                if let Some(ref var_name) = var_name {
                    // 自动查找相关路径
                    let found_paths = self.path_finder.find_paths_by_var_type(var_name);
                    for (i, path) in found_paths.iter().enumerate() {
                        // 检查当前输入是否已经包含部分路径
                        let after_equals = parsed.current_word.split('=').last().unwrap_or("");
                        if after_equals.trim().is_empty() || path.starts_with(after_equals.trim()) {
                            let text = if after_equals.trim().is_empty() {
                                path.clone()
                            } else {
                                path.strip_prefix(after_equals.trim())
                                    .unwrap_or(path)
                                    .to_string()
                            };
                            
                            completions.push(Completion {
                                text,
                                description: format!("找到的路径: {}", path),
                                score: 95 - i as i64,
                                kind: CompletionKind::Argument,
                                match_indices: Vec::new(),
                            });
                        }
                    }
                }
            }
        } else if parsed.current_word.to_uppercase() == "PATH" {
            // 不在 export 命令中，但输入了 PATH，也可能需要建议
            let path_suggestions = self.suggest_path_value();
            for (i, suggestion) in path_suggestions.iter().enumerate() {
                completions.push(Completion {
                    text: format!("={}", suggestion),
                    description: format!("PATH 变量建议 #{}", i + 1),
                    score: 90 - i as i64,
                    kind: CompletionKind::Argument,
                    match_indices: Vec::new(),
                });
            }
        }
        
        completions
    }

    /// 为 PATH 变量生成智能建议
    fn suggest_path_value(&self) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // 查找所有 *_HOME 变量
        let home_vars: Vec<_> = self.env_vars
            .keys()
            .filter(|k| k.ends_with("_HOME"))
            .cloned()
            .collect();
        
        if home_vars.is_empty() {
            return suggestions;
        }
        
        // 获取现有的 PATH 值（如果有，不区分大小写查找）
        let existing_path = self.env_vars.iter()
            .find(|(k, _)| k.to_uppercase() == "PATH")
            .map(|(_, v)| {
                // 确保 PATH 值中的变量引用使用正确的大小写
                v.replace("$path", "$PATH")
            })
            .unwrap_or_else(|| "$PATH".to_string());
        
        // 为每个 *_HOME 变量生成建议
        for home_var in &home_vars {
            let base = format!("${}", home_var);
            
            // 建议 1: $PATH:$XXX_HOME/bin
            suggestions.push(format!("{}:{}/bin", existing_path, base));
            
            // 建议 2: $PATH:$XXX_HOME/bin:$XXX_HOME/sbin
            suggestions.push(format!("{}:{}/bin:{}/sbin", existing_path, base, base));
            
            // 建议 3: $PATH:$XXX_HOME/bin:$XXX_HOME/sbin:$XXX_HOME/lib
            suggestions.push(format!("{}:{}/bin:{}/sbin:{}/lib", existing_path, base, base, base));
        }
        
        // 如果有多个 *_HOME 变量，生成组合建议
        if home_vars.len() > 1 {
            let mut combined = existing_path.to_string();
            for home_var in &home_vars {
                let base = format!("${}", home_var);
                combined.push_str(&format!(":{}/bin", base));
            }
            suggestions.insert(0, combined);
            
            // 更完整的组合（包含 sbin）
            let mut combined_full = existing_path.to_string();
            for home_var in &home_vars {
                let base = format!("${}", home_var);
                combined_full.push_str(&format!(":{}/bin:{}/sbin", base, base));
            }
            suggestions.insert(0, combined_full);
        }
        
        // 去重并限制数量
        suggestions.dedup();
        suggestions.truncate(10);
        
        suggestions
    }

    /// 更新环境变量（用于实时更新）
    pub fn update_env_var(&mut self, var_name: String, var_value: String) {
        self.env_vars.insert(var_name, var_value);
    }

    /// 获取所有环境变量
    pub fn get_env_vars(&self) -> &HashMap<String, String> {
        &self.env_vars
    }
}

impl Default for ContextAwareCompleter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_export_command() {
        let parser = HistoryParser::new();
        
        assert_eq!(
            parser.parse_export_command("export JAVA_HOME=/opt/jdk"),
            Some(("JAVA_HOME".to_string(), "/opt/jdk".to_string()))
        );
        
        assert_eq!(
            parser.parse_export_command("export PATH=$PATH:$JAVA_HOME/bin"),
            Some(("PATH".to_string(), "$PATH:$JAVA_HOME/bin".to_string()))
        );
        
        assert_eq!(
            parser.parse_export_command(r#"export VAR="value with spaces""#),
            Some(("VAR".to_string(), "value with spaces".to_string()))
        );
    }

    #[test]
    fn test_suggest_path_value() {
        let mut completer = ContextAwareCompleter::new();
        
        // 设置一些环境变量
        completer.update_env_var("JAVA_HOME".to_string(), "/opt/jdk".to_string());
        completer.update_env_var("HADOOP_HOME".to_string(), "/opt/hadoop".to_string());
        completer.update_env_var("PATH".to_string(), "$PATH:$JAVA_HOME/bin".to_string());
        
        let suggestions = completer.suggest_path_value();
        
        // 应该包含 HADOOP_HOME 的建议
        assert!(suggestions.iter().any(|s| s.contains("HADOOP_HOME")));
        assert!(suggestions.iter().any(|s| s.contains("/bin")));
    }
}

