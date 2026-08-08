use anyhow::{Context, Result};
use chrono::Local;
use colored::*;
use std::process::Command;

const PROTECTED_BRANCHES: &[&str] = &["main", "master"];

pub fn is_protected_branch(branch: &str) -> bool {
    PROTECTED_BRANCHES.contains(&branch)
}

pub fn get_current_branch() -> Result<String> {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .context("Failed to execute git branch")?;
    let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(name)
}

pub fn ensure_safe_branch() -> Result<String> {
    let current = get_current_branch()?;
    if is_protected_branch(&current) {
        let timestamp = Local::now().format("%Y%m%d-%H%M%S").to_string();
        let safe_branch = format!("work/{}", timestamp);
        println!(
            "{} Protected branch '{}' detected! Creating safe branch '{}'...",
            "⚠".yellow().bold(),
            current,
            safe_branch
        );
        Command::new("git")
            .args(["checkout", "-b", &safe_branch])
            .status()?;
        Ok(safe_branch)
    } else {
        Ok(current)
    }
}

pub fn create_feature_branch(branch_type: &str, description: &str) -> Result<String> {
    let timestamp = Local::now().format("%Y%m%d-%H%M%S").to_string();
    let sanitized_desc = description.to_lowercase().replace(' ', "-");
    let branch_name = format!("{}/{}-{}", branch_type, timestamp, sanitized_desc);

    println!(
        "{} Creating feature branch: {}",
        "✓".green().bold(),
        branch_name.cyan()
    );

    Command::new("git").args(["fetch", "origin"]).status()?;
    Command::new("git")
        .args(["checkout", "-b", &branch_name, "origin/main"])
        .status()?;

    Ok(branch_name)
}

pub fn safe_commit(message: &str, files: &[String]) -> Result<()> {
    ensure_safe_branch()?;
    for file in files {
        Command::new("git").args(["add", file]).status()?;
    }
    Command::new("git")
        .args(["commit", "-m", message])
        .status()?;
    println!("{} Committed: {}", "✓".green().bold(), message);
    Ok(())
}

pub fn safe_push() -> Result<()> {
    let branch = get_current_branch()?;
    if is_protected_branch(&branch) {
        anyhow::bail!(
            "BLOCKED: Cannot push directly to protected branch '{}'",
            branch
        );
    }
    Command::new("git")
        .args(["push", "-u", "origin", &branch])
        .status()?;
    println!(
        "{} Pushed branch '{}' to origin",
        "✓".green().bold(),
        branch
    );
    Ok(())
}
