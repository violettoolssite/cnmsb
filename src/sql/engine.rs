//! SQL 补全引擎

use super::database::DatabaseType;
use super::syntax::{self, SqlSyntax, SqlCompletion};

/// SQL 补全引擎
pub struct SqlEngine {
    db_type: DatabaseType,
    syntax: Box<dyn SqlSyntax>,
    tables: Vec<String>,
    columns: Vec<(String, Vec<String>)>, // (表名, 列名列表)
}

impl SqlEngine {
    /// 创建新的 SQL 引擎
    pub fn new(db_type: DatabaseType) -> Self {
        SqlEngine {
            db_type,
            syntax: syntax::get_syntax(db_type),
            tables: Vec::new(),
            columns: Vec::new(),
        }
    }
    
    /// 获取数据库类型
    pub fn db_type(&self) -> DatabaseType {
        self.db_type
    }
    
    /// 设置表列表
    pub fn set_tables(&mut self, tables: Vec<String>) {
        self.tables = tables;
    }
    
    /// 添加表
    pub fn add_table(&mut self, table: &str) {
        if !self.tables.contains(&table.to_string()) {
            self.tables.push(table.to_string());
        }
    }
    
    /// 设置列信息
    pub fn set_columns(&mut self, table: &str, columns: Vec<String>) {
        // 移除已有的
        self.columns.retain(|(t, _)| t != table);
        self.columns.push((table.to_string(), columns));
    }
    
    /// 获取补全建议
    pub fn complete(&self, input: &str, cursor: usize) -> Vec<SqlCompletion> {
        let mut completions = self.syntax.complete(input, cursor);
        
        // 获取当前词
        let input = if cursor <= input.len() {
            &input[..cursor]
        } else {
            input
        };
        let current_word = input.split_whitespace().last().unwrap_or("").to_uppercase();
        
        // 检查上下文，添加表名和列名
        let input_upper = input.to_uppercase();
        
        // 在 FROM、JOIN 等后面添加表名补全
        if self.should_complete_tables(&input_upper) {
            for table in &self.tables {
                if current_word.is_empty() || table.to_uppercase().starts_with(&current_word) {
                    completions.push(SqlCompletion::table(table));
                }
            }
        }
        
        // 在特定上下文添加列名补全
        if self.should_complete_columns(&input_upper) {
            // 尝试从上下文获取表名
            if let Some(table) = self.extract_table_from_context(&input_upper) {
                if let Some((_, cols)) = self.columns.iter().find(|(t, _)| t.to_uppercase() == table) {
                    for col in cols {
                        if current_word.is_empty() || col.to_uppercase().starts_with(&current_word) {
                            completions.push(SqlCompletion::column(col, &table.to_lowercase()));
                        }
                    }
                }
            } else {
                // 没有特定表，显示所有列
                for (table, cols) in &self.columns {
                    for col in cols {
                        if current_word.is_empty() || col.to_uppercase().starts_with(&current_word) {
                            completions.push(SqlCompletion::column(col, table));
                        }
                    }
                }
            }
        }
        
        completions
    }
    
    /// 检查是否应该补全表名
    fn should_complete_tables(&self, input: &str) -> bool {
        let keywords = ["FROM ", "JOIN ", "INTO ", "UPDATE ", "TABLE ", "VIEW "];
        keywords.iter().any(|k| {
            if let Some(pos) = input.rfind(k) {
                // 检查关键字后面是否已经有完整的表名
                let after = &input[pos + k.len()..];
                let has_space = after.contains(' ');
                !has_space
            } else {
                false
            }
        })
    }
    
    /// 检查是否应该补全列名
    fn should_complete_columns(&self, input: &str) -> bool {
        let keywords = ["SELECT ", "WHERE ", "AND ", "OR ", "SET ", "ORDER BY ", "GROUP BY ", "HAVING "];
        keywords.iter().any(|k| input.contains(k))
    }
    
    /// 从上下文提取表名
    fn extract_table_from_context(&self, input: &str) -> Option<String> {
        // 简单实现：查找 FROM 后面的表名
        if let Some(pos) = input.rfind("FROM ") {
            let after = &input[pos + 5..];
            let words: Vec<&str> = after.split_whitespace().collect();
            if let Some(table) = words.first() {
                // 移除可能的逗号或其他字符
                let table = table.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                if !table.is_empty() {
                    return Some(table.to_uppercase());
                }
            }
        }
        None
    }
    
    /// 获取当前词的补全建议（用于 Tab 补全）
    pub fn get_current_word_completion(&self, input: &str, cursor: usize) -> Option<String> {
        let completions = self.complete(input, cursor);
        
        if completions.is_empty() {
            return None;
        }
        
        let input = if cursor <= input.len() {
            &input[..cursor]
        } else {
            input
        };
        let current_word = input.split_whitespace().last().unwrap_or("");
        
        if current_word.is_empty() {
            return None;
        }
        
        // 返回第一个匹配的补全的后缀
        if let Some(first) = completions.first() {
            let text_upper = first.text.to_uppercase();
            let current_upper = current_word.to_uppercase();
            
            if text_upper.starts_with(&current_upper) && text_upper.len() > current_upper.len() {
                // 检测用户的大小写风格
                let suffix_start = current_word.len();
                let original_suffix = &first.text[suffix_start..];
                
                // 根据用户输入的大小写风格调整补全
                let styled_suffix = if current_word.chars().all(|c| c.is_lowercase() || !c.is_alphabetic()) {
                    // 用户输入全小写，返回小写
                    original_suffix.to_lowercase()
                } else if current_word.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
                    // 用户输入全大写，返回大写
                    original_suffix.to_uppercase()
                } else {
                    // 混合大小写，默认使用大写（SQL 惯例）
                    original_suffix.to_uppercase()
                };
                
                return Some(styled_suffix);
            }
        }
        
        None
    }
}

