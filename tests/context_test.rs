//! 上下文感应补全测试

use cnmsb::completions::context::ContextAwareCompleter;
use cnmsb::parser::CommandParser;

// 注意：HistoryParser 是私有的，这些测试在 context.rs 的 #[cfg(test)] 模块中

#[test]
fn test_suggest_path_value() {
    let mut completer = ContextAwareCompleter::new();
    
    // 设置环境变量
    completer.update_env_var("JAVA_HOME".to_string(), "/opt/jdk".to_string());
    completer.update_env_var("HADOOP_HOME".to_string(), "/opt/hadoop".to_string());
    completer.update_env_var("PATH".to_string(), "$PATH:$JAVA_HOME/bin".to_string());
    
    let suggestions = completer.suggest_path_value();
    
    // 应该包含 HADOOP_HOME 的建议
    assert!(suggestions.iter().any(|s| s.contains("HADOOP_HOME")));
    assert!(suggestions.iter().any(|s| s.contains("/bin")));
    assert!(suggestions.iter().any(|s| s.contains("/sbin")));
    
    // 应该包含 $PATH 前缀
    assert!(suggestions.iter().any(|s| s.contains("$PATH")));
}

#[test]
fn test_path_suggestion_with_multiple_home_vars() {
    let mut completer = ContextAwareCompleter::new();
    
    // 设置多个 *_HOME 变量
    completer.update_env_var("JAVA_HOME".to_string(), "/opt/jdk".to_string());
    completer.update_env_var("HADOOP_HOME".to_string(), "/opt/hadoop".to_string());
    completer.update_env_var("MAVEN_HOME".to_string(), "/opt/maven".to_string());
    
    let suggestions = completer.suggest_path_value();
    
    // 应该包含所有 *_HOME 变量的建议
    assert!(suggestions.iter().any(|s| s.contains("JAVA_HOME")));
    assert!(suggestions.iter().any(|s| s.contains("HADOOP_HOME")));
    assert!(suggestions.iter().any(|s| s.contains("MAVEN_HOME")));
    
    // 应该有组合建议
    let has_combined = suggestions.iter().any(|s| {
        s.contains("JAVA_HOME") && s.contains("HADOOP_HOME")
    });
    assert!(has_combined || suggestions.len() > 3); // 至少应该有多个单独的建议
}

#[test]
fn test_is_export_command() {
    let completer = ContextAwareCompleter::new();
    let parser = CommandParser::new();
    
    // 测试 export 命令
    let parsed = parser.parse("export PATH=", 12);
    assert!(completer.is_export_command(&parsed));
    
    // 测试非 export 命令
    let parsed = parser.parse("ls -la", 6);
    assert!(!completer.is_export_command(&parsed));
}

#[test]
fn test_complete_env_var_path() {
    let mut completer = ContextAwareCompleter::new();
    
    // 设置环境变量
    completer.update_env_var("JAVA_HOME".to_string(), "/opt/jdk".to_string());
    completer.update_env_var("HADOOP_HOME".to_string(), "/opt/hadoop".to_string());
    
    let parser = CommandParser::new();
    
    // 测试 export PATH= 的情况
    let parsed = parser.parse("export PATH=", 12);
    let completions = completer.complete_env_var(&parsed);
    
    // 应该有 PATH 建议
    assert!(!completions.is_empty());
    assert!(completions.iter().any(|c| c.text.contains("HADOOP_HOME")));
}

#[test]
fn test_context_from_history() {
    let mut completer = ContextAwareCompleter::new();
    
    let history = vec![
        "export JAVA_HOME=/opt/jdk".to_string(),
        "export PATH=$PATH:$JAVA_HOME/bin".to_string(),
        "export HADOOP_HOME=/opt/hadoop".to_string(),
    ];
    
    completer.extract_env_vars(&history);
    
    // 验证环境变量已提取
    let env_vars = completer.get_env_vars();
    assert_eq!(env_vars.get("JAVA_HOME"), Some(&"/opt/jdk".to_string()));
    assert_eq!(env_vars.get("HADOOP_HOME"), Some(&"/opt/hadoop".to_string()));
    
    // 测试 PATH 建议
    let suggestions = completer.suggest_path_value();
    assert!(suggestions.iter().any(|s| s.contains("HADOOP_HOME")));
}

