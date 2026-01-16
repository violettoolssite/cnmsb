//! cnmsb - 操你妈傻逼
//! Linux 命令行智能补全工具入口

use clap::{Parser, Subcommand};
use cnmsb::{CompletionEngine, CnmsbShell, SqlShell, DatabaseType, run_editor, AiConfig, AiCompleter};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cnmsb")]
#[command(author = "cnmsb contributors")]
#[command(version = "0.1.0")]
#[command(about = "操你妈傻逼 - Linux 命令行智能补全工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 获取补全建议
    Complete {
        /// 当前命令行输入
        #[arg(short, long)]
        line: String,

        /// 光标位置
        #[arg(short, long)]
        cursor: usize,

        /// 当前 shell 类型 (bash/zsh)
        #[arg(short, long, default_value = "bash")]
        shell: String,
    },

    /// 显示命令帮助（类似交换机的 ? 功能）
    Help {
        /// 命令（如 git, tar），留空显示所有命令
        #[arg(short, long, default_value = "")]
        command: String,
    },

    /// 初始化 shell 集成
    Init {
        /// Shell 类型 (bash/zsh)
        #[arg(value_name = "SHELL")]
        shell: String,
    },

    /// 启动交互式 shell（带内联建议）
    Shell,

    /// 启动 SQL 交互式客户端（带智能补全）
    Sql {
        /// 数据库类型 (mysql/postgresql/sqlite)
        #[arg(short, long)]
        db_type: Option<String>,
    },

    /// 编辑文件（操你他妈的编辑器，带智能补全）
    Edit {
        /// 要编辑的文件
        file: Option<PathBuf>,
    },

    /// 显示版本信息
    Version,

    /// 记录命令执行（用于 NLP 预测学习）
    Record {
        /// 要记录的命令
        command: String,
    },

    /// AI 智能补全（使用大语言模型）
    #[command(name = "ai-complete")]
    AiComplete {
        /// 当前命令行输入
        #[arg(short, long)]
        line: String,

        /// 光标位置
        #[arg(short, long)]
        cursor: usize,
    },

    /// 管理 AI 补全配置
    #[command(name = "ai-config")]
    AiConfig {
        /// 操作: show, set, get
        #[arg(value_name = "ACTION")]
        action: String,

        /// 配置项名称
        #[arg(value_name = "KEY")]
        key: Option<String>,

        /// 配置项值
        #[arg(value_name = "VALUE")]
        value: Option<String>,
    },
}

fn main() {
    // 检测是否通过别名调用
    let args: Vec<String> = std::env::args().collect();
    let prog_name = std::path::Path::new(&args[0])
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("cnmsb");
    
    // 如果通过 cnmsb-sql 调用，直接进入 SQL 模式
    if prog_name == "cnmsb-sql" || prog_name == "cnmsb-sql.exe" {
        run_sql_mode(None);
        return;
    }
    
    // 如果通过 cntmd 或 操你他妈的 调用，直接进入编辑器模式
    if prog_name == "cntmd" || prog_name == "cntmd.exe" || prog_name == "操你他妈的" {
        let file = args.get(1).map(PathBuf::from);
        run_editor_mode(file);
        return;
    }
    
    let cli = Cli::parse();

    match cli.command {
        // 无子命令时显示工具信息
        None => {
            println!("\x1b[1;33m");
            println!("  ██████╗███╗   ██╗███╗   ███╗███████╗██████╗ ");
            println!(" ██╔════╝████╗  ██║████╗ ████║██╔════╝██╔══██╗");
            println!(" ██║     ██╔██╗ ██║██╔████╔██║███████╗██████╔╝");
            println!(" ██║     ██║╚██╗██║██║╚██╔╝██║╚════██║██╔══██╗");
            println!(" ╚██████╗██║ ╚████║██║ ╚═╝ ██║███████║██████╔╝");
            println!("  ╚═════╝╚═╝  ╚═══╝╚═╝     ╚═╝╚══════╝╚═════╝ ");
            println!("\x1b[0m");
            println!("\x1b[1;38;5;208m  操你妈傻逼 (cnmsb)\x1b[0m v0.1.0");
            println!("  他妈的命令行补全工具，让你敲命令不用再查那些狗屁文档");
            println!();
            println!("  你是不是每次用 tar 都记不住参数？git 那一坨命令谁他妈背得下来？");
            println!("  装上这玩意，敲命令的时候自动给你提示，按 Tab 就补全了。");
            println!("  就这么简单，傻逼都会用。");
            println!();
            println!("\x1b[1;32m怎么用:\x1b[0m");
            println!("  装好就完事了，不用配置什么狗屁东西。在 zsh 里直接敲命令就行。");
            println!();
            println!("\x1b[1;32m快捷键:\x1b[0m");
            println!("  \x1b[33mTab\x1b[0m        第一下弹选择器，第二下确认");
            println!("  \x1b[33m↑ ↓\x1b[0m        选哪个你自己挑");
            println!("  \x1b[33m→\x1b[0m          懒得选？直接按右箭头接受");
            println!("  \x1b[33m?\x1b[0m          不知道有啥？按问号看所有选项");
            println!("  \x1b[33mEsc\x1b[0m        不要了，滚");
            println!();
            println!("\x1b[1;32m举个鸡巴例子:\x1b[0m");
            println!("  tar -z?    看看 tar 能加什么参数");
            println!("  git com    按 Tab 补全成 git commit");
            println!("  docker r   按 Tab 补全成 docker run");
            println!();
            println!("\x1b[1;32m其他命令:\x1b[0m");
            println!("  cnmsb help -c git    看 git 有什么命令");
            println!("  cnmsb version        版本号，没啥用");
            println!();
            println!("有问题？复刻自行加入功能: \x1b[36mhttps://github.com/violettoolssite/cnmsb\x1b[0m");
            println!();
        }

        Some(Commands::Complete {
            line,
            cursor,
            shell,
        }) => {
            let engine = CompletionEngine::new();
            let completions = engine.complete(&line, cursor);
            
            let use_color = shell == "zsh" || shell == "color";
            let reset = "\x1b[0m";
            let desc_color = "\x1b[38;5;240m";  // 灰色描述

            // 输出补全结果，每行一个
            for completion in completions {
                if use_color {
                    // 带颜色输出：彩色文本 + 灰色描述
                    println!("{}{}{}\t{}{}{}",
                        completion.kind.color(),
                        completion.text,
                        reset,
                        desc_color,
                        completion.description,
                        reset
                    );
                } else {
                    println!("{}\t{}", completion.text, completion.description);
                }
            }
        }

        Some(Commands::Help { command }) => {
            let engine = CompletionEngine::new();
            
            if command.is_empty() {
                // 显示所有可用命令
                println!("\x1b[1;33m可用命令:\x1b[0m");
                println!();
                let completions = engine.complete("", 0);
                for c in completions.iter().filter(|c| c.kind == cnmsb::CompletionKind::Command) {
                    println!("  \x1b[32m{:<20}\x1b[0m {}", c.text, c.description);
                }
            } else {
                // 显示指定命令的帮助
                println!("\x1b[1;33m{} 可用选项:\x1b[0m", command);
                println!();
                
                // 获取子命令
                let line = format!("{} ", command);
                let completions = engine.complete(&line, line.len());
                
                let subcommands: Vec<_> = completions.iter()
                    .filter(|c| c.kind == cnmsb::CompletionKind::Subcommand)
                    .collect();
                    
                let options: Vec<_> = completions.iter()
                    .filter(|c| c.kind == cnmsb::CompletionKind::Option)
                    .collect();
                
                if !subcommands.is_empty() {
                    println!("\x1b[36m子命令:\x1b[0m");
                    for c in &subcommands {
                        println!("  \x1b[36m{:<20}\x1b[0m {}", c.text, c.description);
                    }
                    println!();
                }
                
                if !options.is_empty() {
                    println!("\x1b[33m选项:\x1b[0m");
                    for c in &options {
                        println!("  \x1b[33m{:<20}\x1b[0m {}", c.text, c.description);
                    }
                }
                
                if subcommands.is_empty() && options.is_empty() {
                    println!("  (没有找到 {} 的帮助信息)", command);
                }
            }
        }

        Some(Commands::Init { shell }) => {
            match shell.as_str() {
                "bash" => {
                    print!("{}", include_str!("../shell/cnmsb.bash"));
                }
                "zsh" => {
                    print!("{}", include_str!("../shell/cnmsb.zsh"));
                }
                _ => {
                    eprintln!("不支持的 shell: {}. 支持: bash, zsh", shell);
                    std::process::exit(1);
                }
            }
        }

        Some(Commands::Shell) => {
            let mut shell = CnmsbShell::new();
            if let Err(e) = shell.run() {
                eprintln!("Shell 错误: {}", e);
                std::process::exit(1);
            }
        }

        Some(Commands::Sql { db_type }) => {
            run_sql_mode(db_type);
        }

        Some(Commands::Version) => {
            println!("cnmsb (操你妈傻逼) v0.1.0");
            println!("Linux 命令行智能补全工具");
        }

        Some(Commands::Record { command }) => {
            let mut engine = CompletionEngine::new();
            engine.record_command(&command);
        }

        Some(Commands::Edit { file }) => {
            run_editor_mode(file);
        }

        Some(Commands::AiComplete { line, cursor }) => {
            run_ai_complete(&line, cursor);
        }

        Some(Commands::AiConfig { action, key, value }) => {
            run_ai_config(&action, key.as_deref(), value.as_deref());
        }
    }
}

/// 运行 AI 补全
fn run_ai_complete(line: &str, cursor: usize) {
    let completer = AiCompleter::new();
    
    if !completer.is_available() {
        eprintln!("\x1b[33mAI 补全未配置。请运行:\x1b[0m");
        eprintln!("  cnmsb ai-config set api_key <your_api_key>");
        std::process::exit(1);
    }
    
    match completer.complete(line, cursor) {
        Ok(completions) => {
            for c in completions {
                println!("{}\t{}", c.text, c.description);
            }
        }
        Err(e) => {
            // 不使用颜色代码，因为输出会被 zsh POSTDISPLAY 显示
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

/// 管理 AI 配置
fn run_ai_config(action: &str, key: Option<&str>, value: Option<&str>) {
    let mut config = AiConfig::load();
    
    match action {
        "show" => {
            println!("{}", config.show());
        }
        "get" => {
            if let Some(key) = key {
                if let Some(val) = config.get(key) {
                    println!("{}", val);
                } else {
                    eprintln!("\x1b[31m未知配置项: {}\x1b[0m", key);
                    std::process::exit(1);
                }
            } else {
                eprintln!("\x1b[31m用法: cnmsb ai-config get <key>\x1b[0m");
                std::process::exit(1);
            }
        }
        "set" => {
            if let (Some(key), Some(value)) = (key, value) {
                if let Err(e) = config.set(key, value) {
                    eprintln!("\x1b[31m设置失败: {}\x1b[0m", e);
                    std::process::exit(1);
                }
                if let Err(e) = config.save() {
                    eprintln!("\x1b[31m保存失败: {}\x1b[0m", e);
                    std::process::exit(1);
                }
                println!("\x1b[32m已设置 {} = {}\x1b[0m", key, value);
            } else {
                eprintln!("\x1b[31m用法: cnmsb ai-config set <key> <value>\x1b[0m");
                eprintln!("可用配置项: enabled, api_key, base_url, model, timeout");
                std::process::exit(1);
            }
        }
        "init" => {
            // 使用默认配置初始化
            let default_config = AiConfig::default();
            if let Err(e) = default_config.save() {
                eprintln!("\x1b[31m初始化失败: {}\x1b[0m", e);
                std::process::exit(1);
            }
            println!("\x1b[32m已创建默认配置文件\x1b[0m");
            if let Some(path) = AiConfig::config_path() {
                println!("配置文件: {}", path.display());
            }
            println!("\n请设置 API 密钥:");
            println!("  cnmsb ai-config set api_key <your_api_key>");
        }
        _ => {
            eprintln!("\x1b[31m未知操作: {}\x1b[0m", action);
            eprintln!("可用操作: show, get, set, init");
            std::process::exit(1);
        }
    }
}

/// 运行编辑器模式
fn run_editor_mode(file: Option<PathBuf>) {
    if let Err(e) = run_editor(file) {
        eprintln!("编辑器错误: {}", e);
        std::process::exit(1);
    }
}

/// 运行 SQL 模式
fn run_sql_mode(db_type: Option<String>) {
    use std::io::{self, Write};
    
    let db = match db_type {
        Some(t) => {
            match DatabaseType::from_str(&t) {
                Some(db) => db,
                None => {
                    eprintln!("\x1b[31m不认识的数据库类型: {}\x1b[0m", t);
                    eprintln!("支持的类型: mysql, postgresql, sqlite, mariadb, oracle, sqlserver");
                    std::process::exit(1);
                }
            }
        }
        None => {
            println!("\x1b[1;33m");
            println!("  ███████╗ ██████╗ ██╗         ███╗   ███╗ ██████╗ ██████╗ ███████╗");
            println!("  ██╔════╝██╔═══██╗██║         ████╗ ████║██╔═══██╗██╔══██╗██╔════╝");
            println!("  ███████╗██║   ██║██║         ██╔████╔██║██║   ██║██║  ██║█████╗  ");
            println!("  ╚════██║██║▄▄ ██║██║         ██║╚██╔╝██║██║   ██║██║  ██║██╔══╝  ");
            println!("  ███████║╚██████╔╝███████╗    ██║ ╚═╝ ██║╚██████╔╝██████╔╝███████╗");
            println!("  ╚══════╝ ╚══▀▀═╝ ╚══════╝    ╚═╝     ╚═╝ ╚═════╝ ╚═════╝ ╚══════╝");
            println!("\x1b[0m");
            println!("  \x1b[1;38;5;208mcnmsb-sql\x1b[0m - SQL 智能补全客户端");
            println!();
            println!("  选择你要连接的数据库类型：");
            println!();
            println!("    \x1b[1;36m1.\x1b[0m MySQL");
            println!("    \x1b[1;36m2.\x1b[0m PostgreSQL");
            println!("    \x1b[1;36m3.\x1b[0m SQLite");
            println!();
            print!("  输入选择 (1-3): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            match input.trim() {
                "1" | "mysql" => DatabaseType::MySQL,
                "2" | "postgresql" | "postgres" | "psql" => DatabaseType::PostgreSQL,
                "3" | "sqlite" | "sqlite3" => DatabaseType::SQLite,
                _ => {
                    eprintln!("\x1b[31m选个鸡巴！输入 1、2 或 3\x1b[0m");
                    std::process::exit(1);
                }
            }
        }
    };
    
    println!();
    println!("  \x1b[32m正在启动 {:?} SQL Shell...\x1b[0m", db);
    println!();
    
    let mut sql_shell = SqlShell::new(db);
    if let Err(e) = sql_shell.run() {
        eprintln!("SQL Shell 错误: {}", e);
        std::process::exit(1);
    }
}

