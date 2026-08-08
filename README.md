# Claude Code Complete Setup (Rust Engine)

A high-performance, single-binary Rust-based deployment system for Claude Code that automates configuration, toolchain setup, MCP server management, and security auditing.

## 🚀 Key Advantages
- **Native Rust Engine:** Runs without Bash or external shell dependencies.
- **Dynamic Path Normalization:** Auto-rewrites paths to match your local environment.
- **SQLite Memory Indexing:** Fast keyword search across your knowledge base.
- **Automated Security:** Pre-commit hooks & branch protection built-in.

## 📦 Installation
1. Build: `cargo build --release`
2. Install: `./target/release/claude-code-setup.exe install`

## 🛠️ CLI Commands

| Command | Description |
| :--- | :--- |
| `install` | Full setup & configuration |
| `update` | Update existing setup |
| `test` | Verify deployment diagnostics |
| `mcp-list` | List MCP servers |
| `memory-index`| Index memory notes |
| `memory-search <q>`| Search memory |
| `security-audit`| Audit security/Git state |

## 🛡️ Security
- **Branch Protection:** Enforces feature-branch workflows.
- **Secret Scanner:** Prevents API key leakage.
