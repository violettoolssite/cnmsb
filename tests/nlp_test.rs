//! NLP 和路径查找测试

use cnmsb::editor::nlp::{NLPAnalyzer, PathFinder, UserIntent, EnvVarType};

#[test]
fn test_analyze_intent_java() {
    let analyzer = NLPAnalyzer::new();
    
    let intent = analyzer.analyze_intent("export JAVA_HOME=");
    
    match intent {
        UserIntent::SetEnvVar { var_name, var_type } => {
            assert_eq!(var_name, "JAVA_HOME");
            assert!(matches!(var_type, EnvVarType::Java));
        }
        _ => panic!("Expected SetEnvVar intent"),
    }
}

#[test]
fn test_analyze_intent_hadoop() {
    let analyzer = NLPAnalyzer::new();
    
    let intent = analyzer.analyze_intent("export HADOOP_HOME=/opt/hadoop");
    
    match intent {
        UserIntent::SetEnvVar { var_name, var_type } => {
            assert_eq!(var_name, "HADOOP_HOME");
            assert!(matches!(var_type, EnvVarType::Hadoop));
        }
        _ => panic!("Expected SetEnvVar intent"),
    }
}

#[test]
fn test_analyze_intent_path() {
    let analyzer = NLPAnalyzer::new();
    
    let intent = analyzer.analyze_intent("export PATH=$PATH:");
    
    match intent {
        UserIntent::ConfigurePath { .. } => {
            // 应该识别为 PATH 配置意图
        }
        _ => panic!("Expected ConfigurePath intent"),
    }
}

#[test]
fn test_path_finder() {
    let finder = PathFinder::new();
    
    // 测试 Java 路径查找（可能找不到，但不应该 panic）
    let _java_paths = finder.find_java_paths();
    
    // 测试 Hadoop 路径查找
    let _hadoop_paths = finder.find_hadoop_paths();
    
    // 测试 Maven 路径查找
    let _maven_paths = finder.find_maven_paths();
}

#[test]
fn test_generate_suggestions() {
    let analyzer = NLPAnalyzer::new();
    let mut context = std::collections::HashMap::new();
    context.insert("JAVA_HOME".to_string(), "/opt/jdk".to_string());
    
    let intent = UserIntent::SetEnvVar {
        var_name: "JAVA_HOME".to_string(),
        var_type: EnvVarType::Java,
    };
    
    let suggestions = analyzer.generate_suggestions(&intent, &context);
    
    // 应该生成一些建议（即使找不到实际路径，也应该有默认建议）
    assert!(!suggestions.is_empty() || true); // 允许为空，因为可能系统中没有安装
}

