//! SQL 交互式 Shell（使用 rustyline）

use super::connection::{DbConnection, QueryResult};
use super::database::{DatabaseType, DatabaseConfig};
use super::engine::SqlEngine;
use std::io::{self, stdout, stdin, Write};
use std::borrow::Cow;

use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};

/// 终端控制序列
mod term {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const GRAY: &str = "\x1b[38;5;240m";
    pub const YELLOW: &str = "\x1b[38;5;226m";
    pub const GREEN: &str = "\x1b[38;5;40m";
    pub const BLUE: &str = "\x1b[38;5;33m";
    pub const CYAN: &str = "\x1b[38;5;117m";
    pub const RED: &str = "\x1b[38;5;196m";
}

/// SQL 补全辅助器
struct SqlHelper {
    engine: SqlEngine,
}

impl SqlHelper {
    fn new(db_type: DatabaseType) -> Self {
        SqlHelper {
            engine: SqlEngine::new(db_type),
        }
    }
    
    fn set_tables(&mut self, tables: Vec<String>) {
        self.engine.set_tables(tables);
    }
    
    fn set_columns(&mut self, table: &str, columns: Vec<String>) {
        self.engine.set_columns(table, columns);
    }
}

impl Completer for SqlHelper {
    type Candidate = Pair;
    
    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let completions = self.engine.complete(line, pos);
        
        // 找到当前词的开始位置
        let start = find_word_start(line, pos);
        
        let pairs: Vec<Pair> = completions.iter()
            .map(|c| {
                let display = format!("{} {}{}{}", 
                    c.text, term::GRAY, c.description, term::RESET);
                Pair {
                    display,
                    replacement: c.text.clone(),
                }
            })
            .collect();
        
        Ok((start, pairs))
    }
}

impl Hinter for SqlHelper {
    type Hint = String;
    
    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<String> {
        // 只在光标位于行尾时显示建议
        if line.is_empty() || pos != line.len() {
            return None;
        }
        
        self.engine.get_current_word_completion(line, pos)
    }
}

impl Highlighter for SqlHelper {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Owned(format!("{}{}{}", term::GRAY, hint, term::RESET))
    }
}

impl Validator for SqlHelper {}

impl Helper for SqlHelper {}

/// 找到当前词的开始位置
fn find_word_start(line: &str, pos: usize) -> usize {
    let bytes = line.as_bytes();
    let mut start = pos;
    
    while start > 0 {
        let c = bytes[start - 1];
        if c.is_ascii_alphanumeric() || c == b'_' || c == b'.' {
            start -= 1;
        } else {
            break;
        }
    }
    
    start
}

/// SQL Shell
pub struct SqlShell {
    db_type: DatabaseType,
    connection: DbConnection,
    connected: bool,
}

impl SqlShell {
    /// 创建新的 SQL Shell
    pub fn new(db_type: DatabaseType) -> Self {
        SqlShell {
            db_type,
            connection: DbConnection::None,
            connected: false,
        }
    }
    
    /// 连接数据库
    pub fn connect(&mut self, conn_str: &str) -> Result<(), String> {
        let result = match self.db_type {
            DatabaseType::SQLite => {
                DbConnection::connect_sqlite(conn_str)
            }
            DatabaseType::MySQL | DatabaseType::MariaDB => {
                if let Some(config) = DatabaseConfig::parse(conn_str) {
                    DbConnection::connect_mysql(&config.host, config.port, &config.username, &config.password, &config.database)
                } else {
                    DbConnection::connect_mysql_url(conn_str)
                }
            }
            DatabaseType::PostgreSQL => {
                if let Some(config) = DatabaseConfig::parse(conn_str) {
                    DbConnection::connect_postgres(&config.host, config.port, &config.username, &config.password, &config.database)
                } else {
                    DbConnection::connect_postgres_url(conn_str)
                }
            }
            _ => {
                return Err(format!("{} 数据库暂不支持", self.db_type.name()));
            }
        };
        
        match result {
            Ok(conn) => {
                self.connection = conn;
                self.connected = true;
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
        
        // 创建 rustyline Editor
        let mut helper = SqlHelper::new(self.db_type);
        
        // 加载 Schema 信息
        if self.connected {
            if let Ok(tables) = self.connection.get_tables() {
                helper.set_tables(tables.clone());
                for table in &tables {
                    if let Ok(columns) = self.connection.get_columns(table) {
                        let col_names: Vec<String> = columns.iter().map(|c| c.name.clone()).collect();
                        helper.set_columns(table, col_names);
                    }
                }
            }
        }
        
        let mut rl = Editor::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        rl.set_helper(Some(helper));
        
        // 构造提示符
        let prompt = format!("{}{}{}{} > ", 
            term::BOLD, self.db_type.color(), self.db_type.prompt(), term::RESET);
        
        loop {
            match rl.readline(&prompt) {
                Ok(line) => {
                    let line = line.trim();
                    
                    if line.is_empty() {
                        continue;
                    }
                    
                    // 添加到历史
                    let _ = rl.add_history_entry(line);
                    
                    // 处理特殊命令
                    if self.handle_command(line, &mut rl) {
                        continue;
                    }
                    
                    // 检查退出命令（去掉分号）
                    let line_no_semicolon = line.trim_end_matches(';').trim();
                    if line_no_semicolon.eq_ignore_ascii_case("exit") || 
                       line_no_semicolon.eq_ignore_ascii_case("quit") ||
                       line_no_semicolon.eq_ignore_ascii_case("\\q") {
                        println!("再见！");
                        break;
                    }
                    
                    // 执行 SQL
                    self.execute_sql(line);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("再见！");
                    break;
                }
                Err(err) => {
                    println!("{}错误: {:?}{}", term::RED, err, term::RESET);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// 提示连接数据库
    fn prompt_connect(&mut self) -> io::Result<()> {
        println!();
        match self.db_type {
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
                        self.print_schema_info();
                    }
                    Err(e) => {
                        println!("{}✗ 连接失败: {}{}", term::RED, e, term::RESET);
                    }
                }
            }
            DatabaseType::MySQL | DatabaseType::MariaDB => {
                println!("{}请输入 MySQL 连接信息（直接回车使用默认值）:{}", term::YELLOW, term::RESET);
                println!("{}也可以直接输入完整 URL: mysql://user:password@host:port/database{}", term::GRAY, term::RESET);
                println!();
                
                print!("  连接方式 [{}1{}=逐项输入, {}2{}=URL]: ", term::CYAN, term::RESET, term::CYAN, term::RESET);
                stdout().flush()?;
                let mut mode = String::new();
                stdin().read_line(&mut mode)?;
                let mode = mode.trim();
                
                if mode == "2" {
                    print!("  URL: ");
                    stdout().flush()?;
                    let mut url = String::new();
                    stdin().read_line(&mut url)?;
                    let url = url.trim();
                    
                    match self.connect(url) {
                        Ok(()) => {
                            println!("{}✓ 已连接到 MySQL{}", term::GREEN, term::RESET);
                            self.print_schema_info();
                        }
                        Err(e) => {
                            println!("{}✗ 连接失败: {}{}", term::RED, e, term::RESET);
                        }
                    }
                } else {
                    println!();
                    
                    print!("  主机 [{}localhost{}]: ", term::CYAN, term::RESET);
                    stdout().flush()?;
                    let mut host = String::new();
                    stdin().read_line(&mut host)?;
                    let host = host.trim();
                    let host = if host.is_empty() { "localhost" } else { host };
                    
                    print!("  端口 [{}3306{}]: ", term::CYAN, term::RESET);
                    stdout().flush()?;
                    let mut port_str = String::new();
                    stdin().read_line(&mut port_str)?;
                    let port: u16 = port_str.trim().parse().unwrap_or(3306);
                    
                    print!("  用户名 [{}root{}]: ", term::CYAN, term::RESET);
                    stdout().flush()?;
                    let mut user = String::new();
                    stdin().read_line(&mut user)?;
                    let user = user.trim();
                    let user = if user.is_empty() { "root" } else { user };
                    
                    print!("  密码 [{}空{}]: ", term::CYAN, term::RESET);
                    stdout().flush()?;
                    let mut password = String::new();
                    stdin().read_line(&mut password)?;
                    let password = password.trim();
                    
                    print!("  数据库 [{}mysql{}]: ", term::CYAN, term::RESET);
                    stdout().flush()?;
                    let mut database = String::new();
                    stdin().read_line(&mut database)?;
                    let database = database.trim();
                    let database = if database.is_empty() { "mysql" } else { database };
                    
                    println!();
                    println!("{}正在连接 {}@{}:{}...{}", term::GRAY, user, host, port, term::RESET);
                    
                    let conn_str = format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, database);
                    
                    match self.connect(&conn_str) {
                        Ok(()) => {
                            println!("{}✓ 已连接到 MySQL {}:{}/{}{}", term::GREEN, host, port, database, term::RESET);
                            self.print_schema_info();
                        }
                        Err(e) => {
                            println!("{}✗ 连接失败: {}{}", term::RED, e, term::RESET);
                        }
                    }
                }
            }
            DatabaseType::PostgreSQL => {
                println!("{}请输入 PostgreSQL 连接信息（直接回车使用默认值）:{}", term::YELLOW, term::RESET);
                println!("{}也可以直接输入完整 URL: postgresql://user:password@host:port/database{}", term::GRAY, term::RESET);
                println!();
                
                print!("  连接方式 [{}1{}=逐项输入, {}2{}=URL]: ", term::CYAN, term::RESET, term::CYAN, term::RESET);
                stdout().flush()?;
                let mut mode = String::new();
                stdin().read_line(&mut mode)?;
                let mode = mode.trim();
                
                if mode == "2" {
                    print!("  URL: ");
                    stdout().flush()?;
                    let mut url = String::new();
                    stdin().read_line(&mut url)?;
                    let url = url.trim();
                    
                    match self.connect(url) {
                        Ok(()) => {
                            println!("{}✓ 已连接到 PostgreSQL{}", term::GREEN, term::RESET);
                            self.print_schema_info();
                        }
                        Err(e) => {
                            println!("{}✗ 连接失败: {}{}", term::RED, e, term::RESET);
                        }
                    }
                } else {
                    println!();
                    
                    print!("  主机 [{}localhost{}]: ", term::CYAN, term::RESET);
                    stdout().flush()?;
                    let mut host = String::new();
                    stdin().read_line(&mut host)?;
                    let host = host.trim();
                    let host = if host.is_empty() { "localhost" } else { host };
                    
                    print!("  端口 [{}5432{}]: ", term::CYAN, term::RESET);
                    stdout().flush()?;
                    let mut port_str = String::new();
                    stdin().read_line(&mut port_str)?;
                    let port: u16 = port_str.trim().parse().unwrap_or(5432);
                    
                    print!("  用户名 [{}postgres{}]: ", term::CYAN, term::RESET);
                    stdout().flush()?;
                    let mut user = String::new();
                    stdin().read_line(&mut user)?;
                    let user = user.trim();
                    let user = if user.is_empty() { "postgres" } else { user };
                    
                    print!("  密码 [{}空{}]: ", term::CYAN, term::RESET);
                    stdout().flush()?;
                    let mut password = String::new();
                    stdin().read_line(&mut password)?;
                    let password = password.trim();
                    
                    print!("  数据库 [{}postgres{}]: ", term::CYAN, term::RESET);
                    stdout().flush()?;
                    let mut database = String::new();
                    stdin().read_line(&mut database)?;
                    let database = database.trim();
                    let database = if database.is_empty() { "postgres" } else { database };
                    
                    println!();
                    println!("{}正在连接 {}@{}:{}...{}", term::GRAY, user, host, port, term::RESET);
                    
                    let conn_str = format!("postgresql://{}:{}@{}:{}/{}", user, password, host, port, database);
                    
                    match self.connect(&conn_str) {
                        Ok(()) => {
                            println!("{}✓ 已连接到 PostgreSQL {}:{}/{}{}", term::GREEN, host, port, database, term::RESET);
                            self.print_schema_info();
                        }
                        Err(e) => {
                            println!("{}✗ 连接失败: {}{}", term::RED, e, term::RESET);
                        }
                    }
                }
            }
            _ => {
                println!("{}{} 数据库暂不支持真实连接{}", term::YELLOW, self.db_type.name(), term::RESET);
                println!("{}将以离线模式运行（仅提供语法补全）{}", term::GRAY, term::RESET);
            }
        }
        println!();
        
        Ok(())
    }
    
    /// 打印 Schema 信息摘要
    fn print_schema_info(&mut self) {
        if let Ok(tables) = self.connection.get_tables() {
            if tables.is_empty() {
                println!("{}  (数据库中没有表){}", term::GRAY, term::RESET);
            } else {
                println!("{}  发现 {} 个表{}", term::GRAY, tables.len(), term::RESET);
            }
        }
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
        println!();
        println!("{}{}╔══════════════════════════════════════════════════════════════╗{}", term::BOLD, self.db_type.color(), term::RESET);
        println!("{}{}║              cnmsb-sql - SQL 智能补全终端                    ║{}", term::BOLD, self.db_type.color(), term::RESET);
        println!("{}{}╚══════════════════════════════════════════════════════════════╝{}", term::BOLD, self.db_type.color(), term::RESET);
        println!();
        println!("  数据库类型: {}{}{}{}", term::BOLD, self.db_type.color(), self.db_type.name(), term::RESET);
        println!("  {}{}", term::GRAY, self.db_type.description());
        println!("{}", term::RESET);
        println!("{}快捷键:{}", term::YELLOW, term::RESET);
        println!("  {}Tab{}        补全 SQL 关键字/表名/列名", term::CYAN, term::RESET);
        println!("  {}↑ ↓{}        浏览历史记录", term::CYAN, term::RESET);
        println!("  {}→{}          接受内联建议", term::CYAN, term::RESET);
        println!("  {}Ctrl+C{}     取消当前输入", term::CYAN, term::RESET);
        println!("  {}Ctrl+D{}     退出", term::CYAN, term::RESET);
        println!();
        println!("{}命令:{}", term::YELLOW, term::RESET);
        println!("  {}.help{}      显示帮助", term::GREEN, term::RESET);
        println!("  {}.tables{}    显示所有表", term::GREEN, term::RESET);
        println!("  {}.desc{}      显示表结构", term::GREEN, term::RESET);
        println!("  {}.schema{}    显示完整 Schema", term::GREEN, term::RESET);
        println!("  {}.clear{}     清屏", term::GREEN, term::RESET);
        println!("  {}exit/quit{} 退出", term::GREEN, term::RESET);
        println!();
    }
    
    /// 处理特殊命令
    fn handle_command(&mut self, line: &str, rl: &mut Editor<SqlHelper, DefaultHistory>) -> bool {
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
        
        // .desc table_name 或 \d table_name
        if lower.starts_with(".desc ") || lower.starts_with("\\d ") || lower.starts_with("describe ") {
            let table = if lower.starts_with(".desc ") {
                &line[6..]
            } else if lower.starts_with("\\d ") {
                &line[3..]
            } else {
                &line[9..]
            };
            self.describe_table(table.trim());
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
            self.reconnect(conn_str.trim(), rl);
            return true;
        }
        
        if lower == "\\c" || lower == ".connect" || lower == "connect" {
            let _ = self.prompt_connect();
            self.update_helper_schema(rl);
            return true;
        }
        
        if lower == ".status" || lower == "\\s" || lower == "status" {
            self.show_status();
            return true;
        }
        
        if lower == ".schema" || lower == "\\ds" {
            self.show_schema();
            return true;
        }
        
        false
    }
    
    /// 更新 Helper 的 Schema 信息
    fn update_helper_schema(&mut self, rl: &mut Editor<SqlHelper, DefaultHistory>) {
        if self.connected {
            if let Some(helper) = rl.helper_mut() {
                if let Ok(tables) = self.connection.get_tables() {
                    helper.set_tables(tables.clone());
                    for table in &tables {
                        if let Ok(columns) = self.connection.get_columns(table) {
                            let col_names: Vec<String> = columns.iter().map(|c| c.name.clone()).collect();
                            helper.set_columns(table, col_names);
                        }
                    }
                }
            }
        }
    }
    
    /// 显示表结构
    fn describe_table(&mut self, table: &str) {
        println!();
        
        if !self.connected {
            println!("{}未连接数据库{}", term::YELLOW, term::RESET);
            println!();
            return;
        }
        
        match self.connection.get_columns(table) {
            Ok(columns) => {
                if columns.is_empty() {
                    println!("{}表 '{}' 不存在或没有列{}", term::YELLOW, table, term::RESET);
                } else {
                    println!("{}表 '{}' 结构:{}", term::YELLOW, table, term::RESET);
                    println!();
                    println!("  {}{:<20} {:<15} {:<10} {}{}", 
                        term::CYAN, "列名", "类型", "可空", "主键", term::RESET);
                    println!("  {}", "-".repeat(55));
                    
                    for col in &columns {
                        let nullable = if col.nullable { "YES" } else { "NO" };
                        let pk = if col.primary_key { "PK" } else { "" };
                        println!("  {:<20} {:<15} {:<10} {}", 
                            col.name, col.data_type, nullable, pk);
                    }
                }
            }
            Err(e) => {
                println!("{}获取表结构失败: {}{}", term::RED, e, term::RESET);
            }
        }
        println!();
    }
    
    /// 显示所有表和列的 Schema
    fn show_schema(&mut self) {
        println!();
        
        if !self.connected {
            println!("{}未连接数据库{}", term::YELLOW, term::RESET);
            println!();
            return;
        }
        
        match self.connection.get_tables() {
            Ok(tables) => {
                if tables.is_empty() {
                    println!("{}数据库中没有表{}", term::GRAY, term::RESET);
                } else {
                    println!("{}数据库 Schema ({} 个表):{}", term::YELLOW, tables.len(), term::RESET);
                    println!();
                    
                    for table in &tables {
                        print!("  {}{}{}", term::GREEN, table, term::RESET);
                        
                        if let Ok(columns) = self.connection.get_columns(table) {
                            let col_names: Vec<&str> = columns.iter().map(|c| c.name.as_str()).collect();
                            print!(" {}({}){}", term::GRAY, col_names.join(", "), term::RESET);
                        }
                        println!();
                    }
                }
            }
            Err(e) => {
                println!("{}获取 Schema 失败: {}{}", term::RED, e, term::RESET);
            }
        }
        println!();
    }
    
    /// 重新连接数据库
    fn reconnect(&mut self, conn_str: &str, rl: &mut Editor<SqlHelper, DefaultHistory>) {
        match self.connect(conn_str) {
            Ok(()) => {
                println!("\n{}✓ 已成功连接数据库{}\n", term::GREEN, term::RESET);
                self.update_helper_schema(rl);
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
        println!("  数据库类型: {}{}{}", term::CYAN, self.db_type.name(), term::RESET);
        if self.connected {
            println!("  状态: {}已连接{}", term::GREEN, term::RESET);
        } else {
            println!("  状态: {}未连接{}", term::RED, term::RESET);
        }
        println!();
    }
    
    /// 打印帮助
    fn print_help(&self) {
        println!();
        println!("{}{} SQL 帮助{}", term::BOLD, self.db_type.name(), term::RESET);
        println!();
        println!("{}Shell 命令:{}", term::YELLOW, term::RESET);
        println!("  {}\\c, .connect{}      连接/重连数据库", term::CYAN, term::RESET);
        println!("  {}\\d, .tables{}       显示所有表", term::CYAN, term::RESET);
        println!("  {}\\d TABLE, .desc{}   显示表结构", term::CYAN, term::RESET);
        println!("  {}\\ds, .schema{}      显示完整 Schema", term::CYAN, term::RESET);
        println!("  {}\\s, .status{}       显示连接状态", term::CYAN, term::RESET);
        println!("  {}.clear{}            清屏", term::CYAN, term::RESET);
        println!("  {}exit, \\q{}          退出", term::CYAN, term::RESET);
        println!();
        println!("{}常用 SQL:{}", term::YELLOW, term::RESET);
        println!("  {}SELECT{} * FROM table_name WHERE condition", term::BLUE, term::RESET);
        println!("  {}INSERT INTO{} table_name (col1, col2) VALUES (val1, val2)", term::BLUE, term::RESET);
        println!("  {}UPDATE{} table_name SET col = value WHERE condition", term::BLUE, term::RESET);
        println!("  {}DELETE FROM{} table_name WHERE condition", term::BLUE, term::RESET);
        println!("  {}CREATE TABLE{} table_name (col1 TYPE, col2 TYPE)", term::BLUE, term::RESET);
        println!();
        println!("{}补全功能:{}", term::YELLOW, term::RESET);
        println!("  - 按 {}Tab{} 显示补全列表", term::CYAN, term::RESET);
        println!("{}  - 输入时自动显示灰色建议{}", term::GRAY, term::RESET);
        println!("{}  - 按右箭头接受建议{}", term::GRAY, term::RESET);
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
    
    DatabaseType::from_str(input)
}
