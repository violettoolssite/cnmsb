//! 命令名补全

use crate::database::CommandDatabase;
use crate::engine::{Completion, CompletionKind};
use std::collections::HashMap;

/// 命令补全器
pub struct CommandCompleter {
    /// 常用命令及其描述
    commands: HashMap<&'static str, &'static str>,
    /// 数据库命令
    database: CommandDatabase,
}

impl CommandCompleter {
    pub fn new() -> Self {
        let mut commands = HashMap::new();
        
        // 文件操作
        commands.insert("ls", "列出目录内容");
        commands.insert("cd", "切换目录");
        commands.insert("pwd", "显示当前目录");
        commands.insert("cp", "复制文件或目录");
        commands.insert("mv", "移动或重命名文件");
        commands.insert("rm", "删除文件或目录");
        commands.insert("mkdir", "创建目录");
        commands.insert("rmdir", "删除空目录");
        commands.insert("touch", "创建空文件或更新时间戳");
        commands.insert("ln", "创建链接");
        commands.insert("chmod", "修改文件权限");
        commands.insert("chown", "修改文件所有者");
        commands.insert("find", "查找文件");
        commands.insert("locate", "快速定位文件");
        commands.insert("tree", "树形显示目录结构");

        // 文本处理
        commands.insert("cat", "显示文件内容");
        commands.insert("less", "分页查看文件");
        commands.insert("more", "分页查看文件");
        commands.insert("head", "显示文件开头");
        commands.insert("tail", "显示文件结尾");
        commands.insert("grep", "搜索文本模式");
        commands.insert("sed", "流编辑器");
        commands.insert("awk", "文本处理工具");
        commands.insert("sort", "排序文本行");
        commands.insert("uniq", "去除重复行");
        commands.insert("wc", "统计行数、单词数、字符数");
        commands.insert("cut", "按列切分文本");
        commands.insert("tr", "字符转换");
        commands.insert("diff", "比较文件差异");
        commands.insert("patch", "应用补丁");

        // 编辑器
        commands.insert("vim", "Vi 改进版编辑器");
        commands.insert("vi", "Vi 编辑器");
        commands.insert("nano", "简单文本编辑器");
        commands.insert("emacs", "Emacs 编辑器");
        commands.insert("code", "VS Code 编辑器");

        // 系统管理
        commands.insert("sudo", "以超级用户身份执行");
        commands.insert("su", "切换用户");
        commands.insert("ps", "显示进程状态");
        commands.insert("top", "动态显示进程");
        commands.insert("htop", "交互式进程查看器");
        commands.insert("kill", "终止进程");
        commands.insert("killall", "按名称终止进程");
        commands.insert("pkill", "按模式终止进程");
        commands.insert("systemctl", "系统服务管理");
        commands.insert("service", "服务管理");
        commands.insert("journalctl", "查看系统日志");
        commands.insert("dmesg", "显示内核消息");
        commands.insert("uptime", "系统运行时间");
        commands.insert("free", "显示内存使用");
        commands.insert("df", "显示磁盘使用");
        commands.insert("du", "显示目录大小");
        commands.insert("mount", "挂载文件系统");
        commands.insert("umount", "卸载文件系统");

        // 用户管理
        commands.insert("useradd", "添加用户");
        commands.insert("userdel", "删除用户");
        commands.insert("usermod", "修改用户");
        commands.insert("passwd", "修改密码");
        commands.insert("groups", "显示用户组");
        commands.insert("id", "显示用户ID");
        commands.insert("who", "显示登录用户");
        commands.insert("whoami", "显示当前用户");

        // 网络
        commands.insert("ping", "测试网络连通性");
        commands.insert("curl", "HTTP 客户端");
        commands.insert("wget", "下载文件");
        commands.insert("ssh", "安全远程登录");
        commands.insert("scp", "安全复制文件");
        commands.insert("rsync", "远程同步");
        commands.insert("netstat", "网络统计");
        commands.insert("ss", "套接字统计");
        commands.insert("ip", "网络配置");
        commands.insert("ifconfig", "网络接口配置");
        commands.insert("nslookup", "DNS 查询");
        commands.insert("dig", "DNS 查询工具");
        commands.insert("host", "DNS 查询");
        commands.insert("traceroute", "路由追踪");
        commands.insert("nc", "网络瑞士军刀");
        commands.insert("nmap", "网络扫描");

        // 压缩/解压
        commands.insert("tar", "打包/解包工具");
        commands.insert("gzip", "压缩工具");
        commands.insert("gunzip", "解压工具");
        commands.insert("zip", "ZIP 压缩");
        commands.insert("unzip", "ZIP 解压");
        commands.insert("xz", "XZ 压缩");
        commands.insert("7z", "7-Zip 压缩");

        // 包管理
        commands.insert("apt", "Debian/Ubuntu 包管理");
        commands.insert("apt-get", "APT 包管理");
        commands.insert("apt-cache", "APT 缓存查询");
        commands.insert("dpkg", "Debian 包管理");
        commands.insert("yum", "RHEL/CentOS 包管理");
        commands.insert("dnf", "Fedora 包管理");
        commands.insert("pacman", "Arch Linux 包管理");
        commands.insert("snap", "Snap 包管理");
        commands.insert("flatpak", "Flatpak 包管理");

        // 开发工具
        commands.insert("git", "版本控制系统");
        commands.insert("docker", "容器管理");
        commands.insert("docker-compose", "Docker Compose");
        commands.insert("kubectl", "Kubernetes 命令行");
        commands.insert("make", "构建工具");
        commands.insert("cmake", "CMake 构建工具");
        commands.insert("gcc", "GNU C 编译器");
        commands.insert("g++", "GNU C++ 编译器");
        commands.insert("python", "Python 解释器");
        commands.insert("python3", "Python 3 解释器");
        commands.insert("pip", "Python 包管理");
        commands.insert("pip3", "Python 3 包管理");
        commands.insert("node", "Node.js 运行时");
        commands.insert("npm", "Node.js 包管理");
        commands.insert("npx", "Node.js 包执行器");
        commands.insert("yarn", "Yarn 包管理");
        commands.insert("cargo", "Rust 包管理");
        commands.insert("rustc", "Rust 编译器");
        commands.insert("go", "Go 语言工具");
        commands.insert("java", "Java 虚拟机");
        commands.insert("javac", "Java 编译器");
        commands.insert("mvn", "Maven 构建工具");
        commands.insert("gradle", "Gradle 构建工具");

        // 其他常用
        commands.insert("echo", "输出文本");
        commands.insert("printf", "格式化输出");
        commands.insert("date", "显示/设置日期时间");
        commands.insert("cal", "显示日历");
        commands.insert("bc", "计算器");
        commands.insert("man", "查看手册");
        commands.insert("info", "查看信息文档");
        commands.insert("help", "显示帮助");
        commands.insert("history", "命令历史");
        commands.insert("alias", "定义别名");
        commands.insert("which", "定位命令");
        commands.insert("whereis", "定位命令位置");
        commands.insert("type", "显示命令类型");
        commands.insert("xargs", "构建并执行命令");
        commands.insert("env", "显示/设置环境变量");
        commands.insert("export", "导出环境变量");
        commands.insert("source", "执行脚本");
        commands.insert("clear", "清屏");
        commands.insert("reset", "重置终端");
        commands.insert("exit", "退出 shell");
        commands.insert("logout", "登出");
        commands.insert("reboot", "重启系统");
        commands.insert("shutdown", "关机");
        commands.insert("poweroff", "关机");

        CommandCompleter { 
            commands,
            database: CommandDatabase::new(),
        }
    }

    /// 获取命令补全
    pub fn complete(&self, prefix: &str) -> Vec<Completion> {
        let prefix_lower = prefix.to_lowercase();
        
        let mut completions: Vec<Completion> = self.commands
            .iter()
            .filter(|(cmd, _)| {
                if prefix.is_empty() {
                    return true;
                }
                let cmd_lower = cmd.to_lowercase();
                // 宽松匹配：前缀、包含、或者字符都在命令中（顺序匹配）
                cmd_lower.starts_with(&prefix_lower) 
                    || cmd_lower.contains(&prefix_lower)
                    || Self::chars_match(&cmd_lower, &prefix_lower)
            })
            .map(|(cmd, desc)| Completion {
                text: cmd.to_string(),
                description: desc.to_string(),
                score: 50, // 让引擎来重新评分
                kind: CompletionKind::Command,
                match_indices: Vec::new(),
            })
            .collect();
        
        // 同时从数据库加载命令
        for cmd_name in self.database.all_commands() {
            // 避免重复
            if self.commands.contains_key(cmd_name) {
                continue;
            }
            
            let cmd_lower = cmd_name.to_lowercase();
            let should_include = prefix.is_empty() 
                || cmd_lower.starts_with(&prefix_lower)
                || cmd_lower.contains(&prefix_lower)
                || Self::chars_match(&cmd_lower, &prefix_lower);
            
            if should_include {
                if let Some(cmd_def) = self.database.get_command(cmd_name) {
                    completions.push(Completion {
                        text: cmd_name.to_string(),
                        description: cmd_def.description.clone(),
                        score: 50,
                        kind: CompletionKind::Command,
                        match_indices: Vec::new(),
                    });
                }
            }
        }
        
        completions
    }
    
    /// 检查 pattern 中的所有字符是否按顺序出现在 text 中
    fn chars_match(text: &str, pattern: &str) -> bool {
        let mut pattern_chars = pattern.chars().peekable();
        for ch in text.chars() {
            if pattern_chars.peek() == Some(&ch) {
                pattern_chars.next();
            }
            if pattern_chars.peek().is_none() {
                return true;
            }
        }
        pattern_chars.peek().is_none()
    }

    /// 检查命令是否存在
    pub fn exists(&self, cmd: &str) -> bool {
        self.commands.contains_key(cmd)
    }
}

impl Default for CommandCompleter {
    fn default() -> Self {
        Self::new()
    }
}

