use anyhow::{Context, Result};
use chrono::Local;
use colored::*;
use fs_extra::dir::{copy, CopyOptions};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::installer::get_home_dir;
use crate::mcp;

pub fn run_update(home_override: Option<String>) -> Result<()> {
    println!("{}", "Claude Code Rust Update Engine".blue().bold());
    println!("========================================");

    let home = get_home_dir(home_override.clone())?;
    let current_dir = env::current_dir().context("Failed to get current working directory")?;
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();

    println!("{} {}", "INFO:".blue().bold(), "Updating framework files...");

    let claude_dir = home.join(".claude");
    let config_dir = home.join(".config").join("claude-code");
    let memory_dir = home.join("claude_global_memory");
    let _ = memory_dir;

    // 1. Backups
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
        println!("{} Backed up MCP config", "⚠".yellow());
    }

    // 2. Updates
    let src_claude = current_dir.join("config").join("claude");
    // Ensure parent exists
    if !claude_dir.exists() { fs::create_dir_all(&claude_dir)?; }
    if src_claude.exists() {
        let mut options = CopyOptions::new();
        options.overwrite = true;
        options.content_only = true;
        let _ = copy(&src_claude, &claude_dir, &options);
        println!("{} Updated SuperClaude framework", "✓".green());
    }

    let src_mcp_file = current_dir.join("config").join("claude-code").join("claude_desktop_config.json");
    let dst_mcp_dir = config_dir.clone();
    if !dst_mcp_dir.exists() { fs::create_dir_all(&dst_mcp_dir)?; }
    let dst_mcp_file = dst_mcp_dir.join("claude_desktop_config.json");
    if src_mcp_file.exists() {
        let raw = fs::read_to_string(&src_mcp_file)?;
        let normalized = mcp::normalize_mcp_config(&raw, &home)?;
        fs::write(&dst_mcp_file, normalized)?;
        println!("{} Updated and re-normalized MCP configurations", "✓".green());
    }

    println!("{} {}", "INFO:".blue().bold(), "Updating packages (npm/uv)...");
    let mcp_projects_dir = home.join("claude-code-desktop02-setup").join("mcp-servers");
    if mcp_projects_dir.exists() {
        // npm update
        let _ = Command::new("npm").args(["update"]).current_dir(&mcp_projects_dir).status();
        println!("{} Updated npm packages", "✓".green());
    }

    println!("========================================");
    println!("{}", "✅ Update completed successfully!".green().bold());

    Ok(())
}
