//! 文件路径补全

use crate::engine::{Completion, CompletionKind};
use std::fs;
use std::path::Path;

/// 需要在 shell 中转义的特殊字符
const SHELL_SPECIAL_CHARS: &[char] = &[
    ' ',  // 空格
    '[', ']',  // 通配符
    '&',  // 后台运行
    '(', ')',  // 子 shell
    ';',  // 命令分隔
    '|',  // 管道
    '*', '?',  // 通配符
    '$',  // 变量
    '!',  // 历史扩展
    '#',  // 注释
    '{', '}',  // 花括号展开
    '<', '>',  // 重定向
    '`',  // 命令替换
    '\'', '"',  // 引号
    '\\',  // 转义符
];

/// 文件补全器
pub struct FileCompleter;

impl FileCompleter {
    pub fn new() -> Self {
        FileCompleter
    }

    /// 转义文件名中的特殊字符
    /// 将空格、[]、& 等 shell 特殊字符前添加反斜杠
    fn escape_shell_chars(s: &str) -> String {
        // 检查是否需要转义
        let needs_escape = s.chars().any(|c| SHELL_SPECIAL_CHARS.contains(&c));
        if !needs_escape {
            return s.to_string();
        }
        
        let mut result = String::with_capacity(s.len() * 2);
        for c in s.chars() {
            if SHELL_SPECIAL_CHARS.contains(&c) {
                result.push('\\');
            }
            result.push(c);
        }
        result
    }

    /// 反转义 shell 字符串
    /// 将 \[ 还原为 [，将 \ 空格还原为空格等
    fn unescape_shell_chars(s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                // 检查下一个字符
                if let Some(&next) = chars.peek() {
                    if SHELL_SPECIAL_CHARS.contains(&next) {
                        // 跳过反斜杠，直接添加下一个字符
                        result.push(chars.next().unwrap());
                        continue;
                    }
                }
            }
            result.push(c);
        }
        result
    }

    /// 获取文件/目录补全
    pub fn complete(&self, prefix: &str) -> Vec<Completion> {
        let mut completions = Vec::new();

        // 先反转义用户输入的前缀（用户可能输入了 cd \[Neko 这样的内容）
        let unescaped_prefix = Self::unescape_shell_chars(prefix);
        let prefix = unescaped_prefix.as_str();

        // 确定要搜索的目录和文件名前缀
        let (dir_path, file_prefix) = if prefix.is_empty() {
            (".", "")
        } else if prefix.ends_with('/') || prefix.ends_with('\\') {
            // 以 / 或 \ 结尾，说明是目录路径
            let dir = prefix.trim_end_matches('/').trim_end_matches('\\');
            (if dir.is_empty() { "." } else { dir }, "")
        } else {
            let path = Path::new(prefix);
            // 检查路径是否存在且是目录
            if path.exists() && path.is_dir() {
                (prefix, "")
            } else {
                // 尝试检查父目录是否存在
                let parent = path.parent().map(|p| p.to_str().unwrap_or(".")).unwrap_or(".");
                let parent = if parent.is_empty() { "." } else { parent };
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                
                // 如果父目录存在，使用父目录和文件名前缀
                if Path::new(parent).exists() {
                    (parent, name)
                } else {
                    // 父目录不存在，尝试将整个路径作为目录
                    (prefix, "")
                }
            }
        };

        // 展开 ~ 为用户主目录
        let dir_path = if dir_path.starts_with('~') {
            if let Some(home) = dirs::home_dir() {
                dir_path.replacen('~', home.to_str().unwrap_or("~"), 1)
            } else {
                dir_path.to_string()
            }
        } else {
            dir_path.to_string()
        };

        // 读取目录内容
        if let Ok(entries) = fs::read_dir(&dir_path) {
            for entry in entries.filter_map(|e| e.ok()) {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();

                // 跳过隐藏文件，除非前缀是 .
                if name_str.starts_with('.') && !file_prefix.starts_with('.') {
                    continue;
                }

                // 匹配前缀
                if !file_prefix.is_empty() && !name_str.starts_with(file_prefix) {
                    continue;
                }

                let is_dir = entry.path().is_dir();
                
                // 转义文件名中的特殊字符
                let escaped_name = Self::escape_shell_chars(&name_str);
                
                let full_path = if prefix.contains('/') || prefix.contains('\\') {
                    let base = if dir_path == "." {
                        String::new()
                    } else if dir_path.ends_with('/') {
                        dir_path.clone()
                    } else {
                        format!("{}/", dir_path)
                    };
                    format!("{}{}", base, escaped_name)
                } else {
                    escaped_name
                };

                let display_name = if is_dir {
                    format!("{}/", full_path)
                } else {
                    full_path.clone()
                };

                completions.push(Completion {
                    text: display_name.clone(),
                    description: if is_dir {
                        "目录".to_string()
                    } else {
                        Self::get_file_type(&entry.path())
                    },
                    score: if is_dir { 80 } else { 70 },
                    kind: if is_dir {
                        CompletionKind::Directory
                    } else {
                        CompletionKind::File
                    },
                    match_indices: Vec::new(),
                });
            }
        }

        completions
    }

    /// 获取文件类型描述
    fn get_file_type(path: &Path) -> String {
        if let Some(ext) = path.extension() {
            match ext.to_str().unwrap_or("").to_lowercase().as_str() {
                "txt" => "文本文件".to_string(),
                "md" => "Markdown 文件".to_string(),
                "rs" => "Rust 源文件".to_string(),
                "py" => "Python 脚本".to_string(),
                "js" => "JavaScript 文件".to_string(),
                "ts" => "TypeScript 文件".to_string(),
                "c" | "h" => "C 源文件".to_string(),
                "cpp" | "hpp" | "cc" => "C++ 源文件".to_string(),
                "go" => "Go 源文件".to_string(),
                "java" => "Java 源文件".to_string(),
                "sh" | "bash" => "Shell 脚本".to_string(),
                "json" => "JSON 文件".to_string(),
                "yaml" | "yml" => "YAML 文件".to_string(),
                "toml" => "TOML 文件".to_string(),
                "xml" => "XML 文件".to_string(),
                "html" | "htm" => "HTML 文件".to_string(),
                "css" => "CSS 文件".to_string(),
                "sql" => "SQL 文件".to_string(),
                "log" => "日志文件".to_string(),
                "tar" | "gz" | "zip" | "7z" | "rar" => "压缩文件".to_string(),
                "pdf" => "PDF 文档".to_string(),
                "doc" | "docx" => "Word 文档".to_string(),
                "xls" | "xlsx" => "Excel 表格".to_string(),
                "png" | "jpg" | "jpeg" | "gif" | "svg" => "图片文件".to_string(),
                "mp3" | "wav" | "flac" => "音频文件".to_string(),
                "mp4" | "mkv" | "avi" => "视频文件".to_string(),
                _ => "文件".to_string(),
            }
        } else {
            "文件".to_string()
        }
    }
}

impl Default for FileCompleter {
    fn default() -> Self {
        Self::new()
    }
}

