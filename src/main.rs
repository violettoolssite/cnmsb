//! cnmsb - 操你妈傻逼
//! Linux 命令行智能补全工具入口

use clap::{Parser, Subcommand};
use cnmsb::{CompletionEngine, CnmsbShell};

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

    /// 初始化 shell 集成
    Init {
        /// Shell 类型 (bash/zsh)
        #[arg(value_name = "SHELL")]
        shell: String,
    },

    /// 启动交互式 shell（带内联建议）
    Shell,

    /// 显示版本信息
    Version,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        // 无子命令时启动交互式 shell
        None => {
            let mut shell = CnmsbShell::new();
            if let Err(e) = shell.run() {
                eprintln!("Shell 错误: {}", e);
                std::process::exit(1);
            }
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

        Some(Commands::Version) => {
            println!("cnmsb (操你妈傻逼) v0.1.0");
            println!("Linux 命令行智能补全工具");
        }
    }
}

