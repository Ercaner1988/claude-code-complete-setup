use anyhow::Result;
use colored::*;
use serde_json::Value;
use std::env;
use std::fs;
use std::process::Command;

use crate::installer::get_home_dir;

pub fn run_tests(home_override: Option<String>) -> Result<()> {
    println!(
        "{}",
        "Claude Code Deployment Diagnostic Suite".cyan().bold()
    );
    println!("===============================================");

    let home = get_home_dir(home_override)?;

    // Test 1: Claude CLI
    print!("Testing Claude CLI... ");
    if Command::new("claude").arg("--version").output().is_ok() {
        println!("{}", "✓ Installed".green());
    } else {
        println!("{}", "✗ Not found in PATH".red());
    }

    // Test 2: SuperClaude Framework
    print!("Testing SuperClaude Framework... ");
    let superclaude_file = home.join(".claude").join("CLAUDE.md");
    if superclaude_file.exists() {
        println!("{}", "✓ Found (.claude/CLAUDE.md)".green());
    } else {
        println!("{}", "✗ Missing (.claude/CLAUDE.md)".red());
    }

    // Test 3: MCP Configuration
    print!("Testing MCP Configuration... ");
    let mcp_cfg_file = home
        .join(".config")
        .join("claude-code")
        .join("claude_desktop_config.json");
    if mcp_cfg_file.exists() {
        if let Ok(content) = fs::read_to_string(&mcp_cfg_file) {
            if let Ok(v) = serde_json::from_str::<Value>(&content) {
                if let Some(mcp_servers) = v.get("mcpServers").and_then(|s| s.as_object()) {
                    println!(
                        "{} ({} MCP servers configured)",
                        "✓ Found".green(),
                        mcp_servers.len()
                    );
                } else {
                    println!("{}", "✓ Config file readable".green());
                }
            } else {
                println!("{}", "⚠ Invalid JSON structure".yellow());
            }
        } else {
            println!("{}", "✓ File exists".green());
        }
    } else {
        println!("{}", "✗ Missing".red());
    }

    // Test 4: Global Memory
    print!("Testing Global Memory... ");
    let knowledge_dir = home.join("claude_global_memory").join("knowledge");
    if knowledge_dir.exists() {
        if let Ok(entries) = fs::read_dir(&knowledge_dir) {
            let count = entries.filter_map(|e| e.ok()).count();
            println!("{} ({} files in knowledge)", "✓ Found".green(), count);
        } else {
            println!("{}", "✓ Directory exists".green());
        }
    } else {
        println!("{}", "✗ Missing directory".red());
    }

    // Test 5: Environment Variables
    print!("Testing Environment Variables... ");
    if env::var("GITHUB_TOKEN").is_ok() || env::var("ANTHROPIC_API_KEY").is_ok() {
        println!("{}", "✓ Set".green());
    } else {
        println!("{}", "⚠ API Keys not set in shell environment".yellow());
    }

    println!("===============================================");
    println!("{}", "Diagnostic verification completed!".green().bold());

    Ok(())
}
