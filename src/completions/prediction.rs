//! 命令序列预测模块
//!
//! 基于历史命令模式，预测下一个可能的命令

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

/// 命令序列预测器
pub struct CommandSequencePredictor {
    /// 命令序列频率表：command1 -> [(command2, frequency), ...]
    sequences: HashMap<String, Vec<(String, usize)>>,
    /// 命令上下文：工作目录 -> 常用命令
    context_commands: HashMap<String, Vec<String>>,
    /// 数据文件路径
    data_path: PathBuf,
}

/// 命令对（用于序列统计）
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct CommandPair {
    first: String,
    second: String,
}

impl CommandSequencePredictor {
    /// 创建新的预测器
    pub fn new() -> Self {
        let data_path = Self::get_data_path();
        
        let mut predictor = CommandSequencePredictor {
            sequences: HashMap::new(),
            context_commands: HashMap::new(),
            data_path,
        };
        
        // 加载已保存的数据
        predictor.load_data();
        
        predictor
    }

    /// 获取数据文件路径
    fn get_data_path() -> PathBuf {
        if let Some(mut cache_dir) = dirs::cache_dir() {
            cache_dir.push("cnmsb");
            // 确保目录存在
            let _ = fs::create_dir_all(&cache_dir);
            cache_dir.push("sequences.json");
            cache_dir
        } else {
            // 回退到临时目录
            std::env::temp_dir().join("cnmsb_sequences.json")
        }
    }

    /// 从历史命令中学习序列模式
    pub fn learn_from_history(&mut self, history: &[String]) {
        if history.len() < 2 {
            return;
        }

        // 提取命令序列对（考虑时间衰减：最近的命令权重更高）
        for i in 0..history.len() - 1 {
            let cmd1 = Self::extract_command(&history[i]);
            let cmd2 = Self::extract_command(&history[i + 1]);
            
            if !cmd1.is_empty() && !cmd2.is_empty() {
                // 更新序列频率
                let entry = self.sequences.entry(cmd1.clone()).or_insert_with(Vec::new);
                
                // 计算权重（最近的命令权重更高）
                let weight = if i < 10 { 2 } else if i < 50 { 1 } else { 1 };
                
                // 查找是否已存在
                if let Some(pos) = entry.iter().position(|(cmd, _)| cmd == &cmd2) {
                    entry[pos].1 += weight;
                } else {
                    entry.push((cmd2, weight));
                }
                
                // 按频率排序
                entry.sort_by(|a, b| b.1.cmp(&a.1));
            }
        }
    }
    
    /// 学习单条命令序列（实时学习）
    pub fn learn_sequence(&mut self, cmd1: &str, cmd2: &str) {
        let cmd1_clean = Self::extract_command(cmd1);
        let cmd2_clean = Self::extract_command(cmd2);
        
        if !cmd1_clean.is_empty() && !cmd2_clean.is_empty() {
            let entry = self.sequences.entry(cmd1_clean).or_insert_with(Vec::new);
            
            // 查找是否已存在
            if let Some(pos) = entry.iter().position(|(cmd, _)| cmd == &cmd2_clean) {
                entry[pos].1 += 2; // 实时学习的权重更高
            } else {
                entry.push((cmd2_clean, 2));
            }
            
            // 按频率排序
            entry.sort_by(|a, b| b.1.cmp(&a.1));
            
            // 限制每个命令的预测数量
            if entry.len() > 20 {
                entry.truncate(20);
            }
        }
    }

    /// 从命令字符串中提取命令名（第一个词）
    fn extract_command(cmd: &str) -> String {
        cmd.trim()
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string()
    }

    /// 预测下一个可能的命令
    pub fn predict_next(&self, current_command: &str) -> Vec<String> {
        let cmd = Self::extract_command(current_command);
        
        if let Some(next_commands) = self.sequences.get(&cmd) {
            // 返回前 10 个最可能的命令（增加数量以提供更多选择）
            next_commands
                .iter()
                .take(10)
                .map(|(cmd, _)| cmd.clone())
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// 基于上一条命令和用户输入预测（智能过滤）
    pub fn predict_next_filtered(&self, current_command: &str, user_input: &str) -> Vec<String> {
        let mut predictions = self.predict_next(current_command);
        
        // 如果用户已输入字符，进行过滤
        if !user_input.is_empty() {
            let input_lower = user_input.to_lowercase();
            predictions.retain(|cmd| {
                cmd.to_lowercase().starts_with(&input_lower)
            });
        }
        
        predictions
    }

    /// 基于上下文预测
    pub fn predict_from_context(&self, cwd: &str, recent: &[String]) -> Vec<String> {
        let mut predictions = Vec::new();
        
        // 基于工作目录的预测
        if let Some(commands) = self.context_commands.get(cwd) {
            predictions.extend(commands.iter().cloned());
        }
        
        // 基于最近命令的预测
        if let Some(last_cmd) = recent.last() {
            let next = self.predict_next(last_cmd);
            predictions.extend(next);
        }
        
        // 去重并限制数量
        predictions.sort();
        predictions.dedup();
        predictions.truncate(10);
        
        predictions
    }

    /// 记录命令在工作目录中的使用
    pub fn record_command_in_context(&mut self, cwd: &str, command: &str) {
        let cmd = Self::extract_command(command);
        if cmd.is_empty() {
            return;
        }
        
        let entry = self.context_commands
            .entry(cwd.to_string())
            .or_insert_with(Vec::new);
        
        // 如果命令不在列表中，添加它
        if !entry.contains(&cmd) {
            entry.push(cmd);
            // 限制每个目录的命令数量
            if entry.len() > 20 {
                entry.remove(0);
            }
        } else {
            // 如果已存在，移到末尾（表示最近使用）
            if let Some(pos) = entry.iter().position(|c| c == &cmd) {
                let cmd = entry.remove(pos);
                entry.push(cmd);
            }
        }
    }

    /// 保存学习到的数据
    pub fn save_data(&self) {
        #[derive(serde::Serialize, serde::Deserialize)]
        struct PredictionData {
            sequences: HashMap<String, Vec<(String, usize)>>,
            context_commands: HashMap<String, Vec<String>>,
        }
        
        let data = PredictionData {
            sequences: self.sequences.clone(),
            context_commands: self.context_commands.clone(),
        };
        
        if let Ok(json) = serde_json::to_string_pretty(&data) {
            let _ = fs::write(&self.data_path, json);
        }
    }

    /// 加载已保存的数据
    fn load_data(&mut self) {
        #[derive(serde::Serialize, serde::Deserialize)]
        struct PredictionData {
            sequences: HashMap<String, Vec<(String, usize)>>,
            context_commands: HashMap<String, Vec<String>>,
        }
        
        if let Ok(content) = fs::read_to_string(&self.data_path) {
            if let Ok(data) = serde_json::from_str::<PredictionData>(&content) {
                self.sequences = data.sequences;
                self.context_commands = data.context_commands;
            }
        }
    }

    /// 获取所有序列统计（用于调试）
    pub fn get_sequences(&self) -> &HashMap<String, Vec<(String, usize)>> {
        &self.sequences
    }
}

impl Default for CommandSequencePredictor {
    fn default() -> Self {
        Self::new()
    }
}

