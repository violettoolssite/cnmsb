//! 语义匹配测试

use cnmsb::completions::semantic::SemanticMatcher;

#[test]
fn test_intent_identification() {
    let matcher = SemanticMatcher::new();
    
    // 测试查看文件意图
    let commands = matcher.identify_intent("查看文件");
    assert!(!commands.is_empty(), "应该识别出查看文件的意图");
    assert!(commands.contains(&"cat".to_string()) || commands.contains(&"less".to_string()),
        "应该包含 cat 或 less");
    
    // 测试搜索文本意图
    let commands = matcher.identify_intent("搜索文本");
    assert!(!commands.is_empty(), "应该识别出搜索文本的意图");
    assert!(commands.contains(&"grep".to_string()), "应该包含 grep");
}

#[test]
fn test_looks_like_intent() {
    let matcher = SemanticMatcher::new();
    
    // 测试中文输入
    assert!(matcher.looks_like_intent("查看"), "中文输入应该被识别为意图");
    
    // 测试英文关键词
    assert!(matcher.looks_like_intent("view file"), "英文关键词应该被识别为意图");
    
    // 测试普通命令
    assert!(!matcher.looks_like_intent("ls"), "普通命令不应该被识别为意图");
}

