# Contribution Guide

> [查看另一个版本](CONTRIBUTING.md)

---

## Project Structure

```
cnmsb-tool/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library entry
│   ├── engine.rs            # Completion engine (fuzzy matching, ranking)
│   ├── parser.rs            # Command line parser (prefix command support)
│   ├── shell.rs             # Interactive shell (deprecated, interface only)
│   ├── completions/         # Completion implementations
│   │   ├── mod.rs
│   │   ├── commands.rs      # Command completion
│   │   ├── args.rs          # Argument completion (combinable options)
│   │   ├── files.rs         # File path completion
│   │   ├── history.rs       # History completion
│   │   └── context.rs       # Context-aware completion (env vars, path finding)
│   ├── database/            # Command database
│   │   ├── mod.rs           # Database loading
│   │   └── commands/        # Command definition files (YAML format)
│   ├── sql/                 # SQL completion module
│   │   ├── mod.rs
│   │   ├── connection.rs    # Database connection (SQLite/MySQL/PostgreSQL)
│   │   ├── database.rs      # Database types and configuration
│   │   ├── engine.rs        # SQL completion engine
│   │   ├── shell.rs         # SQL interactive shell (using rustyline)
│   │   └── syntax/          # SQL syntax definitions
│   │       ├── mod.rs
│   │       ├── common.rs    # Common SQL syntax
│   │       ├── mysql.rs     # MySQL syntax
│   │       ├── postgresql.rs # PostgreSQL syntax
│   │       └── sqlite.rs    # SQLite syntax
│   └── editor/              # Text editor module (cntmd)
│       ├── mod.rs           # Editor main logic
│       ├── buffer.rs        # Text buffer
│       ├── cursor.rs        # Cursor control
│       ├── mode.rs          # Edit modes (Normal/Insert/Command)
│       ├── render.rs        # Renderer
│       ├── input.rs         # Input handling
│       ├── history.rs       # History management
│       ├── completion.rs    # History-based completion (Trie structure)
│       ├── context.rs       # Context-aware completion (env vars, PATH suggestions)
│       └── nlp.rs           # Natural language understanding and path finding
├── shell/
│   ├── cnmsb.zsh            # Zsh integration (inline completion, selector menu)
│   └── cnmsb.bash           # Bash integration (deprecated, Zsh only)
├── debian/                  # Debian packaging
├── build-deb.sh             # Deb build script
├── install-universal.sh     # Universal installation script
└── Cargo.toml               # Rust project config
```

## Adding New Commands

Command definitions are in `src/database/commands/` directory, organized by category:

```
commands/
├── git.yaml           # Git version control
├── docker.yaml        # Docker containers
├── kubernetes.yaml    # Kubernetes
├── files.yaml         # File operations
├── text.yaml          # Text processing
├── network.yaml       # Network tools
├── system.yaml        # System management
├── package.yaml       # Package managers
├── archive.yaml       # Compression/archiving
├── devtools.yaml      # Development tools
├── cloud.yaml         # Cloud services
├── database.yaml      # Database clients
├── editors.yaml       # Text editors
├── shell.yaml         # Shell utilities
├── hardware.yaml      # Hardware info
├── security.yaml      # Security tools
├── info.yaml          # System info
├── kernel.yaml        # Kernel tools
├── multimedia.yaml    # Multimedia tools
├── virtualization.yaml # Virtualization
├── monitoring.yaml    # Monitoring tools
├── messaging.yaml     # Message queues
├── backup.yaml        # Backup tools
└── cnmsb.yaml         # cnmsb itself
```

### YAML Format

```yaml
command_name:
  name: command_name
  description: Brief description
  combinable_options:            # Optional: predefined option combinations
    - "-abc"
    - "-xyz"
  options:
    - short: "-o"
      long: "--option"
      description: Option description
      takes_value: true
      values: ["val1", "val2"]
  subcommands:
    subcommand_name:
      name: subcommand_name
      description: Subcommand description
      options:
        - short: "-x"
          long: "--example"
          description: Subcommand option
```

### Combinable Options

Many commands support combining short options (e.g., `rm -rf`, `ls -la`, `tar -xzvf`).

To provide better completion, you can add a `combinable_options` field listing common combinations:

```yaml
rm:
  name: rm
  description: Remove files or directories
  combinable_options:
    - "-rf"      # Force recursive delete (most common)
    - "-rv"      # Recursive delete with verbose
    - "-rfv"     # Force recursive with verbose
    - "-ri"      # Recursive with confirmation
    - "-rI"      # Recursive with bulk confirmation
  options:
    - short: "-r"
      long: "--recursive"
      description: Remove directories recursively
    # ...
```

#### Commands with Combinable Options

| Command | Common Combinations | Description |
|---------|---------------------|-------------|
| `rm` | `-rf`, `-rv`, `-rfv` | Force recursive delete |
| `ls` | `-la`, `-lah`, `-ltr`, `-latr` | List with details |
| `cp` | `-rv`, `-rpv`, `-a`, `-av` | Copy with attributes |
| `mv` | `-vf`, `-vi` | Move files |
| `chmod` | `-Rv`, `-Rc` | Recursive permission change |
| `chown` | `-Rv`, `-Rc` | Recursive ownership change |
| `mkdir` | `-p`, `-pv` | Create parent directories |
| `grep` | `-rn`, `-rni`, `-rnw`, `-rE` | Recursive search |
| `ps` | `aux`, `auxf`, `-ef` | Process list |
| `df` | `-h`, `-hT` | Disk usage |
| `du` | `-sh`, `-shc` | Directory size |
| `tar` | `-xvf`, `-xzvf`, `-czvf` | Archive operations |
| `pacman` | `-Syu`, `-Syyu`, `-Rs`, `-Rns` | Arch package management |
| `rsync` | `-av`, `-avz`, `-avzP` | File synchronization |
| `curl` | `-sSL`, `-fsSL`, `-LO` | HTTP requests |
| `scp` | `-rv`, `-rpv`, `-rC` | Secure copy |

#### Guidelines for Adding Combinations

1. **Only add common combinations**: Don't list every possible combination, just the frequently used ones
2. **Keep it concise**: Generally 5-10 combinations is enough
3. **Most common first**: When scores are equal, earlier items are shown first

### Example: Simple Command

```yaml
htop:
  name: htop
  description: Interactive process viewer
  options:
    - short: "-d"
      long: "--delay"
      description: Delay between updates in seconds
      takes_value: true
    - short: "-s"
      long: "--sort-key"
      description: Sort column
      takes_value: true
      values: ["PID", "USER", "CPU", "MEM", "TIME", "COMMAND"]
    - short: "-u"
      long: "--user"
      description: Show only processes of given user
      takes_value: true
    - short: "-t"
      long: "--tree"
      description: Show tree view
    - short: "-h"
      long: "--help"
      description: Show help
  subcommands: {}
```

### Example: Command with Subcommands

```yaml
mycli:
  name: mycli
  description: Example CLI tool
  options:
    - short: "-v"
      long: "--verbose"
      description: Verbose output
    - short: "-c"
      long: "--config"
      description: Config file
      takes_value: true
  subcommands:
    init:
      name: init
      description: Initialize project
      options:
        - short: "-f"
          long: "--force"
          description: Force initialization
        - short: "-t"
          long: "--template"
          description: Template name
          takes_value: true
          values: ["default", "minimal", "full"]
    build:
      name: build
      description: Build project
      options:
        - short: "-r"
          long: "--release"
          description: Release build
        - short: "-j"
          long: "--jobs"
          description: Number of parallel jobs
          takes_value: true
```

### Adding a New Category

1. Create a new `.yaml` file in `commands/`
2. Add `include_str!` in `src/database/mod.rs`:

```rust
let files = [
    // ... existing files
    include_str!("commands/your_new_file.yaml"),
];
```

## Building and Testing

```bash
# Build
cargo build --release

# Test command completion
./target/release/cnmsb complete --line "git " --cursor 4 --shell zsh

# Test subcommand completion
./target/release/cnmsb complete --line "apt ins" --cursor 6 --shell zsh

# Test prefix command completion
./target/release/cnmsb complete --line "sudo ap" --cursor 7 --shell zsh

# Test fuzzy matching
./target/release/cnmsb complete --line "ar" --cursor 2 --shell zsh

# Test help mode
./target/release/cnmsb complete --line "tar ?" --cursor 5 --shell zsh

# Test combinable options
./target/release/cnmsb complete --line "tar -z" --cursor 6 --shell zsh

# Test SQL Shell
./target/release/cnmsb sql

# Test editor
./target/release/cnmsb edit test.txt
# or
./target/release/cntmd test.txt
```

### Build Deb Package

```bash
./build-deb.sh
```

Generates `cnmsb_0.1.0_amd64.deb`

## Pull Requests

1. Fork the repository
2. Create feature branch: `git checkout -b feature/xxx`
3. Commit changes: `git commit -m 'Add xxx'`
4. Push: `git push origin feature/xxx`
5. Open Pull Request

### Commit Message Format

```
feat: add htop command definition
fix: fix tar argument completion
perf: optimize fuzzy matching
docs: update README
```

### PR Checklist

- [ ] YAML format is correct and parseable
- [ ] `cargo build --release` compiles successfully
- [ ] Command options and subcommands are complete
- [ ] Descriptions are clear and accurate

## Issue Reporting

When opening an issue, please include:

1. Operating system (Ubuntu 22.04, Debian 12, etc.)
2. Zsh version
3. Steps to reproduce
4. Expected behavior
5. Actual behavior

## Core Features

### Fuzzy Matching

The completion engine supports multiple matching methods (sorted by priority):

1. **Exact match** (case-insensitive) - Score 300
2. **Prefix match** (case-insensitive) - Score 200+
3. **Contains match** (case-insensitive) - Score 150+
4. **Abbreviation match** (e.g., `ar` -> `tar`) - Score 100+
5. **Fuzzy match** (using `fuzzy-matcher`) - Score 50+
6. **Subsequence match** (e.g., `ar` -> `tar`) - Score 30+

### Prefix Command Support

The system automatically recognizes the following prefix commands and correctly completes the actual command after them:

- `sudo` - Execute with administrator privileges
- `time` - Measure execution time
- `env` - Execute with environment variables
- `nice` - Adjust process priority
- `nohup` - Run in background
- `strace` - System call tracing
- `gdb` - Debugger
- `valgrind` - Memory checker

### Combinable Options

Supports short option combination completion, for example:

- `tar -z` -> Suggests `-zx`, `-zv`, `-zf`, etc.
- `rm -r` -> Suggests `-rf`, `-rv`, `-ri`, etc.
- `ls -l` -> Suggests `-la`, `-lah`, `-ltr`, etc.

### SQL Completion Features

- **Context-aware**: Provides appropriate completions based on SQL context (SELECT, FROM, WHERE, etc.)
- **Schema-aware**: Automatically loads database table and column names
- **Alias resolution**: Supports alias completion in queries like `SELECT u.id FROM users u`
- **Case preservation**: Adjusts completions based on user's case style
- **Table.column format**: Supports `table.column` format completion

### Editor Features

- **History completion**: Trie-based completion using edit history and preloaded common words
- **Context-aware completion**: Automatically analyzes file content, identifies environment variable definitions, provides intelligent suggestions
- **Natural language understanding**: Understands user intent (e.g., `export JAVA_HOME=`), automatically finds system paths
- **PATH intelligent suggestions**: Based on defined `*_HOME` variables, automatically generates PATH configuration suggestions
- **Variable reference completion**: When typing `$VAR`, automatically matches defined variables (case-insensitive)
- **Mode switching**: Three modes (Normal/Insert/Command)
- **Auto file headers**: Automatically adds appropriate file headers based on file extension
- **Welcome screen**: Shows help information for new files
- **History persistence**: Saves edit history for future completions
- **Right arrow acceptance**: Press right arrow (->) to accept context-aware suggestions

### Context-Aware Completion

#### Context Completion in Editor

The editor automatically analyzes file content, extracts environment variable definitions, and provides intelligent completion:

- **Environment variable recognition**: Recognizes `export VAR=value` and `VAR=value` formats
- **Automatic path finding**: When typing `export JAVA_HOME=`, automatically finds Java installation paths in the system
- **PATH intelligent suggestions**: When typing `export PATH=`, generates suggestions based on defined `*_HOME` variables
- **Variable reference completion**: When typing `$VAR`, automatically matches defined variables

#### Context Completion in Command Line

Command line completion also supports context awareness:

- **Environment variable completion**: Provides environment variable name and value completion in `export` commands
- **Path finding**: Automatically finds installation paths for Java, Hadoop, Maven, and other tools
- **PATH suggestions**: Intelligently generates PATH configuration suggestions

#### Path Finding Features

The system automatically searches for installation paths in the following locations:

- **Java**: `/usr/lib/jvm`, `/opt/jdk`, `/opt/java`, etc.
- **Hadoop**: `/opt/hadoop`, `/usr/local/hadoop`, etc.
- **Maven**: `/opt/maven`, `/opt/apache-maven`, etc.
- **Python/Node.js**: Uses `which` command to find paths
- **Generic search**: Searches in `/opt`, `/usr/local`

#### Natural Language Understanding

The system can understand the following intents:

- **Set environment variable**: Recognizes intent of `export VAR=`, finds paths based on variable type
- **Configure PATH**: Recognizes intent of `export PATH=`, generates intelligent suggestions
- **Find paths**: Extracts keywords from text, finds related paths

## Code Style

- Format Rust code with `cargo fmt`
- Check Rust code with `cargo clippy`
- Use 2-space indentation for shell scripts
- Use 2-space indentation for YAML files
- Ensure `cargo build --release` compiles before submitting

## Common Questions

### Q: How to add a new prefix command?

A: Add it to the `prefix_commands` array in `src/parser.rs`, and use `compdef -d` in `shell/cnmsb.zsh` to disable default completion.

### Q: How to add a new SQL database type?

A: Add it to the `DatabaseType` enum in `src/sql/database.rs`, implement the corresponding syntax file in `src/sql/syntax/`, and implement connection logic in `src/sql/connection.rs`.

### Q: Combinable options completion not working?

A: Check if the YAML file defines the `combinable_options` field, and ensure option `short` fields are correctly formatted (e.g., `"-r"` not `"r"`).

### Q: Subcommand completion shows files instead of subcommands?

A: Check the file completion logic in `src/engine.rs` to ensure file completion is skipped when subcommand completions are available.

## License

Contributions are licensed under MIT. By submitting a PR, you agree to this license.

