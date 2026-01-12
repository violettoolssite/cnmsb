//! 补全逻辑
//!
//! 使用 Trie 树实现快速前缀匹配

use std::collections::HashMap;
use super::{Buffer, HistoryManager};

/// Trie 树节点
#[derive(Debug, Clone, Default)]
struct TrieNode {
    /// 子节点
    children: HashMap<char, TrieNode>,
    /// 是否是单词结尾
    is_end: bool,
    /// 词频（用于排序）
    frequency: usize,
    /// 完整单词（仅在 is_end 为 true 时有效）
    word: Option<String>,
}

/// Trie 树
#[derive(Debug, Clone, Default)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    /// 创建新 Trie
    fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }
    
    /// 插入单词
    fn insert(&mut self, word: &str, frequency: usize) {
        let mut node = &mut self.root;
        
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::default);
        }
        
        node.is_end = true;
        node.frequency = node.frequency.max(frequency);
        node.word = Some(word.to_string());
    }
    
    /// 查找以 prefix 开头的所有单词
    fn find_with_prefix(&self, prefix: &str) -> Vec<(String, usize)> {
        let mut node = &self.root;
        
        // 定位到 prefix 对应的节点
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(child) => node = child,
                None => return Vec::new(),
            }
        }
        
        // 收集所有以 prefix 开头的单词
        let mut results = Vec::new();
        self.collect_words(node, &mut results);
        
        // 按频率排序
        results.sort_by(|a, b| b.1.cmp(&a.1));
        
        results
    }
    
    /// 递归收集单词
    fn collect_words(&self, node: &TrieNode, results: &mut Vec<(String, usize)>) {
        if node.is_end {
            if let Some(ref word) = node.word {
                results.push((word.clone(), node.frequency));
            }
        }
        
        for child in node.children.values() {
            self.collect_words(child, results);
        }
    }
}

/// 补全器
pub struct Completer {
    /// 当前文件的 Trie
    file_trie: Trie,
    /// 全局 Trie
    global_trie: Trie,
}

impl Completer {
    /// 创建补全器
    pub fn new() -> Self {
        Self {
            file_trie: Trie::new(),
            global_trie: Trie::new(),
        }
    }
    
    /// 从缓冲区和历史构建补全数据
    pub fn build_from_buffer(&mut self, buffer: &Buffer, history: &HistoryManager) {
        self.file_trie = Trie::new();
        self.global_trie = Trie::new();
        
        // 从当前缓冲区构建
        for word in buffer.all_words() {
            self.file_trie.insert(&word, 10); // 当前文件的词优先级高
        }
        
        // 从文件历史构建
        for (word, freq) in history.get_file_words() {
            self.file_trie.insert(&word, freq + 5); // 文件历史优先级次之
        }
        
        // 从全局历史构建
        for (word, freq) in history.get_global_words() {
            self.global_trie.insert(&word, freq);
        }
    }
    
    /// 获取补全建议
    pub fn get_suggestion(&self, prefix: &str) -> Option<String> {
        if prefix.len() < 2 {
            return None;
        }
        
        let prefix_lower = prefix.to_lowercase();
        
        // 优先从文件 Trie 查找
        let file_matches = self.file_trie.find_with_prefix(&prefix_lower);
        
        // 查找完全匹配前缀的建议（大小写不敏感）
        for (word, _) in &file_matches {
            if word.to_lowercase().starts_with(&prefix_lower) && word.len() > prefix.len() {
                // 返回需要补全的部分
                return Some(word[prefix.len()..].to_string());
            }
        }
        
        // 从全局 Trie 查找
        let global_matches = self.global_trie.find_with_prefix(&prefix_lower);
        
        for (word, _) in &global_matches {
            if word.to_lowercase().starts_with(&prefix_lower) && word.len() > prefix.len() {
                return Some(word[prefix.len()..].to_string());
            }
        }
        
        None
    }
    
    /// 添加新词到补全器
    pub fn add_word(&mut self, word: &str) {
        if word.len() >= 2 {
            self.file_trie.insert(&word.to_lowercase(), 10);
        }
    }
    
    /// 获取所有匹配的建议列表
    pub fn get_all_suggestions(&self, prefix: &str, limit: usize) -> Vec<String> {
        if prefix.len() < 2 {
            return Vec::new();
        }
        
        let prefix_lower = prefix.to_lowercase();
        let mut results = Vec::new();
        
        // 文件 Trie 的结果
        for (word, _) in self.file_trie.find_with_prefix(&prefix_lower) {
            if word.len() > prefix.len() && !results.contains(&word) {
                results.push(word);
                if results.len() >= limit {
                    return results;
                }
            }
        }
        
        // 全局 Trie 的结果
        for (word, _) in self.global_trie.find_with_prefix(&prefix_lower) {
            if word.len() > prefix.len() && !results.contains(&word) {
                results.push(word);
                if results.len() >= limit {
                    return results;
                }
            }
        }
        
        results
    }
}

impl Default for Completer {
    fn default() -> Self {
        Self::new()
    }
}

