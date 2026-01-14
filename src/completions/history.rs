//! 历史命令补全

use crate::engine::{Completion, CompletionKind};
use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};

/// 历史命令补全器
pub struct HistoryCompleter {
    /// 缓存的历史命令
    history: Vec<String>,
}

impl HistoryCompleter {
    pub fn new() -> Self {
        let history = Self::load_history();
        HistoryCompleter { history }
    }

    /// 加载历史命令
    fn load_history() -> Vec<String> {
        let mut history = Vec::new();
        let mut seen = HashSet::new();

        // 尝试读取 bash 历史
        if let Some(home) = dirs::home_dir() {
            // Bash history
            let bash_history = home.join(".bash_history");
            if let Ok(file) = fs::File::open(&bash_history) {
                let reader = BufReader::new(file);
                for line in reader.lines().filter_map(|l| l.ok()) {
                    let line = line.trim().to_string();
                    if !line.is_empty() && !seen.contains(&line) {
                        seen.insert(line.clone());
                        history.push(line);
                    }
                }
            }

            // Zsh history (格式可能是 : timestamp:0;command)
            let zsh_history = home.join(".zsh_history");
            if let Ok(file) = fs::File::open(&zsh_history) {
                let reader = BufReader::new(file);
                for line in reader.lines().filter_map(|l| l.ok()) {
                    let cmd = if line.starts_with(':') {
                        // Zsh extended history format
                        line.splitn(2, ';').nth(1).unwrap_or(&line).to_string()
                    } else {
                        line.clone()
                    };
                    let cmd = cmd.trim().to_string();
                    if !cmd.is_empty() && !seen.contains(&cmd) {
                        seen.insert(cmd.clone());
                        history.push(cmd);
                    }
                }
            }
        }

        // 反转，最近的命令在前
        history.reverse();
        
        // 限制数量
        history.truncate(1000);
        
        history
    }

    /// 获取历史命令补全
    pub fn complete(&self, prefix: &str) -> Vec<Completion> {
        if prefix.is_empty() {
            // 返回最近的几条命令
            return self
                .history
                .iter()
                .take(5)
                .enumerate()
                .map(|(i, cmd)| Completion {
                    text: cmd.clone(),
                    description: format!("历史 #{}", i + 1),
                    score: 90 - i as i64,
                    kind: CompletionKind::History,
                    match_indices: Vec::new(),
                })
                .collect();
        }

        self.history
            .iter()
            .filter(|cmd| cmd.starts_with(prefix) || cmd.contains(prefix))
            .take(10)
            .enumerate()
            .map(|(i, cmd)| Completion {
                text: cmd.clone(),
                description: "历史命令".to_string(),
                score: if cmd.starts_with(prefix) {
                    95 - i as i64
                } else {
                    85 - i as i64
                },
                kind: CompletionKind::History,
                match_indices: Vec::new(),
            })
            .collect()
    }

    /// 获取最近的 export 命令
    pub fn get_recent_exports(&self, limit: usize) -> Vec<String> {
        self.history
            .iter()
            .filter(|cmd| cmd.trim().starts_with("export"))
            .take(limit)
            .cloned()
            .collect()
    }

    /// 获取所有历史命令（用于上下文分析）
    pub fn get_all_history(&self) -> &[String] {
        &self.history
    }
}

impl Default for HistoryCompleter {
    fn default() -> Self {
        Self::new()
    }
}

