//! SQL 补全引擎测试

use cnmsb::sql::{SqlEngine, DatabaseType};

#[test]
fn test_sql_keyword_completion() {
    let engine = SqlEngine::new(DatabaseType::SQLite);
    let completions = engine.complete("SEL", 3);
    
    // 应该包含 SELECT
    assert!(completions.iter().any(|c| c.text.to_uppercase() == "SELECT"));
    println!("SELECT 关键字补全: {:?}", completions.iter().map(|c| &c.text).collect::<Vec<_>>());
}

#[test]
fn test_table_completion() {
    let mut engine = SqlEngine::new(DatabaseType::SQLite);
    engine.set_tables(vec!["users".to_string(), "orders".to_string(), "products".to_string()]);
    
    let completions = engine.complete("SELECT * FROM u", 15);
    
    // 应该包含 users 表
    assert!(completions.iter().any(|c| c.text.to_lowercase() == "users"));
    println!("表名补全: {:?}", completions.iter().map(|c| &c.text).collect::<Vec<_>>());
}

#[test]
fn test_column_completion() {
    let mut engine = SqlEngine::new(DatabaseType::SQLite);
    engine.set_tables(vec!["users".to_string()]);
    engine.set_columns("users", vec!["id".to_string(), "name".to_string(), "email".to_string()]);
    
    let completions = engine.complete("SELECT n FROM users", 8);
    
    // 应该包含 name 列
    assert!(completions.iter().any(|c| c.text.to_lowercase() == "name"));
    println!("列名补全: {:?}", completions.iter().map(|c| &c.text).collect::<Vec<_>>());
}

#[test]
fn test_dot_notation_completion() {
    let mut engine = SqlEngine::new(DatabaseType::SQLite);
    engine.set_tables(vec!["users".to_string()]);
    engine.set_columns("users", vec!["id".to_string(), "name".to_string(), "email".to_string()]);
    
    let completions = engine.complete("SELECT users.n", 14);
    
    // 应该包含 users.name
    println!("table.column 补全: {:?}", completions.iter().map(|c| &c.text).collect::<Vec<_>>());
    assert!(completions.iter().any(|c| c.text.contains("name")));
}

#[test]
fn test_inline_suggestion() {
    let engine = SqlEngine::new(DatabaseType::SQLite);
    
    // 测试 sel -> SELECT
    let suggestion = engine.get_current_word_completion("sel", 3);
    assert!(suggestion.is_some());
    println!("内联建议 'sel' -> 'sel{}'", suggestion.unwrap());
}

fn main() {
    println!("=== SQL 补全引擎测试 ===\n");
    
    let mut engine = SqlEngine::new(DatabaseType::SQLite);
    engine.set_tables(vec!["users".to_string(), "orders".to_string(), "products".to_string()]);
    engine.set_columns("users", vec!["id".to_string(), "name".to_string(), "email".to_string()]);
    engine.set_columns("orders", vec!["id".to_string(), "user_id".to_string(), "total".to_string()]);
    engine.set_columns("products", vec!["id".to_string(), "name".to_string(), "price".to_string()]);
    
    println!("1. SELECT 关键字补全 (输入: 'SEL'):");
    for c in engine.complete("SEL", 3).iter().take(5) {
        println!("   - {} ({})", c.text, c.description);
    }
    
    println!("\n2. FROM 后表名补全 (输入: 'SELECT * FROM u'):");
    for c in engine.complete("SELECT * FROM u", 15) {
        println!("   - {} ({})", c.text, c.description);
    }
    
    println!("\n3. SELECT 后列名补全 (输入: 'SELECT n FROM users'):");
    for c in engine.complete("SELECT n FROM users", 8).iter().take(10) {
        println!("   - {} ({})", c.text, c.description);
    }
    
    println!("\n4. table.column 格式补全 (输入: 'SELECT users.n'):");
    for c in engine.complete("SELECT users.n", 14) {
        println!("   - {} ({})", c.text, c.description);
    }
    
    println!("\n5. 内联建议:");
    if let Some(s) = engine.get_current_word_completion("sel", 3) {
        println!("   'sel' -> 'sel{}'", s);
    }
    if let Some(s) = engine.get_current_word_completion("SELECT * FROM us", 16) {
        println!("   'us' -> 'us{}'", s);
    }
    
    println!("\n=== 测试完成 ===");
}

