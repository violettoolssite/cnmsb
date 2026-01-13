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
        show_welcome: bool,
    ) -> io::Result<()> {
        // 更新终端大小
        let (width, height) = terminal::size()?;
        self.width = width;
        self.height = height;
        
        // 计算可用于文本的行数（减去状态栏）
        let text_height = (self.height as usize).saturating_sub(2);
        
        // 使用 BufWriter 减少系统调用
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout.lock());
        
        // 隐藏光标并移动到开始位置（减少闪烁）
        queue!(writer, Hide, MoveTo(0, 0))?;
        
        // 如果显示欢迎屏幕
        if show_welcome {
            self.render_welcome_screen(&mut writer, text_height)?;
            
            // 渲染状态栏
            queue!(writer, MoveTo(0, text_height as u16))?;
            self.render_welcome_status_bar(&mut writer)?;
            
            // 渲染消息行
            queue!(writer, MoveTo(0, (text_height + 1) as u16), Clear(ClearType::CurrentLine))?;
            queue!(writer, SetForegroundColor(Color::DarkGrey))?;
            write!(writer, "按 i 开始编辑  |  :q 退出  |  cntmd <文件> 打开文件")?;
            queue!(writer, ResetColor)?;
            
            writer.flush()?;
            return Ok(());
        }
        
        // 更新滚动偏移
        self.update_scroll(cursor, text_height);
        
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
    
    /// 渲染欢迎屏幕
    fn render_welcome_screen(&self, writer: &mut impl Write, text_height: usize) -> io::Result<()> {
        // 欢迎屏幕内容定义
        struct WelcomeLine {
            text: &'static str,
            color: (u8, u8, u8),
        }
        
        let cyan = (100, 180, 200);
        let yellow = (200, 180, 100);
        let gray = (120, 120, 130);
        let dim = (80, 80, 90);
        let green = (100, 180, 120);
        
        let welcome_content: Vec<WelcomeLine> = vec![
            WelcomeLine { text: "", color: dim },
            WelcomeLine { text: "█▀▀ █▄░█ ▀█▀ █▀▄▀█ █▀▄", color: cyan },
            WelcomeLine { text: "█▄▄ █░▀█ ░█░ █░▀░█ █▄▀", color: cyan },
            WelcomeLine { text: "", color: dim },
            WelcomeLine { text: "操你他妈的编辑器  v0.1.0", color: yellow },
            WelcomeLine { text: "", color: dim },
            WelcomeLine { text: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━", color: gray },
            WelcomeLine { text: "", color: dim },
            WelcomeLine { text: "i         进入插入模式", color: dim },
            WelcomeLine { text: "Esc       返回普通模式", color: dim },
            WelcomeLine { text: ":w        保存文件", color: dim },
            WelcomeLine { text: ":q        退出", color: dim },
            WelcomeLine { text: ":wq       保存并退出", color: dim },
            WelcomeLine { text: "", color: dim },
            WelcomeLine { text: "Tab / →   接受补全", color: dim },
            WelcomeLine { text: "h j k l   移动光标", color: dim },
            WelcomeLine { text: "", color: dim },
            WelcomeLine { text: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━", color: gray },
            WelcomeLine { text: "", color: dim },
            WelcomeLine { text: "使用: cntmd <文件>", color: green },
            WelcomeLine { text: "", color: dim },
        ];
        
        // 计算垂直居中的起始行
        let start_row = if text_height > welcome_content.len() {
            (text_height - welcome_content.len()) / 2
        } else {
            0
        };
        
        for i in 0..text_height {
            queue!(writer, Clear(ClearType::CurrentLine))?;
            
            // 行号区域（显示波浪号）
            queue!(writer, SetForegroundColor(Color::Rgb { r: 60, g: 60, b: 80 }))?;
            write!(writer, "   ~ ")?;
            
            // 欢迎内容
            let content_idx = i.saturating_sub(start_row);
            if i >= start_row && content_idx < welcome_content.len() {
                let line = &welcome_content[content_idx];
                
                // 计算显示宽度（处理中文字符）
                let display_width = line.text.chars().map(|c| {
                    if c.is_ascii() { 1 } else { 2 }
                }).sum::<usize>();
                
                // 计算水平居中填充
                let available_width = (self.width as usize).saturating_sub(5);
                let padding = if available_width > display_width {
                    (available_width - display_width) / 2
                } else {
                    0
                };
                
                // 设置颜色
                queue!(writer, SetForegroundColor(Color::Rgb { 
                    r: line.color.0, 
                    g: line.color.1, 
                    b: line.color.2 
                }))?;
                
                write!(writer, "{:padding$}{}", "", line.text, padding = padding)?;
            }
            
            queue!(writer, ResetColor)?;
            
            // 移动到下一行
            if i < text_height - 1 {
                queue!(writer, MoveTo(0, (i + 1) as u16))?;
            }
        }
        
        Ok(())
    }
    
    /// 渲染欢迎屏幕的状态栏
    fn render_welcome_status_bar(&self, writer: &mut impl Write) -> io::Result<()> {
        queue!(writer, Clear(ClearType::CurrentLine))?;
        queue!(writer, SetBackgroundColor(Color::Rgb { r: 40, g: 40, b: 50 }), 
               SetForegroundColor(Color::Rgb { r: 140, g: 140, b: 150 }))?;
        
        let left = " NORMAL ";
        let middle = " cntmd - 操你他妈的编辑器 ";
        let right = " 无文件 ";
        
        let padding = (self.width as usize).saturating_sub(left.len() + middle.len() + right.len());
        let left_pad = padding / 2;
        let right_pad = padding - left_pad;
        
        write!(writer, "{}{:left_pad$}{}{:right_pad$}{}", 
               left, "", middle, "", right,
               left_pad = left_pad, right_pad = right_pad)?;
        
        queue!(writer, ResetColor)?;
        
        Ok(())
    }
}

