use anyhow::{Context, Result};
use chrono::Local;
use colored::*;
use fs_extra::dir::{copy, CopyOptions};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::mcp;

pub fn get_home_dir(override_path: Option<String>) -> Result<PathBuf> {
    if let Some(path) = override_path {
        Ok(PathBuf::from(path))
    } else {
        env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .map(PathBuf::from)
            .context("Could not determine user home directory")
    }
}

fn log_info(msg: &str) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    println!("{} {} {}", format!("[{}]", now).blue(), "INFO:".bold(), msg);
}

fn log_success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

fn log_warning(msg: &str) {
    println!("{} {}", "⚠".yellow().bold(), msg);
}

fn check_cmd(cmd: &str) -> bool {
    Command::new(cmd).arg("--version").output().is_ok()
}

pub fn run_install(skip_prereqs: bool, home_override: Option<String>) -> Result<()> {
    println!("{}", "Claude Code Rust Complete Setup".cyan().bold());
    println!("========================================");

    let home = get_home_dir(home_override)?;
    let current_dir = env::current_dir().context("Failed to get current working directory")?;
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();

    if !skip_prereqs {
        log_info("Checking prerequisites...");
        if check_cmd("git") {
            log_success("Git is installed");
        } else {
            log_warning("Git command not found in PATH");
        }

        if check_cmd("node") {
            log_success("Node.js is installed");
        } else {
            log_warning("Node.js command not found in PATH");
        }

        if check_cmd("python3") || check_cmd("python") {
            log_success("Python is installed");
        } else {
            log_warning("Python command not found in PATH");
        }

        if check_cmd("uv") {
            log_success("UV (Python package manager) is installed");
        } else {
            log_warning("UV command not found in PATH");
        }
    }

    log_info("Backing up existing configurations...");
    let claude_dir = home.join(".claude");
    let config_dir = home.join(".config").join("claude-code");
    let memory_dir = home.join("claude_global_memory");

    if claude_dir.exists() {
        let backup_path = home.join(format!(".claude.backup.{}", timestamp));
        let mut options = CopyOptions::new();
        options.copy_inside = true;
        let _ = copy(&claude_dir, &backup_path, &options);
        log_warning(&format!("Backed up .claude to {:?}", backup_path));
    }

    if config_dir.exists() {
        let backup_path = home.join(format!(".config.claude-code.backup.{}", timestamp));
        let mut options = CopyOptions::new();
        options.copy_inside = true;
        let _ = copy(&config_dir, &backup_path, &options);
        log_warning(&format!("Backed up claude-code config to {:?}", backup_path));
    }

    if memory_dir.exists() {
        let backup_path = home.join(format!("claude_global_memory.backup.{}", timestamp));
        let mut options = CopyOptions::new();
        options.copy_inside = true;
        let _ = copy(&memory_dir, &backup_path, &options);
        log_warning(&format!("Backed up global memory to {:?}", backup_path));
    }

    log_info("Copying configurations & normalizing MCP paths...");
    fs::create_dir_all(&claude_dir)?;
    fs::create_dir_all(&config_dir)?;
    fs::create_dir_all(&memory_dir)?;

    let src_claude_cfg = current_dir.join("config").join("claude");
    if src_claude_cfg.exists() {
        let mut options = CopyOptions::new();
        options.overwrite = true;
        options.content_only = true;
        let _ = copy(&src_claude_cfg, &claude_dir, &options);
        log_success("Copied SuperClaude framework");
    }

    let src_mcp_file = current_dir.join("config").join("claude-code").join("claude_desktop_config.json");
    let dst_mcp_file = config_dir.join("claude_desktop_config.json");
    if src_mcp_file.exists() {
        let raw = fs::read_to_string(&src_mcp_file)?;
        let normalized = mcp::normalize_mcp_config(&raw, &home)?;
        fs::write(&dst_mcp_file, normalized)?;
        log_success("Copied and normalized MCP server configurations");
    }

    let src_memory = current_dir.join("global_memory");
    if src_memory.exists() {
        let mut options = CopyOptions::new();
        options.overwrite = true;
        options.content_only = true;
        let _ = copy(&src_memory, &memory_dir, &options);
        log_success("Copied global memory system");
    }

    log_info("Setting secure file permissions...");

    let env_src = current_dir.join(".env");
    let env_dst = home.join(".env.claude");
    if env_src.exists() {
        fs::copy(&env_src, &env_dst)?;
        log_success("Configured .env.claude environment file");
    } else {
        log_warning("No .env file found in repository root. Copy .env.example to .env to set up secrets.");
    }

    println!("========================================");
    println!("{}", "✅ Setup completed successfully via Rust engine!".green().bold());

    Ok(())
}
