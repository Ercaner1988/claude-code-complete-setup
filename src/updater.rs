use anyhow::{Context, Result};
use chrono::Local;
use colored::*;
use fs_extra::dir::{copy, CopyOptions};
use std::env;

use crate::installer::get_home_dir;

pub fn run_update(home_override: Option<String>) -> Result<()> {
    println!("{}", "Claude Code Rust Update Engine".blue().bold());
    println!("========================================");

    let home = get_home_dir(home_override)?;
    let current_dir = env::current_dir().context("Failed to get current working directory")?;
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();

    println!("{} {}", "INFO:".blue().bold(), "Updating configurations...");

    let claude_dir = home.join(".claude");
    let config_dir = home.join(".config").join("claude-code");

    if claude_dir.exists() {
        let backup_path = home.join(format!(".claude.backup.{}", timestamp));
        let mut options = CopyOptions::new();
        options.copy_inside = true;
        let _ = copy(&claude_dir, &backup_path, &options);
        println!("{} Backed up .claude", "⚠".yellow());
    }

    if config_dir.exists() {
        let backup_path = home.join(format!(".config.claude-code.backup.{}", timestamp));
        let mut options = CopyOptions::new();
        options.copy_inside = true;
        let _ = copy(&config_dir, &backup_path, &options);
        println!("{} Backed up claude-code config", "⚠".yellow());
    }

    let src_claude = current_dir.join("config").join("claude");
    if src_claude.exists() {
        let mut options = CopyOptions::new();
        options.overwrite = true;
        options.content_only = true;
        let _ = copy(&src_claude, &claude_dir, &options);
        println!("{} Updated SuperClaude framework", "✓".green());
    }

    let src_mcp = current_dir.join("config").join("claude-code");
    if src_mcp.exists() {
        let mut options = CopyOptions::new();
        options.overwrite = true;
        options.content_only = true;
        let _ = copy(&src_mcp, &config_dir, &options);
        println!("{} Updated MCP configurations", "✓".green());
    }

    println!("========================================");
    println!("{}", "✅ Update completed successfully!".green().bold());

    Ok(())
}
