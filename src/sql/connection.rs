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

/// 列信息
#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub primary_key: bool,
}

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
    MySQL(mysql::Pool),
    PostgreSQL(postgres::Client),
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
    
    /// 连接 MySQL 数据库
    pub fn connect_mysql(host: &str, port: u16, user: &str, password: &str, database: &str) -> Result<Self, DbError> {
        let url = if password.is_empty() {
            format!("mysql://{}@{}:{}/{}", user, host, port, database)
        } else {
            format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, database)
        };
        
        mysql::Pool::new(url.as_str())
            .map(DbConnection::MySQL)
            .map_err(|e| {
                let err_msg = e.to_string();
                // 改进常见错误的提示信息
                if err_msg.contains("ERROR 1698") || err_msg.contains("Access denied") {
                    if user == "root" && password.is_empty() {
                        DbError::Connection(format!("{} (提示: MySQL 8.0+ 的 root 用户可能使用 auth_socket 认证。请尝试: 1) 使用其他用户连接 2) 设置 root 密码: ALTER USER 'root'@'localhost' IDENTIFIED WITH mysql_native_password BY 'your_password'; 3) 或使用 sudo mysql 登录)", err_msg))
                    } else {
                        DbError::Connection(format!("{} (提示: 请检查用户名、密码是否正确，或用户是否有远程连接权限)", err_msg))
                    }
                } else if err_msg.contains("Can't connect") {
                    DbError::Connection(format!("{} (提示: 请检查 MySQL 服务是否运行，主机和端口是否正确)", err_msg))
                } else {
                    DbError::Connection(err_msg)
                }
            })
    }
    
    /// 从连接字符串连接 MySQL
    pub fn connect_mysql_url(url: &str) -> Result<Self, DbError> {
        mysql::Pool::new(url)
            .map(DbConnection::MySQL)
            .map_err(|e| {
                let err_msg = e.to_string();
                // 改进常见错误的提示信息
                if err_msg.contains("ERROR 1698") || err_msg.contains("Access denied") {
                    DbError::Connection(format!("{} (提示: MySQL 8.0+ 的 root 用户可能使用 auth_socket 认证。请尝试: 1) 使用其他用户连接 2) 设置 root 密码 3) 或使用 sudo mysql 登录)", err_msg))
                } else if err_msg.contains("Can't connect") {
                    DbError::Connection(format!("{} (提示: 请检查 MySQL 服务是否运行，主机和端口是否正确)", err_msg))
                } else {
                    DbError::Connection(err_msg)
                }
            })
    }
    
    /// 连接 PostgreSQL 数据库
    pub fn connect_postgres(host: &str, port: u16, user: &str, password: &str, database: &str) -> Result<Self, DbError> {
        let conn_str = if password.is_empty() {
            format!("host={} port={} user={} dbname={}", host, port, user, database)
        } else {
            format!("host={} port={} user={} password={} dbname={}", host, port, user, password, database)
        };
        
        postgres::Client::connect(&conn_str, postgres::NoTls)
            .map(DbConnection::PostgreSQL)
            .map_err(|e| DbError::Connection(e.to_string()))
    }
    
    /// 从连接字符串连接 PostgreSQL
    pub fn connect_postgres_url(url: &str) -> Result<Self, DbError> {
        postgres::Client::connect(url, postgres::NoTls)
            .map(DbConnection::PostgreSQL)
            .map_err(|e| DbError::Connection(e.to_string()))
    }
    
    /// 执行 SQL 查询
    pub fn execute(&mut self, sql: &str) -> Result<QueryResult, DbError> {
        match self {
            DbConnection::SQLite(conn) => Self::execute_sqlite(conn, sql),
            DbConnection::MySQL(pool) => Self::execute_mysql(pool, sql),
            DbConnection::PostgreSQL(client) => Self::execute_postgres(client, sql),
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
            let affected = conn.execute(sql, []).map_err(|e| {
                let err_msg = e.to_string();
                // 检查是否是 CREATE TABLE 缺少列定义的情况
                let sql_upper = sql.trim().to_uppercase();
                if sql_upper.starts_with("CREATE TABLE") && !sql_upper.contains("(") {
                    DbError::Query("CREATE TABLE 语句需要定义至少一列。示例: CREATE TABLE test (id INTEGER);".to_string())
                } else {
                    // 尝试提取 SQLite 错误信息中的关键部分
                    // SQLite 错误格式通常是: "near \"token\": syntax error"
                    if let Some(near_pos) = err_msg.find("near \"") {
                        let after_near = &err_msg[near_pos + 6..];
                        if let Some(quote_end) = after_near.find("\"") {
                            let near_token = &after_near[..quote_end];
                            DbError::Query(format!("语法错误: 在 '{}' 附近", near_token))
                        } else {
                            DbError::Query(err_msg)
                        }
                    } else {
                        DbError::Query(err_msg)
                    }
                }
            })?;
            Ok(QueryResult {
                columns: Vec::new(),
                rows: Vec::new(),
                affected_rows: affected as u64,
                message: Some(format!("Query OK, {} row(s) affected", affected)),
            })
        }
    }
    
    /// MySQL 执行
    fn execute_mysql(pool: &mysql::Pool, sql: &str) -> Result<QueryResult, DbError> {
        use mysql::prelude::*;
        
        let mut conn = pool.get_conn().map_err(|e| DbError::Query(e.to_string()))?;
        let sql_upper = sql.trim().to_uppercase();
        
        if sql_upper.starts_with("SELECT") || sql_upper.starts_with("SHOW") || sql_upper.starts_with("DESCRIBE") || sql_upper.starts_with("EXPLAIN") {
            let result: Vec<mysql::Row> = conn.query(sql).map_err(|e| DbError::Query(e.to_string()))?;
            
            if result.is_empty() {
                return Ok(QueryResult {
                    columns: Vec::new(),
                    rows: Vec::new(),
                    affected_rows: 0,
                    message: Some("Empty set".to_string()),
                });
            }
            
            // 获取列名
            let columns: Vec<String> = result.first()
                .map(|row| row.columns_ref().iter().map(|c| c.name_str().to_string()).collect())
                .unwrap_or_default();
            
            // 获取数据行
            let rows: Vec<Vec<String>> = result.iter().map(|row| {
                (0..row.len()).map(|i| {
                    row.get::<mysql::Value, _>(i)
                        .map(|v| mysql_value_to_string(&v))
                        .unwrap_or_else(|| "NULL".to_string())
                }).collect()
            }).collect();
            
            Ok(QueryResult {
                columns,
                rows,
                affected_rows: 0,
                message: None,
            })
        } else {
            let _result = conn.exec_drop(sql, ()).map_err(|e| DbError::Query(e.to_string()))?;
            let affected = conn.affected_rows();
            Ok(QueryResult {
                columns: Vec::new(),
                rows: Vec::new(),
                affected_rows: affected,
                message: Some(format!("Query OK, {} row(s) affected", affected)),
            })
        }
    }
    
    /// PostgreSQL 执行
    fn execute_postgres(client: &mut postgres::Client, sql: &str) -> Result<QueryResult, DbError> {
        let sql_upper = sql.trim().to_uppercase();
        
        if sql_upper.starts_with("SELECT") || sql_upper.starts_with("SHOW") || sql_upper.starts_with("EXPLAIN") {
            let rows_result = client.query(sql, &[]).map_err(|e| DbError::Query(e.to_string()))?;
            
            if rows_result.is_empty() {
                return Ok(QueryResult {
                    columns: Vec::new(),
                    rows: Vec::new(),
                    affected_rows: 0,
                    message: Some("Empty set".to_string()),
                });
            }
            
            // 获取列名
            let columns: Vec<String> = rows_result.first()
                .map(|row| row.columns().iter().map(|c| c.name().to_string()).collect())
                .unwrap_or_default();
            
            // 获取数据行
            let rows: Vec<Vec<String>> = rows_result.iter().map(|row| {
                (0..row.len()).map(|i| {
                    postgres_value_to_string(row, i)
                }).collect()
            }).collect();
            
            Ok(QueryResult {
                columns,
                rows,
                affected_rows: 0,
                message: None,
            })
        } else {
            let affected = client.execute(sql, &[]).map_err(|e| DbError::Query(e.to_string()))?;
            Ok(QueryResult {
                columns: Vec::new(),
                rows: Vec::new(),
                affected_rows: affected,
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
            DbConnection::MySQL(pool) => {
                use mysql::prelude::*;
                let mut conn = pool.get_conn().map_err(|e| DbError::Query(e.to_string()))?;
                let tables: Vec<String> = conn.query("SHOW TABLES")
                    .map_err(|e| DbError::Query(e.to_string()))?;
                Ok(tables)
            }
            DbConnection::PostgreSQL(client) => {
                let rows = client.query(
                    "SELECT tablename FROM pg_tables WHERE schemaname = 'public' ORDER BY tablename",
                    &[]
                ).map_err(|e| DbError::Query(e.to_string()))?;
                let tables: Vec<String> = rows.iter()
                    .map(|row| row.get::<_, String>(0))
                    .collect();
                Ok(tables)
            }
            DbConnection::None => Err(DbError::NotConnected),
        }
    }
    
    /// 获取表的列信息
    pub fn get_columns(&mut self, table: &str) -> Result<Vec<ColumnInfo>, DbError> {
        match self {
            DbConnection::SQLite(conn) => {
                let sql = format!("PRAGMA table_info({})", table);
                let mut stmt = conn.prepare(&sql).map_err(|e| DbError::Query(e.to_string()))?;
                let columns: Vec<ColumnInfo> = stmt.query_map([], |row| {
                    Ok(ColumnInfo {
                        name: row.get(1)?,
                        data_type: row.get(2)?,
                        nullable: row.get::<_, i32>(3)? == 0,
                        primary_key: row.get::<_, i32>(5)? == 1,
                    })
                })
                .map_err(|e| DbError::Query(e.to_string()))?
                .filter_map(|r| r.ok())
                .collect();
                Ok(columns)
            }
            DbConnection::MySQL(pool) => {
                use mysql::prelude::*;
                let mut conn = pool.get_conn().map_err(|e| DbError::Query(e.to_string()))?;
                let sql = format!("DESCRIBE {}", table);
                let rows: Vec<mysql::Row> = conn.query(&sql).map_err(|e| DbError::Query(e.to_string()))?;
                
                let columns: Vec<ColumnInfo> = rows.iter().map(|row| {
                    ColumnInfo {
                        name: row.get::<String, _>(0).unwrap_or_default(),
                        data_type: row.get::<String, _>(1).unwrap_or_default(),
                        nullable: row.get::<String, _>(2).map(|s| s == "YES").unwrap_or(true),
                        primary_key: row.get::<String, _>(3).map(|s| s == "PRI").unwrap_or(false),
                    }
                }).collect();
                Ok(columns)
            }
            DbConnection::PostgreSQL(client) => {
                let sql = format!(
                    "SELECT column_name, data_type, is_nullable, 
                     (SELECT COUNT(*) FROM information_schema.key_column_usage k 
                      WHERE k.table_name = c.table_name AND k.column_name = c.column_name) > 0 as is_pk
                     FROM information_schema.columns c 
                     WHERE table_name = $1 AND table_schema = 'public'
                     ORDER BY ordinal_position"
                );
                let rows = client.query(&sql, &[&table]).map_err(|e| DbError::Query(e.to_string()))?;
                
                let columns: Vec<ColumnInfo> = rows.iter().map(|row| {
                    ColumnInfo {
                        name: row.get::<_, String>(0),
                        data_type: row.get::<_, String>(1),
                        nullable: row.get::<_, String>(2) == "YES",
                        primary_key: row.get::<_, bool>(3),
                    }
                }).collect();
                Ok(columns)
            }
            DbConnection::None => Err(DbError::NotConnected),
        }
    }
    
    /// 获取数据库类型
    pub fn db_type(&self) -> Option<DatabaseType> {
        match self {
            DbConnection::SQLite(_) => Some(DatabaseType::SQLite),
            DbConnection::MySQL(_) => Some(DatabaseType::MySQL),
            DbConnection::PostgreSQL(_) => Some(DatabaseType::PostgreSQL),
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

/// MySQL 值转字符串
fn mysql_value_to_string(value: &mysql::Value) -> String {
    match value {
        mysql::Value::NULL => "NULL".to_string(),
        mysql::Value::Bytes(b) => String::from_utf8_lossy(b).to_string(),
        mysql::Value::Int(i) => i.to_string(),
        mysql::Value::UInt(u) => u.to_string(),
        mysql::Value::Float(f) => f.to_string(),
        mysql::Value::Double(d) => d.to_string(),
        mysql::Value::Date(y, m, d, h, mi, s, _) => {
            format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", y, m, d, h, mi, s)
        }
        mysql::Value::Time(neg, d, h, m, s, _) => {
            let sign = if *neg { "-" } else { "" };
            let hours = *d * 24 + *h as u32;
            format!("{}{}:{:02}:{:02}", sign, hours, m, s)
        }
    }
}

/// PostgreSQL 值转字符串
fn postgres_value_to_string(row: &postgres::Row, idx: usize) -> String {
    // 尝试各种类型
    if let Ok(v) = row.try_get::<_, Option<String>>(idx) {
        return v.unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(v) = row.try_get::<_, Option<i32>>(idx) {
        return v.map(|i| i.to_string()).unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(v) = row.try_get::<_, Option<i64>>(idx) {
        return v.map(|i| i.to_string()).unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(v) = row.try_get::<_, Option<f64>>(idx) {
        return v.map(|f| f.to_string()).unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(v) = row.try_get::<_, Option<bool>>(idx) {
        return v.map(|b| b.to_string()).unwrap_or_else(|| "NULL".to_string());
    }
    "?".to_string()
}
