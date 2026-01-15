//! 智能学习模块
//!
//! 实时学习用户习惯，个性化建议

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use std::time::SystemTime;
use crate::completions::context_analyzer::WorkContext;
use crate::engine::Completion;

/// 用户画像
#[derive(Debug, Clone)]
pub struct UserProfile {
    /// 常用命令及使用频率
    pub favorite_commands: HashMap<String, usize>,
    /// 命令模式（命令序列）
    pub command_patterns: Vec<CommandPattern>,
    /// 时间偏好（小时 -> 常用命令）
    pub time_preferences: HashMap<u8, Vec<String>>,
    /// 项目偏好（项目类型 -> 常用命令）
    pub project_preferences: HashMap<String, Vec<String>>,
    /// 命令选择历史（用于反馈学习）
    pub selection_history: Vec<SelectionRecord>,
}

/// 命令模式
#[derive(Debug, Clone)]
pub struct CommandPattern {
    /// 模式名称
    pub name: String,
    /// 命令序列
    pub sequence: Vec<String>,
    /// 使用频率
    pub frequency: usize,
    /// 最后使用时间（Unix 时间戳）
    pub last_used: u64,
}

/// 选择记录（用于反馈学习）
#[derive(Debug, Clone)]
pub struct SelectionRecord {
    /// 选择的命令
    pub command: String,
    /// 上下文
    pub context: String,
    /// 时间戳
    pub timestamp: u64,
    /// 是否被接受
    pub accepted: bool,
}

/// 学习引擎
pub struct LearningEngine {
    /// 用户画像
    profile: UserProfile,
    /// 反馈历史
    feedback_history: Vec<SelectionRecord>,
    /// 数据文件路径
    data_path: PathBuf,
}

impl LearningEngine {
    /// 创建新的学习引擎
    pub fn new() -> Self {
        let data_path = Self::get_data_path();
        
        let mut engine = LearningEngine {
            profile: UserProfile {
                favorite_commands: HashMap::new(),
                command_patterns: Vec::new(),
                time_preferences: HashMap::new(),
                project_preferences: HashMap::new(),
                selection_history: Vec::new(),
            },
            feedback_history: Vec::new(),
            data_path,
        };
        
        // 加载已保存的数据
        engine.load_data();
        
        engine
    }
    
    /// 获取数据文件路径
    fn get_data_path() -> PathBuf {
        if let Some(mut cache_dir) = dirs::cache_dir() {
            cache_dir.push("cnmsb");
            let _ = fs::create_dir_all(&cache_dir);
            cache_dir.push("user_profile.json");
            cache_dir
        } else {
            std::env::temp_dir().join("cnmsb_profile.json")
        }
    }
    
    /// 学习用户选择
    pub fn learn_from_selection(&mut self, selected: &str, context: &WorkContext) {
        // 更新常用命令频率
        *self.profile.favorite_commands.entry(selected.to_string()).or_insert(0) += 1;
        
        // 更新时间偏好
        let hour = context.time_context.hour;
        let time_commands = self.profile.time_preferences.entry(hour).or_insert_with(Vec::new);
        if !time_commands.contains(&selected.to_string()) {
            time_commands.push(selected.to_string());
            // 限制每个小时的命令数量
            if time_commands.len() > 10 {
                time_commands.remove(0);
            }
        }
        
        // 更新项目偏好
        if let Some(ref project_type) = context.project_type {
            let project_key = format!("{:?}", project_type);
            let project_commands = self.profile.project_preferences.entry(project_key).or_insert_with(Vec::new);
            if !project_commands.contains(&selected.to_string()) {
                project_commands.push(selected.to_string());
                if project_commands.len() > 10 {
                    project_commands.remove(0);
                }
            }
        }
        
        // 记录选择历史
        let timestamp = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        self.profile.selection_history.push(SelectionRecord {
            command: selected.to_string(),
            context: context.cwd.clone(),
            timestamp,
            accepted: true,
        });
        
        // 限制历史记录数量
        if self.profile.selection_history.len() > 1000 {
            self.profile.selection_history.remove(0);
        }
        
        // 定期保存
        if self.profile.selection_history.len() % 10 == 0 {
            self.save_data();
        }
    }
    
    /// 学习命令模式
    pub fn learn_pattern(&mut self, sequence: Vec<String>) {
        if sequence.len() < 2 {
            return;
        }
        
        let pattern_name = sequence.join(" -> ");
        
        // 查找是否已存在类似模式
        if let Some(pattern) = self.profile.command_patterns.iter_mut()
            .find(|p| p.sequence == sequence) {
            pattern.frequency += 1;
            pattern.last_used = SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
        } else {
            // 创建新模式
            self.profile.command_patterns.push(CommandPattern {
                name: pattern_name,
                sequence,
                frequency: 1,
                last_used: SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0),
            });
        }
        
        // 限制模式数量
        if self.profile.command_patterns.len() > 100 {
            // 按频率和时间排序，保留最常用的
            self.profile.command_patterns.sort_by(|a, b| {
                b.frequency.cmp(&a.frequency).then_with(|| b.last_used.cmp(&a.last_used))
            });
            self.profile.command_patterns.truncate(100);
        }
    }
    
    /// 个性化排序建议
    pub fn personalize_ranking(&self, suggestions: &mut Vec<Completion>, context: &WorkContext) {
        // 根据用户偏好调整分数
        for suggestion in suggestions.iter_mut() {
            let mut boost = 0i64;
            
            // 常用命令加分
            if let Some(&freq) = self.profile.favorite_commands.get(&suggestion.text) {
                boost += (freq.min(100) as i64) * 2; // 最多加200分
            }
            
            // 时间偏好加分
            let hour = context.time_context.hour;
            if let Some(commands) = self.profile.time_preferences.get(&hour) {
                if commands.contains(&suggestion.text) {
                    boost += 50;
                }
            }
            
            // 项目偏好加分
            if let Some(ref project_type) = context.project_type {
                let project_key = format!("{:?}", project_type);
                if let Some(commands) = self.profile.project_preferences.get(&project_key) {
                    if commands.contains(&suggestion.text) {
                        boost += 30;
                    }
                }
            }
            
            // 应用加分
            suggestion.score += boost;
        }
        
        // 重新排序
        suggestions.sort_by(|a, b| b.score.cmp(&a.score));
    }
    
    /// 获取个性化建议（基于用户习惯）
    pub fn get_personalized_suggestions(&self, context: &WorkContext) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // 基于时间偏好
        let hour = context.time_context.hour;
        if let Some(commands) = self.profile.time_preferences.get(&hour) {
            suggestions.extend(commands.clone());
        }
        
        // 基于项目偏好
        if let Some(ref project_type) = context.project_type {
            let project_key = format!("{:?}", project_type);
            if let Some(commands) = self.profile.project_preferences.get(&project_key) {
                suggestions.extend(commands.clone());
            }
        }
        
        // 基于常用命令（取前5个）
        let mut favorite: Vec<_> = self.profile.favorite_commands.iter().collect();
        favorite.sort_by(|a, b| b.1.cmp(a.1));
        for (cmd, _) in favorite.iter().take(5) {
            suggestions.push(cmd.to_string());
        }
        
        suggestions.dedup();
        suggestions.truncate(10);
        suggestions
    }
    
    /// 保存数据
    fn save_data(&self) {
        #[derive(serde::Serialize)]
        struct ProfileData {
            favorite_commands: HashMap<String, usize>,
            time_preferences: HashMap<u8, Vec<String>>,
            project_preferences: HashMap<String, Vec<String>>,
        }
        
        let data = ProfileData {
            favorite_commands: self.profile.favorite_commands.clone(),
            time_preferences: self.profile.time_preferences.clone(),
            project_preferences: self.profile.project_preferences.clone(),
        };
        
        if let Ok(json) = serde_json::to_string_pretty(&data) {
            let _ = fs::write(&self.data_path, json);
        }
    }
    
    /// 加载数据
    fn load_data(&mut self) {
        #[derive(serde::Deserialize)]
        struct ProfileData {
            favorite_commands: HashMap<String, usize>,
            time_preferences: HashMap<u8, Vec<String>>,
            project_preferences: HashMap<String, Vec<String>>,
        }
        
        if let Ok(content) = fs::read_to_string(&self.data_path) {
            if let Ok(data) = serde_json::from_str::<ProfileData>(&content) {
                self.profile.favorite_commands = data.favorite_commands;
                self.profile.time_preferences = data.time_preferences;
                self.profile.project_preferences = data.project_preferences;
            }
        }
    }
}

impl Default for LearningEngine {
    fn default() -> Self {
        Self::new()
    }
}

