//! 命令参数数据库
//! 
//! 命令定义文件按类别分类存储在 commands/ 目录中：
//! - git.yaml        : Git 版本控制
//! - docker.yaml     : Docker 容器
//! - kubernetes.yaml : Kubernetes
//! - files.yaml      : 文件操作
//! - text.yaml       : 文本处理
//! - network.yaml    : 网络工具
//! - system.yaml     : 系统管理
//! - package.yaml    : 包管理
//! - archive.yaml    : 压缩归档
//! 
//! 贡献者可以添加新的 YAML 文件或修改现有文件来扩展命令支持。

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 命令定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandDef {
    /// 命令名称
    pub name: String,
    /// 命令描述
    pub description: String,
    /// 选项列表
    #[serde(default)]
    pub options: Vec<OptionDef>,
    /// 子命令
    #[serde(default)]
    pub subcommands: HashMap<String, CommandDef>,
}

/// 选项定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionDef {
    /// 短选项 (如 -v)
    #[serde(default)]
    pub short: String,
    /// 长选项 (如 --verbose)
    #[serde(default)]
    pub long: String,
    /// 描述
    pub description: String,
    /// 是否需要参数
    #[serde(default)]
    pub takes_value: bool,
    /// 预定义的可选值
    #[serde(default)]
    pub values: Option<Vec<String>>,
}

/// 加载并合并所有命令定义文件
fn load_all_commands() -> HashMap<String, CommandDef> {
    let mut commands = HashMap::new();
    
    // 加载各个分类文件
    let files = [
        include_str!("commands/git.yaml"),
        include_str!("commands/docker.yaml"),
        include_str!("commands/kubernetes.yaml"),
        include_str!("commands/files.yaml"),
        include_str!("commands/text.yaml"),
        include_str!("commands/network.yaml"),
        include_str!("commands/system.yaml"),
        include_str!("commands/package.yaml"),
        include_str!("commands/archive.yaml"),
    ];
    
    for yaml_content in files {
        if let Ok(parsed) = serde_yaml::from_str::<HashMap<String, CommandDef>>(yaml_content) {
            commands.extend(parsed);
        }
    }
    
    commands
}

/// 内置命令数据库
static COMMAND_DATABASE: Lazy<HashMap<String, CommandDef>> = Lazy::new(load_all_commands);

/// 命令数据库
pub struct CommandDatabase;

impl CommandDatabase {
    pub fn new() -> Self {
        CommandDatabase
    }

    /// 获取命令定义
    pub fn get_command(&self, name: &str) -> Option<&CommandDef> {
        COMMAND_DATABASE.get(name)
    }

    /// 获取子命令定义
    pub fn get_subcommand(&self, cmd: &str, subcmd: &str) -> Option<&CommandDef> {
        COMMAND_DATABASE
            .get(cmd)
            .and_then(|c| c.subcommands.get(subcmd))
    }

    /// 获取命令的所有子命令
    pub fn get_subcommands(&self, cmd: &str) -> Option<Vec<(String, String)>> {
        COMMAND_DATABASE.get(cmd).map(|c| {
            c.subcommands
                .iter()
                .map(|(name, def)| (name.clone(), def.description.clone()))
                .collect()
        })
    }
    
    /// 获取所有命令名称
    pub fn all_commands(&self) -> Vec<&str> {
        COMMAND_DATABASE.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for CommandDatabase {
    fn default() -> Self {
        Self::new()
    }
}
