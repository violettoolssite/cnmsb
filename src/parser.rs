//! 命令行解析器

use crate::database::CommandDatabase;

/// 解析后的命令行结构
#[derive(Debug, Clone)]
pub struct ParsedCommand {
    /// 命令名称
    pub command: String,
    /// 子命令（如 git commit 中的 commit）
    pub subcommand: Option<String>,
    /// 参数列表
    pub args: Vec<String>,
    /// 当前正在输入的词
    pub current_word: String,
    /// 当前词的索引位置
    pub current_word_index: usize,
    /// 是否正在输入选项（以 - 开头）
    pub is_option: bool,
    /// 前一个词（用于判断选项值）
    pub previous_word: Option<String>,
}

/// 命令行解析器
pub struct CommandParser {
    database: CommandDatabase,
}

impl CommandParser {
    pub fn new() -> Self {
        CommandParser {
            database: CommandDatabase::new(),
        }
    }

    /// 解析命令行
    pub fn parse(&self, line: &str, cursor: usize) -> ParsedCommand {
        // 处理 UTF-8 字符边界
        let line_to_cursor = if cursor >= line.len() {
            line
        } else {
            // 找到有效的 UTF-8 边界
            let mut end = cursor;
            while end < line.len() && !line.is_char_boundary(end) {
                end += 1;
            }
            if end > line.len() {
                line
            } else {
                &line[..end]
            }
        };

        let words: Vec<&str> = line_to_cursor.split_whitespace().collect();
        
        // 判断光标是否在空格后（正在开始新词）
        let ends_with_space = line_to_cursor.ends_with(' ') || line_to_cursor.is_empty();
        
        let (current_word, words_before) = if ends_with_space {
            (String::new(), words.clone())
        } else if let Some((last, rest)) = words.split_last() {
            (last.to_string(), rest.to_vec())
        } else {
            (String::new(), vec![])
        };

        // 识别前缀命令（如 sudo, time, env, nice 等）
        // 这些命令后面跟的是实际要执行的命令
        let prefix_commands = ["sudo", "time", "env", "nice", "nohup", "strace", "gdb", "valgrind"];
        
        // 找到实际命令（跳过前缀命令）
        let (command, command_index) = if let Some(first) = words_before.first() {
            if prefix_commands.contains(first) && words_before.len() > 1 {
                // 如果第一个词是前缀命令，使用第二个词作为实际命令
                (words_before[1].to_string(), 1)
            } else {
                (first.to_string(), 0)
            }
        } else {
            (String::new(), 0)
        };
        
        // 检测子命令（对于 git, docker 等多级命令）
        // 子命令位置 = command_index + 1（在 words_before 中）
        let subcommand = if self.has_subcommands(&command) {
            // 检查 words_before 中是否有子命令（在命令之后）
            if words_before.len() > command_index + 1 {
                let potential_sub = words_before.get(command_index + 1).map(|s| s.to_string());
                // 只有不以 - 开头的才算子命令
                potential_sub.filter(|s| !s.starts_with('-'))
            } else if !ends_with_space && !current_word.is_empty() && !current_word.starts_with('-') {
                // 如果当前词可能是子命令（不在 words_before 中，但在 current_word 中）
                // 这种情况不需要设置 subcommand，因为还在输入中
                None
            } else {
                None
            }
        } else {
            None
        };

        // 参数从实际命令之后开始
        let args: Vec<String> = words_before.iter().skip(command_index + 1).map(|s| s.to_string()).collect();
        
        // 计算当前词相对于实际命令的索引
        let current_word_index = if ends_with_space {
            words.len().saturating_sub(command_index)
        } else {
            words.len().saturating_sub(1).saturating_sub(command_index)
        };

        let is_option = current_word.starts_with('-');
        
        let previous_word = if current_word_index > 0 {
            words_before.last().map(|s| s.to_string())
        } else {
            None
        };

        ParsedCommand {
            command,
            subcommand,
            args,
            current_word,
            current_word_index,
            is_option,
            previous_word,
        }
    }

    /// 检查命令是否有子命令（从数据库动态加载）
    fn has_subcommands(&self, cmd: &str) -> bool {
        // 从数据库检查命令是否定义了子命令
        if let Some(cmd_def) = self.database.get_command(cmd) {
            !cmd_def.subcommands.is_empty()
        } else {
            false
        }
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_command() {
        let parser = CommandParser::new();
        let result = parser.parse("ls -l", 5);
        assert_eq!(result.command, "ls");
        assert_eq!(result.current_word, "-l");
        assert!(result.is_option);
    }

    #[test]
    fn test_parse_git_command() {
        let parser = CommandParser::new();
        let result = parser.parse("git commit -m ", 14);
        assert_eq!(result.command, "git");
        assert_eq!(result.subcommand, Some("commit".to_string()));
        assert_eq!(result.current_word, "");
    }
}

