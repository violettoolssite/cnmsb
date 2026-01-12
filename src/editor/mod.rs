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
        
        loop {
            self.renderer.render(&self.buffer, &self.cursor, &self.mode, 
                                  &self.status_message, &self.current_suggestion)?;
            
            if self.should_quit {
                break;
            }
            
            if let Some(event) = self.input.read_event()? {
                self.handle_event(event);
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
        self.buffer.insert_char(self.cursor.row, self.cursor.col, c);
        self.cursor.col += 1;
        self.modified = true;
        self.current_suggestion = None;
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
        
        let prefix = &line[..self.cursor.col.min(line.len())];
        
        // 找到当前正在输入的词
        let word_start = prefix.rfind(|c: char| c.is_whitespace() || c == '(' || c == '{' || c == '[' || c == '"' || c == '\'')
            .map(|i| i + 1)
            .unwrap_or(0);
        
        if word_start >= prefix.len() {
            self.current_suggestion = None;
            return;
        }
        
        let current_word = &prefix[word_start..];
        if current_word.len() < 2 {
            self.current_suggestion = None;
            return;
        }
        
        // 获取补全建议
        self.current_suggestion = self.completer.get_suggestion(current_word);
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
    
    if let Some(ref path) = file {
        if path.exists() {
            editor.open(path)?;
        } else {
            editor.file_path = Some(path.clone());
            editor.status_message = format!("新文件: {}", path.display());
        }
    }
    
    editor.run()
}

