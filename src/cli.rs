use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "claude-code-setup")]
#[command(about = "Rust-optimized setup, update and diagnostic CLI for Claude Code Complete Setup", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Perform full automated setup and configuration deployment
    Install {
        #[arg(short, long, help = "Skip prerequisite installation checks")]
        skip_prereqs: bool,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Update existing configuration and MCP servers
    Update {
        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Run full deployment verification diagnostic test suite
    Test {
        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// List all configured MCP servers and environment settings
    McpList {
        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Set or update configuration fields for an MCP server dynamically
    McpSet {
        #[arg(help = "Server name")]
        server: String,

        #[arg(short, long, help = "Command binary")]
        command: Option<String>,

        #[arg(short, long, help = "Arguments (multiple allowed)")]
        arg: Vec<String>,

        #[arg(short, long, help = "Environment variables (KEY=VALUE)")]
        env: Vec<String>,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Unset configuration fields or remove an MCP server
    McpUnset {
        #[arg(help = "Server name")]
        server: String,

        #[arg(short, long, help = "Environment variable keys to remove")]
        env: Vec<String>,

        #[arg(long, help = "Clear all command arguments")]
        clear_args: bool,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Enable a disabled MCP server
    McpEnable {
        #[arg(help = "Server name")]
        server: String,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Disable an MCP server without removing its configuration
    McpDisable {
        #[arg(help = "Server name")]
        server: String,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Index global memory Markdown files into SQLite database
    MemoryIndex {
        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Search indexed global memory notes
    MemorySearch {
        #[arg(help = "Search query keyword")]
        query: String,

        #[arg(
            short,
            long,
            default_value = "hybrid",
            help = "Search mode: keyword, semantic, hybrid"
        )]
        mode: String,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Show related notes via graph edges (wikilinks + semantic ties)
    MemoryRelated {
        #[arg(help = "Target note filename (e.g. SYSTEM-STATUS-AND-SETUP.md)")]
        note: String,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Install security & Git pre-commit branch protection hooks
    InstallHooks {
        #[arg(short, long, help = "Target repository path")]
        repo_dir: Option<String>,
    },

    /// Run security audit on active configurations and Git state
    SecurityAudit {
        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Execute autonomous repository manager workflow
    AgentWorkflow {
        #[arg(short, long, default_value = "feature", help = "Branch type prefix")]
        branch_type: String,

        #[arg(short, long, help = "Workflow description")]
        description: String,

        #[arg(short, long, help = "Files to commit")]
        files: Vec<String>,
    },

    /// Display environment status summary
    Status,
}
