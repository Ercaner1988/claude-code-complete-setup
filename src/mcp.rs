use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use crate::installer::get_home_dir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct McpServerConfig {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct McpConfigFile {
    #[serde(rename = "mcpServers")]
    pub mcp_servers: BTreeMap<String, McpServerConfig>,
}

pub fn load_mcp_config(config_path: &PathBuf) -> Result<McpConfigFile> {
    let content = fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read MCP config from {:?}", config_path))?;
    let config: McpConfigFile = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from {:?}", config_path))?;
    Ok(config)
}

pub fn normalize_mcp_config(
    raw_json: &str,
    target_home: &PathBuf,
) -> Result<String> {
    let home_str = target_home.to_string_lossy().replace('\\', "/");
    let mut val: Value = serde_json::from_str(raw_json)?;

    if let Some(servers) = val.get_mut("mcpServers").and_then(|s| s.as_object_mut()) {
        for (_name, server) in servers.iter_mut() {
            if let Some(env_obj) = server.get_mut("env").and_then(|e| e.as_object_mut()) {
                for (_k, v) in env_obj.iter_mut() {
                    if let Some(s) = v.as_str() {
                        if s.contains("/home/jb_remus") {
                            let new_s = s.replace("/home/jb_remus", &home_str);
                            *v = Value::String(new_s);
                        }
                    }
                }
            }
        }
    }

    Ok(serde_json::to_string_pretty(&val)?)
}

pub fn list_mcp_servers(home_override: Option<String>) -> Result<()> {
    let home = get_home_dir(home_override)?;
    let config_path = home.join(".config").join("claude-code").join("claude_desktop_config.json");

    if !config_path.exists() {
        println!("{} MCP config file not found at {:?}", "✗".red(), config_path);
        return Ok(());
    }

    let config = load_mcp_config(&config_path)?;
    println!("{}", "Configured MCP Servers".cyan().bold());
    println!("========================================");
    for (name, server) in &config.mcp_servers {
        println!("• {}: {} {}", name.green().bold(), server.command, server.args.join(" "));
        if !server.env.is_empty() {
            for (k, v) in &server.env {
                println!("    env: {}={}", k.dimmed(), v);
            }
        }
    }
    println!("========================================");
    println!("Total servers: {}", config.mcp_servers.len().to_string().yellow().bold());

    Ok(())
}
