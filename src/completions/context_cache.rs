//! 上下文缓存系统
//!
//! 缓存项目上下文和文件内容分析结果，提高性能

use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;
use crate::completions::context_analyzer::WorkContext;

/// 缓存条目
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    /// 缓存的数据
    data: T,
    /// 创建时间
    created_at: SystemTime,
    /// 最后访问时间
    last_accessed: SystemTime,
}

/// 上下文缓存
pub struct ContextCache {
    /// 项目上下文缓存（路径 -> 上下文）
    project_contexts: HashMap<String, CacheEntry<WorkContext>>,
    /// 文件内容缓存（文件路径 -> 内容哈希）
    file_hashes: HashMap<String, u64>,
    /// 缓存过期时间（秒）
    ttl: u64,
}

impl ContextCache {
    /// 创建新的缓存
    pub fn new() -> Self {
        ContextCache {
            project_contexts: HashMap::new(),
            file_hashes: HashMap::new(),
            ttl: 300, // 5分钟
        }
    }
    
    /// 获取项目上下文（如果缓存有效）
    pub fn get_context(&mut self, path: &str) -> Option<WorkContext> {
        if let Some(entry) = self.project_contexts.get_mut(path) {
            // 检查是否过期
            if entry.created_at.elapsed().map(|d| d.as_secs() < self.ttl).unwrap_or(false) {
                entry.last_accessed = SystemTime::now();
                return Some(entry.data.clone());
            } else {
                // 过期，移除
                self.project_contexts.remove(path);
            }
        }
        None
    }
    
    /// 缓存项目上下文
    pub fn set_context(&mut self, path: &str, context: WorkContext) {
        let now = SystemTime::now();
        self.project_contexts.insert(path.to_string(), CacheEntry {
            data: context,
            created_at: now,
            last_accessed: now,
        });
        
        // 限制缓存大小
        if self.project_contexts.len() > 50 {
            // 移除最久未访问的
            let mut entries: Vec<(String, SystemTime)> = self.project_contexts.iter()
                .map(|(k, v)| (k.clone(), v.last_accessed))
                .collect();
            entries.sort_by(|a, b| a.1.cmp(&b.1));
            if let Some((key, _)) = entries.first() {
                self.project_contexts.remove(key);
            }
        }
    }
    
    /// 检查文件是否已更改
    pub fn is_file_changed(&self, file_path: &str) -> bool {
        // 计算文件哈希
        if let Ok(content) = fs::read_to_string(file_path) {
            let hash = Self::hash_string(&content);
            
            if let Some(&cached_hash) = self.file_hashes.get(file_path) {
                return hash != cached_hash;
            }
        }
        true // 如果无法读取或不存在，认为已更改
    }
    
    /// 更新文件哈希
    pub fn update_file_hash(&mut self, file_path: &str) {
        if let Ok(content) = fs::read_to_string(file_path) {
            let hash = Self::hash_string(&content);
            self.file_hashes.insert(file_path.to_string(), hash);
            
            // 限制缓存大小
            if self.file_hashes.len() > 200 {
                // 移除一些旧的条目（简单策略：移除前100个）
                let keys: Vec<String> = self.file_hashes.keys().take(100).cloned().collect();
                for key in keys {
                    self.file_hashes.remove(&key);
                }
            }
        }
    }
    
    /// 简单的字符串哈希（FNV-1a）
    fn hash_string(s: &str) -> u64 {
        let mut hash: u64 = 14695981039346656037;
        for byte in s.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(1099511628211);
        }
        hash
    }
    
    /// 清理过期缓存
    pub fn cleanup(&mut self) {
        self.project_contexts.retain(|_, entry| {
            entry.created_at.elapsed()
                .map(|d| d.as_secs() < self.ttl)
                .unwrap_or(false)
        });
    }
    
    /// 清空所有缓存
    pub fn clear(&mut self) {
        self.project_contexts.clear();
        self.file_hashes.clear();
    }
}

impl Default for ContextCache {
    fn default() -> Self {
        Self::new()
    }
}

