//! 光标位置管理

use super::Buffer;

/// 光标位置
pub struct Cursor {
    /// 行号（从 0 开始）
    pub row: usize,
    /// 列号（从 0 开始）
    pub col: usize,
    /// 记住的列位置（用于上下移动时保持列位置）
    desired_col: usize,
}

impl Cursor {
    /// 创建新光标
    pub fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            desired_col: 0,
        }
    }
    
    /// 向上移动
    pub fn move_up(&mut self) {
        if self.row > 0 {
            self.row -= 1;
            self.col = self.desired_col;
        }
    }
    
    /// 向下移动
    pub fn move_down(&mut self, buffer: &Buffer) {
        if self.row < buffer.line_count().saturating_sub(1) {
            self.row += 1;
            self.col = self.desired_col.min(buffer.line_len(self.row));
        }
    }
    
    /// 向左移动
    pub fn move_left(&mut self) {
        if self.col > 0 {
            self.col -= 1;
            self.desired_col = self.col;
        }
    }
    
    /// 向右移动
    pub fn move_right(&mut self, buffer: &Buffer) {
        let line_len = buffer.line_len(self.row);
        if self.col < line_len {
            self.col += 1;
            self.desired_col = self.col;
        }
    }
    
    /// 移动到行首
    pub fn move_to_start_of_line(&mut self) {
        self.col = 0;
        self.desired_col = 0;
    }
    
    /// 移动到行尾
    pub fn move_to_end_of_line(&mut self, buffer: &Buffer) {
        self.col = buffer.line_len(self.row);
        self.desired_col = self.col;
    }
    
    /// 移动到文档开头
    pub fn move_to_start(&mut self) {
        self.row = 0;
        self.col = 0;
        self.desired_col = 0;
    }
    
    /// 移动到文档末尾
    pub fn move_to_end(&mut self, buffer: &Buffer) {
        self.row = buffer.line_count().saturating_sub(1);
        self.col = buffer.line_len(self.row);
        self.desired_col = self.col;
    }
    
    /// 向前移动一个单词
    pub fn move_word_forward(&mut self, buffer: &Buffer) {
        let line = buffer.get_line(self.row);
        let len = line.len();
        
        // 跳过当前单词的剩余部分
        while self.col < len {
            let c = line.chars().nth(self.col).unwrap_or(' ');
            if c.is_whitespace() {
                break;
            }
            self.col += 1;
        }
        
        // 跳过空白
        while self.col < len {
            let c = line.chars().nth(self.col).unwrap_or(' ');
            if !c.is_whitespace() {
                break;
            }
            self.col += 1;
        }
        
        // 如果到行尾了，移动到下一行开头
        if self.col >= len && self.row < buffer.line_count().saturating_sub(1) {
            self.row += 1;
            self.col = 0;
        }
        
        self.desired_col = self.col;
    }
    
    /// 向后移动一个单词
    pub fn move_word_backward(&mut self, buffer: &Buffer) {
        let line = buffer.get_line(self.row);
        
        if self.col == 0 {
            // 移动到上一行末尾
            if self.row > 0 {
                self.row -= 1;
                self.col = buffer.line_len(self.row);
            }
            return;
        }
        
        self.col -= 1;
        
        // 跳过空白
        while self.col > 0 {
            let c = line.chars().nth(self.col).unwrap_or(' ');
            if !c.is_whitespace() {
                break;
            }
            self.col -= 1;
        }
        
        // 跳过单词
        while self.col > 0 {
            let c = line.chars().nth(self.col - 1).unwrap_or(' ');
            if c.is_whitespace() {
                break;
            }
            self.col -= 1;
        }
        
        self.desired_col = self.col;
    }
    
    /// 确保光标在有效范围内
    pub fn clamp(&mut self, buffer: &Buffer) {
        self.row = self.row.min(buffer.line_count().saturating_sub(1));
        self.col = self.col.min(buffer.line_len(self.row));
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

