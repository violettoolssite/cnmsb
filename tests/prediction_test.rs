//! 命令序列预测测试

use cnmsb::completions::prediction::CommandSequencePredictor;

#[test]
fn test_command_sequence_learning() {
    let mut predictor = CommandSequencePredictor::new();
    
    // 模拟历史命令序列
    let history = vec![
        "git add .".to_string(),
        "git commit -m 'test'".to_string(),
        "git push".to_string(),
        "git add .".to_string(),
        "git commit -m 'fix'".to_string(),
        "git push".to_string(),
    ];
    
    // 学习序列模式
    predictor.learn_from_history(&history);
    
    // 测试预测
    let predictions = predictor.predict_next("git");
    assert!(!predictions.is_empty(), "应该能预测到下一个命令");
    
    // git add 后应该预测到 git commit
    let predictions = predictor.predict_next("git add");
    assert!(predictions.contains(&"commit".to_string()) || predictions.contains(&"git".to_string()),
        "git add 后应该预测到 commit");
}

#[test]
fn test_context_prediction() {
    let mut predictor = CommandSequencePredictor::new();
    
    // 记录在不同目录下的命令
    predictor.record_command_in_context("/home/user/project", "cargo build");
    predictor.record_command_in_context("/home/user/project", "cargo test");
    predictor.record_command_in_context("/home/user/project", "git status");
    
    // 测试上下文预测
    let predictions = predictor.predict_from_context("/home/user/project", &[]);
    assert!(!predictions.is_empty(), "应该能基于上下文预测命令");
}

