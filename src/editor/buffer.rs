//! 文本缓冲区

use std::fs;
use std::io;
use std::path::Path;

/// 文本缓冲区
pub struct Buffer {
    /// 文本行
    lines: Vec<String>,
    /// 命令缓冲区（用于 : 命令）
    pub command_buffer: String,
}

impl Buffer {
    /// 创建空缓冲区
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            command_buffer: String::new(),
        }
    }
    
    /// 从文件创建缓冲区
    pub fn from_file(path: &Path) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let lines: Vec<String> = content.lines().map(String::from).collect();
        
        Ok(Self {
            lines: if lines.is_empty() { vec![String::new()] } else { lines },
            command_buffer: String::new(),
        })
    }
    
    /// 保存到文件
    pub fn save_to_file(&self, path: &Path) -> io::Result<()> {
        let content = self.lines.join("\n");
        fs::write(path, content)
    }
    
    /// 获取行数
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
    
    /// 获取指定行
    pub fn get_line(&self, row: usize) -> &str {
        self.lines.get(row).map(|s| s.as_str()).unwrap_or("")
    }
    
    /// 获取指定行长度
    pub fn line_len(&self, row: usize) -> usize {
        self.lines.get(row).map(|s| s.len()).unwrap_or(0)
    }
    
    /// 插入字符
    pub fn insert_char(&mut self, row: usize, col: usize, c: char) {
        if row < self.lines.len() {
            let line = &mut self.lines[row];
            let col = col.min(line.len());
            line.insert(col, c);
        }
    }
    
    /// 删除字符
    pub fn delete_char(&mut self, row: usize, col: usize) -> bool {
        if row < self.lines.len() {
            let line = &mut self.lines[row];
            if col < line.len() {
                line.remove(col);
                return true;
            }
        }
        false
    }
    
    /// 插入换行
    pub fn insert_newline(&mut self, row: usize, col: usize) {
        if row < self.lines.len() {
            let line = &self.lines[row];
            let col = col.min(line.len());
            let rest = line[col..].to_string();
            self.lines[row] = line[..col].to_string();
            self.lines.insert(row + 1, rest);
        }
    }
    
    /// 在指定行上方插入新行
    pub fn insert_line_above(&mut self, row: usize) {
        self.lines.insert(row, String::new());
    }
    
    /// 删除行
    pub fn delete_line(&mut self, row: usize) {
        if self.lines.len() > 1 && row < self.lines.len() {
            self.lines.remove(row);
        }
    }
    
    /// 追加到行末
    pub fn append_to_line(&mut self, row: usize, text: &str) {
        if row < self.lines.len() {
            self.lines[row].push_str(text);
        }
    }
    
    /// 获取所有行的迭代器
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.lines.iter().map(|s| s.as_str())
    }
    
    /// 获取所有单词（用于补全）
    pub fn all_words(&self) -> Vec<String> {
        let mut words = Vec::new();
        for line in &self.lines {
            for word in line.split(|c: char| !c.is_alphanumeric() && c != '_') {
                if word.len() >= 2 {
                    words.push(word.to_string());
                }
            }
        }
        words
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

