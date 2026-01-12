//! SQL 交互式 Shell

use super::connection::{DbConnection, QueryResult};
use super::database::DatabaseType;
use super::engine::SqlEngine;
use super::syntax::SqlCompletionKind;
use std::io::{self, Read, Write, stdout, stdin};

/// 终端控制序列
mod term {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const GRAY: &str = "\x1b[38;5;240m";
    pub const YELLOW: &str = "\x1b[38;5;226m";
    pub const GREEN: &str = "\x1b[38;5;40m";
    pub const BLUE: &str = "\x1b[38;5;33m";
    pub const CYAN: &str = "\x1b[38;5;117m";
    pub const ORANGE: &str = "\x1b[38;5;208m";
    pub const RED: &str = "\x1b[38;5;196m";
    
    pub const CLEAR_LINE: &str = "\x1b[2K";
    pub const CURSOR_START: &str = "\x1b[G";
    pub const CLEAR_BELOW: &str = "\x1b[J";
    pub const MOVE_UP: &str = "\x1b[A";
}

/// SQL Shell
pub struct SqlShell {
    engine: SqlEngine,
    connection: DbConnection,
    history: Vec<String>,
    history_index: usize,
    connected: bool,
}

impl SqlShell {
    /// 创建新的 SQL Shell
    pub fn new(db_type: DatabaseType) -> Self {
        SqlShell {
            engine: SqlEngine::new(db_type),
            connection: DbConnection::None,
            history: Vec::new(),
            history_index: 0,
            connected: false,
        }
    }
    
    /// 连接数据库
    pub fn connect(&mut self, conn_str: &str) -> Result<(), String> {
        let db_type = self.engine.db_type();
        
        let result = match db_type {
            DatabaseType::SQLite => {
                // SQLite: 路径或 :memory:
                DbConnection::connect_sqlite(conn_str)
            }
            _ => {
                return Err(format!("当前仅支持 SQLite 数据库连接（{:?} 需要更新 Rust 版本）", db_type));
            }
        };
        
        match result {
            Ok(conn) => {
                self.connection = conn;
                self.connected = true;
                
                // 获取表列表用于补全
                if let Ok(tables) = self.connection.get_tables() {
                    self.engine.set_tables(tables);
                }
                
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    
    
    /// 运行 SQL Shell
    pub fn run(&mut self) -> io::Result<()> {
        self.print_welcome();
        
        // 如果未连接，提示连接
        if !self.connected {
            self.prompt_connect()?;
        }
        
        // 设置终端为原始模式
        #[cfg(unix)]
        let _raw_mode = RawMode::enable()?;
        
        loop {
            match self.read_line()? {
                Some(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    
                    // 处理特殊命令
                    if self.handle_command(line) {
                        continue;
                    }
                    
                    // 检查退出命令
                    if line.eq_ignore_ascii_case("exit") || 
                       line.eq_ignore_ascii_case("quit") ||
                       line.eq_ignore_ascii_case("\\q") {
                        println!("\n再见！");
                        break;
                    }
                    
                    // 执行 SQL
                    self.execute_sql(&line);
                    
                    // 保存到历史
                    self.history.push(line.to_string());
                    self.history_index = self.history.len();
                }
                None => {
                    println!("\n再见！");
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// 提示连接数据库
    fn prompt_connect(&mut self) -> io::Result<()> {
        let db_type = self.engine.db_type();
        
        println!();
        match db_type {
            DatabaseType::SQLite => {
                println!("{}请输入 SQLite 数据库路径（直接回车使用内存数据库）:{}", term::YELLOW, term::RESET);
                print!("> ");
                stdout().flush()?;
                
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                let path = input.trim();
                
                match self.connect(path) {
                    Ok(()) => {
                        if path.is_empty() || path == ":memory:" {
                            println!("{}✓ 已连接到内存数据库{}", term::GREEN, term::RESET);
                        } else {
                            println!("{}✓ 已连接到 {}{}", term::GREEN, path, term::RESET);
                        }
                    }
                    Err(e) => {
                        println!("{}✗ 连接失败: {}{}", term::RED, e, term::RESET);
                        return Ok(());
                    }
                }
            }
            _ => {
                println!("{}当前仅支持 SQLite 数据库（{:?} 需要更新 Rust 版本）{}", term::YELLOW, db_type, term::RESET);
                println!("{}将以离线模式运行（仅提供语法补全）{}", term::GRAY, term::RESET);
            }
        }
        println!();
        
        Ok(())
    }
    
    /// 执行 SQL 语句
    fn execute_sql(&mut self, sql: &str) {
        if !self.connected {
            println!("\n{}未连接数据库，无法执行 SQL{}", term::YELLOW, term::RESET);
            println!("{}提示: 使用 \\c 或 \\connect 连接数据库{}\n", term::GRAY, term::RESET);
            return;
        }
        
        match self.connection.execute(sql) {
            Ok(result) => {
                self.display_result(&result);
            }
            Err(e) => {
                println!("\n{}错误: {}{}\n", term::RED, e, term::RESET);
            }
        }
    }
    
    /// 显示查询结果
    fn display_result(&self, result: &QueryResult) {
        println!();
        
        if let Some(ref msg) = result.message {
            println!("{}{}{}", term::GREEN, msg, term::RESET);
            println!();
            return;
        }
        
        if result.columns.is_empty() {
            return;
        }
        
        // 计算列宽
        let mut widths: Vec<usize> = result.columns.iter().map(|c| c.len()).collect();
        for row in &result.rows {
            for (i, val) in row.iter().enumerate() {
                if i < widths.len() && val.len() > widths[i] {
                    widths[i] = val.len();
                }
            }
        }
        
        // 打印分隔线
        let sep: String = widths.iter().map(|w| "-".repeat(*w + 2)).collect::<Vec<_>>().join("+");
        println!("+{}+", sep);
        
        // 打印表头
        let header: String = result.columns.iter().enumerate()
            .map(|(i, c)| format!(" {}{:<width$}{} ", term::CYAN, c, term::RESET, width = widths[i]))
            .collect::<Vec<_>>().join("|");
        println!("|{}|", header);
        println!("+{}+", sep);
        
        // 打印数据行
        for row in &result.rows {
            let line: String = row.iter().enumerate()
                .map(|(i, v)| format!(" {:<width$} ", v, width = widths.get(i).copied().unwrap_or(10)))
                .collect::<Vec<_>>().join("|");
            println!("|{}|", line);
        }
        println!("+{}+", sep);
        
        // 打印行数
        println!("{}{} row(s) in set{}", term::GRAY, result.rows.len(), term::RESET);
        println!();
    }
    
    /// 打印欢迎信息
    fn print_welcome(&self) {
        let db_type = self.engine.db_type();
        
        println!();
        println!("{}{}╔══════════════════════════════════════════════════════════════╗{}", term::BOLD, db_type.color(), term::RESET);
        println!("{}{}║              cnmsb-sql - SQL 智能补全终端                    ║{}", term::BOLD, db_type.color(), term::RESET);
        println!("{}{}╚══════════════════════════════════════════════════════════════╝{}", term::BOLD, db_type.color(), term::RESET);
        println!();
        println!("  数据库类型: {}{}{}{}", term::BOLD, db_type.color(), db_type.name(), term::RESET);
        println!("  {}{}", term::GRAY, db_type.description());
        println!("{}", term::RESET);
        println!("{}快捷键:{}", term::YELLOW, term::RESET);
        println!("  {}Tab{}        补全 SQL 关键字/函数", term::CYAN, term::RESET);
        println!("  {}↑ ↓{}        浏览历史", term::CYAN, term::RESET);
        println!("  {}Ctrl+C{}     取消当前输入", term::CYAN, term::RESET);
        println!("  {}Ctrl+D{}     退出", term::CYAN, term::RESET);
        println!();
        println!("{}命令:{}", term::YELLOW, term::RESET);
        println!("  {}.help{}      显示帮助", term::GREEN, term::RESET);
        println!("  {}.tables{}    显示表列表（模拟）", term::GREEN, term::RESET);
        println!("  {}.clear{}     清屏", term::GREEN, term::RESET);
        println!("  {}exit/quit{} 退出", term::GREEN, term::RESET);
        println!();
    }
    
    /// 处理特殊命令
    fn handle_command(&mut self, line: &str) -> bool {
        let lower = line.to_lowercase();
        
        if lower == ".help" || lower == "\\?" || lower == "help" {
            self.print_help();
            return true;
        }
        
        if lower == ".clear" || lower == "clear" {
            print!("\x1b[2J\x1b[H");
            let _ = stdout().flush();
            return true;
        }
        
        if lower == ".tables" || lower == "\\dt" || lower == "\\d" {
            self.show_tables();
            return true;
        }
        
        if lower.starts_with("\\c ") || lower.starts_with(".connect ") || lower.starts_with("connect ") {
            let conn_str = if lower.starts_with("\\c ") {
                &line[3..]
            } else if lower.starts_with(".connect ") {
                &line[9..]
            } else {
                &line[8..]
            };
            self.reconnect(conn_str.trim());
            return true;
        }
        
        if lower == "\\c" || lower == ".connect" || lower == "connect" {
            // 重新连接提示
            let _ = self.prompt_connect();
            return true;
        }
        
        if lower == ".status" || lower == "\\s" || lower == "status" {
            self.show_status();
            return true;
        }
        
        false
    }
    
    /// 重新连接数据库
    fn reconnect(&mut self, conn_str: &str) {
        match self.connect(conn_str) {
            Ok(()) => {
                println!("\n{}✓ 已成功连接数据库{}\n", term::GREEN, term::RESET);
            }
            Err(e) => {
                println!("\n{}✗ 连接失败: {}{}\n", term::RED, e, term::RESET);
            }
        }
    }
    
    /// 显示连接状态
    fn show_status(&self) {
        println!();
        println!("{}连接状态:{}", term::YELLOW, term::RESET);
        println!("  数据库类型: {}{}{}", term::CYAN, self.engine.db_type().name(), term::RESET);
        if self.connected {
            println!("  状态: {}已连接{}", term::GREEN, term::RESET);
        } else {
            println!("  状态: {}未连接{}", term::RED, term::RESET);
        }
        println!();
    }
    
    /// 打印帮助
    fn print_help(&self) {
        let db_type = self.engine.db_type();
        println!();
        println!("{}{} SQL 帮助{}", term::BOLD, db_type.name(), term::RESET);
        println!();
        println!("{}Shell 命令:{}", term::YELLOW, term::RESET);
        println!("  {}\\c, .connect{}   连接/重连数据库", term::CYAN, term::RESET);
        println!("  {}\\d, .tables{}    显示所有表", term::CYAN, term::RESET);
        println!("  {}\\s, .status{}    显示连接状态", term::CYAN, term::RESET);
        println!("  {}.clear{}         清屏", term::CYAN, term::RESET);
        println!("  {}exit, \\q{}       退出", term::CYAN, term::RESET);
        println!();
        println!("{}常用 SQL:{}", term::YELLOW, term::RESET);
        println!("  {}SELECT{} * FROM table_name WHERE condition", term::BLUE, term::RESET);
        println!("  {}INSERT INTO{} table_name (col1, col2) VALUES (val1, val2)", term::BLUE, term::RESET);
        println!("  {}UPDATE{} table_name SET col = value WHERE condition", term::BLUE, term::RESET);
        println!("  {}DELETE FROM{} table_name WHERE condition", term::BLUE, term::RESET);
        println!("  {}CREATE TABLE{} table_name (col1 TYPE, col2 TYPE)", term::BLUE, term::RESET);
        println!();
        println!("按 {}Tab{} 查看补全建议", term::CYAN, term::RESET);
        println!();
    }
    
    /// 显示表列表
    fn show_tables(&mut self) {
        println!();
        
        if !self.connected {
            println!("{}未连接数据库，无法获取表列表{}", term::YELLOW, term::RESET);
            println!("{}提示: 使用 \\c 或 .connect 连接数据库{}\n", term::GRAY, term::RESET);
            return;
        }
        
        match self.connection.get_tables() {
            Ok(tables) => {
                if tables.is_empty() {
                    println!("{}当前数据库没有表{}", term::GRAY, term::RESET);
                } else {
                    println!("{}表列表 ({} 个表):{}", term::YELLOW, tables.len(), term::RESET);
                    println!();
                    for table in &tables {
                        println!("  {}{}{}", term::GREEN, table, term::RESET);
                    }
                }
            }
            Err(e) => {
                println!("{}获取表列表失败: {}{}", term::RED, e, term::RESET);
            }
        }
        println!();
        println!("{}提示:{} 实际连接数据库后，这里会显示真实的表列表", term::GRAY, term::RESET);
        println!();
        
        // 添加示例表到引擎
        // 这里只是演示，实际应该从数据库获取
    }
    
    /// 显示 SQL（用于演示）
    
    /// 读取一行输入
    fn read_line(&mut self) -> io::Result<Option<String>> {
        let mut buffer = String::new();
        let mut cursor = 0usize;
        let mut suggestion = String::new();
        let mut show_menu = false;
        let mut menu_index = 0usize;
        let mut menu_items: Vec<(String, String, SqlCompletionKind)> = Vec::new();
        
        self.print_prompt();
        
        loop {
            stdout().flush()?;
            
            let key = self.read_key()?;
            
            match key {
                Key::Char(c) => {
                    buffer.insert(cursor, c);
                    cursor += 1;
                    suggestion = self.get_suggestion(&buffer);
                    show_menu = false;
                    self.redraw_line(&buffer, cursor, &suggestion, None);
                }
                Key::Backspace => {
                    if cursor > 0 {
                        cursor -= 1;
                        buffer.remove(cursor);
                        suggestion = self.get_suggestion(&buffer);
                        show_menu = false;
                        self.redraw_line(&buffer, cursor, &suggestion, None);
                    }
                }
                Key::Delete => {
                    if cursor < buffer.len() {
                        buffer.remove(cursor);
                        suggestion = self.get_suggestion(&buffer);
                        show_menu = false;
                        self.redraw_line(&buffer, cursor, &suggestion, None);
                    }
                }
                Key::Left => {
                    if cursor > 0 {
                        cursor -= 1;
                        self.redraw_line(&buffer, cursor, &suggestion, if show_menu { Some((&menu_items, menu_index)) } else { None });
                    }
                }
                Key::Right => {
                    if !suggestion.is_empty() {
                        // 接受建议
                        buffer.push_str(&suggestion);
                        cursor = buffer.len();
                        suggestion.clear();
                        show_menu = false;
                        self.redraw_line(&buffer, cursor, &suggestion, None);
                    } else if cursor < buffer.len() {
                        cursor += 1;
                        self.redraw_line(&buffer, cursor, &suggestion, if show_menu { Some((&menu_items, menu_index)) } else { None });
                    }
                }
                Key::Tab => {
                    if show_menu && !menu_items.is_empty() {
                        // 第二次 Tab：接受选中的菜单项
                        let (text, _, _) = &menu_items[menu_index];
                        
                        // 计算要替换的词
                        let current_word = buffer.split_whitespace().last().unwrap_or("").to_string();
                        let current_word_upper = current_word.to_uppercase();
                        
                        if text.to_uppercase().starts_with(&current_word_upper) {
                            // 删除当前词
                            for _ in 0..current_word.len() {
                                if cursor > 0 {
                                    cursor -= 1;
                                    buffer.remove(cursor);
                                }
                            }
                            
                            // 根据用户的大小写风格调整补全文本
                            let styled_text = if current_word.chars().all(|c| c.is_lowercase() || !c.is_alphabetic()) {
                                text.to_lowercase()
                            } else if current_word.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
                                text.to_uppercase()
                            } else {
                                // 混合大小写，默认大写
                                text.to_uppercase()
                            };
                            
                            buffer.push_str(&styled_text);
                            cursor = buffer.len();
                        } else {
                            buffer.push_str(text);
                            cursor = buffer.len();
                        }
                        
                        suggestion.clear();
                        show_menu = false;
                        self.redraw_line(&buffer, cursor, &suggestion, None);
                        
                        // 补全后继续获取新的建议
                        suggestion = self.get_suggestion(&buffer);
                        if !suggestion.is_empty() {
                            self.redraw_line(&buffer, cursor, &suggestion, None);
                        }
                    } else {
                        // 第一次 Tab：显示补全菜单
                        let completions = self.engine.complete(&buffer, cursor);
                        if !completions.is_empty() {
                            menu_items = completions.iter()
                                .take(10)
                                .map(|c| (c.text.clone(), c.description.clone(), c.kind))
                                .collect();
                            menu_index = 0;
                            show_menu = true;
                            self.redraw_line(&buffer, cursor, "", Some((&menu_items, menu_index)));
                        }
                    }
                }
                Key::Up => {
                    if show_menu && !menu_items.is_empty() {
                        // 菜单导航
                        if menu_index > 0 {
                            menu_index -= 1;
                        } else {
                            menu_index = menu_items.len() - 1;
                        }
                        self.redraw_line(&buffer, cursor, &suggestion, Some((&menu_items, menu_index)));
                    } else if self.history_index > 0 {
                        // 历史导航
                        self.history_index -= 1;
                        buffer = self.history[self.history_index].clone();
                        cursor = buffer.len();
                        suggestion = self.get_suggestion(&buffer);
                        self.redraw_line(&buffer, cursor, &suggestion, None);
                    }
                }
                Key::Down => {
                    if show_menu && !menu_items.is_empty() {
                        // 菜单导航
                        if menu_index < menu_items.len() - 1 {
                            menu_index += 1;
                        } else {
                            menu_index = 0;
                        }
                        self.redraw_line(&buffer, cursor, &suggestion, Some((&menu_items, menu_index)));
                    } else if self.history_index < self.history.len() {
                        // 历史导航
                        self.history_index += 1;
                        if self.history_index < self.history.len() {
                            buffer = self.history[self.history_index].clone();
                        } else {
                            buffer.clear();
                        }
                        cursor = buffer.len();
                        suggestion = self.get_suggestion(&buffer);
                        self.redraw_line(&buffer, cursor, &suggestion, None);
                    }
                }
                Key::Home => {
                    cursor = 0;
                    self.redraw_line(&buffer, cursor, &suggestion, if show_menu { Some((&menu_items, menu_index)) } else { None });
                }
                Key::End => {
                    cursor = buffer.len();
                    self.redraw_line(&buffer, cursor, &suggestion, if show_menu { Some((&menu_items, menu_index)) } else { None });
                }
                Key::Enter => {
                    // 清除菜单和建议显示
                    print!("{}", term::CLEAR_BELOW);
                    let _ = stdout().flush();
                    println!();
                    return Ok(Some(buffer));
                }
                Key::CtrlC => {
                    if show_menu {
                        self.clear_menu(menu_items.len());
                    }
                    println!("^C");
                    buffer.clear();
                    cursor = 0;
                    suggestion.clear();
                    show_menu = false;
                    self.print_prompt();
                }
                Key::CtrlD => {
                    if buffer.is_empty() {
                        return Ok(None);
                    }
                }
                Key::Escape => {
                    // 取消菜单
                    if show_menu {
                        show_menu = false;
                        self.redraw_line(&buffer, cursor, &suggestion, None);
                    }
                }
                Key::CtrlU => {
                    if show_menu {
                        self.clear_menu(menu_items.len());
                    }
                    buffer.clear();
                    cursor = 0;
                    suggestion.clear();
                    show_menu = false;
                    self.redraw_line(&buffer, cursor, &suggestion, None);
                }
                Key::CtrlW => {
                    // 删除前一个词
                    if show_menu {
                        self.clear_menu(menu_items.len());
                        show_menu = false;
                    }
                    while cursor > 0 && buffer.chars().nth(cursor - 1) == Some(' ') {
                        cursor -= 1;
                        buffer.remove(cursor);
                    }
                    while cursor > 0 && buffer.chars().nth(cursor - 1) != Some(' ') {
                        cursor -= 1;
                        buffer.remove(cursor);
                    }
                    suggestion = self.get_suggestion(&buffer);
                    self.redraw_line(&buffer, cursor, &suggestion, None);
                }
                _ => {}
            }
        }
    }
    
    /// 打印提示符
    fn print_prompt(&self) {
        let db_type = self.engine.db_type();
        print!("{}{}{}{} > {}", 
            term::BOLD, db_type.color(), db_type.prompt(), term::RESET, term::RESET);
        let _ = stdout().flush();
    }
    
    /// 获取建议后缀
    fn get_suggestion(&self, buffer: &str) -> String {
        if buffer.is_empty() {
            return String::new();
        }
        
        if let Some(suffix) = self.engine.get_current_word_completion(buffer, buffer.len()) {
            return suffix;
        }
        
        String::new()
    }
    
    /// 重绘当前行
    fn redraw_line(
        &self, 
        buffer: &str, 
        cursor: usize, 
        suggestion: &str,
        menu: Option<(&Vec<(String, String, SqlCompletionKind)>, usize)>,
    ) {
        // 先清除当前行下方的所有内容
        print!("{}", term::CLEAR_BELOW);
        
        // 清除并重绘当前行
        print!("{}{}", term::CLEAR_LINE, term::CURSOR_START);
        self.print_prompt();
        print!("{}", buffer);
        
        // 只有在没有菜单时才显示内联建议
        if menu.is_none() && !suggestion.is_empty() {
            print!("{}{}{}", term::GRAY, suggestion, term::RESET);
        }
        
        // 显示菜单
        if let Some((items, selected)) = menu {
            // 清除行尾
            print!("{}", term::CLEAR_BELOW);
            println!();
            
            for (i, (text, desc, kind)) in items.iter().enumerate() {
                print!("{}", term::CLEAR_LINE);
                if i == selected {
                    print!("  {}> {}{:<20}{} {}{}{}", 
                        term::BOLD, kind.color(), text, term::RESET, 
                        term::GRAY, desc, term::RESET);
                } else {
                    print!("    {}{:<20}{} {}{}{}", 
                        kind.color(), text, term::RESET, 
                        term::GRAY, desc, term::RESET);
                }
                println!();
            }
            print!("{}  {}[Tab=确认  ↑↓=选择  →=接受  Esc=取消]{}", 
                term::CLEAR_LINE, term::YELLOW, term::RESET);
            
            // 移动光标回到输入行
            let menu_lines = items.len() + 1;
            for _ in 0..menu_lines {
                print!("{}", term::MOVE_UP);
            }
        }
        
        // 移动光标到正确位置
        let prompt_len = self.get_prompt_len();
        let cursor_pos = prompt_len + cursor + 1;
        print!("\x1b[{}G", cursor_pos);
        
        let _ = stdout().flush();
    }
    
    fn get_prompt_len(&self) -> usize {
        // "mysql > " 等
        self.engine.db_type().prompt().len() + 3
    }
    
    /// 清除菜单
    fn clear_menu(&self, menu_lines: usize) {
        // 移动到菜单区域并清除
        for _ in 0..=menu_lines {
            println!();
        }
        for _ in 0..=menu_lines {
            print!("{}{}{}", term::MOVE_UP, term::CLEAR_LINE, term::CURSOR_START);
        }
        let _ = stdout().flush();
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
            3 => Ok(Key::CtrlC),
            4 => Ok(Key::CtrlD),
            9 => Ok(Key::Tab),
            13 => Ok(Key::Enter),
            21 => Ok(Key::CtrlU),
            23 => Ok(Key::CtrlW),
            27 => {
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
        let mut buf = [0u8; 1];
        stdin().read_exact(&mut buf)?;
        
        match buf[0] {
            3 => Ok(Key::CtrlC),
            4 => Ok(Key::CtrlD),
            9 => Ok(Key::Tab),
            13 => Ok(Key::Enter),
            8 | 127 => Ok(Key::Backspace),
            27 => Ok(Key::Escape),
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

/// 显示数据库选择菜单
pub fn select_database() -> Option<DatabaseType> {
    println!();
    println!("{}{}╔══════════════════════════════════════════════════════════════╗{}", term::BOLD, term::YELLOW, term::RESET);
    println!("{}{}║            cnmsb-sql - 选择数据库类型                         ║{}", term::BOLD, term::YELLOW, term::RESET);
    println!("{}{}╚══════════════════════════════════════════════════════════════╝{}", term::BOLD, term::YELLOW, term::RESET);
    println!();
    
    let databases = DatabaseType::all();
    for (i, db) in databases.iter().enumerate() {
        println!("  {}{}{}[{}]{} {}{:<15}{} - {}", 
            term::BOLD, db.color(), term::RESET,
            i + 1,
            term::RESET,
            db.color(), db.name(), term::RESET,
            db.description()
        );
    }
    
    println!();
    print!("{}请选择 (1-{}): {}", term::YELLOW, databases.len(), term::RESET);
    let _ = stdout().flush();
    
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return None;
    }
    
    let input = input.trim();
    
    // 尝试解析
    DatabaseType::from_str(input)
}

