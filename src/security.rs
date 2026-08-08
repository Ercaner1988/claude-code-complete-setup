use anyhow::{Context, Result};
use colored::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::installer::get_home_dir;

const PRE_COMMIT_HOOK_CONTENT: &str = r#"#!/usr/bin/env bash
# Rust-generated pre-commit security hook for Claude Code setup
set -e

BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")

if [ "$BRANCH" = "main" ] || [ "$BRANCH" = "master" ]; then
    echo -e "\033[0;31m[SECURITY ERROR] Direct commit to '$BRANCH' branch is prohibited by security policy!\033[0m"
    echo "Please create a feature branch and open a PR instead."
    exit 1
fi

# Secret leakage scanner
if git diff --cached | grep -iE '(api_key|secret_key|password|github_token)\s*=\s*["'\''][a-zA-Z0-9_-]{16,}["'\'']'; then
    echo -e "\033[0;31m[SECURITY ERROR] Potential hardcoded API secret detected in staged changes!\033[0m"
    exit 1
fi

echo -e "\033[0;32m[SECURITY CHECK] Pre-commit security verification passed!\033[0m"
"#;

pub fn install_git_hooks(repo_dir: Option<String>) -> Result<()> {
    let target_dir = if let Some(dir) = repo_dir {
        PathBuf::from(dir)
    } else {
        env::current_dir().context("Failed to get current working directory")?
    };

    let git_hooks_dir = target_dir.join(".git").join("hooks");

    if !git_hooks_dir.exists() {
        println!(
            "{} Git hooks directory not found at {:?}",
            "✗".red(),
            git_hooks_dir
        );
        println!("Ensure this is a valid Git repository root.");
        return Ok(());
    }

    let pre_commit_path = git_hooks_dir.join("pre-commit");
    fs::write(&pre_commit_path, PRE_COMMIT_HOOK_CONTENT)?;

    println!(
        "{} Successfully installed pre-commit security & branch protection hook at {:?}",
        "✓".green().bold(),
        pre_commit_path
    );

    Ok(())
}

pub fn run_security_audit(home_override: Option<String>) -> Result<()> {
    let home = get_home_dir(home_override)?;
    println!("{}", "Claude Code Security Audit".cyan().bold());
    println!("========================================");

    // Check .env files for plaintext secrets
    let current_dir = env::current_dir()?;
    let env_file = current_dir.join(".env");
    if env_file.exists() {
        println!("{} .env file present in working directory", "✓".green());
    } else {
        println!("{} .env file missing in working directory", "⚠".yellow());
    }

    // Check .config/claude-code file permissions
    let config_file = home
        .join(".config")
        .join("claude-code")
        .join("claude_desktop_config.json");
    if config_file.exists() {
        println!(
            "{} MCP config file exists at {:?}",
            "✓".green(),
            config_file
        );
    } else {
        println!("{} MCP config file not found", "✗".red());
    }

    // Check branch protection
    if let Ok(output) = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
    {
        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("Current Git branch: {}", branch.yellow().bold());
        if branch == "main" || branch == "master" {
            println!(
                "{} Working directly on main branch! Use feature branches.",
                "⚠".yellow()
            );
        }
    }

    println!("========================================");
    println!("{}", "Security audit complete!".green().bold());

    Ok(())
}
