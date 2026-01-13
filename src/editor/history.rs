//! 历史记录管理
//!
//! 管理编辑器的历史记录，用于补全建议

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

use super::Buffer;

/// 历史记录数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoryData {
    /// 全局词频统计
    pub global_words: HashMap<String, usize>,
    /// 文件特定词频统计
    pub file_words: HashMap<String, HashMap<String, usize>>,
}

/// 历史管理器
pub struct HistoryManager {
    /// 历史数据
    data: HistoryData,
    /// 当前文件的词汇
    current_file_words: HashMap<String, usize>,
    /// 配置目录
    config_dir: PathBuf,
}

impl HistoryManager {
    /// 加载历史记录
    pub fn load() -> Self {
        let config_dir = dirs::home_dir()
            .map(|h| h.join(".cnmsb"))
            .unwrap_or_else(|| PathBuf::from(".cnmsb"));
        
        let history_file = config_dir.join("editor_history.json");
        
        let data = if history_file.exists() {
            fs::read_to_string(&history_file)
                .ok()
                .and_then(|content| serde_json::from_str(&content).ok())
                .unwrap_or_default()
        } else {
            HistoryData::default()
        };
        
        Self {
            data,
            current_file_words: HashMap::new(),
            config_dir,
        }
    }
    
    /// 保存历史记录
    pub fn save(&self) {
        if let Err(e) = fs::create_dir_all(&self.config_dir) {
            eprintln!("创建配置目录失败: {}", e);
            return;
        }
        
        let history_file = self.config_dir.join("editor_history.json");
        
        if let Ok(content) = serde_json::to_string_pretty(&self.data) {
            if let Err(e) = fs::write(&history_file, content) {
                eprintln!("保存历史记录失败: {}", e);
            }
        }
    }
    
    /// 加载文件特定历史
    pub fn load_file_history(&mut self, path: &Path) {
        let key = path.to_string_lossy().to_string();
        self.current_file_words = self.data.file_words
            .get(&key)
            .cloned()
            .unwrap_or_default();
    }
    
    /// 保存文件特定历史
    pub fn save_file_history(&mut self, path: &Path) {
        let key = path.to_string_lossy().to_string();
        self.data.file_words.insert(key, self.current_file_words.clone());
    }
    
    /// 从缓冲区更新历史
    pub fn update_from_buffer(&mut self, buffer: &Buffer) {
        for word in buffer.all_words() {
            // 更新当前文件词频
            *self.current_file_words.entry(word.clone()).or_insert(0) += 1;
            
            // 更新全局词频
            *self.data.global_words.entry(word).or_insert(0) += 1;
        }
    }
    
    /// 获取当前文件的词汇（按频率排序）
    pub fn get_file_words(&self) -> Vec<(String, usize)> {
        let mut words: Vec<_> = self.current_file_words.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        words.sort_by(|a, b| b.1.cmp(&a.1));
        words
    }
    
    /// 获取全局词汇（按频率排序）
    pub fn get_global_words(&self) -> Vec<(String, usize)> {
        let mut words: Vec<_> = self.data.global_words.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        words.sort_by(|a, b| b.1.cmp(&a.1));
        words
    }
    
    /// 添加单个词
    pub fn add_word(&mut self, word: &str) {
        if word.len() >= 2 {
            *self.current_file_words.entry(word.to_string()).or_insert(0) += 1;
            *self.data.global_words.entry(word.to_string()).or_insert(0) += 1;
        }
    }
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self::load()
    }
}

