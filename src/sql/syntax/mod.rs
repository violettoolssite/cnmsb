//! SQL 语法定义模块

mod common;
mod mysql;
mod postgresql;
mod sqlite;

pub use common::{SqlSyntax, SqlCompletion, SqlCompletionKind, SqlContext};
pub use mysql::MySqlSyntax;
pub use postgresql::PostgreSqlSyntax;
pub use sqlite::SqliteSyntax;

use super::DatabaseType;

/// 获取对应数据库的语法
pub fn get_syntax(db_type: DatabaseType) -> Box<dyn SqlSyntax> {
    match db_type {
        DatabaseType::MySQL | DatabaseType::MariaDB => Box::new(MySqlSyntax),
        DatabaseType::PostgreSQL => Box::new(PostgreSqlSyntax),
        DatabaseType::SQLite => Box::new(SqliteSyntax),
        DatabaseType::Oracle => Box::new(MySqlSyntax), // 暂时使用 MySQL 语法
        DatabaseType::SQLServer => Box::new(MySqlSyntax), // 暂时使用 MySQL 语法
    }
}

