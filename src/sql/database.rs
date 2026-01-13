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

impl DatabaseConfig {
    /// 从连接字符串解析配置
    /// 支持格式：
    /// - MySQL: mysql://user:pass@host:port/database
    /// - PostgreSQL: postgresql://user:pass@host:port/database 或 postgres://...
    /// - SQLite: sqlite:///path/to/db 或直接路径
    pub fn parse(url: &str) -> Option<Self> {
        let url = url.trim();
        
        // SQLite
        if url.starts_with("sqlite://") || url.starts_with("sqlite:") {
            let path = url.trim_start_matches("sqlite://").trim_start_matches("sqlite:");
            return Some(Self::sqlite(path));
        }
        
        // 如果看起来像文件路径，当作 SQLite
        if url.ends_with(".db") || url.ends_with(".sqlite") || url.ends_with(".sqlite3") || url == ":memory:" {
            return Some(Self::sqlite(url));
        }
        
        // MySQL
        if url.starts_with("mysql://") {
            return Self::parse_mysql_url(url);
        }
        
        // PostgreSQL
        if url.starts_with("postgresql://") || url.starts_with("postgres://") {
            return Self::parse_postgres_url(url);
        }
        
        None
    }
    
    /// 解析 MySQL URL
    fn parse_mysql_url(url: &str) -> Option<Self> {
        // mysql://user:pass@host:port/database
        let rest = url.strip_prefix("mysql://")?;
        Self::parse_db_url(rest, DatabaseType::MySQL)
    }
    
    /// 解析 PostgreSQL URL
    fn parse_postgres_url(url: &str) -> Option<Self> {
        // postgresql://user:pass@host:port/database
        let rest = url.strip_prefix("postgresql://")
            .or_else(|| url.strip_prefix("postgres://"))?;
        Self::parse_db_url(rest, DatabaseType::PostgreSQL)
    }
    
    /// 通用 URL 解析
    fn parse_db_url(rest: &str, db_type: DatabaseType) -> Option<Self> {
        let mut config = Self::new(db_type);
        
        // 分离用户信息和主机信息
        let (auth_part, host_part) = if let Some(at_pos) = rest.rfind('@') {
            (&rest[..at_pos], &rest[at_pos + 1..])
        } else {
            ("", rest)
        };
        
        // 解析用户名和密码
        if !auth_part.is_empty() {
            if let Some(colon_pos) = auth_part.find(':') {
                config.username = auth_part[..colon_pos].to_string();
                config.password = auth_part[colon_pos + 1..].to_string();
            } else {
                config.username = auth_part.to_string();
            }
        }
        
        // 解析主机、端口和数据库
        let (host_port, database) = if let Some(slash_pos) = host_part.find('/') {
            (&host_part[..slash_pos], &host_part[slash_pos + 1..])
        } else {
            (host_part, "")
        };
        
        // 解析主机和端口
        if let Some(colon_pos) = host_port.rfind(':') {
            config.host = host_port[..colon_pos].to_string();
            if let Ok(port) = host_port[colon_pos + 1..].parse() {
                config.port = port;
            }
        } else {
            config.host = host_port.to_string();
        }
        
        config.database = database.to_string();
        
        Some(config)
    }
    
    /// 生成连接字符串
    pub fn to_connection_string(&self) -> String {
        match self.db_type {
            DatabaseType::SQLite => {
                self.file_path.clone().unwrap_or_else(|| ":memory:".to_string())
            }
            DatabaseType::MySQL | DatabaseType::MariaDB => {
                if self.password.is_empty() {
                    format!("mysql://{}@{}:{}/{}", self.username, self.host, self.port, self.database)
                } else {
                    format!("mysql://{}:{}@{}:{}/{}", self.username, self.password, self.host, self.port, self.database)
                }
            }
            DatabaseType::PostgreSQL => {
                if self.password.is_empty() {
                    format!("postgresql://{}@{}:{}/{}", self.username, self.host, self.port, self.database)
                } else {
                    format!("postgresql://{}:{}@{}:{}/{}", self.username, self.password, self.host, self.port, self.database)
                }
            }
            _ => String::new(),
        }
    }
    
    /// 生成安全的连接字符串（隐藏密码）
    pub fn to_safe_string(&self) -> String {
        match self.db_type {
            DatabaseType::SQLite => {
                self.file_path.clone().unwrap_or_else(|| ":memory:".to_string())
            }
            DatabaseType::MySQL | DatabaseType::MariaDB => {
                let pass = if self.password.is_empty() { "" } else { ":***" };
                format!("mysql://{}{}@{}:{}/{}", self.username, pass, self.host, self.port, self.database)
            }
            DatabaseType::PostgreSQL => {
                let pass = if self.password.is_empty() { "" } else { ":***" };
                format!("postgresql://{}{}@{}:{}/{}", self.username, pass, self.host, self.port, self.database)
            }
            _ => format!("{} ({}:{})", self.db_type, self.host, self.port),
        }
    }
}

