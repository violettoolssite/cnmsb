//! AI 补全配置管理

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

/// AI 配置
#[derive(Debug, Clone)]
pub struct AiConfig {
    /// 是否启用 AI 补全
    pub enabled: bool,
    /// API 密钥
    pub api_key: String,
    /// API 基础 URL
    pub base_url: String,
    /// 模型名称
    pub model: String,
    /// 超时时间（秒）
    pub timeout: u64,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_key: String::new(),
            base_url: "https://api-inference.modelscope.cn/v1/".to_string(),
            model: "Qwen/Qwen2.5-Coder-32B-Instruct".to_string(),
            timeout: 30,
        }
    }
}

impl AiConfig {
    /// 获取配置文件路径
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("cnmsb").join("ai.conf"))
    }

    /// 从配置文件加载
    pub fn load() -> Self {
        let mut config = Self::default();
        
        if let Some(path) = Self::config_path() {
            if path.exists() {
                if let Ok(file) = fs::File::open(&path) {
                    let reader = BufReader::new(file);
                    for line in reader.lines().flatten() {
                        let line = line.trim();
                        if line.is_empty() || line.starts_with('#') {
                            continue;
                        }
                        if let Some((key, value)) = line.split_once('=') {
                            let key = key.trim();
                            let value = value.trim();
                            match key {
                                "enabled" => config.enabled = value == "true",
                                "api_key" => config.api_key = value.to_string(),
                                "base_url" => config.base_url = value.to_string(),
                                "model" => config.model = value.to_string(),
                                "timeout" => {
                                    if let Ok(t) = value.parse() {
                                        config.timeout = t;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        
        config
    }

    /// 保存配置到文件
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path().ok_or("无法获取配置目录")?;
        
        // 确保目录存在
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
        }
        
        let mut file = fs::File::create(&path).map_err(|e| format!("创建文件失败: {}", e))?;
        
        writeln!(file, "# CNMSB AI 补全配置").map_err(|e| e.to_string())?;
        writeln!(file, "# 使用 Alt+F4 触发 AI 补全").map_err(|e| e.to_string())?;
        writeln!(file).map_err(|e| e.to_string())?;
        writeln!(file, "enabled={}", self.enabled).map_err(|e| e.to_string())?;
        writeln!(file, "api_key={}", self.api_key).map_err(|e| e.to_string())?;
        writeln!(file, "base_url={}", self.base_url).map_err(|e| e.to_string())?;
        writeln!(file, "model={}", self.model).map_err(|e| e.to_string())?;
        writeln!(file, "timeout={}", self.timeout).map_err(|e| e.to_string())?;
        
        Ok(())
    }

    /// 设置配置项
    pub fn set(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key {
            "enabled" => self.enabled = value == "true",
            "api_key" => self.api_key = value.to_string(),
            "base_url" => self.base_url = value.to_string(),
            "model" => self.model = value.to_string(),
            "timeout" => {
                self.timeout = value.parse().map_err(|_| "timeout 必须是数字")?;
            }
            _ => return Err(format!("未知配置项: {}", key)),
        }
        Ok(())
    }

    /// 获取配置项
    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "enabled" => Some(self.enabled.to_string()),
            "api_key" => Some(self.api_key.clone()),
            "base_url" => Some(self.base_url.clone()),
            "model" => Some(self.model.clone()),
            "timeout" => Some(self.timeout.to_string()),
            _ => None,
        }
    }

    /// 显示所有配置
    pub fn show(&self) -> String {
        format!(
            "AI 补全配置:\n\
             enabled  = {}\n\
             api_key  = {}\n\
             base_url = {}\n\
             model    = {}\n\
             timeout  = {}s",
            self.enabled,
            if self.api_key.is_empty() { "(未设置)" } else { &self.api_key[..8.min(self.api_key.len())] },
            self.base_url,
            self.model,
            self.timeout
        )
    }

    /// 检查配置是否有效
    pub fn is_valid(&self) -> bool {
        self.enabled && !self.api_key.is_empty() && !self.base_url.is_empty()
    }

    /// 创建默认配置文件（如果不存在）
    pub fn ensure_config_exists() -> Result<(), String> {
        if let Some(path) = Self::config_path() {
            if !path.exists() {
                let config = Self::default();
                config.save()?;
            }
        }
        Ok(())
    }
}

