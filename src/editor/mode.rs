//! 编辑器模式

/// 编辑器模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// 普通模式（vim 风格导航）
    Normal,
    /// 插入模式（输入文本 + 补全）
    Insert,
    /// 命令模式（:w, :q 等）
    Command,
}

impl Mode {
    /// 获取模式显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Command => "COMMAND",
        }
    }
    
    /// 是否是插入模式
    pub fn is_insert(&self) -> bool {
        matches!(self, Mode::Insert)
    }
    
    /// 是否是普通模式
    pub fn is_normal(&self) -> bool {
        matches!(self, Mode::Normal)
    }
    
    /// 是否是命令模式
    pub fn is_command(&self) -> bool {
        matches!(self, Mode::Command)
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

