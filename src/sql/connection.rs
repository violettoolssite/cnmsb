//! 数据库连接管理

use super::database::DatabaseType;
use std::fmt;

/// 数据库连接错误
#[derive(Debug)]
pub enum DbError {
    Connection(String),
    Query(String),
    NotConnected,
    Unsupported(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::Connection(msg) => write!(f, "连接错误: {}", msg),
            DbError::Query(msg) => write!(f, "查询错误: {}", msg),
            DbError::NotConnected => write!(f, "未连接数据库"),
            DbError::Unsupported(msg) => write!(f, "不支持: {}", msg),
        }
    }
}

impl std::error::Error for DbError {}

/// 查询结果
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub affected_rows: u64,
    pub message: Option<String>,
}

impl QueryResult {
    pub fn empty() -> Self {
        QueryResult {
            columns: Vec::new(),
            rows: Vec::new(),
            affected_rows: 0,
            message: None,
        }
    }
    
    pub fn with_message(msg: &str) -> Self {
        QueryResult {
            columns: Vec::new(),
            rows: Vec::new(),
            affected_rows: 0,
            message: Some(msg.to_string()),
        }
    }
}

/// 数据库连接
pub enum DbConnection {
    SQLite(rusqlite::Connection),
    None,
}

impl DbConnection {
    /// 连接 SQLite 数据库
    pub fn connect_sqlite(path: &str) -> Result<Self, DbError> {
        let path = if path.is_empty() || path == ":memory:" {
            ":memory:".to_string()
        } else {
            path.to_string()
        };
        
        rusqlite::Connection::open(&path)
            .map(DbConnection::SQLite)
            .map_err(|e| DbError::Connection(e.to_string()))
    }
    
    /// 执行 SQL 查询
    pub fn execute(&mut self, sql: &str) -> Result<QueryResult, DbError> {
        match self {
            DbConnection::SQLite(conn) => Self::execute_sqlite(conn, sql),
            DbConnection::None => Err(DbError::NotConnected),
        }
    }
    
    /// SQLite 执行
    fn execute_sqlite(conn: &rusqlite::Connection, sql: &str) -> Result<QueryResult, DbError> {
        let sql_upper = sql.trim().to_uppercase();
        
        // 判断是否是查询语句
        if sql_upper.starts_with("SELECT") || sql_upper.starts_with("PRAGMA") || sql_upper.starts_with("EXPLAIN") {
            let mut stmt = conn.prepare(sql).map_err(|e| DbError::Query(e.to_string()))?;
            
            let columns: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
            let column_count = columns.len();
            
            let rows: Vec<Vec<String>> = stmt
                .query_map([], |row| {
                    let mut values = Vec::new();
                    for i in 0..column_count {
                        let value: String = match row.get_ref(i) {
                            Ok(rusqlite::types::ValueRef::Null) => "NULL".to_string(),
                            Ok(rusqlite::types::ValueRef::Integer(i)) => i.to_string(),
                            Ok(rusqlite::types::ValueRef::Real(f)) => f.to_string(),
                            Ok(rusqlite::types::ValueRef::Text(s)) => String::from_utf8_lossy(s).to_string(),
                            Ok(rusqlite::types::ValueRef::Blob(b)) => format!("<BLOB {} bytes>", b.len()),
                            Err(_) => "?".to_string(),
                        };
                        values.push(value);
                    }
                    Ok(values)
                })
                .map_err(|e| DbError::Query(e.to_string()))?
                .filter_map(|r| r.ok())
                .collect();
            
            Ok(QueryResult {
                columns,
                rows,
                affected_rows: 0,
                message: None,
            })
        } else {
            // 非查询语句
            let affected = conn.execute(sql, []).map_err(|e| DbError::Query(e.to_string()))?;
            Ok(QueryResult {
                columns: Vec::new(),
                rows: Vec::new(),
                affected_rows: affected as u64,
                message: Some(format!("Query OK, {} row(s) affected", affected)),
            })
        }
    }
    
    /// 获取表列表
    pub fn get_tables(&mut self) -> Result<Vec<String>, DbError> {
        match self {
            DbConnection::SQLite(conn) => {
                let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
                    .map_err(|e| DbError::Query(e.to_string()))?;
                let tables: Vec<String> = stmt.query_map([], |row| row.get(0))
                    .map_err(|e| DbError::Query(e.to_string()))?
                    .filter_map(|r| r.ok())
                    .collect();
                Ok(tables)
            }
            DbConnection::None => Err(DbError::NotConnected),
        }
    }
    
    /// 获取数据库类型
    pub fn db_type(&self) -> Option<DatabaseType> {
        match self {
            DbConnection::SQLite(_) => Some(DatabaseType::SQLite),
            DbConnection::None => None,
        }
    }
    
    /// 是否已连接
    pub fn is_connected(&self) -> bool {
        !matches!(self, DbConnection::None)
    }
}

impl Default for DbConnection {
    fn default() -> Self {
        DbConnection::None
    }
}
