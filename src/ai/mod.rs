//! AI 补全模块
//!
//! 提供基于大语言模型的智能命令补全功能。
//! 使用 OpenAI 兼容的 API（默认使用 ModelScope Qwen2.5-Coder）。
//!
//! # 使用方式
//!
//! 1. 配置 API 密钥：`cnmsb ai-config set api_key <your_key>`
//! 2. 按 Alt+F4 触发 AI 补全
//!
//! # 配置文件
//!
//! 配置存储在 `~/.config/cnmsb/ai.conf`

pub mod config;
pub mod completer;

pub use config::AiConfig;
pub use completer::{AiCompleter, AiCompletion};

