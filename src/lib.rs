//! cnmsb - 操你妈傻逼
//! Linux 命令行智能补全工具

pub mod completions;
pub mod database;
pub mod engine;
pub mod parser;
pub mod shell;
pub mod sql;

pub use engine::{CompletionEngine, CompletionKind};
pub use parser::CommandParser;
pub use shell::CnmsbShell;
pub use sql::{DatabaseType, SqlEngine, SqlShell};

