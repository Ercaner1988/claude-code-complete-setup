mod cli;
mod installer;
mod mcp;
mod memory_engine;
mod security;
mod tester;
mod updater;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { skip_prereqs, home_dir } => {
            installer::run_install(skip_prereqs, home_dir)?;
        }
        Commands::Update { home_dir } => {
            updater::run_update(home_dir)?;
        }
        Commands::Test { home_dir } => {
            tester::run_tests(home_dir)?;
        }
        Commands::McpList { home_dir } => {
            mcp::list_mcp_servers(home_dir)?;
        }
        Commands::MemoryIndex { home_dir } => {
            memory_engine::index_memory(home_dir)?;
        }
        Commands::MemorySearch { query, home_dir } => {
            memory_engine::search_memory(&query, home_dir)?;
        }
        Commands::InstallHooks { repo_dir } => {
            security::install_git_hooks(repo_dir)?;
        }
        Commands::SecurityAudit { home_dir } => {
            security::run_security_audit(home_dir)?;
        }
        Commands::Status => {
            tester::run_tests(None)?;
        }
    }

    Ok(())
}
