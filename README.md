# Claude Code Complete Setup (100% Rust Engine)

A high-performance, single-binary **100% Rust-native** deployment and management system for **Claude Code**.

All legacy Bash (`.sh`) and Python (`.py`) scripts have been completely removed and refactored into a unified Rust CLI tool (`claude-code-setup.exe`).

## 🚀 Key Differences from Upstream

1. **100% Pure Rust Architecture:** Zero dependencies on Shell scripts or Python runtimes.
2. **Dynamic Path Normalization:** Hardcoded paths (`/home/jb_remus`) auto-resolve to the target local environment.
3. **SQLite-Backed Fast Memory Engine:** Embedded SQLite database indexes Markdown notes for instant keyword search.
4. **Built-in Security & Autonomous Workflow:** Branch protection and pre-commit hooks run directly via Rust.

## 📦 Building & Usage
```bash
# Build binary
cargo build --release

# Run automated setup
./target/release/claude-code-setup.exe install

# Run diagnostic verification
./target/release/claude-code-setup.exe test
```

## 🛠️ CLI Commands

| Command | Description |
| :--- | :--- |
| `install` | Full automated setup & configuration |
| `update` | Update existing configurations |
| `test` | Run diagnostic verification suite |
| `mcp-list` | List and inspect configured MCP servers |
| `memory-index`| Index memory notes into SQLite |
| `memory-search <q>`| Instant full-text search across knowledge base |
| `security-audit`| Audit permissions & Git branch security |
| `agent-workflow`| Execute autonomous repository branch workflow |
