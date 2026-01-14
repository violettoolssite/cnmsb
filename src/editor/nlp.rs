//! 自然语言理解和智能补全
//!
//! 基于自然语言分析用户意图，提供智能补全建议

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

/// 自然语言分析器
pub struct NLPAnalyzer {
    /// 意图模式匹配
    intent_patterns: HashMap<String, Vec<String>>,
    /// 路径查找器
    path_finder: PathFinder,
}

/// 路径查找器
pub struct PathFinder;

/// 用户意图
#[derive(Debug, Clone)]
pub enum UserIntent {
    /// 设置环境变量
    SetEnvVar { var_name: String, var_type: EnvVarType },
    /// 配置 PATH
    ConfigurePath { home_vars: Vec<String> },
    /// 查找路径
    FindPath { keywords: Vec<String> },
    /// 其他
    Unknown,
}

/// 环境变量类型
#[derive(Debug, Clone)]
pub enum EnvVarType {
    /// Java 相关
    Java,
    /// Hadoop 相关
    Hadoop,
    /// Maven 相关
    Maven,
    /// Python 相关
    Python,
    /// Node.js 相关
    Node,
    /// 其他
    Other(String),
}

impl PathFinder {
    pub fn new() -> Self {
        PathFinder
    }

    /// 查找 Java 安装路径
    pub fn find_java_paths(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        // 常见 Java 安装路径
        let common_paths = vec![
            "/usr/lib/jvm",
            "/opt/jdk",
            "/opt/java",
            "/usr/java",
            "/usr/local/java",
            "/opt/openjdk",
        ];
        
        for base in &common_paths {
            let path = Path::new(base);
            if path.exists() {
                // 查找子目录
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries.flatten() {
                        let entry_path = entry.path();
                        if entry_path.is_dir() {
                            // 检查是否是有效的 Java 目录（包含 bin/java）
                            let java_bin = entry_path.join("bin").join("java");
                            if java_bin.exists() {
                                paths.push(entry_path);
                            }
                        }
                    }
                }
            }
        }
        
        // 也检查直接路径
        for base in &common_paths {
            let java_bin = Path::new(base).join("bin").join("java");
            if java_bin.exists() {
                paths.push(PathBuf::from(*base));
            }
        }
        
        paths
    }

    /// 查找 Hadoop 安装路径
    pub fn find_hadoop_paths(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        let common_paths = vec![
            "/opt/hadoop",
            "/usr/local/hadoop",
            "/opt/hadoop-*",
            "/usr/lib/hadoop",
        ];
        
        for base in common_paths {
            let path = Path::new(base);
            if path.exists() && path.is_dir() {
                // 检查是否是有效的 Hadoop 目录（包含 bin/hadoop）
                let hadoop_bin = path.join("bin").join("hadoop");
                if hadoop_bin.exists() {
                    paths.push(path.to_path_buf());
                }
            }
        }
        
        // 使用 glob 模式查找
        if let Ok(entries) = fs::read_dir("/opt") {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    let name = entry_path.file_name().and_then(|n| n.to_str());
                    if let Some(name) = name {
                        if name.starts_with("hadoop") {
                            let hadoop_bin = entry_path.join("bin").join("hadoop");
                            if hadoop_bin.exists() {
                                paths.push(entry_path);
                            }
                        }
                    }
                }
            }
        }
        
        paths
    }

    /// 查找 Maven 安装路径
    pub fn find_maven_paths(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        let common_paths = vec![
            "/opt/maven",
            "/opt/apache-maven",
            "/usr/local/maven",
            "/usr/lib/maven",
        ];
        
        for base in common_paths {
            let path = Path::new(base);
            if path.exists() && path.is_dir() {
                let mvn_bin = path.join("bin").join("mvn");
                if mvn_bin.exists() {
                    paths.push(path.to_path_buf());
                }
            }
        }
        
        // 使用 glob 模式查找
        if let Ok(entries) = fs::read_dir("/opt") {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    let name = entry_path.file_name().and_then(|n| n.to_str());
                    if let Some(name) = name {
                        if name.contains("maven") {
                            let mvn_bin = entry_path.join("bin").join("mvn");
                            if mvn_bin.exists() {
                                paths.push(entry_path);
                            }
                        }
                    }
                }
            }
        }
        
        paths
    }

    /// 查找 Python 安装路径
    pub fn find_python_paths(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        // 查找 python3 可执行文件
        if let Ok(output) = std::process::Command::new("which")
            .arg("python3")
            .output()
        {
            if let Ok(path_str) = String::from_utf8(output.stdout) {
                let path_str = path_str.trim();
                if let Some(parent) = Path::new(path_str).parent().and_then(|p| p.parent()) {
                    if parent.exists() {
                        paths.push(parent.to_path_buf());
                    }
                }
            }
        }
        
        // 常见路径
        let common_paths = vec![
            "/usr/bin",
            "/usr/local/bin",
            "/opt/python",
            "/opt/python3",
        ];
        
        for base in common_paths {
            let python_bin = Path::new(base).join("python3");
            if python_bin.exists() {
                if let Some(parent) = python_bin.parent().and_then(|p| p.parent()) {
                    paths.push(parent.to_path_buf());
                }
            }
        }
        
        paths
    }

    /// 查找 Node.js 安装路径
    pub fn find_node_paths(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        // 查找 node 可执行文件
        if let Ok(output) = std::process::Command::new("which")
            .arg("node")
            .output()
        {
            if let Ok(path_str) = String::from_utf8(output.stdout) {
                let path_str = path_str.trim();
                if let Some(parent) = Path::new(path_str).parent().and_then(|p| p.parent()) {
                    if parent.exists() {
                        paths.push(parent.to_path_buf());
                    }
                }
            }
        }
        
        // 常见路径
        let common_paths = vec![
            "/usr/bin",
            "/usr/local/bin",
            "/opt/node",
            "/opt/nodejs",
            "/usr/local/node",
        ];
        
        for base in common_paths {
            let node_bin = Path::new(base).join("node");
            if node_bin.exists() {
                if let Some(parent) = node_bin.parent().and_then(|p| p.parent()) {
                    paths.push(parent.to_path_buf());
                }
            }
        }
        
        paths
    }

    /// 通用路径查找（在 /opt, /usr/local, /usr 中搜索）
    pub fn find_paths_generic(&self, keyword: &str) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        let keyword_lower = keyword.to_lowercase();
        
        let search_dirs = vec!["/opt", "/usr/local", "/usr"];
        
        for search_dir in search_dirs {
            if let Ok(entries) = fs::read_dir(search_dir) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        let name = entry_path.file_name().and_then(|n| n.to_str());
                        if let Some(name) = name {
                            let name_lower = name.to_lowercase();
                            // 检查名称是否包含关键词
                            if name_lower.contains(&keyword_lower) {
                                paths.push(entry_path);
                            }
                        }
                    }
                }
            }
        }
        
        paths
    }

    /// 根据关键词查找路径
    pub fn find_paths_by_keywords(&self, keywords: &[String]) -> Vec<PathBuf> {
        let mut all_paths = Vec::new();
        
        for keyword in keywords {
            let keyword_lower = keyword.to_lowercase();
            
            if keyword_lower.contains("java") || keyword_lower.contains("jdk") {
                all_paths.extend(self.find_java_paths());
            }
            if keyword_lower.contains("hadoop") {
                all_paths.extend(self.find_hadoop_paths());
            }
            if keyword_lower.contains("maven") {
                all_paths.extend(self.find_maven_paths());
            }
            if keyword_lower.contains("python") {
                all_paths.extend(self.find_python_paths());
            }
            if keyword_lower.contains("node") {
                all_paths.extend(self.find_node_paths());
            }
            
            // 通用查找
            all_paths.extend(self.find_paths_generic(keyword));
        }
        
        // 去重
        all_paths.sort();
        all_paths.dedup();
        
        all_paths
    }
}

impl NLPAnalyzer {
    pub fn new() -> Self {
        let mut intent_patterns = HashMap::new();
        
        // Java 相关意图
        intent_patterns.insert("java".to_string(), vec![
            "java".to_string(),
            "jdk".to_string(),
            "jre".to_string(),
            "openjdk".to_string(),
        ]);
        
        // Hadoop 相关意图
        intent_patterns.insert("hadoop".to_string(), vec![
            "hadoop".to_string(),
        ]);
        
        // Maven 相关意图
        intent_patterns.insert("maven".to_string(), vec![
            "maven".to_string(),
            "apache-maven".to_string(),
        ]);
        
        NLPAnalyzer {
            intent_patterns,
            path_finder: PathFinder::new(),
        }
    }

    /// 分析用户输入，识别意图
    pub fn analyze_intent(&self, text: &str) -> UserIntent {
        let text_lower = text.to_lowercase();
        
        // 检查是否是设置环境变量的意图
        if text_lower.contains("export") {
            // 提取变量名
            if let Some(var_name) = self.extract_var_name(text) {
                // 识别变量类型
                let var_type = self.identify_var_type(&var_name, &text_lower);
                
                return UserIntent::SetEnvVar {
                    var_name,
                    var_type,
                };
            }
        }
        
        // 检查是否是配置 PATH 的意图
        if text_lower.contains("path") && (text_lower.contains("export") || text_lower.contains("=")) {
            // 查找相关的 *_HOME 变量
            let home_vars = self.extract_home_vars(text);
            return UserIntent::ConfigurePath { home_vars };
        }
        
        // 检查是否是查找路径的意图
        let keywords = self.extract_keywords(text);
        if !keywords.is_empty() {
            return UserIntent::FindPath { keywords };
        }
        
        UserIntent::Unknown
    }

    /// 提取变量名
    fn extract_var_name(&self, text: &str) -> Option<String> {
        // 匹配 export VAR= 或 VAR=
        let patterns = vec![
            r"export\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=",
            r"([a-zA-Z_][a-zA-Z0-9_]*)\s*=",
        ];
        
        for pattern in patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                if let Some(captures) = re.captures(text) {
                    if let Some(var_name) = captures.get(1) {
                        return Some(var_name.as_str().to_string());
                    }
                }
            }
        }
        
        None
    }

    /// 识别变量类型
    fn identify_var_type(&self, var_name: &str, text: &str) -> EnvVarType {
        let var_lower = var_name.to_lowercase();
        let text_lower = text.to_lowercase();
        
        if var_lower.contains("java") || var_lower.contains("jdk") || 
           text_lower.contains("java") || text_lower.contains("jdk") {
            return EnvVarType::Java;
        }
        
        if var_lower.contains("hadoop") || text_lower.contains("hadoop") {
            return EnvVarType::Hadoop;
        }
        
        if var_lower.contains("maven") || text_lower.contains("maven") {
            return EnvVarType::Maven;
        }
        
        if var_lower.contains("python") || text_lower.contains("python") {
            return EnvVarType::Python;
        }
        
        if var_lower.contains("node") || text_lower.contains("node") {
            return EnvVarType::Node;
        }
        
        EnvVarType::Other(var_name.to_string())
    }

    /// 提取关键词（增强版，支持更多模式）
    fn extract_keywords(&self, text: &str) -> Vec<String> {
        let text_lower = text.to_lowercase();
        let mut keywords = Vec::new();
        
        // 检查各种技术关键词
        let tech_keywords = vec![
            "java", "jdk", "jre", "openjdk", "oracle-jdk",
            "hadoop", "spark", "hdfs", "yarn",
            "maven", "gradle", "ant",
            "python", "python3", "pyenv",
            "node", "nodejs", "npm", "nvm",
            "go", "golang",
            "rust", "cargo",
            "docker", "kubernetes", "k8s",
            "mysql", "postgresql", "mongodb",
            "redis", "elasticsearch",
        ];
        
        for keyword in &tech_keywords {
            if text_lower.contains(keyword) {
                keywords.push(keyword.to_string());
            }
        }
        
        // 从变量名中提取关键词
        if let Ok(re) = regex::Regex::new(r"([A-Z_][A-Z0-9_]*_HOME)") {
            for cap in re.captures_iter(text) {
                if let Some(var) = cap.get(1) {
                    let var_lower = var.as_str().to_lowercase();
                    // 提取变量名中的技术关键词
                    for keyword in &tech_keywords {
                        if var_lower.contains(keyword) {
                            keywords.push(keyword.to_string());
                        }
                    }
                }
            }
        }
        
        keywords
    }

    /// 提取 *_HOME 变量
    fn extract_home_vars(&self, text: &str) -> Vec<String> {
        let mut home_vars = Vec::new();
        
        // 匹配 *_HOME 模式
        if let Ok(re) = regex::Regex::new(r"([A-Z_][A-Z0-9_]*_HOME)") {
            for cap in re.captures_iter(text) {
                if let Some(var) = cap.get(1) {
                    home_vars.push(var.as_str().to_string());
                }
            }
        }
        
        home_vars
    }

    /// 基于意图生成智能建议
    pub fn generate_suggestions(&self, intent: &UserIntent, context: &HashMap<String, String>) -> Vec<String> {
        match intent {
            UserIntent::SetEnvVar { var_name, var_type } => {
                self.suggest_env_var_value(var_name, var_type, context)
            }
            UserIntent::ConfigurePath { home_vars } => {
                self.suggest_path_configuration(home_vars, context)
            }
            UserIntent::FindPath { keywords } => {
                self.suggest_paths_by_keywords(keywords)
            }
            UserIntent::Unknown => Vec::new(),
        }
    }

    /// 建议环境变量值
    fn suggest_env_var_value(
        &self,
        _var_name: &str,
        var_type: &EnvVarType,
        _context: &HashMap<String, String>,
    ) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        match var_type {
            EnvVarType::Java => {
                let paths = self.path_finder.find_java_paths();
                for path in paths {
                    suggestions.push(path.to_string_lossy().to_string());
                }
                // 如果没有找到，提供常见路径建议
                if suggestions.is_empty() {
                    suggestions.push("/opt/jdk".to_string());
                    suggestions.push("/usr/lib/jvm/default-java".to_string());
                }
            }
            EnvVarType::Hadoop => {
                let paths = self.path_finder.find_hadoop_paths();
                for path in paths {
                    suggestions.push(path.to_string_lossy().to_string());
                }
                if suggestions.is_empty() {
                    suggestions.push("/opt/hadoop".to_string());
                }
            }
            EnvVarType::Maven => {
                let paths = self.path_finder.find_maven_paths();
                for path in paths {
                    suggestions.push(path.to_string_lossy().to_string());
                }
                if suggestions.is_empty() {
                    suggestions.push("/opt/maven".to_string());
                    suggestions.push("/opt/apache-maven".to_string());
                }
            }
            EnvVarType::Python => {
                let paths = self.path_finder.find_python_paths();
                for path in paths {
                    suggestions.push(path.to_string_lossy().to_string());
                }
                if suggestions.is_empty() {
                    suggestions.push("/usr".to_string());
                    suggestions.push("/usr/local".to_string());
                }
            }
            EnvVarType::Node => {
                let paths = self.path_finder.find_node_paths();
                for path in paths {
                    suggestions.push(path.to_string_lossy().to_string());
                }
                if suggestions.is_empty() {
                    suggestions.push("/usr".to_string());
                    suggestions.push("/usr/local".to_string());
                }
            }
            EnvVarType::Other(ref var_name) => {
                // 对于其他变量，尝试根据变量名查找路径
                let keywords = vec![var_name.clone()];
                let paths = self.path_finder.find_paths_by_keywords(&keywords);
                for path in paths {
                    suggestions.push(path.to_string_lossy().to_string());
                }
            }
        }
        
        suggestions
    }

    /// 建议 PATH 配置
    fn suggest_path_configuration(
        &self,
        home_vars: &[String],
        context: &HashMap<String, String>,
    ) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // 获取现有的 PATH
        let existing_path = context.get("PATH")
            .map(|s| s.as_str())
            .unwrap_or("$PATH");
        
        // 为每个 *_HOME 变量生成建议
        for home_var in home_vars {
            let base = format!("${}", home_var);
            suggestions.push(format!("{}:{}/bin:{}/sbin", existing_path, base, base));
            suggestions.push(format!("{}:{}/bin", existing_path, base));
        }
        
        // 如果没有 *_HOME 变量，但上下文中有相关变量，生成建议
        if home_vars.is_empty() {
            // 查找可能的 *_HOME 变量
            for (key, _) in context {
                if key.ends_with("_HOME") {
                    let base = format!("${}", key);
                    suggestions.push(format!("{}:{}/bin:{}/sbin", existing_path, base, base));
                }
            }
        }
        
        suggestions
    }

    /// 根据关键词建议路径
    fn suggest_paths_by_keywords(&self, keywords: &[String]) -> Vec<String> {
        self.path_finder.find_paths_by_keywords(keywords)
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect()
    }
}

impl Default for NLPAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PathFinder {
    fn default() -> Self {
        Self::new()
    }
}

