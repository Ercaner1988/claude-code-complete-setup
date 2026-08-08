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

        #[arg(short, long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Update existing configuration and MCP servers
    Update {
        #[arg(short, long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Run full deployment verification diagnostic test suite
    Test {
        #[arg(short, long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// List all configured MCP servers and environment settings
    McpList {
        #[arg(short, long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Index global memory Markdown files into SQLite database
    MemoryIndex {
        #[arg(short, long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Search indexed global memory notes
    MemorySearch {
        #[arg(help = "Search query keyword")]
        query: String,

        #[arg(short, long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Install security & Git pre-commit branch protection hooks
    InstallHooks {
        #[arg(short, long, help = "Target repository path")]
        repo_dir: Option<String>,
    },

    /// Run security audit on active configurations and Git state
    SecurityAudit {
        #[arg(short, long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Display environment status summary
    Status,
}
