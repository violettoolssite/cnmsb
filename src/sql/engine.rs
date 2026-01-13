//! SQL 补全引擎

use super::database::DatabaseType;
use super::syntax::{self, SqlSyntax, SqlCompletion};
use std::collections::HashMap;

/// SQL 补全引擎
pub struct SqlEngine {
    db_type: DatabaseType,
    syntax: Box<dyn SqlSyntax>,
    tables: Vec<String>,
    columns: Vec<(String, Vec<String>)>, // (表名, 列名列表)
    aliases: HashMap<String, String>,     // (别名 -> 表名)
}

impl SqlEngine {
    /// 创建新的 SQL 引擎
    pub fn new(db_type: DatabaseType) -> Self {
        SqlEngine {
            db_type,
            syntax: syntax::get_syntax(db_type),
            tables: Vec::new(),
            columns: Vec::new(),
            aliases: HashMap::new(),
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
        let input_slice = if cursor <= input.len() {
            &input[..cursor]
        } else {
            input
        };
        
        let current_word = self.get_current_word(input_slice);
        let current_word_upper = current_word.to_uppercase();
        let input_upper = input_slice.to_uppercase();
        
        // 解析别名
        let aliases = self.parse_aliases(&input_upper);
        
        // 检查是否在输入 table.column 格式
        if let Some((table_or_alias, col_prefix)) = self.parse_dot_notation(&current_word) {
            // 解析表名（可能是别名）
            let table = aliases.get(&table_or_alias.to_uppercase())
                .map(|s| s.as_str())
                .unwrap_or(&table_or_alias);
            
            // 查找表的列
            if let Some((_, cols)) = self.columns.iter().find(|(t, _)| t.eq_ignore_ascii_case(table)) {
                for col in cols {
                    if col_prefix.is_empty() || col.to_uppercase().starts_with(&col_prefix.to_uppercase()) {
                        let text = format!("{}.{}", table_or_alias, col);
                        completions.push(SqlCompletion::column(&text, table));
                    }
                }
            }
            return completions;
        }
        
        // 在 FROM、JOIN 等后面添加表名补全
        if self.should_complete_tables(&input_upper) {
            for table in &self.tables {
                if current_word_upper.is_empty() || table.to_uppercase().starts_with(&current_word_upper) {
                    completions.push(SqlCompletion::table(table));
                }
            }
        }
        
        // 在特定上下文添加列名补全
        if self.should_complete_columns(&input_upper) {
            // 尝试从上下文获取表名
            let context_tables = self.extract_tables_from_context(&input_upper, &aliases);
            
            if context_tables.is_empty() {
                // 没有特定表，显示所有列
                for (table, cols) in &self.columns {
                    for col in cols {
                        if current_word_upper.is_empty() || col.to_uppercase().starts_with(&current_word_upper) {
                            completions.push(SqlCompletion::column(col, table));
                        }
                    }
                }
            } else {
                // 显示上下文表的列
                for table in &context_tables {
                    if let Some((_, cols)) = self.columns.iter().find(|(t, _)| t.eq_ignore_ascii_case(table)) {
                        for col in cols {
                            if current_word_upper.is_empty() || col.to_uppercase().starts_with(&current_word_upper) {
                                completions.push(SqlCompletion::column(col, table));
                            }
                        }
                    }
                }
            }
        }
        
        completions
    }
    
    /// 获取当前正在输入的词
    fn get_current_word<'a>(&self, input: &'a str) -> &'a str {
        // 从后向前找到词的开始
        let bytes = input.as_bytes();
        let mut end = bytes.len();
        
        // 跳过尾部空格
        while end > 0 && bytes[end - 1] == b' ' {
            end -= 1;
        }
        
        // 找词的开始
        let mut start = end;
        while start > 0 {
            let c = bytes[start - 1];
            // 允许字母、数字、下划线、点
            if c.is_ascii_alphanumeric() || c == b'_' || c == b'.' {
                start -= 1;
            } else {
                break;
            }
        }
        
        &input[start..end]
    }
    
    /// 解析 table.column 格式
    fn parse_dot_notation(&self, word: &str) -> Option<(String, String)> {
        if let Some(dot_pos) = word.rfind('.') {
            let table = word[..dot_pos].to_string();
            let column = word[dot_pos + 1..].to_string();
            if !table.is_empty() {
                return Some((table, column));
            }
        }
        None
    }
    
    /// 解析 SQL 中的别名
    fn parse_aliases(&self, input: &str) -> HashMap<String, String> {
        let mut aliases = HashMap::new();
        
        // 简单解析: FROM table alias 或 FROM table AS alias
        // 以及 JOIN table alias
        let patterns = ["FROM ", "JOIN "];
        
        for pattern in &patterns {
            let mut pos = 0;
            while let Some(found) = input[pos..].find(pattern) {
                let start = pos + found + pattern.len();
                let rest = &input[start..];
                
                // 获取接下来的词
                let words: Vec<&str> = rest.split_whitespace().take(3).collect();
                
                if words.len() >= 2 {
                    let table = words[0].trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                    
                    if words.len() >= 2 && words[1].eq_ignore_ascii_case("AS") && words.len() >= 3 {
                        // FROM table AS alias
                        let alias = words[2].trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                        if !alias.is_empty() && !table.is_empty() {
                            aliases.insert(alias.to_uppercase(), table.to_string());
                        }
                    } else if words.len() >= 2 {
                        // FROM table alias (without AS)
                        let potential_alias = words[1].trim_matches(|c: char| !c.is_alphanumeric() && c != '_').to_uppercase();
                        // 检查是否是关键字
                        let keywords = ["WHERE", "JOIN", "LEFT", "RIGHT", "INNER", "OUTER", "ON", "AND", "OR", "GROUP", "ORDER", "HAVING", "LIMIT", "SET"];
                        if !keywords.contains(&potential_alias.as_str()) && !potential_alias.is_empty() && !table.is_empty() {
                            aliases.insert(potential_alias, table.to_string());
                        }
                    }
                }
                
                pos = start;
            }
        }
        
        aliases
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
        let keywords = ["SELECT ", "WHERE ", "AND ", "OR ", "SET ", "ORDER BY ", "GROUP BY ", "HAVING ", "ON "];
        keywords.iter().any(|k| input.contains(k))
    }
    
    /// 从上下文提取所有相关表名
    fn extract_tables_from_context(&self, input: &str, aliases: &HashMap<String, String>) -> Vec<String> {
        let mut tables = Vec::new();
        
        // 查找 FROM 和 JOIN 后面的表名
        let patterns = ["FROM ", "JOIN "];
        
        for pattern in &patterns {
            let mut pos = 0;
            while let Some(found) = input[pos..].find(pattern) {
                let start = pos + found + pattern.len();
                let rest = &input[start..];
                
                if let Some(table) = rest.split_whitespace().next() {
                    let table = table.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                    if !table.is_empty() && !tables.contains(&table.to_string()) {
                        tables.push(table.to_string());
                    }
                }
                
                pos = start;
            }
        }
        
        // 添加别名对应的表
        for table in aliases.values() {
            if !tables.contains(table) {
                tables.push(table.clone());
            }
        }
        
        tables
    }
    
    /// 获取当前词的补全建议（用于内联建议）
    pub fn get_current_word_completion(&self, input: &str, cursor: usize) -> Option<String> {
        let completions = self.complete(input, cursor);
        
        if completions.is_empty() {
            return None;
        }
        
        let input_slice = if cursor <= input.len() {
            &input[..cursor]
        } else {
            input
        };
        let current_word = self.get_current_word(input_slice);
        
        if current_word.is_empty() {
            return None;
        }
        
        // 处理 table.column 格式
        let (prefix, search_word) = if let Some((table, col)) = self.parse_dot_notation(current_word) {
            (format!("{}.", table), col)
        } else {
            (String::new(), current_word.to_string())
        };
        
        // 返回第一个匹配的补全的后缀
        for completion in &completions {
            let text = if prefix.is_empty() {
                &completion.text
            } else {
                // 对于 table.column 格式，只匹配列名部分
                if completion.text.starts_with(&prefix) {
                    &completion.text[prefix.len()..]
                } else {
                    continue;
                }
            };
            
            let text_upper = text.to_uppercase();
            let search_upper = search_word.to_uppercase();
            
            if text_upper.starts_with(&search_upper) && text_upper.len() > search_upper.len() {
                let suffix_start = search_word.len();
                let original_suffix = &text[suffix_start..];
                
                // 根据用户输入的大小写风格调整补全
                let styled_suffix = if search_word.chars().all(|c| c.is_lowercase() || !c.is_alphabetic()) {
                    original_suffix.to_lowercase()
                } else if search_word.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
                    original_suffix.to_uppercase()
                } else {
                    original_suffix.to_uppercase()
                };
                
                return Some(styled_suffix);
            }
        }
        
        None
    }
}

