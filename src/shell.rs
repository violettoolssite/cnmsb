//! cnmsb 交互式 Shell
//! 提供类似 IDE/Fish 的命令行体验

use crate::engine::CompletionEngine;
use std::io::{self, Read, Write, stdout, stdin};
use std::process::Command;

/// 终端控制序列
mod term {
    pub const RESET: &str = "\x1b[0m";
    pub const GRAY: &str = "\x1b[38;5;240m";
    pub const CYAN: &str = "\x1b[36m";
    pub const GREEN: &str = "\x1b[32m";
    pub const BOLD: &str = "\x1b[1m";
    
    pub const CLEAR_LINE: &str = "\x1b[2K";
    pub const CURSOR_START: &str = "\x1b[G";
    pub const SAVE_CURSOR: &str = "\x1b[s";
    pub const RESTORE_CURSOR: &str = "\x1b[u";
    pub const HIDE_CURSOR: &str = "\x1b[?25l";
    pub const SHOW_CURSOR: &str = "\x1b[?25h";
}

/// 交互式 Shell
pub struct CnmsbShell {
    engine: CompletionEngine,
    history: Vec<String>,
    history_index: usize,
}

impl CnmsbShell {
    pub fn new() -> Self {
        CnmsbShell {
            engine: CompletionEngine::new(),
            history: Vec::new(),
            history_index: 0,
        }
    }
    
    /// 运行交互式 shell
    pub fn run(&mut self) -> io::Result<()> {
        println!("{}{}cnmsb shell{} - 操你妈傻逼交互式命令行", term::BOLD, term::CYAN, term::RESET);
        println!("输入命令，灰色文字为建议。Tab 接受，Ctrl+D 退出。\n");
        
        // 设置终端为原始模式
        #[cfg(unix)]
        let _raw_mode = RawMode::enable()?;
        
        loop {
            match self.read_line()? {
                Some(line) => {
                    if !line.trim().is_empty() {
                        self.execute(&line);
                        self.history.push(line);
                        self.history_index = self.history.len();
                    }
                }
                None => {
                    // Ctrl+D
                    println!("\n再见！");
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// 读取一行输入（带自动建议）
    fn read_line(&mut self) -> io::Result<Option<String>> {
        let mut buffer = String::new();
        let mut cursor = 0usize;
        let mut suggestion = String::new();
        
        self.print_prompt();
        
        loop {
            stdout().flush()?;
            
            let key = self.read_key()?;
            
            match key {
                Key::Char(c) => {
                    buffer.insert(cursor, c);
                    cursor += 1;
                    suggestion = self.get_suggestion(&buffer);
                    self.redraw_line(&buffer, cursor, &suggestion);
                }
                Key::Backspace => {
                    if cursor > 0 {
                        cursor -= 1;
                        buffer.remove(cursor);
                        suggestion = self.get_suggestion(&buffer);
                        self.redraw_line(&buffer, cursor, &suggestion);
                    }
                }
                Key::Delete => {
                    if cursor < buffer.len() {
                        buffer.remove(cursor);
                        suggestion = self.get_suggestion(&buffer);
                        self.redraw_line(&buffer, cursor, &suggestion);
                    }
                }
                Key::Left => {
                    if cursor > 0 {
                        cursor -= 1;
                        self.redraw_line(&buffer, cursor, &suggestion);
                    }
                }
                Key::Right => {
                    if !suggestion.is_empty() {
                        // 接受建议
                        buffer.push_str(&suggestion);
                        cursor = buffer.len();
                        suggestion.clear();
                        self.redraw_line(&buffer, cursor, &suggestion);
                    } else if cursor < buffer.len() {
                        cursor += 1;
                        self.redraw_line(&buffer, cursor, &suggestion);
                    }
                }
                Key::Tab => {
                    if !suggestion.is_empty() {
                        buffer.push_str(&suggestion);
                        cursor = buffer.len();
                        suggestion = self.get_suggestion(&buffer);
                        self.redraw_line(&buffer, cursor, &suggestion);
                    }
                }
                Key::Up => {
                    if self.history_index > 0 {
                        self.history_index -= 1;
                        buffer = self.history[self.history_index].clone();
                        cursor = buffer.len();
                        suggestion = self.get_suggestion(&buffer);
                        self.redraw_line(&buffer, cursor, &suggestion);
                    }
                }
                Key::Down => {
                    if self.history_index < self.history.len() {
                        self.history_index += 1;
                        if self.history_index < self.history.len() {
                            buffer = self.history[self.history_index].clone();
                        } else {
                            buffer.clear();
                        }
                        cursor = buffer.len();
                        suggestion = self.get_suggestion(&buffer);
                        self.redraw_line(&buffer, cursor, &suggestion);
                    }
                }
                Key::Home => {
                    cursor = 0;
                    self.redraw_line(&buffer, cursor, &suggestion);
                }
                Key::End => {
                    cursor = buffer.len();
                    self.redraw_line(&buffer, cursor, &suggestion);
                }
                Key::Enter => {
                    println!();
                    return Ok(Some(buffer));
                }
                Key::CtrlC => {
                    println!("^C");
                    buffer.clear();
                    cursor = 0;
                    suggestion.clear();
                    self.print_prompt();
                }
                Key::CtrlD => {
                    if buffer.is_empty() {
                        return Ok(None);
                    }
                }
                Key::CtrlU => {
                    buffer.clear();
                    cursor = 0;
                    suggestion.clear();
                    self.redraw_line(&buffer, cursor, &suggestion);
                }
                Key::CtrlW => {
                    // 删除前一个词
                    while cursor > 0 && buffer.chars().nth(cursor - 1) == Some(' ') {
                        cursor -= 1;
                        buffer.remove(cursor);
                    }
                    while cursor > 0 && buffer.chars().nth(cursor - 1) != Some(' ') {
                        cursor -= 1;
                        buffer.remove(cursor);
                    }
                    suggestion = self.get_suggestion(&buffer);
                    self.redraw_line(&buffer, cursor, &suggestion);
                }
                _ => {}
            }
        }
    }
    
    /// 打印提示符
    fn print_prompt(&self) {
        let cwd = std::env::current_dir()
            .map(|p| {
                let home = dirs::home_dir().unwrap_or_default();
                if p.starts_with(&home) {
                    format!("~{}", p.strip_prefix(&home).unwrap().display())
                } else {
                    p.display().to_string()
                }
            })
            .unwrap_or_else(|_| "?".to_string());
        
        print!("{}{}cnmsb{} {}{}>{} ", 
            term::BOLD, term::CYAN, term::RESET,
            term::GREEN, cwd, term::RESET);
        let _ = stdout().flush();
    }
    
    /// 获取建议后缀
    fn get_suggestion(&self, buffer: &str) -> String {
        if buffer.is_empty() {
            return String::new();
        }
        
        let completions = self.engine.complete(buffer, buffer.len());
        
        if let Some(first) = completions.first() {
            let text = &first.text;
            
            // 如果补全是当前输入的扩展
            if text.starts_with(buffer) {
                return text[buffer.len()..].to_string();
            }
            
            // 如果补全是当前词的扩展
            let words: Vec<&str> = buffer.split_whitespace().collect();
            if let Some(last_word) = words.last() {
                if text.starts_with(last_word) {
                    return text[last_word.len()..].to_string();
                }
            }
            
            // 如果以空格结尾
            if buffer.ends_with(' ') {
                return text.clone();
            }
        }
        
        String::new()
    }
    
    /// 重绘当前行
    fn redraw_line(&self, buffer: &str, cursor: usize, suggestion: &str) {
        print!("{}{}", term::CLEAR_LINE, term::CURSOR_START);
        self.print_prompt();
        print!("{}", buffer);
        
        if !suggestion.is_empty() {
            print!("{}{}{}", term::GRAY, suggestion, term::RESET);
        }
        
        // 移动光标到正确位置
        let prompt_len = self.get_prompt_len();
        let cursor_pos = prompt_len + cursor + 1;
        print!("\x1b[{}G", cursor_pos);
        
        let _ = stdout().flush();
    }
    
    fn get_prompt_len(&self) -> usize {
        let cwd = std::env::current_dir()
            .map(|p| {
                let home = dirs::home_dir().unwrap_or_default();
                if p.starts_with(&home) {
                    format!("~{}", p.strip_prefix(&home).unwrap().display())
                } else {
                    p.display().to_string()
                }
            })
            .unwrap_or_else(|_| "?".to_string());
        
        // "cnmsb " + cwd + "> "
        6 + cwd.len() + 2
    }
    
    /// 执行命令
    fn execute(&self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }
        
        let cmd = parts[0];
        let args = &parts[1..];
        
        // 内置命令
        match cmd {
            "cd" => {
                let dir = args.first().map(|s| *s).unwrap_or("~");
                let dir = if dir == "~" {
                    dirs::home_dir().unwrap_or_default()
                } else {
                    std::path::PathBuf::from(dir)
                };
                if let Err(e) = std::env::set_current_dir(&dir) {
                    eprintln!("cd: {}: {}", dir.display(), e);
                }
            }
            "exit" | "quit" => {
                std::process::exit(0);
            }
            "history" => {
                for (i, h) in self.history.iter().enumerate() {
                    println!("{:4}  {}", i + 1, h);
                }
            }
            _ => {
                // 执行外部命令
                match Command::new(cmd).args(args).spawn() {
                    Ok(mut child) => {
                        let _ = child.wait();
                    }
                    Err(e) => {
                        eprintln!("{}: {}", cmd, e);
                    }
                }
            }
        }
    }
    
    /// 读取按键
    #[cfg(unix)]
    fn read_key(&self) -> io::Result<Key> {
        let mut buf = [0u8; 8];
        let n = stdin().read(&mut buf)?;
        
        if n == 0 {
            return Ok(Key::CtrlD);
        }
        
        match buf[0] {
            3 => Ok(Key::CtrlC),      // Ctrl+C
            4 => Ok(Key::CtrlD),      // Ctrl+D
            9 => Ok(Key::Tab),        // Tab
            13 => Ok(Key::Enter),     // Enter
            21 => Ok(Key::CtrlU),     // Ctrl+U
            23 => Ok(Key::CtrlW),     // Ctrl+W
            27 => {
                // 转义序列
                if n >= 3 && buf[1] == b'[' {
                    match buf[2] {
                        b'A' => Ok(Key::Up),
                        b'B' => Ok(Key::Down),
                        b'C' => Ok(Key::Right),
                        b'D' => Ok(Key::Left),
                        b'H' => Ok(Key::Home),
                        b'F' => Ok(Key::End),
                        b'3' if n >= 4 && buf[3] == b'~' => Ok(Key::Delete),
                        _ => Ok(Key::Unknown),
                    }
                } else {
                    Ok(Key::Escape)
                }
            }
            127 => Ok(Key::Backspace),
            c if c >= 32 && c < 127 => Ok(Key::Char(c as char)),
            _ => Ok(Key::Unknown),
        }
    }
    
    #[cfg(windows)]
    fn read_key(&self) -> io::Result<Key> {
        use std::io::Read;
        let mut buf = [0u8; 1];
        stdin().read_exact(&mut buf)?;
        
        match buf[0] {
            3 => Ok(Key::CtrlC),
            4 => Ok(Key::CtrlD),
            9 => Ok(Key::Tab),
            13 => Ok(Key::Enter),
            8 | 127 => Ok(Key::Backspace),
            c if c >= 32 && c < 127 => Ok(Key::Char(c as char)),
            _ => Ok(Key::Unknown),
        }
    }
}

/// 按键类型
#[derive(Debug)]
enum Key {
    Char(char),
    Tab,
    Enter,
    Backspace,
    Delete,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    Escape,
    CtrlC,
    CtrlD,
    CtrlU,
    CtrlW,
    Unknown,
}

/// Unix 终端原始模式
#[cfg(unix)]
struct RawMode {
    original: libc::termios,
}

#[cfg(unix)]
impl RawMode {
    fn enable() -> io::Result<Self> {
        use std::mem::MaybeUninit;
        use std::os::unix::io::AsRawFd;
        
        let fd = stdin().as_raw_fd();
        let mut termios = MaybeUninit::uninit();
        
        if unsafe { libc::tcgetattr(fd, termios.as_mut_ptr()) } != 0 {
            return Err(io::Error::last_os_error());
        }
        
        let original = unsafe { termios.assume_init() };
        let mut raw = original;
        
        // 禁用规范模式和回显
        raw.c_lflag &= !(libc::ICANON | libc::ECHO);
        raw.c_cc[libc::VMIN] = 1;
        raw.c_cc[libc::VTIME] = 0;
        
        if unsafe { libc::tcsetattr(fd, libc::TCSANOW, &raw) } != 0 {
            return Err(io::Error::last_os_error());
        }
        
        Ok(RawMode { original })
    }
}

#[cfg(unix)]
impl Drop for RawMode {
    fn drop(&mut self) {
        use std::os::unix::io::AsRawFd;
        let fd = stdin().as_raw_fd();
        unsafe { libc::tcsetattr(fd, libc::TCSANOW, &self.original) };
    }
}

impl Default for CnmsbShell {
    fn default() -> Self {
        Self::new()
    }
}

