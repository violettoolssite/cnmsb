# Contribution Guide

> [查看另一个版本](CONTRIBUTING.md)

---

## Project Structure

```
cnmsb-tool/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library entry
│   ├── engine.rs            # Completion engine
│   ├── parser.rs            # Command line parser
│   ├── shell.rs             # Interactive shell
│   ├── completions/         # Completion implementations
│   │   ├── mod.rs
│   │   ├── commands.rs      # Command completion
│   │   ├── args.rs          # Argument completion
│   │   ├── files.rs         # File path completion
│   │   └── history.rs       # History completion
│   ├── database/            # Command database
│   │   ├── mod.rs           # Database loading
│   │   └── commands/        # Command definition files
│   └── sql/                 # SQL completion module
│       ├── mod.rs
│       ├── connection.rs    # Database connection
│       ├── engine.rs        # SQL completion engine
│       ├── shell.rs         # SQL interactive shell
│       └── syntax/          # SQL syntax definitions
├── shell/
│   ├── cnmsb.zsh            # Zsh integration
│   └── cnmsb.bash           # Bash integration
├── debian/                  # Debian packaging
├── build-deb.sh             # Deb build script
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

# Test completion output
./target/release/cnmsb complete --line "git " --cursor 4 --shell zsh

# Test help mode
./target/release/cnmsb complete --line "tar ?" --cursor 5 --shell zsh

# Test interactive mode
./target/release/cnmsb shell

# Test SQL mode
./target/release/cnmsb sql
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

## Code Style

- Format Rust code with `cargo fmt`
- Check Rust code with `cargo clippy`
- Use 2-space indentation for shell scripts
- Use 2-space indentation for YAML files

## License

Contributions are licensed under MIT. By submitting a PR, you agree to this license.

