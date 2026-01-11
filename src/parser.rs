//! 命令行解析器

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
pub struct CommandParser;

impl CommandParser {
    pub fn new() -> Self {
        CommandParser
    }

    /// 解析命令行
    pub fn parse(&self, line: &str, cursor: usize) -> ParsedCommand {
        let line_to_cursor = if cursor <= line.len() {
            &line[..cursor]
        } else {
            line
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

        let command = words_before.first().map(|s| s.to_string()).unwrap_or_default();
        
        // 检测子命令（对于 git, docker 等多级命令）
        let subcommand = if Self::has_subcommands(&command) && words_before.len() > 1 {
            let potential_sub = words_before.get(1).map(|s| s.to_string());
            // 只有不以 - 开头的才算子命令
            potential_sub.filter(|s| !s.starts_with('-'))
        } else {
            None
        };

        let args: Vec<String> = words_before.iter().skip(1).map(|s| s.to_string()).collect();
        
        let current_word_index = if ends_with_space {
            words.len()
        } else {
            words.len().saturating_sub(1)
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

    /// 检查命令是否有子命令
    fn has_subcommands(cmd: &str) -> bool {
        matches!(
            cmd,
            "git" | "docker" | "kubectl" | "apt" | "apt-get" | "systemctl" 
            | "npm" | "cargo" | "pip" | "conda" | "brew" | "pacman" | "yum"
            | "dnf" | "zypper" | "snap" | "flatpak" | "journalctl" | "ip"
        )
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

