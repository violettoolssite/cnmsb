//! 上下文分析模块
//!
//! 分析当前工作目录、Git 状态、项目类型等，提供智能建议

use std::path::Path;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

/// 工作上下文（增强版）
#[derive(Debug, Clone)]
pub struct WorkContext {
    /// 当前工作目录
    pub cwd: String,
    /// 是否在 Git 仓库中
    pub is_git_repo: bool,
    /// Git 上下文（如果有）
    pub git_context: Option<GitContext>,
    /// 项目类型
    pub project_type: Option<ProjectType>,
    /// 最近执行的命令
    pub recent_commands: Vec<String>,
    /// 最近访问的文件
    pub recent_files: Vec<String>,
    /// 文件内容提示（从最近编辑的文件中提取）
    pub file_content_hints: Vec<String>,
    /// 当前工作流模式
    pub workflow_pattern: Option<WorkflowPattern>,
    /// 时间上下文
    pub time_context: TimeContext,
}

/// 工作流模式
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowPattern {
    Development,
    Testing,
    Deployment,
    Debugging,
    GitWorkflow,
    Build,
}

/// 时间上下文
#[derive(Debug, Clone)]
pub struct TimeContext {
    /// 当前小时（0-23）
    pub hour: u8,
    /// 是否在工作时间（9-18）
    pub is_work_hours: bool,
    /// 是否在早上（6-12）
    pub is_morning: bool,
    /// 是否在下午（12-18）
    pub is_afternoon: bool,
    /// 是否在晚上（18-24）
    pub is_evening: bool,
}

/// Git 上下文
#[derive(Debug, Clone)]
pub struct GitContext {
    /// 是否有未提交的更改
    pub has_uncommitted_changes: bool,
    /// 是否有未推送的提交
    pub has_unpushed_commits: bool,
    /// 当前分支
    pub current_branch: Option<String>,
    /// 是否有未跟踪的文件
    pub has_untracked_files: bool,
}

/// 项目类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectType {
    Rust,
    Python,
    NodeJs,
    Go,
    Java,
    Docker,
    Kubernetes,
    Terraform,
    Ansible,
}

/// 上下文分析器
pub struct ContextAnalyzer;

impl ContextAnalyzer {
    /// 创建新的上下文分析器
    pub fn new() -> Self {
        ContextAnalyzer
    }

    /// 分析当前工作目录（基础版本，保持向后兼容）
    pub fn analyze_cwd(&self) -> WorkContext {
        self.analyze_cwd_enhanced(Vec::new())
    }
    
    /// 分析当前工作目录（增强版）
    pub fn analyze_cwd_enhanced(&self, recent_commands: Vec<String>) -> WorkContext {
        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        
        let git_context = self.detect_git_context(&cwd);
        let project_type = self.detect_project_type(&cwd);
        let recent_files = self.get_recent_files(&cwd);
        let file_content_hints = self.extract_file_hints(&cwd, &recent_files);
        let workflow_pattern = self.detect_workflow(&recent_commands);
        let time_context = self.get_time_context();
        
        WorkContext {
            cwd: cwd.clone(),
            is_git_repo: git_context.is_some(),
            git_context,
            project_type,
            recent_commands,
            recent_files,
            file_content_hints,
            workflow_pattern,
            time_context,
        }
    }
    
    /// 获取最近访问的文件
    fn get_recent_files(&self, cwd: &str) -> Vec<String> {
        let mut files = Vec::new();
        let path = Path::new(cwd);
        
        // 查找最近修改的文件（最多10个）
        if let Ok(entries) = fs::read_dir(path) {
            let mut file_times: Vec<(String, SystemTime)> = Vec::new();
            
            for entry in entries.flatten() {
                let file_path = entry.path();
                if file_path.is_file() {
                    if let Ok(metadata) = fs::metadata(&file_path) {
                        if let Ok(modified) = metadata.modified() {
                            if let Some(name) = file_path.file_name().and_then(|n| n.to_str()) {
                                // 跳过隐藏文件和临时文件
                                if !name.starts_with('.') && !name.ends_with('~') {
                                    file_times.push((name.to_string(), modified));
                                }
                            }
                        }
                    }
                }
            }
            
            // 按修改时间排序
            file_times.sort_by(|a, b| b.1.cmp(&a.1));
            
            // 取前10个
            for (name, _) in file_times.into_iter().take(10) {
                files.push(name);
            }
        }
        
        files
    }
    
    /// 从文件内容中提取提示
    fn extract_file_hints(&self, cwd: &str, recent_files: &[String]) -> Vec<String> {
        let mut hints = Vec::new();
        let path = Path::new(cwd);
        
        // 从最近的文件中提取关键词
        for file in recent_files.iter().take(5) {
            let file_path = path.join(file);
            if let Ok(content) = fs::read_to_string(&file_path) {
                // 提取常见的命令提示（如 TODO、FIXME、import 等）
                for line in content.lines().take(20) {
                    let line_lower = line.to_lowercase();
                    if line_lower.contains("todo") || line_lower.contains("fixme") {
                        hints.push("查看待办事项".to_string());
                    }
                    if line_lower.contains("import") || line_lower.contains("require") {
                        hints.push("检查依赖".to_string());
                    }
                    if line_lower.contains("test") || line_lower.contains("测试") {
                        hints.push("运行测试".to_string());
                    }
                }
            }
        }
        
        hints.dedup();
        hints.truncate(5);
        hints
    }
    
    /// 检测工作流模式
    pub fn detect_workflow(&self, recent_commands: &[String]) -> Option<WorkflowPattern> {
        if recent_commands.is_empty() {
            return None;
        }
        
        // 分析最近命令的模式
        let commands_str = recent_commands.join(" ").to_lowercase();
        
        // Git 工作流
        if commands_str.contains("git") {
            return Some(WorkflowPattern::GitWorkflow);
        }
        
        // 测试工作流
        if commands_str.contains("test") || commands_str.contains("pytest") || commands_str.contains("cargo test") {
            return Some(WorkflowPattern::Testing);
        }
        
        // 构建工作流
        if commands_str.contains("build") || commands_str.contains("make") || commands_str.contains("cargo build") {
            return Some(WorkflowPattern::Build);
        }
        
        // 调试工作流
        if commands_str.contains("debug") || commands_str.contains("gdb") || commands_str.contains("strace") {
            return Some(WorkflowPattern::Debugging);
        }
        
        // 部署工作流
        if commands_str.contains("deploy") || commands_str.contains("docker") || commands_str.contains("kubectl") {
            return Some(WorkflowPattern::Deployment);
        }
        
        // 开发工作流（默认）
        if commands_str.contains("run") || commands_str.contains("cargo run") || commands_str.contains("python") {
            return Some(WorkflowPattern::Development);
        }
        
        None
    }
    
    /// 获取时间上下文
    fn get_time_context(&self) -> TimeContext {
        let hour = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()
            .and_then(|d| {
                let secs = d.as_secs();
                let hours = (secs / 3600) % 24;
                Some(hours as u8)
            })
            .unwrap_or(12);
        
        TimeContext {
            hour,
            is_work_hours: hour >= 9 && hour < 18,
            is_morning: hour >= 6 && hour < 12,
            is_afternoon: hour >= 12 && hour < 18,
            is_evening: hour >= 18 || hour < 6,
        }
    }

    /// 检测 Git 仓库状态
    pub fn detect_git_context(&self, cwd: &str) -> Option<GitContext> {
        let git_dir = Path::new(cwd).join(".git");
        if !git_dir.exists() {
            return None;
        }
        
        // 检查是否有未提交的更改
        let has_uncommitted_changes = self.check_git_uncommitted(cwd);
        let has_untracked_files = self.check_git_untracked(cwd);
        let current_branch = self.get_git_branch(cwd);
        let has_unpushed_commits = self.check_git_unpushed(cwd);
        
        Some(GitContext {
            has_uncommitted_changes,
            has_unpushed_commits,
            current_branch,
            has_untracked_files,
        })
    }

    /// 检查是否有未提交的更改
    fn check_git_uncommitted(&self, cwd: &str) -> bool {
        if let Ok(output) = std::process::Command::new("git")
            .args(&["status", "--porcelain"])
            .current_dir(cwd)
            .output()
        {
            if let Ok(status) = String::from_utf8(output.stdout) {
                // 排除未跟踪的文件
                return status.lines()
                    .any(|line| !line.trim_start().starts_with("??"));
            }
        }
        false
    }

    /// 检查是否有未跟踪的文件
    fn check_git_untracked(&self, cwd: &str) -> bool {
        if let Ok(output) = std::process::Command::new("git")
            .args(&["status", "--porcelain"])
            .current_dir(cwd)
            .output()
        {
            if let Ok(status) = String::from_utf8(output.stdout) {
                return status.lines()
                    .any(|line| line.trim_start().starts_with("??"));
            }
        }
        false
    }

    /// 获取当前 Git 分支
    fn get_git_branch(&self, cwd: &str) -> Option<String> {
        if let Ok(output) = std::process::Command::new("git")
            .args(&["branch", "--show-current"])
            .current_dir(cwd)
            .output()
        {
            if let Ok(branch) = String::from_utf8(output.stdout) {
                let branch = branch.trim().to_string();
                if !branch.is_empty() {
                    return Some(branch);
                }
            }
        }
        None
    }

    /// 检查是否有未推送的提交
    fn check_git_unpushed(&self, cwd: &str) -> bool {
        if let Ok(output) = std::process::Command::new("git")
            .args(&["log", "@{u}..HEAD", "--oneline"])
            .current_dir(cwd)
            .output()
        {
            if let Ok(log) = String::from_utf8(output.stdout) {
                return !log.trim().is_empty();
            }
        }
        false
    }

    /// 检测项目类型
    pub fn detect_project_type(&self, cwd: &str) -> Option<ProjectType> {
        let path = Path::new(cwd);
        
        // Rust 项目
        if path.join("Cargo.toml").exists() || path.join("Cargo.lock").exists() {
            return Some(ProjectType::Rust);
        }
        
        // Python 项目
        if path.join("setup.py").exists()
            || path.join("pyproject.toml").exists()
            || path.join("requirements.txt").exists()
            || path.join("Pipfile").exists()
        {
            return Some(ProjectType::Python);
        }
        
        // Node.js 项目
        if path.join("package.json").exists() {
            return Some(ProjectType::NodeJs);
        }
        
        // Go 项目
        if path.join("go.mod").exists() || path.join("Gopkg.toml").exists() {
            return Some(ProjectType::Go);
        }
        
        // Java 项目
        if path.join("pom.xml").exists()
            || path.join("build.gradle").exists()
            || path.join("build.gradle.kts").exists()
        {
            return Some(ProjectType::Java);
        }
        
        // Docker 项目
        if path.join("Dockerfile").exists() || path.join("docker-compose.yml").exists() {
            return Some(ProjectType::Docker);
        }
        
        // Kubernetes 项目
        if path.join("k8s").exists() || path.join("kubernetes").exists() {
            return Some(ProjectType::Kubernetes);
        }
        
        // Terraform 项目
        if path.join("terraform.tf").exists()
            || path.join("main.tf").exists()
            || path.join(".terraform").exists()
        {
            return Some(ProjectType::Terraform);
        }
        
        // Ansible 项目
        if path.join("ansible.cfg").exists() || path.join("playbook.yml").exists() {
            return Some(ProjectType::Ansible);
        }
        
        None
    }

    /// 基于上下文推荐命令（增强版）
    pub fn suggest_commands(&self, context: &WorkContext) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Git 相关建议
        if let Some(ref git_ctx) = context.git_context {
            if git_ctx.has_uncommitted_changes {
                suggestions.push("git status".to_string());
                suggestions.push("git add".to_string());
                suggestions.push("git diff".to_string());
            }
            if git_ctx.has_untracked_files {
                suggestions.push("git add".to_string());
            }
            if git_ctx.has_unpushed_commits {
                suggestions.push("git push".to_string());
            }
            suggestions.push("git log".to_string());
        }
        
        // 项目类型相关建议
        if let Some(ref project_type) = context.project_type {
            match project_type {
                ProjectType::Rust => {
                    suggestions.push("cargo build".to_string());
                    suggestions.push("cargo run".to_string());
                    suggestions.push("cargo test".to_string());
                }
                ProjectType::Python => {
                    suggestions.push("python".to_string());
                    suggestions.push("pip install".to_string());
                    suggestions.push("python -m pytest".to_string());
                }
                ProjectType::NodeJs => {
                    suggestions.push("npm install".to_string());
                    suggestions.push("npm run".to_string());
                    suggestions.push("node".to_string());
                }
                ProjectType::Go => {
                    suggestions.push("go build".to_string());
                    suggestions.push("go run".to_string());
                    suggestions.push("go test".to_string());
                }
                ProjectType::Java => {
                    suggestions.push("mvn".to_string());
                    suggestions.push("gradle".to_string());
                }
                ProjectType::Docker => {
                    suggestions.push("docker build".to_string());
                    suggestions.push("docker-compose up".to_string());
                }
                ProjectType::Kubernetes => {
                    suggestions.push("kubectl".to_string());
                    suggestions.push("helm".to_string());
                }
                ProjectType::Terraform => {
                    suggestions.push("terraform".to_string());
                }
                ProjectType::Ansible => {
                    suggestions.push("ansible-playbook".to_string());
                }
            }
        }
        
        // 工作流模式相关建议
        if let Some(ref workflow) = context.workflow_pattern {
            match workflow {
                WorkflowPattern::Development => {
                    suggestions.push("cargo run".to_string());
                    suggestions.push("python".to_string());
                    suggestions.push("npm run dev".to_string());
                }
                WorkflowPattern::Testing => {
                    suggestions.push("cargo test".to_string());
                    suggestions.push("pytest".to_string());
                    suggestions.push("npm test".to_string());
                }
                WorkflowPattern::Build => {
                    suggestions.push("cargo build --release".to_string());
                    suggestions.push("make".to_string());
                    suggestions.push("npm run build".to_string());
                }
                WorkflowPattern::Debugging => {
                    suggestions.push("gdb".to_string());
                    suggestions.push("strace".to_string());
                    suggestions.push("valgrind".to_string());
                }
                WorkflowPattern::Deployment => {
                    suggestions.push("docker build".to_string());
                    suggestions.push("kubectl apply".to_string());
                    suggestions.push("terraform apply".to_string());
                }
                WorkflowPattern::GitWorkflow => {
                    suggestions.push("git status".to_string());
                    suggestions.push("git add".to_string());
                    suggestions.push("git commit".to_string());
                }
            }
        }
        
        // 时间上下文相关建议
        if context.time_context.is_morning {
            // 早上常用构建和运行
            suggestions.push("cargo build".to_string());
            suggestions.push("make".to_string());
        } else if context.time_context.is_evening {
            // 晚上常用测试和清理
            suggestions.push("cargo test".to_string());
            suggestions.push("make clean".to_string());
        }
        
        // 去重并限制数量
        suggestions.sort();
        suggestions.dedup();
        suggestions.truncate(10);
        
        suggestions
    }
}

impl Default for ContextAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

