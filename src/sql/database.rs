//! 数据库类型和配置定义

use std::fmt;

/// 支持的数据库类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseType {
    MySQL,
    PostgreSQL,
    SQLite,
    MariaDB,
    Oracle,
    SQLServer,
}

impl DatabaseType {
    /// 获取所有支持的数据库类型
    pub fn all() -> &'static [DatabaseType] {
        &[
            DatabaseType::MySQL,
            DatabaseType::PostgreSQL,
            DatabaseType::SQLite,
            DatabaseType::MariaDB,
            DatabaseType::Oracle,
            DatabaseType::SQLServer,
        ]
    }
    
    /// 获取数据库显示名称
    pub fn name(&self) -> &'static str {
        match self {
            DatabaseType::MySQL => "MySQL",
            DatabaseType::PostgreSQL => "PostgreSQL",
            DatabaseType::SQLite => "SQLite",
            DatabaseType::MariaDB => "MariaDB",
            DatabaseType::Oracle => "Oracle",
            DatabaseType::SQLServer => "SQL Server",
        }
    }
    
    /// 获取数据库描述
    pub fn description(&self) -> &'static str {
        match self {
            DatabaseType::MySQL => "MySQL 数据库，最流行的开源关系型数据库",
            DatabaseType::PostgreSQL => "PostgreSQL 数据库，功能强大的开源对象关系型数据库",
            DatabaseType::SQLite => "SQLite 数据库，轻量级嵌入式数据库",
            DatabaseType::MariaDB => "MariaDB 数据库，MySQL 的开源分支",
            DatabaseType::Oracle => "Oracle 数据库，企业级商业数据库",
            DatabaseType::SQLServer => "Microsoft SQL Server，微软的商业数据库",
        }
    }
    
    /// 获取默认端口
    pub fn default_port(&self) -> u16 {
        match self {
            DatabaseType::MySQL | DatabaseType::MariaDB => 3306,
            DatabaseType::PostgreSQL => 5432,
            DatabaseType::SQLite => 0, // 无端口
            DatabaseType::Oracle => 1521,
            DatabaseType::SQLServer => 1433,
        }
    }
    
    /// 是否需要网络连接
    pub fn requires_network(&self) -> bool {
        !matches!(self, DatabaseType::SQLite)
    }
    
    /// 获取提示符
    pub fn prompt(&self) -> &'static str {
        match self {
            DatabaseType::MySQL | DatabaseType::MariaDB => "mysql",
            DatabaseType::PostgreSQL => "psql",
            DatabaseType::SQLite => "sqlite",
            DatabaseType::Oracle => "oracle",
            DatabaseType::SQLServer => "mssql",
        }
    }
    
    /// 获取颜色
    pub fn color(&self) -> &'static str {
        match self {
            DatabaseType::MySQL | DatabaseType::MariaDB => "\x1b[38;5;208m", // 橙色
            DatabaseType::PostgreSQL => "\x1b[38;5;33m",  // 蓝色
            DatabaseType::SQLite => "\x1b[38;5;117m",     // 浅蓝色
            DatabaseType::Oracle => "\x1b[38;5;196m",     // 红色
            DatabaseType::SQLServer => "\x1b[38;5;39m",   // 天蓝色
        }
    }
    
    /// 从字符串解析
    pub fn from_str(s: &str) -> Option<DatabaseType> {
        match s.to_lowercase().as_str() {
            "mysql" | "1" => Some(DatabaseType::MySQL),
            "postgresql" | "postgres" | "psql" | "pg" | "2" => Some(DatabaseType::PostgreSQL),
            "sqlite" | "sqlite3" | "3" => Some(DatabaseType::SQLite),
            "mariadb" | "maria" | "4" => Some(DatabaseType::MariaDB),
            "oracle" | "5" => Some(DatabaseType::Oracle),
            "sqlserver" | "mssql" | "sql server" | "6" => Some(DatabaseType::SQLServer),
            _ => None,
        }
    }
}

impl fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// 数据库连接配置
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub db_type: DatabaseType,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub file_path: Option<String>, // 用于 SQLite
}

impl DatabaseConfig {
    /// 创建新的配置
    pub fn new(db_type: DatabaseType) -> Self {
        DatabaseConfig {
            db_type,
            host: "localhost".to_string(),
            port: db_type.default_port(),
            username: String::new(),
            password: String::new(),
            database: String::new(),
            file_path: None,
        }
    }
    
    /// 创建 SQLite 配置
    pub fn sqlite(file_path: &str) -> Self {
        DatabaseConfig {
            db_type: DatabaseType::SQLite,
            host: String::new(),
            port: 0,
            username: String::new(),
            password: String::new(),
            database: String::new(),
            file_path: Some(file_path.to_string()),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self::new(DatabaseType::MySQL)
    }
}

