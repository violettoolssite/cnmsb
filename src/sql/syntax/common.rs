//! 通用 SQL 语法定义

/// SQL 补全项
#[derive(Debug, Clone)]
pub struct SqlCompletion {
    pub text: String,
    pub description: String,
    pub kind: SqlCompletionKind,
}

impl SqlCompletion {
    pub fn keyword(text: &str, desc: &str) -> Self {
        SqlCompletion {
            text: text.to_string(),
            description: desc.to_string(),
            kind: SqlCompletionKind::Keyword,
        }
    }
    
    pub fn function(text: &str, desc: &str) -> Self {
        SqlCompletion {
            text: text.to_string(),
            description: desc.to_string(),
            kind: SqlCompletionKind::Function,
        }
    }
    
    pub fn data_type(text: &str, desc: &str) -> Self {
        SqlCompletion {
            text: text.to_string(),
            description: desc.to_string(),
            kind: SqlCompletionKind::DataType,
        }
    }
    
    pub fn table(text: &str) -> Self {
        SqlCompletion {
            text: text.to_string(),
            description: "表".to_string(),
            kind: SqlCompletionKind::Table,
        }
    }
    
    pub fn column(text: &str, table: &str) -> Self {
        SqlCompletion {
            text: text.to_string(),
            description: format!("列 ({})", table),
            kind: SqlCompletionKind::Column,
        }
    }
}

/// SQL 补全类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlCompletionKind {
    Keyword,
    Function,
    DataType,
    Table,
    Column,
    Database,
    Operator,
    Snippet,
}

impl SqlCompletionKind {
    pub fn color(&self) -> &'static str {
        match self {
            SqlCompletionKind::Keyword => "\x1b[38;5;226m",   // 鲜艳黄色（与主 shell 一致）
            SqlCompletionKind::Function => "\x1b[38;5;226m",  // 鲜艳黄色
            SqlCompletionKind::DataType => "\x1b[38;5;171m",  // 紫色
            SqlCompletionKind::Table => "\x1b[38;5;40m",      // 绿色
            SqlCompletionKind::Column => "\x1b[38;5;51m",     // 亮青色
            SqlCompletionKind::Database => "\x1b[38;5;208m",  // 橙色
            SqlCompletionKind::Operator => "\x1b[38;5;226m",  // 鲜艳黄色
            SqlCompletionKind::Snippet => "\x1b[36m",         // 青色（子命令风格）
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            SqlCompletionKind::Keyword => "K",
            SqlCompletionKind::Function => "F",
            SqlCompletionKind::DataType => "T",
            SqlCompletionKind::Table => "表",
            SqlCompletionKind::Column => "列",
            SqlCompletionKind::Database => "库",
            SqlCompletionKind::Operator => "O",
            SqlCompletionKind::Snippet => "S",
        }
    }
}

/// SQL 语法特征
pub trait SqlSyntax: Send + Sync {
    /// 获取关键字列表
    fn keywords(&self) -> Vec<SqlCompletion>;
    
    /// 获取函数列表
    fn functions(&self) -> Vec<SqlCompletion>;
    
    /// 获取数据类型列表
    fn data_types(&self) -> Vec<SqlCompletion>;
    
    /// 获取操作符列表
    fn operators(&self) -> Vec<SqlCompletion>;
    
    /// 获取代码片段
    fn snippets(&self) -> Vec<SqlCompletion>;
    
    /// 获取所有补全项
    fn all_completions(&self) -> Vec<SqlCompletion> {
        let mut completions = Vec::new();
        completions.extend(self.keywords());
        completions.extend(self.functions());
        completions.extend(self.data_types());
        completions.extend(self.operators());
        completions.extend(self.snippets());
        completions
    }
    
    /// 根据上下文获取补全建议
    fn complete(&self, input: &str, cursor: usize) -> Vec<SqlCompletion> {
        let input = if cursor <= input.len() {
            &input[..cursor]
        } else {
            input
        };
        
        let input_upper = input.to_uppercase();
        let input_lower = input.to_lowercase();
        
        // 获取当前正在输入的词
        let current_word = input.split_whitespace().last().unwrap_or("");
        let current_word_upper = current_word.to_uppercase();
        
        // 判断上下文
        let context = self.detect_context(&input_upper);
        
        // 根据上下文过滤补全项
        let mut completions = match context {
            SqlContext::Start | SqlContext::AfterSemicolon => {
                self.keywords().into_iter()
                    .filter(|c| {
                        let starts = ["SELECT", "INSERT", "UPDATE", "DELETE", "CREATE", 
                                     "ALTER", "DROP", "SHOW", "DESCRIBE", "USE", 
                                     "EXPLAIN", "BEGIN", "COMMIT", "ROLLBACK", "SET"];
                        starts.iter().any(|s| c.text.to_uppercase().starts_with(s))
                    })
                    .collect()
            }
            SqlContext::AfterSelect => {
                let mut comps = vec![
                    SqlCompletion::keyword("*", "所有列"),
                    SqlCompletion::keyword("DISTINCT", "去重"),
                    SqlCompletion::keyword("COUNT(*)", "计数"),
                    SqlCompletion::keyword("FROM", "数据来源"),
                ];
                comps.extend(self.functions());
                comps
            }
            SqlContext::AfterFrom | SqlContext::AfterJoin => {
                // 这里应该返回表名，暂时返回空
                vec![
                    SqlCompletion::keyword("WHERE", "条件过滤"),
                    SqlCompletion::keyword("JOIN", "连接表"),
                    SqlCompletion::keyword("LEFT JOIN", "左连接"),
                    SqlCompletion::keyword("RIGHT JOIN", "右连接"),
                    SqlCompletion::keyword("INNER JOIN", "内连接"),
                    SqlCompletion::keyword("ORDER BY", "排序"),
                    SqlCompletion::keyword("GROUP BY", "分组"),
                    SqlCompletion::keyword("LIMIT", "限制行数"),
                ]
            }
            SqlContext::AfterWhere | SqlContext::AfterAnd | SqlContext::AfterOr => {
                let mut comps = self.operators();
                comps.extend(vec![
                    SqlCompletion::keyword("AND", "与条件"),
                    SqlCompletion::keyword("OR", "或条件"),
                    SqlCompletion::keyword("NOT", "非条件"),
                    SqlCompletion::keyword("IN", "包含"),
                    SqlCompletion::keyword("LIKE", "模糊匹配"),
                    SqlCompletion::keyword("BETWEEN", "范围"),
                    SqlCompletion::keyword("IS NULL", "空值"),
                    SqlCompletion::keyword("IS NOT NULL", "非空值"),
                    SqlCompletion::keyword("EXISTS", "存在"),
                ]);
                comps
            }
            SqlContext::AfterCreate => {
                vec![
                    SqlCompletion::keyword("TABLE", "创建表"),
                    SqlCompletion::keyword("DATABASE", "创建数据库"),
                    SqlCompletion::keyword("INDEX", "创建索引"),
                    SqlCompletion::keyword("VIEW", "创建视图"),
                    SqlCompletion::keyword("PROCEDURE", "创建存储过程"),
                    SqlCompletion::keyword("FUNCTION", "创建函数"),
                    SqlCompletion::keyword("TRIGGER", "创建触发器"),
                    SqlCompletion::keyword("USER", "创建用户"),
                ]
            }
            SqlContext::AfterAlter => {
                vec![
                    SqlCompletion::keyword("TABLE", "修改表"),
                    SqlCompletion::keyword("DATABASE", "修改数据库"),
                    SqlCompletion::keyword("INDEX", "修改索引"),
                    SqlCompletion::keyword("USER", "修改用户"),
                ]
            }
            SqlContext::AfterDrop => {
                vec![
                    SqlCompletion::keyword("TABLE", "删除表"),
                    SqlCompletion::keyword("DATABASE", "删除数据库"),
                    SqlCompletion::keyword("INDEX", "删除索引"),
                    SqlCompletion::keyword("VIEW", "删除视图"),
                    SqlCompletion::keyword("PROCEDURE", "删除存储过程"),
                    SqlCompletion::keyword("FUNCTION", "删除函数"),
                    SqlCompletion::keyword("TRIGGER", "删除触发器"),
                    SqlCompletion::keyword("USER", "删除用户"),
                ]
            }
            SqlContext::AfterInsert => {
                vec![
                    SqlCompletion::keyword("INTO", "插入到"),
                ]
            }
            SqlContext::AfterUpdate => {
                vec![
                    SqlCompletion::keyword("SET", "设置值"),
                ]
            }
            SqlContext::AfterSet => {
                vec![]
            }
            SqlContext::AfterOrderBy | SqlContext::AfterGroupBy => {
                vec![
                    SqlCompletion::keyword("ASC", "升序"),
                    SqlCompletion::keyword("DESC", "降序"),
                    SqlCompletion::keyword("HAVING", "分组过滤"),
                ]
            }
            SqlContext::DataType => {
                self.data_types()
            }
            SqlContext::Unknown => {
                self.all_completions()
            }
        };
        
        // 根据当前输入过滤
        if !current_word.is_empty() {
            completions = completions.into_iter()
                .filter(|c| c.text.to_uppercase().starts_with(&current_word_upper))
                .collect();
        }
        
        // 根据用户输入的大小写风格调整补全项
        if !current_word.is_empty() {
            let is_all_lower = current_word.chars().all(|c| c.is_lowercase() || !c.is_alphabetic());
            let is_all_upper = current_word.chars().all(|c| c.is_uppercase() || !c.is_alphabetic());
            
            if is_all_lower {
                // 用户输入全小写，补全项也用小写
                completions = completions.into_iter()
                    .map(|mut c| {
                        c.text = c.text.to_lowercase();
                        c
                    })
                    .collect();
            } else if is_all_upper {
                // 用户输入全大写，补全项也用大写（默认就是大写，不需要改）
            } else {
                // 混合大小写，保持补全项的大写格式（SQL 关键字通常大写）
            }
        }
        
        // 排序：精确匹配优先
        completions.sort_by(|a, b| {
            let a_exact = a.text.to_uppercase() == current_word_upper;
            let b_exact = b.text.to_uppercase() == current_word_upper;
            b_exact.cmp(&a_exact).then(a.text.cmp(&b.text))
        });
        
        completions
    }
    
    /// 检测 SQL 上下文
    fn detect_context(&self, input: &str) -> SqlContext {
        let input = input.trim();
        
        if input.is_empty() {
            return SqlContext::Start;
        }
        
        // 检查最后一个分号后的内容
        if let Some(last_stmt) = input.rsplit(';').next() {
            let last_stmt = last_stmt.trim();
            if last_stmt.is_empty() {
                return SqlContext::AfterSemicolon;
            }
        }
        
        // 从后向前查找关键字
        let words: Vec<&str> = input.split_whitespace().collect();
        
        for i in (0..words.len()).rev() {
            let word = words[i].to_uppercase();
            match word.as_str() {
                "SELECT" => return SqlContext::AfterSelect,
                "FROM" => return SqlContext::AfterFrom,
                "WHERE" => return SqlContext::AfterWhere,
                "AND" => return SqlContext::AfterAnd,
                "OR" => return SqlContext::AfterOr,
                "CREATE" => return SqlContext::AfterCreate,
                "ALTER" => return SqlContext::AfterAlter,
                "DROP" => return SqlContext::AfterDrop,
                "INSERT" => return SqlContext::AfterInsert,
                "UPDATE" => return SqlContext::AfterUpdate,
                "SET" => return SqlContext::AfterSet,
                "JOIN" | "INNER" | "LEFT" | "RIGHT" | "OUTER" | "CROSS" => return SqlContext::AfterJoin,
                "ORDER" if i + 1 < words.len() && words[i + 1].to_uppercase() == "BY" => {
                    return SqlContext::AfterOrderBy;
                }
                "GROUP" if i + 1 < words.len() && words[i + 1].to_uppercase() == "BY" => {
                    return SqlContext::AfterGroupBy;
                }
                "BY" => {
                    if i > 0 {
                        let prev = words[i - 1].to_uppercase();
                        if prev == "ORDER" {
                            return SqlContext::AfterOrderBy;
                        } else if prev == "GROUP" {
                            return SqlContext::AfterGroupBy;
                        }
                    }
                }
                _ => {}
            }
        }
        
        SqlContext::Unknown
    }
}

/// SQL 上下文
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlContext {
    Start,
    AfterSemicolon,
    AfterSelect,
    AfterFrom,
    AfterWhere,
    AfterAnd,
    AfterOr,
    AfterJoin,
    AfterCreate,
    AfterAlter,
    AfterDrop,
    AfterInsert,
    AfterUpdate,
    AfterSet,
    AfterOrderBy,
    AfterGroupBy,
    DataType,
    Unknown,
}

