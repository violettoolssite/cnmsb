//! cntmd - 操你他妈的文本编辑器
//!
//! 基于历史的智能补全编辑器，类似 vim 但更智能

pub mod buffer;
pub mod cursor;
pub mod mode;
pub mod render;
pub mod input;
pub mod history;
pub mod completion;

use std::io;
use std::path::PathBuf;

pub use buffer::Buffer;
pub use cursor::Cursor;
pub use mode::Mode;
pub use render::Renderer;
pub use input::InputHandler;
pub use history::HistoryManager;
pub use completion::Completer;

/// 编辑器主结构
pub struct Editor {
    /// 文本缓冲区
    buffer: Buffer,
    /// 光标位置
    cursor: Cursor,
    /// 当前模式
    mode: Mode,
    /// 渲染器
    renderer: Renderer,
    /// 输入处理器
    input: InputHandler,
    /// 历史管理器
    history: HistoryManager,
    /// 补全器
    completer: Completer,
    /// 当前文件路径
    file_path: Option<PathBuf>,
    /// 是否已修改
    modified: bool,
    /// 是否应该退出
    should_quit: bool,
    /// 状态消息
    status_message: String,
    /// 当前补全建议
    current_suggestion: Option<String>,
}

impl Editor {
    /// 创建新编辑器
    pub fn new() -> io::Result<Self> {
        let history = HistoryManager::load();
        let completer = Completer::new();
        
        Ok(Self {
            buffer: Buffer::new(),
            cursor: Cursor::new(),
            mode: Mode::Normal,
            renderer: Renderer::new()?,
            input: InputHandler::new(),
            history,
            completer,
            file_path: None,
            modified: false,
            should_quit: false,
            status_message: String::new(),
            current_suggestion: None,
        })
    }
    
    /// 打开文件
    pub fn open(&mut self, path: &PathBuf) -> io::Result<()> {
        self.buffer = Buffer::from_file(path)?;
        self.file_path = Some(path.clone());
        self.cursor = Cursor::new();
        self.modified = false;
        
        // 加载文件特定历史
        self.history.load_file_history(path);
        
        // 从当前文件内容构建补全数据
        self.completer.build_from_buffer(&self.buffer, &self.history);
        
        self.status_message = format!("打开: {}", path.display());
        Ok(())
    }
    
    /// 保存文件
    pub fn save(&mut self) -> io::Result<()> {
        if let Some(ref path) = self.file_path {
            self.buffer.save_to_file(path)?;
            self.modified = false;
            
            // 更新历史
            self.history.update_from_buffer(&self.buffer);
            self.history.save_file_history(path);
            self.history.save();
            
            self.status_message = format!("已保存: {}", path.display());
            Ok(())
        } else {
            self.status_message = "没有文件名".to_string();
            Err(io::Error::new(io::ErrorKind::Other, "No filename"))
        }
    }
    
    /// 运行编辑器主循环
    pub fn run(&mut self) -> io::Result<()> {
        self.renderer.enter_alternate_screen()?;
        self.renderer.enable_raw_mode()?;
        
        // 初始渲染
        self.renderer.render(&self.buffer, &self.cursor, &self.mode, 
                              &self.status_message, &self.current_suggestion)?;
        
        loop {
            if self.should_quit {
                break;
            }
            
            if let Some(event) = self.input.read_event()? {
                // 只有在有实际事件时才处理和重新渲染
                if !matches!(event, input::EditorEvent::None) {
                    self.handle_event(event);
                    self.renderer.render(&self.buffer, &self.cursor, &self.mode, 
                                          &self.status_message, &self.current_suggestion)?;
                }
            }
        }
        
        self.renderer.disable_raw_mode()?;
        self.renderer.leave_alternate_screen()?;
        
        Ok(())
    }
    
    /// 处理事件
    fn handle_event(&mut self, event: input::EditorEvent) {
        use input::EditorEvent::*;
        
        match event {
            Char(c) => self.handle_char(c),
            Enter => self.handle_enter(),
            Backspace => self.handle_backspace(),
            Delete => self.handle_delete(),
            Tab => self.handle_tab(),
            Escape => self.handle_escape(),
            Up => self.handle_up(),
            Down => self.handle_down(),
            Left => self.handle_left(),
            Right => self.handle_right(),
            Home => self.handle_home(),
            End => self.handle_end(),
            PageUp => self.handle_page_up(),
            PageDown => self.handle_page_down(),
            Ctrl(c) => self.handle_ctrl(c),
            None => {}
        }
        
        // 更新补全建议
        if matches!(self.mode, Mode::Insert) {
            self.update_suggestion();
        }
    }
    
    /// 处理字符输入
    fn handle_char(&mut self, c: char) {
        match self.mode {
            Mode::Normal => self.handle_normal_char(c),
            Mode::Insert => self.handle_insert_char(c),
            Mode::Command => self.handle_command_char(c),
        }
    }
    
    /// Normal 模式字符处理
    fn handle_normal_char(&mut self, c: char) {
        match c {
            'i' => {
                self.mode = Mode::Insert;
                self.status_message = "-- 插入 --".to_string();
            }
            'a' => {
                self.cursor.move_right(&self.buffer);
                self.mode = Mode::Insert;
                self.status_message = "-- 插入 --".to_string();
            }
            'A' => {
                self.cursor.move_to_end_of_line(&self.buffer);
                self.mode = Mode::Insert;
                self.status_message = "-- 插入 --".to_string();
            }
            'I' => {
                self.cursor.move_to_start_of_line();
                self.mode = Mode::Insert;
                self.status_message = "-- 插入 --".to_string();
            }
            'o' => {
                self.cursor.move_to_end_of_line(&self.buffer);
                self.buffer.insert_newline(self.cursor.row, self.cursor.col);
                self.cursor.row += 1;
                self.cursor.col = 0;
                self.mode = Mode::Insert;
                self.modified = true;
                self.status_message = "-- 插入 --".to_string();
            }
            'O' => {
                self.buffer.insert_line_above(self.cursor.row);
                self.cursor.col = 0;
                self.mode = Mode::Insert;
                self.modified = true;
                self.status_message = "-- 插入 --".to_string();
            }
            'h' => self.cursor.move_left(),
            'j' => self.cursor.move_down(&self.buffer),
            'k' => self.cursor.move_up(),
            'l' => self.cursor.move_right(&self.buffer),
            'x' => {
                if self.buffer.delete_char(self.cursor.row, self.cursor.col) {
                    self.modified = true;
                }
            }
            'd' => {
                // 简单实现：dd 删除行
                self.status_message = "按 d 删除行".to_string();
            }
            'G' => self.cursor.move_to_end(&self.buffer),
            'g' => {
                // gg 移到开头
                self.status_message = "按 g 到开头".to_string();
            }
            '0' => self.cursor.move_to_start_of_line(),
            '$' => self.cursor.move_to_end_of_line(&self.buffer),
            ':' => {
                self.mode = Mode::Command;
                self.buffer.command_buffer.clear();
                self.status_message = ":".to_string();
            }
            'w' => self.cursor.move_word_forward(&self.buffer),
            'b' => self.cursor.move_word_backward(&self.buffer),
            _ => {}
        }
    }
    
    /// Insert 模式字符处理
    fn handle_insert_char(&mut self, c: char) {
        // 如果输入的是空格或标点，学习刚才输入的词
        if c.is_whitespace() || c == '(' || c == ')' || c == '{' || c == '}' || 
           c == '[' || c == ']' || c == ';' || c == ':' || c == ',' || c == '.' {
            self.learn_current_word();
        }
        
        self.buffer.insert_char(self.cursor.row, self.cursor.col, c);
        self.cursor.col += 1;
        self.modified = true;
        self.current_suggestion = None;
    }
    
    /// 学习当前光标前的词
    fn learn_current_word(&mut self) {
        let line = self.buffer.get_line(self.cursor.row);
        if self.cursor.col == 0 || line.is_empty() {
            return;
        }
        
        let prefix = &line[..self.cursor.col.min(line.len())];
        
        // 找到当前词的开始位置
        let word_start = prefix.rfind(|c: char| c.is_whitespace() || c == '(' || c == '{' || c == '[')
            .map(|i| i + 1)
            .unwrap_or(0);
        
        if word_start < prefix.len() {
            let word = &prefix[word_start..];
            if word.len() >= 2 {
                self.completer.learn_word(word);
            }
        }
    }
    
    /// Command 模式字符处理
    fn handle_command_char(&mut self, c: char) {
        self.buffer.command_buffer.push(c);
        self.status_message = format!(":{}", self.buffer.command_buffer);
    }
    
    /// 处理 Enter
    fn handle_enter(&mut self) {
        match self.mode {
            Mode::Insert => {
                // 学习当前词
                self.learn_current_word();
                
                self.buffer.insert_newline(self.cursor.row, self.cursor.col);
                self.cursor.row += 1;
                self.cursor.col = 0;
                self.modified = true;
                self.current_suggestion = None;
            }
            Mode::Command => {
                self.execute_command();
                self.mode = Mode::Normal;
            }
            Mode::Normal => {}
        }
    }
    
    /// 执行命令
    fn execute_command(&mut self) {
        let cmd = self.buffer.command_buffer.clone();
        self.buffer.command_buffer.clear();
        
        match cmd.as_str() {
            "w" => {
                if let Err(e) = self.save() {
                    self.status_message = format!("保存失败: {}", e);
                }
            }
            "q" => {
                if self.modified {
                    self.status_message = "文件已修改，使用 :q! 强制退出或 :wq 保存退出".to_string();
                } else {
                    self.should_quit = true;
                }
            }
            "q!" => {
                self.should_quit = true;
            }
            "wq" | "x" => {
                if self.save().is_ok() {
                    self.should_quit = true;
                }
            }
            _ => {
                self.status_message = format!("未知命令: {}", cmd);
            }
        }
    }
    
    /// 处理 Backspace
    fn handle_backspace(&mut self) {
        if matches!(self.mode, Mode::Command) {
            self.buffer.command_buffer.pop();
            self.status_message = format!(":{}", self.buffer.command_buffer);
            return;
        }
        
        if !matches!(self.mode, Mode::Insert) {
            return;
        }
        
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
            self.buffer.delete_char(self.cursor.row, self.cursor.col);
            self.modified = true;
        } else if self.cursor.row > 0 {
            // 合并到上一行
            let current_line = self.buffer.get_line(self.cursor.row).to_string();
            self.buffer.delete_line(self.cursor.row);
            self.cursor.row -= 1;
            self.cursor.col = self.buffer.line_len(self.cursor.row);
            self.buffer.append_to_line(self.cursor.row, &current_line);
            self.modified = true;
        }
        self.current_suggestion = None;
    }
    
    /// 处理 Delete
    fn handle_delete(&mut self) {
        if matches!(self.mode, Mode::Insert) {
            if self.buffer.delete_char(self.cursor.row, self.cursor.col) {
                self.modified = true;
            }
        }
    }
    
    /// 处理 Tab - 接受补全
    fn handle_tab(&mut self) {
        if matches!(self.mode, Mode::Insert) {
            if let Some(ref suggestion) = self.current_suggestion.clone() {
                // 插入补全内容
                for c in suggestion.chars() {
                    self.buffer.insert_char(self.cursor.row, self.cursor.col, c);
                    self.cursor.col += 1;
                }
                self.modified = true;
                self.current_suggestion = None;
            } else {
                // 没有建议时插入制表符（4个空格）
                for _ in 0..4 {
                    self.buffer.insert_char(self.cursor.row, self.cursor.col, ' ');
                    self.cursor.col += 1;
                }
                self.modified = true;
            }
        }
    }
    
    /// 处理 Escape
    fn handle_escape(&mut self) {
        match self.mode {
            Mode::Insert | Mode::Command => {
                self.mode = Mode::Normal;
                self.current_suggestion = None;
                self.buffer.command_buffer.clear();
                self.status_message.clear();
            }
            Mode::Normal => {}
        }
    }
    
    /// 方向键处理
    fn handle_up(&mut self) {
        self.cursor.move_up();
    }
    
    fn handle_down(&mut self) {
        self.cursor.move_down(&self.buffer);
    }
    
    fn handle_left(&mut self) {
        self.cursor.move_left();
    }
    
    fn handle_right(&mut self) {
        // 在 Insert 模式下，右箭头也可以接受建议
        if matches!(self.mode, Mode::Insert) {
            if let Some(ref suggestion) = self.current_suggestion.clone() {
                for c in suggestion.chars() {
                    self.buffer.insert_char(self.cursor.row, self.cursor.col, c);
                    self.cursor.col += 1;
                }
                self.modified = true;
                self.current_suggestion = None;
                return;
            }
        }
        self.cursor.move_right(&self.buffer);
    }
    
    fn handle_home(&mut self) {
        self.cursor.move_to_start_of_line();
    }
    
    fn handle_end(&mut self) {
        self.cursor.move_to_end_of_line(&self.buffer);
    }
    
    fn handle_page_up(&mut self) {
        for _ in 0..20 {
            self.cursor.move_up();
        }
    }
    
    fn handle_page_down(&mut self) {
        for _ in 0..20 {
            self.cursor.move_down(&self.buffer);
        }
    }
    
    /// Ctrl 组合键
    fn handle_ctrl(&mut self, c: char) {
        match c {
            's' => {
                if let Err(e) = self.save() {
                    self.status_message = format!("保存失败: {}", e);
                }
            }
            'q' => {
                if self.modified {
                    self.status_message = "文件已修改！按 Ctrl+Q 两次强制退出".to_string();
                } else {
                    self.should_quit = true;
                }
            }
            _ => {}
        }
    }
    
    /// 更新补全建议
    fn update_suggestion(&mut self) {
        // 获取当前行光标前的文本
        let line = self.buffer.get_line(self.cursor.row);
        if self.cursor.col == 0 || line.is_empty() {
            self.current_suggestion = None;
            return;
        }
        
        let col = self.cursor.col.min(line.len());
        
        // 处理 UTF-8：确保在字符边界上切片
        let prefix: String = line.chars().take(col).collect();
        
        // 找到当前正在输入的词
        let word_start = prefix.rfind(|c: char| c.is_whitespace() || c == '(' || c == '{' || c == '[' || c == '"' || c == '\'')
            .map(|i| i + 1)
            .unwrap_or(0);
        
        if word_start >= prefix.len() {
            self.current_suggestion = None;
            return;
        }
        
        let current_word = &prefix[word_start..];
        
        // 只需要 1 个字符就开始建议
        if current_word.is_empty() {
            self.current_suggestion = None;
            return;
        }
        
        // 获取补全建议
        self.current_suggestion = self.completer.get_suggestion(current_word);
    }
    
    /// 根据文件扩展名添加文件头
    fn add_file_header(&mut self, path: &std::path::Path) {
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        
        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        let header = match ext.to_lowercase().as_str() {
            "sh" => Some(SHEBANG_BASH),
            "bash" => Some(SHEBANG_BASH),
            "zsh" => Some(SHEBANG_ZSH),
            "py" => Some(SHEBANG_PYTHON),
            "python" => Some(SHEBANG_PYTHON),
            "pl" => Some(SHEBANG_PERL),
            "rb" => Some(SHEBANG_RUBY),
            "js" if filename.ends_with(".mjs") || filename.starts_with("cli") => Some(SHEBANG_NODE),
            "rs" => Some(HEADER_RUST),
            "c" => Some(HEADER_C),
            "cpp" | "cc" | "cxx" => Some(HEADER_CPP),
            "go" => Some(HEADER_GO),
            "java" => Some(HEADER_JAVA),
            "html" | "htm" => Some(HEADER_HTML),
            "yml" | "yaml" => Some(HEADER_YAML),
            _ => None,
        };
        
        // 检查特殊文件名
        let header = header.or_else(|| {
            match filename.to_lowercase().as_str() {
                "makefile" | "gnumakefile" => Some(HEADER_MAKEFILE),
                "dockerfile" => Some(HEADER_DOCKERFILE),
                _ => None,
            }
        });
        
        if let Some(content) = header {
            // 将内容按行分割并添加到缓冲区
            self.buffer = Buffer::new();
            for (i, line) in content.lines().enumerate() {
                if i == 0 {
                    // 第一行替换空缓冲区
                    for c in line.chars() {
                        self.buffer.insert_char(0, self.buffer.line_len(0), c);
                    }
                } else {
                    // 后续行添加
                    self.buffer.insert_newline(i - 1, self.buffer.line_len(i - 1));
                    for c in line.chars() {
                        self.buffer.insert_char(i, self.buffer.line_len(i), c);
                    }
                }
            }
            // 添加最后一个空行
            let last = self.buffer.line_count() - 1;
            self.buffer.insert_newline(last, self.buffer.line_len(last));
            
            // 将光标移到合适位置
            self.cursor.row = self.buffer.line_count().saturating_sub(2);
            self.cursor.col = 0;
        }
    }
    
    /// 显示欢迎信息
    fn show_welcome(&mut self) {
        self.buffer = Buffer::new();
        for (i, line) in WELCOME_MESSAGE.lines().enumerate() {
            if i == 0 {
                for c in line.chars() {
                    self.buffer.insert_char(0, self.buffer.line_len(0), c);
                }
            } else {
                self.buffer.insert_newline(i - 1, self.buffer.line_len(i - 1));
                for c in line.chars() {
                    self.buffer.insert_char(i, self.buffer.line_len(i), c);
                }
            }
        }
        self.status_message = "cntmd - 按 i 开始编辑，:q 退出".to_string();
        // 欢迎屏幕不算修改
        self.modified = false;
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new().expect("Failed to create editor")
    }
}

/// 运行编辑器
pub fn run_editor(file: Option<PathBuf>) -> io::Result<()> {
    let mut editor = Editor::new()?;
    
    match file {
        Some(ref path) if path.exists() => {
            // 打开已存在的文件
            editor.open(path)?;
        }
        Some(ref path) => {
            // 新文件：自动添加文件头
            editor.file_path = Some(path.clone());
            editor.add_file_header(path);
            editor.modified = true;
            editor.status_message = format!("新文件: {}", path.display());
        }
        None => {
            // 无文件参数：显示欢迎信息
            editor.show_welcome();
        }
    }
    
    editor.run()
}

/// 文件头模板
const SHEBANG_BASH: &str = "#!/bin/bash\n# \n# 描述: \n# 作者: \n# 日期: \n\nset -euo pipefail\n\n";
const SHEBANG_ZSH: &str = "#!/bin/zsh\n# \n# 描述: \n# 作者: \n# 日期: \n\nset -euo pipefail\n\n";
const SHEBANG_PYTHON: &str = "#!/usr/bin/env python3\n# -*- coding: utf-8 -*-\n\"\"\"\n描述: \n作者: \n日期: \n\"\"\"\n\n";
const SHEBANG_PERL: &str = "#!/usr/bin/env perl\nuse strict;\nuse warnings;\n\n";
const SHEBANG_RUBY: &str = "#!/usr/bin/env ruby\n# frozen_string_literal: true\n\n";
const SHEBANG_NODE: &str = "#!/usr/bin/env node\n\n";
const HEADER_RUST: &str = "//! \n//! 描述: \n//! \n\n";
const HEADER_C: &str = "/**\n * 描述: \n * 作者: \n * 日期: \n */\n\n#include <stdio.h>\n\nint main(int argc, char *argv[]) {\n    \n    return 0;\n}\n";
const HEADER_CPP: &str = "/**\n * 描述: \n * 作者: \n * 日期: \n */\n\n#include <iostream>\n\nint main(int argc, char *argv[]) {\n    \n    return 0;\n}\n";
const HEADER_GO: &str = "// 描述: \n// 作者: \n// 日期: \n\npackage main\n\nimport \"fmt\"\n\nfunc main() {\n\t\n}\n";
const HEADER_JAVA: &str = "/**\n * 描述: \n * 作者: \n * 日期: \n */\n\npublic class Main {\n    public static void main(String[] args) {\n        \n    }\n}\n";
const HEADER_HTML: &str = "<!DOCTYPE html>\n<html lang=\"zh-CN\">\n<head>\n    <meta charset=\"UTF-8\">\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n    <title></title>\n</head>\n<body>\n    \n</body>\n</html>\n";
const HEADER_MAKEFILE: &str = "# Makefile\n# 描述: \n\n.PHONY: all clean\n\nall:\n\t\n\nclean:\n\trm -rf \n";
const HEADER_DOCKERFILE: &str = "# Dockerfile\n# 描述: \n\nFROM ubuntu:22.04\n\nRUN apt-get update && apt-get install -y \\\n    && rm -rf /var/lib/apt/lists/*\n\nWORKDIR /app\n\nCOPY . .\n\nCMD [\"bash\"]\n";
const HEADER_YAML: &str = "# \n# 描述: \n#\n\n";

/// 欢迎信息
const WELCOME_MESSAGE: &str = r#"
    ██████╗███╗   ██╗████████╗███╗   ███╗██████╗ 
   ██╔════╝████╗  ██║╚══██╔══╝████╗ ████║██╔══██╗
   ██║     ██╔██╗ ██║   ██║   ██╔████╔██║██║  ██║
   ██║     ██║╚██╗██║   ██║   ██║╚██╔╝██║██║  ██║
   ╚██████╗██║ ╚████║   ██║   ██║ ╚═╝ ██║██████╔╝
    ╚═════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝     ╚═╝╚═════╝ 

   操你他妈的编辑器 (cntmd) v0.1.0
   基于历史的智能补全文本编辑器

   快捷键:
     i          进入插入模式
     Esc        返回普通模式
     :w         保存文件
     :q         退出
     :wq        保存并退出
     Tab        接受补全建议
     →          接受补全建议
     h/j/k/l    光标移动

   使用方法:
     cntmd <文件名>    打开或创建文件
     cnmsb edit <文件>  同上

   智能补全:
     输入时自动显示灰色建议
     按 Tab 或 → 接受建议
     支持 100+ 常用词汇
     实时学习你输入的词

   按 i 开始编辑，或 :q 退出
"#;

