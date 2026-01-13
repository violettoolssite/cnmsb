//! 终端渲染

use std::io::{self, Write, stdout, BufWriter};
use crossterm::{
    cursor::{Hide, Show, MoveTo},
    event::{DisableMouseCapture, EnableMouseCapture},
    execute, queue,
    style::{Color, SetForegroundColor, ResetColor, SetBackgroundColor},
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
        enable_raw_mode, disable_raw_mode,
    },
};

use super::{Buffer, Cursor, Mode};

/// 渲染器
pub struct Renderer {
    /// 终端宽度
    width: u16,
    /// 终端高度
    height: u16,
    /// 滚动偏移
    scroll_offset: usize,
}

impl Renderer {
    /// 创建渲染器
    pub fn new() -> io::Result<Self> {
        let (width, height) = terminal::size()?;
        Ok(Self {
            width,
            height,
            scroll_offset: 0,
        })
    }
    
    /// 进入备用屏幕
    pub fn enter_alternate_screen(&self) -> io::Result<()> {
        execute!(stdout(), EnterAlternateScreen, EnableMouseCapture, Hide)?;
        Ok(())
    }
    
    /// 离开备用屏幕
    pub fn leave_alternate_screen(&self) -> io::Result<()> {
        execute!(stdout(), Show, DisableMouseCapture, LeaveAlternateScreen)?;
        Ok(())
    }
    
    /// 启用原始模式
    pub fn enable_raw_mode(&self) -> io::Result<()> {
        enable_raw_mode()
    }
    
    /// 禁用原始模式
    pub fn disable_raw_mode(&self) -> io::Result<()> {
        disable_raw_mode()
    }
    
    /// 渲染编辑器
    pub fn render(
        &mut self,
        buffer: &Buffer,
        cursor: &Cursor,
        mode: &Mode,
        status_message: &str,
        suggestion: &Option<String>,
    ) -> io::Result<()> {
        // 更新终端大小
        let (width, height) = terminal::size()?;
        self.width = width;
        self.height = height;
        
        // 计算可用于文本的行数（减去状态栏）
        let text_height = (self.height as usize).saturating_sub(2);
        
        // 更新滚动偏移
        self.update_scroll(cursor, text_height);
        
        // 使用 BufWriter 减少系统调用
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout.lock());
        
        // 隐藏光标并移动到开始位置（减少闪烁）
        queue!(writer, Hide, MoveTo(0, 0))?;
        
        // 渲染文本行
        for i in 0..text_height {
            let line_num = self.scroll_offset + i;
            
            // 清除当前行
            queue!(writer, Clear(ClearType::CurrentLine))?;
            
            // 行号
            let line_num_str = if line_num < buffer.line_count() {
                format!("{:4} ", line_num + 1)
            } else {
                "   ~ ".to_string()
            };
            
            queue!(writer, SetForegroundColor(Color::DarkGrey))?;
            write!(writer, "{}", line_num_str)?;
            queue!(writer, ResetColor)?;
            
            // 文本内容
            if line_num < buffer.line_count() {
                let line = buffer.get_line(line_num);
                
                // 如果是当前行并且在插入模式，显示补全建议
                if line_num == cursor.row && mode.is_insert() {
                    // 光标前的文本
                    let cursor_col = cursor.col.min(line.len());
                    write!(writer, "{}", &line[..cursor_col])?;
                    
                    // 补全建议（灰色）
                    if let Some(ref sug) = suggestion {
                        queue!(writer, SetForegroundColor(Color::DarkGrey))?;
                        write!(writer, "{}", sug)?;
                        queue!(writer, ResetColor)?;
                    }
                    
                    // 光标后的文本
                    if cursor_col < line.len() {
                        write!(writer, "{}", &line[cursor_col..])?;
                    }
                } else {
                    let display_line = self.truncate_line(line, 5);
                    write!(writer, "{}", display_line)?;
                }
            }
            
            // 移动到下一行
            if i < text_height - 1 {
                queue!(writer, MoveTo(0, (i + 1) as u16))?;
            }
        }
        
        // 渲染状态栏
        queue!(writer, MoveTo(0, text_height as u16))?;
        self.render_status_bar_buffered(&mut writer, buffer, cursor, mode)?;
        
        // 渲染消息行
        queue!(writer, MoveTo(0, (text_height + 1) as u16), Clear(ClearType::CurrentLine))?;
        self.render_message_line_buffered(&mut writer, status_message, mode, &buffer.command_buffer)?;
        
        // 设置光标位置并显示光标
        let cursor_x = 5 + cursor.col as u16;
        let cursor_y = cursor.row.saturating_sub(self.scroll_offset) as u16;
        queue!(writer, MoveTo(cursor_x, cursor_y), Show)?;
        
        writer.flush()?;
        Ok(())
    }
    
    /// 更新滚动偏移
    fn update_scroll(&mut self, cursor: &Cursor, text_height: usize) {
        if cursor.row < self.scroll_offset {
            self.scroll_offset = cursor.row;
        } else if cursor.row >= self.scroll_offset + text_height {
            self.scroll_offset = cursor.row - text_height + 1;
        }
    }
    
    /// 截断行以适应屏幕宽度
    fn truncate_line(&self, line: &str, offset: u16) -> String {
        let max_len = (self.width - offset) as usize;
        if line.len() > max_len {
            format!("{}…", &line[..max_len.saturating_sub(1)])
        } else {
            line.to_string()
        }
    }
    
    /// 渲染状态栏（使用 queue! 宏的缓冲版本）
    fn render_status_bar_buffered(
        &self,
        writer: &mut impl Write,
        buffer: &Buffer,
        cursor: &Cursor,
        mode: &Mode,
    ) -> io::Result<()> {
        queue!(writer, Clear(ClearType::CurrentLine))?;
        queue!(writer, SetBackgroundColor(Color::DarkGrey), SetForegroundColor(Color::White))?;
        
        // 模式
        let mode_str = match mode {
            Mode::Normal => " NORMAL ",
            Mode::Insert => " INSERT ",
            Mode::Command => " COMMAND ",
        };
        
        // 文件信息
        let file_info = format!(" {} ", "cntmd");
        
        // 位置信息
        let pos_info = format!(" {}:{} ", cursor.row + 1, cursor.col + 1);
        
        // 行数信息
        let lines_info = format!(" {} lines ", buffer.line_count());
        
        // 计算填充
        let left = format!("{}{}", mode_str, file_info);
        let right = format!("{}{}", lines_info, pos_info);
        let padding = (self.width as usize).saturating_sub(left.len() + right.len());
        
        write!(writer, "{}{:padding$}{}", left, "", right, padding = padding)?;
        
        queue!(writer, ResetColor)?;
        
        Ok(())
    }
    
    /// 渲染消息行（使用 queue! 宏的缓冲版本）
    fn render_message_line_buffered(
        &self,
        writer: &mut impl Write,
        status_message: &str,
        mode: &Mode,
        command_buffer: &str,
    ) -> io::Result<()> {
        if mode.is_command() {
            write!(writer, ":{}", command_buffer)?;
        } else if !status_message.is_empty() {
            write!(writer, "{}", status_message)?;
        } else {
            // 帮助提示
            queue!(writer, SetForegroundColor(Color::DarkGrey))?;
            write!(writer, "操你他妈的编辑器 | i=插入 :w=保存 :q=退出 Tab=补全")?;
            queue!(writer, ResetColor)?;
        }
        
        Ok(())
    }
}

