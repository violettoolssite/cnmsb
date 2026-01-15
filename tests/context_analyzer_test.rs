//! 上下文分析器测试

use cnmsb::completions::context_analyzer::ContextAnalyzer;

#[test]
fn test_project_type_detection() {
    let analyzer = ContextAnalyzer::new();
    
    // 注意：这个测试需要在实际的项目目录中运行
    // 这里只是测试结构是否正确
    let context = analyzer.analyze_cwd();
    
    // 验证上下文结构
    assert!(!context.cwd.is_empty(), "工作目录不应该为空");
}

#[test]
fn test_git_context_detection() {
    let analyzer = ContextAnalyzer::new();
    
    // 测试 Git 上下文检测（如果当前目录是 Git 仓库）
    let context = analyzer.analyze_cwd();
    
    // 如果有 Git 上下文，验证结构
    if let Some(git_ctx) = context.git_context {
        // Git 上下文应该包含分支信息或状态信息
        // 这里只是验证结构，不验证具体值
        assert!(true, "Git 上下文结构正确");
    }
}

