//! AI 补全器 - 使用 OpenAI 兼容 API 生成补全建议

use super::config::AiConfig;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// AI 补全器
pub struct AiCompleter {
    config: AiConfig,
    client: Client,
}

/// OpenAI 兼容的消息格式
#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

/// OpenAI 兼容的请求格式
#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
    temperature: f32,
    stream: bool,
}

/// OpenAI 兼容的响应格式
#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessageResponse,
}

#[derive(Debug, Deserialize)]
struct ChatMessageResponse {
    content: String,
}

/// AI 补全结果
#[derive(Debug, Clone)]
pub struct AiCompletion {
    /// 补全文本
    pub text: String,
    /// 描述
    pub description: String,
}

impl AiCompleter {
    /// 创建新的 AI 补全器
    pub fn new() -> Self {
        let config = AiConfig::load();
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .build()
            .unwrap_or_else(|_| Client::new());
        
        Self { config, client }
    }

    /// 重新加载配置
    pub fn reload_config(&mut self) {
        self.config = AiConfig::load();
    }

    /// 检查是否可用
    pub fn is_available(&self) -> bool {
        self.config.is_valid()
    }

    /// 获取 AI 补全建议
    pub fn complete(&self, line: &str, cursor: usize) -> Result<Vec<AiCompletion>, String> {
        if !self.config.is_valid() {
            return Err("AI 补全未配置，请运行 'cnmsb ai-config set api_key <your_key>'".to_string());
        }

        // 构建上下文
        let context = self.build_context(line, cursor);
        
        // 构建请求
        let request = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: SYSTEM_PROMPT.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: context,
                },
            ],
            max_tokens: 200,
            temperature: 0.3,
            stream: false,
        };

        // 发送请求
        let url = format!("{}chat/completions", self.config.base_url);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .map_err(|e| format!("请求失败: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(format!("API 错误 ({}): {}", status, body));
        }

        let response: ChatCompletionResponse = response
            .json()
            .map_err(|e| format!("解析响应失败: {}", e))?;

        // 解析 AI 响应
        if let Some(choice) = response.choices.first() {
            let completions = self.parse_ai_response(&choice.message.content, line);
            Ok(completions)
        } else {
            Ok(Vec::new())
        }
    }

    /// 构建发送给 AI 的上下文
    fn build_context(&self, line: &str, cursor: usize) -> String {
        let before_cursor = &line[..cursor.min(line.len())];
        let after_cursor = &line[cursor.min(line.len())..];
        
        // 获取当前工作目录
        let cwd = std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "~".to_string());
        
        format!(
            "当前目录: {}\n\
             命令行输入: {}\n\
             光标位置: {} (光标后: {})\n\
             请提供补全建议。",
            cwd,
            before_cursor,
            cursor,
            if after_cursor.is_empty() { "(行尾)" } else { after_cursor }
        )
    }

    /// 解析 AI 响应，提取补全建议
    fn parse_ai_response(&self, response: &str, original_line: &str) -> Vec<AiCompletion> {
        let mut completions = Vec::new();
        
        // AI 返回的每一行都是一个补全建议
        for line in response.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            // 跳过注释行
            if line.starts_with('#') || line.starts_with("//") {
                continue;
            }
            
            // 提取命令和描述（格式: 命令 # 描述 或 命令 - 描述）
            let (text, description) = if let Some(pos) = line.find(" # ") {
                (line[..pos].trim().to_string(), line[pos + 3..].trim().to_string())
            } else if let Some(pos) = line.find(" - ") {
                (line[..pos].trim().to_string(), line[pos + 3..].trim().to_string())
            } else {
                (line.to_string(), "AI 建议".to_string())
            };
            
            // 跳过空建议或与原输入相同的建议
            if text.is_empty() || text == original_line {
                continue;
            }
            
            completions.push(AiCompletion { text, description });
        }
        
        // 限制数量
        completions.truncate(10);
        completions
    }
}

/// Shell 命令补全专用的 system prompt
const SYSTEM_PROMPT: &str = r#"你是一个 Linux/Unix shell 命令补全助手。根据用户的部分输入，提供最可能的命令补全建议。

规则：
1. 每行输出一个完整的命令建议
2. 格式：命令 # 简短描述
3. 只输出最相关的 3-5 个建议
4. 不要输出解释性文字，只输出命令建议
5. 考虑当前目录上下文
6. 建议应该是用户输入的自然延续

示例输入：
命令行输入: git com

示例输出：
git commit -m "" # 提交更改
git commit --amend # 修改上次提交
git commit -a -m "" # 提交所有更改
"#;

impl Default for AiCompleter {
    fn default() -> Self {
        Self::new()
    }
}

