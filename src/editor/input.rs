//! 键盘输入处理

use std::io;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

/// 编辑器事件
#[derive(Debug, Clone)]
pub enum EditorEvent {
    /// 普通字符
    Char(char),
    /// 回车
    Enter,
    /// 退格
    Backspace,
    /// 删除
    Delete,
    /// Tab
    Tab,
    /// Escape
    Escape,
    /// 上箭头
    Up,
    /// 下箭头
    Down,
    /// 左箭头
    Left,
    /// 右箭头
    Right,
    /// Home
    Home,
    /// End
    End,
    /// Page Up
    PageUp,
    /// Page Down
    PageDown,
    /// Ctrl 组合键
    Ctrl(char),
    /// 无事件
    None,
}

/// 输入处理器
pub struct InputHandler;

impl InputHandler {
    /// 创建输入处理器
    pub fn new() -> Self {
        Self
    }
    
    /// 读取事件
    pub fn read_event(&self) -> io::Result<Option<EditorEvent>> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                return Ok(Some(self.map_key_event(key)));
            }
        }
        Ok(Some(EditorEvent::None))
    }
    
    /// 映射按键事件
    fn map_key_event(&self, key: KeyEvent) -> EditorEvent {
        // 处理 Ctrl 组合键
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            if let KeyCode::Char(c) = key.code {
                return EditorEvent::Ctrl(c);
            }
        }
        
        match key.code {
            KeyCode::Char(c) => EditorEvent::Char(c),
            KeyCode::Enter => EditorEvent::Enter,
            KeyCode::Backspace => EditorEvent::Backspace,
            KeyCode::Delete => EditorEvent::Delete,
            KeyCode::Tab => EditorEvent::Tab,
            KeyCode::Esc => EditorEvent::Escape,
            KeyCode::Up => EditorEvent::Up,
            KeyCode::Down => EditorEvent::Down,
            KeyCode::Left => EditorEvent::Left,
            KeyCode::Right => EditorEvent::Right,
            KeyCode::Home => EditorEvent::Home,
            KeyCode::End => EditorEvent::End,
            KeyCode::PageUp => EditorEvent::PageUp,
            KeyCode::PageDown => EditorEvent::PageDown,
            _ => EditorEvent::None,
        }
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}

