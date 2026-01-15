//! 语义匹配模块
//!
//! 理解用户意图，提供语义相关的命令建议

use std::collections::HashMap;
use crate::completions::context_analyzer::WorkContext;

/// 意图匹配结果
#[derive(Debug, Clone)]
pub struct IntentMatch {
    /// 意图名称
    pub intent: String,
    /// 匹配的命令列表
    pub commands: Vec<String>,
    /// 匹配分数（0-100）
    pub score: u8,
    /// 是否包含参数建议
    pub has_args: bool,
}

/// 语义匹配器
pub struct SemanticMatcher {
    /// 意图 -> 命令列表
    intent_commands: HashMap<String, Vec<String>>,
    /// 命令 -> 关键词列表
    command_keywords: HashMap<String, Vec<String>>,
    /// 同义词表
    synonyms: HashMap<String, Vec<String>>,
    /// 命令参数映射（命令 -> 常用参数组合）
    command_args: HashMap<String, Vec<String>>,
}

impl SemanticMatcher {
    /// 创建新的语义匹配器
    pub fn new() -> Self {
        let mut matcher = SemanticMatcher {
            intent_commands: HashMap::new(),
            command_keywords: HashMap::new(),
            synonyms: HashMap::new(),
            command_args: HashMap::new(),
        };
        
        // 初始化意图和命令映射
        matcher.init_intents();
        matcher.init_synonyms();
        matcher.init_command_args();
        
        matcher
    }

    /// 初始化命令意图映射
    fn init_intents(&mut self) {
        // 查看文件意图
        self.intent_commands.insert("view_file".to_string(), vec![
            "cat".to_string(),
            "less".to_string(),
            "head".to_string(),
            "tail".to_string(),
            "more".to_string(),
            "bat".to_string(),
        ]);
        
        // 搜索文本意图
        self.intent_commands.insert("search_text".to_string(), vec![
            "grep".to_string(),
            "rg".to_string(),
            "ag".to_string(),
            "ack".to_string(),
        ]);
        
        // 列出文件意图
        self.intent_commands.insert("list_files".to_string(), vec![
            "ls".to_string(),
            "tree".to_string(),
            "exa".to_string(),
            "fd".to_string(),
        ]);
        
        // 编辑文件意图
        self.intent_commands.insert("edit_file".to_string(), vec![
            "vim".to_string(),
            "vi".to_string(),
            "nano".to_string(),
            "emacs".to_string(),
            "code".to_string(),
        ]);
        
        // 复制文件意图
        self.intent_commands.insert("copy_file".to_string(), vec![
            "cp".to_string(),
            "rsync".to_string(),
        ]);
        
        // 移动文件意图
        self.intent_commands.insert("move_file".to_string(), vec![
            "mv".to_string(),
        ]);
        
        // 删除文件意图
        self.intent_commands.insert("delete_file".to_string(), vec![
            "rm".to_string(),
            "trash".to_string(),
        ]);
        
        // 查找文件意图
        self.intent_commands.insert("find_file".to_string(), vec![
            "find".to_string(),
            "fd".to_string(),
            "locate".to_string(),
        ]);
        
        // 压缩文件意图
        self.intent_commands.insert("compress".to_string(), vec![
            "tar".to_string(),
            "zip".to_string(),
            "gzip".to_string(),
            "bzip2".to_string(),
            "xz".to_string(),
        ]);
        
        // 解压文件意图
        self.intent_commands.insert("extract".to_string(), vec![
            "tar".to_string(),
            "unzip".to_string(),
            "gunzip".to_string(),
            "bunzip2".to_string(),
            "unxz".to_string(),
        ]);
        
        // 网络请求意图
        self.intent_commands.insert("http_request".to_string(), vec![
            "curl".to_string(),
            "wget".to_string(),
        ]);
        
        // 进程管理意图
        self.intent_commands.insert("process_manage".to_string(), vec![
            "ps".to_string(),
            "top".to_string(),
            "htop".to_string(),
            "kill".to_string(),
            "pkill".to_string(),
        ]);
        
        // 系统信息意图
        self.intent_commands.insert("system_info".to_string(), vec![
            "uname".to_string(),
            "hostname".to_string(),
            "uptime".to_string(),
            "df".to_string(),
            "du".to_string(),
            "free".to_string(),
        ]);
    }

    /// 初始化同义词表
    fn init_synonyms(&mut self) {
        // 查看相关
        self.synonyms.insert("查看".to_string(), vec!["view".to_string(), "show".to_string(), "display".to_string(), "read".to_string()]);
        self.synonyms.insert("显示".to_string(), vec!["view".to_string(), "show".to_string(), "display".to_string()]);
        self.synonyms.insert("看".to_string(), vec!["view".to_string(), "show".to_string(), "read".to_string()]);
        
        // 搜索相关
        self.synonyms.insert("搜索".to_string(), vec!["search".to_string(), "find".to_string(), "grep".to_string()]);
        self.synonyms.insert("查找".to_string(), vec!["search".to_string(), "find".to_string(), "locate".to_string()]);
        self.synonyms.insert("找".to_string(), vec!["find".to_string(), "search".to_string()]);
        
        // 列表相关
        self.synonyms.insert("列表".to_string(), vec!["list".to_string(), "ls".to_string()]);
        self.synonyms.insert("列出".to_string(), vec!["list".to_string(), "ls".to_string()]);
        
        // 编辑相关
        self.synonyms.insert("编辑".to_string(), vec!["edit".to_string(), "modify".to_string()]);
        self.synonyms.insert("修改".to_string(), vec!["edit".to_string(), "modify".to_string()]);
        
        // 复制相关
        self.synonyms.insert("复制".to_string(), vec!["copy".to_string(), "cp".to_string()]);
        self.synonyms.insert("拷贝".to_string(), vec!["copy".to_string(), "cp".to_string()]);
        
        // 移动相关
        self.synonyms.insert("移动".to_string(), vec!["move".to_string(), "mv".to_string()]);
        
        // 删除相关
        self.synonyms.insert("删除".to_string(), vec!["delete".to_string(), "remove".to_string(), "rm".to_string()]);
        self.synonyms.insert("移除".to_string(), vec!["remove".to_string(), "delete".to_string()]);
    }

    /// 识别用户意图（基础版本，保持向后兼容）
    pub fn identify_intent(&self, input: &str) -> Vec<String> {
        let matches = self.identify_intent_advanced(input, None);
        matches.into_iter()
            .flat_map(|m| m.commands)
            .collect()
    }
    
    /// 识别用户意图（增强版：支持上下文和模糊匹配）
    pub fn identify_intent_advanced(&self, input: &str, context: Option<&WorkContext>) -> Vec<IntentMatch> {
        let input_lower = input.to_lowercase();
        let mut matches = Vec::new();
        
        // 1. 基础意图匹配（精确匹配）
        for (intent, commands) in &self.intent_commands {
            let score = self.calculate_intent_score(&input_lower, intent, context);
            if score > 0 {
                matches.push(IntentMatch {
                    intent: intent.clone(),
                    commands: commands.clone(),
                    score,
                    has_args: self.command_args.contains_key(&commands[0]),
                });
            }
        }
        
        // 2. 复合意图识别（如"查看日志文件"）
        let composite_matches = self.identify_composite_intent(&input_lower, context);
        matches.extend(composite_matches);
        
        // 3. 模糊匹配（编辑距离）
        let fuzzy_matches = self.fuzzy_match_intent(&input_lower);
        matches.extend(fuzzy_matches);
        
        // 按分数排序
        matches.sort_by(|a, b| b.score.cmp(&a.score));
        
        // 去重（保留分数最高的）
        let mut seen = HashMap::new();
        matches.retain(|m| {
            if let Some(existing) = seen.get(&m.intent) {
                if m.score > *existing {
                    *seen.get_mut(&m.intent).unwrap() = m.score;
                    true
                } else {
                    false
                }
            } else {
                seen.insert(m.intent.clone(), m.score);
                true
            }
        });
        
        matches.truncate(5); // 返回前5个最匹配的
        matches
    }
    
    /// 计算意图匹配分数
    fn calculate_intent_score(&self, input: &str, intent: &str, context: Option<&WorkContext>) -> u8 {
        let mut score = 0u8;
        
        // 直接匹配意图名（高分）
        if input.contains(intent) {
            score += 80;
        }
        
        // 匹配关键词
        let keywords = self.get_intent_keywords(intent);
        for keyword in &keywords {
            if input.contains(keyword) {
                score += 20;
            }
        }
        
        // 上下文增强（如在 Git 仓库中，"提交"→git commit）
        if let Some(ctx) = context {
            score += self.context_boost(input, intent, ctx);
        }
        
        score.min(100)
    }
    
    /// 上下文增强分数
    fn context_boost(&self, input: &str, intent: &str, context: &WorkContext) -> u8 {
        let mut boost = 0u8;
        
        // Git 上下文增强
        if context.is_git_repo {
            if intent == "git_operations" || input.contains("提交") || input.contains("commit") {
                boost += 15;
            }
            if input.contains("状态") || input.contains("status") {
                boost += 10;
            }
        }
        
        // 项目类型增强
        if let Some(ref project_type) = context.project_type {
            match project_type {
                crate::completions::context_analyzer::ProjectType::Rust => {
                    if input.contains("运行") || input.contains("run") || input.contains("构建") || input.contains("build") {
                        boost += 10;
                    }
                }
                crate::completions::context_analyzer::ProjectType::Python => {
                    if input.contains("运行") || input.contains("run") || input.contains("测试") || input.contains("test") {
                        boost += 10;
                    }
                }
                crate::completions::context_analyzer::ProjectType::NodeJs => {
                    if input.contains("安装") || input.contains("install") || input.contains("运行") || input.contains("run") {
                        boost += 10;
                    }
                }
                _ => {}
            }
        }
        
        boost
    }
    
    /// 识别复合意图（如"查看日志文件"→tail -f）
    fn identify_composite_intent(&self, input: &str, _context: Option<&WorkContext>) -> Vec<IntentMatch> {
        let mut matches = Vec::new();
        
        // 日志查看模式
        if input.contains("日志") || input.contains("log") {
            if input.contains("实时") || input.contains("实时") || input.contains("follow") || input.contains("tail") {
                matches.push(IntentMatch {
                    intent: "view_log_realtime".to_string(),
                    commands: vec!["tail -f".to_string(), "journalctl -f".to_string()],
                    score: 90,
                    has_args: true,
                });
            } else if input.contains("最近") || input.contains("recent") || input.contains("last") {
                matches.push(IntentMatch {
                    intent: "view_log_recent".to_string(),
                    commands: vec!["tail -n 100".to_string(), "journalctl -n 100".to_string()],
                    score: 85,
                    has_args: true,
                });
            }
        }
        
        // 文件搜索组合（find + grep）
        if (input.contains("查找") || input.contains("find")) && (input.contains("内容") || input.contains("content") || input.contains("文本") || input.contains("text")) {
            matches.push(IntentMatch {
                intent: "find_and_grep".to_string(),
                commands: vec!["find . -type f -exec grep -l".to_string()],
                score: 80,
                has_args: true,
            });
        }
        
        matches
    }
    
    /// 模糊匹配意图（使用编辑距离）
    fn fuzzy_match_intent(&self, input: &str) -> Vec<IntentMatch> {
        let mut matches = Vec::new();
        let input_lower = input.to_lowercase();
        
        // 简单的模糊匹配：检查输入是否与关键词相似
        for (intent, commands) in &self.intent_commands {
            let keywords = self.get_intent_keywords(intent);
            for keyword in keywords {
                let similarity = self.calculate_similarity(&input_lower, &keyword);
                if similarity > 0.6 { // 相似度阈值
                    matches.push(IntentMatch {
                        intent: intent.clone(),
                        commands: commands.clone(),
                        score: (similarity * 100.0) as u8,
                        has_args: self.command_args.contains_key(&commands[0]),
                    });
                    break;
                }
            }
        }
        
        matches
    }
    
    /// 计算字符串相似度（简单的编辑距离）
    fn calculate_similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }
        
        // 如果完全匹配
        if s1 == s2 {
            return 1.0;
        }
        
        // 如果一个是另一个的子串
        if s1.contains(s2) || s2.contains(s1) {
            return 0.8;
        }
        
        // 简单的字符重叠度
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        
        let mut matches = 0;
        let min_len = s1_chars.len().min(s2_chars.len());
        
        for i in 0..min_len {
            if s1_chars[i] == s2_chars[i] {
                matches += 1;
            }
        }
        
        if min_len > 0 {
            (matches as f64) / (min_len as f64)
        } else {
            0.0
        }
    }
    
    /// 初始化命令参数映射
    fn init_command_args(&mut self) {
        // Git 命令常用参数
        self.command_args.insert("git".to_string(), vec![
            "status".to_string(),
            "add".to_string(),
            "commit -m".to_string(),
            "push".to_string(),
            "pull".to_string(),
        ]);
        
        // Docker 命令常用参数
        self.command_args.insert("docker".to_string(), vec![
            "ps".to_string(),
            "images".to_string(),
            "run".to_string(),
            "build".to_string(),
        ]);
        
        // Cargo 命令常用参数
        self.command_args.insert("cargo".to_string(), vec![
            "build".to_string(),
            "run".to_string(),
            "test".to_string(),
            "check".to_string(),
        ]);
        
        // Tail 命令常用参数
        self.command_args.insert("tail".to_string(), vec![
            "-f".to_string(),
            "-n 100".to_string(),
            "-F".to_string(),
        ]);
        
        // Grep 命令常用参数
        self.command_args.insert("grep".to_string(), vec![
            "-r".to_string(),
            "-i".to_string(),
            "-n".to_string(),
            "-E".to_string(),
        ]);
    }
    
    /// 根据意图和上下文推荐命令参数
    pub fn infer_command_args(&self, intent: &IntentMatch, context: Option<&WorkContext>) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // 从命令参数映射中获取
        for cmd in &intent.commands {
            if let Some(args) = self.command_args.get(cmd) {
                for arg in args {
                    suggestions.push(format!("{} {}", cmd, arg));
                }
            }
        }
        
        // 根据上下文推荐参数
        if let Some(ctx) = context {
            // Git 上下文
            if ctx.is_git_repo && intent.intent == "git_operations" {
                if let Some(ref git_ctx) = ctx.git_context {
                    if git_ctx.has_uncommitted_changes {
                        suggestions.push("git add".to_string());
                        suggestions.push("git commit -m".to_string());
                    }
                    if git_ctx.has_unpushed_commits {
                        suggestions.push("git push".to_string());
                    }
                }
            }
            
            // 项目类型相关
            if let Some(ref project_type) = ctx.project_type {
                match project_type {
                    crate::completions::context_analyzer::ProjectType::Rust => {
                        if intent.commands.iter().any(|c| c == "cargo") {
                            suggestions.push("cargo build".to_string());
                            suggestions.push("cargo run".to_string());
                        }
                    }
                    crate::completions::context_analyzer::ProjectType::Python => {
                        if intent.commands.iter().any(|c| c.contains("python")) {
                            suggestions.push("python -m pytest".to_string());
                        }
                    }
                    crate::completions::context_analyzer::ProjectType::NodeJs => {
                        if intent.commands.iter().any(|c| c == "npm") {
                            suggestions.push("npm install".to_string());
                            suggestions.push("npm run".to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
        
        suggestions
    }

    /// 检查输入是否匹配意图
    fn matches_intent(&self, input: &str, intent: &str) -> bool {
        // 直接匹配意图名
        if input.contains(intent) {
            return true;
        }
        
        // 匹配意图关键词
        let keywords = self.get_intent_keywords(intent);
        for keyword in keywords {
            if input.contains(&keyword) {
                return true;
            }
        }
        
        false
    }

    /// 获取意图的关键词
    fn get_intent_keywords(&self, intent: &str) -> Vec<String> {
        match intent {
            "view_file" => vec!["view".to_string(), "查看".to_string(), "显示".to_string(), "看".to_string(), "file".to_string(), "文件".to_string()],
            "search_text" => vec!["search".to_string(), "搜索".to_string(), "查找".to_string(), "找".to_string(), "text".to_string(), "文本".to_string()],
            "list_files" => vec!["list".to_string(), "列表".to_string(), "列出".to_string(), "files".to_string(), "文件".to_string()],
            "edit_file" => vec!["edit".to_string(), "编辑".to_string(), "修改".to_string()],
            "copy_file" => vec!["copy".to_string(), "复制".to_string(), "拷贝".to_string()],
            "move_file" => vec!["move".to_string(), "移动".to_string()],
            "delete_file" => vec!["delete".to_string(), "删除".to_string(), "移除".to_string()],
            "find_file" => vec!["find".to_string(), "查找".to_string(), "找".to_string()],
            "compress" => vec!["compress".to_string(), "压缩".to_string(), "打包".to_string()],
            "extract" => vec!["extract".to_string(), "解压".to_string(), "解包".to_string()],
            "http_request" => vec!["http".to_string(), "download".to_string(), "下载".to_string(), "请求".to_string()],
            "process_manage" => vec!["process".to_string(), "进程".to_string(), "kill".to_string()],
            "system_info" => vec!["info".to_string(), "信息".to_string(), "system".to_string(), "系统".to_string()],
            _ => Vec::new(),
        }
    }

    /// 基于意图推荐命令
    pub fn recommend_commands(&self, intent: &str) -> Vec<String> {
        self.intent_commands
            .get(intent)
            .cloned()
            .unwrap_or_default()
    }

    /// 语义相似度匹配
    pub fn semantic_match(&self, query: &str, commands: &[String]) -> Vec<String> {
        let query_lower = query.to_lowercase();
        let mut matches = Vec::new();
        
        // 扩展查询词（包括同义词）
        let expanded_query = self.expand_query(&query_lower);
        
        for cmd in commands {
            // 检查命令名是否包含查询词
            if cmd.to_lowercase().contains(&query_lower) {
                matches.push(cmd.clone());
                continue;
            }
            
            // 检查是否匹配同义词
            for synonym in &expanded_query {
                if cmd.to_lowercase().contains(synonym) {
                    matches.push(cmd.clone());
                    break;
                }
            }
        }
        
        matches
    }

    /// 扩展查询词（添加同义词）
    fn expand_query(&self, query: &str) -> Vec<String> {
        let mut expanded = vec![query.to_string()];
        
        // 查找同义词
        for (word, synonyms) in &self.synonyms {
            if query.contains(word) {
                expanded.extend(synonyms.clone());
            }
        }
        
        expanded
    }

    /// 检查输入是否看起来像意图描述
    pub fn looks_like_intent(&self, input: &str) -> bool {
        if input.is_empty() || input.len() < 2 {
            return false;
        }
        
        let input_lower = input.to_lowercase();
        
        // 检查是否包含中文（可能是意图描述）
        if input.chars().any(|c| c as u32 >= 0x4E00 && c as u32 <= 0x9FFF) {
            return true;
        }
        
        // 检查是否包含常见意图关键词
        let intent_keywords = vec![
            "view", "show", "display", "read",
            "search", "find", "look",
            "list", "ls",
            "edit", "modify",
            "copy", "cp",
            "move", "mv",
            "delete", "remove", "rm",
        ];
        
        for keyword in intent_keywords {
            if input_lower.contains(keyword) {
                return true;
            }
        }
        
        false
    }
}

impl Default for SemanticMatcher {
    fn default() -> Self {
        Self::new()
    }
}

