# Claude Code Complete Setup (Rust Engine)

A high-performance, single-binary Rust port of the deployment, configuration, MCP management, security auditing, and memory search toolchain for **Claude Code Complete Setup**.

## 🚀 Key Advantages of the Rust Engine

1. **Native Performance & Zero Dependency Overhead**: Runs without requiring pre-installed Bash scripts, `rsync`, or shell-specific utilities.
2. **Dynamic Path Normalization**: Automatically resolves and rewrites hardcoded Linux/user paths (`/home/jb_remus`) to the local target home directory (`C:/Users/...` or custom targets).
3. **SQLite-Backed Fast Memory Indexing**: Indexes global memory Markdown files into an embedded SQLite database (`memory_index.db`), enabling fast offline keyword search.
4. **Built-in Security & Audit Suite**: Programmatically installs pre-commit branch protection hooks and validates environment secret safety.

---

## 🛠️ CLI Usage & Commands

Build the release binary:
```bash
cargo build --release
```

The resulting executable is located at `./target/release/claude-code-setup.exe`.

### 1. Perform Full Deployment
```bash
# Standard automated setup
./target/release/claude-code-setup.exe install

# Deploy to a custom home directory or test target
./target/release/claude-code-setup.exe install --home-dir "C:/CustomHome"

# Fast install skipping prerequisite binary checks
./target/release/claude-code-setup.exe install --skip-prereqs
```

### 2. Run Deployment Verification Diagnostics
```bash
./target/release/claude-code-setup.exe test
```

### 3. Manage & Inspect MCP Servers
```bash
./target/release/claude-code-setup.exe mcp-list
```

### 4. Global Memory Indexing & Fast Search
```bash
# Index knowledge files into SQLite
./target/release/claude-code-setup.exe memory-index

# Search indexed knowledge base
./target/release/claude-code-setup.exe memory-search "rules"
```

### 5. Security Audit & Pre-Commit Hook Installation
```bash
# Run security audit
./target/release/claude-code-setup.exe security-audit

# Install Git branch protection pre-commit hook
./target/release/claude-code-setup.exe install-hooks --repo-dir "."
```

---

## 📂 Project Architecture

```
claude-code-complete-setup/
├── Cargo.toml                  # Rust project manifest
├── src/
│   ├── main.rs                 # CLI entrypoint
│   ├── cli.rs                  # Clap subcommand definitions
│   ├── installer.rs            # Core deployment & backup logic
│   ├── updater.rs              # Configuration update engine
│   ├── mcp.rs                  # MCP JSON parser & path normalizer
│   ├── memory_engine.rs        # SQLite memory index & search
│   ├── security.rs             # Security auditor & Git hook generator
│   └── tester.rs               # Diagnostic test suite
├── config/                     # SuperClaude & MCP JSON configs
├── global_memory/              # Global knowledge Markdown files
├── setup.sh / update.sh        # Legacy Bash scripts (maintained for compatibility)
└── README.md                   # Project documentation
```

---

## 🛡️ License & Authors

- **Authors**: Ercan ER (`ercaner1988@gmail.com`), Kassam (`agent@hermes.local`)
- **License**: MIT
