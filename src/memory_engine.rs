use anyhow::{Context, Result};
use colored::*;
use rusqlite::{params, Connection};
use std::fs;
use std::path::PathBuf;

use crate::installer::get_home_dir;

pub fn get_db_path(home_override: Option<String>) -> Result<PathBuf> {
    let home = get_home_dir(home_override)?;
    let claude_dir = home.join(".claude");
    fs::create_dir_all(&claude_dir)?;
    Ok(claude_dir.join("memory_index.db"))
}

pub fn index_memory(home_override: Option<String>) -> Result<()> {
    let home = get_home_dir(home_override.clone())?;
    let db_path = get_db_path(home_override)?;
    let knowledge_dir = home.join("claude_global_memory").join("knowledge");

    if !knowledge_dir.exists() {
        // Otomatik oluşturma veya uyarı
        println!("{} Memory directory missing, skipping index: {:?}", "⚠".yellow(), knowledge_dir);
        return Ok(());
    }

    let conn = Connection::open(&db_path)
        .with_context(|| format!("Failed to open SQLite database at {:?}", db_path))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS knowledge_notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            filename TEXT UNIQUE NOT NULL,
            title TEXT,
            content TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute("DELETE FROM knowledge_notes", [])?;

    let mut count = 0;
    for entry in fs::read_dir(&knowledge_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            let content = fs::read_to_string(&path)?;
            let title = content
                .lines()
                .find(|l| l.starts_with("# "))
                .map(|l| l.trim_start_matches("# ").to_string())
                .unwrap_or_else(|| filename.clone());

            conn.execute(
                "INSERT INTO knowledge_notes (filename, title, content) VALUES (?1, ?2, ?3)",
                params![filename, title, content],
            )?;
            count += 1;
        }
    }

    println!(
        "{} Successfully indexed {} knowledge notes into SQLite ({:?})",
        "✓".green().bold(),
        count,
        db_path
    );

    Ok(())
}

pub fn search_memory(query: &str, home_override: Option<String>) -> Result<()> {
    let db_path = get_db_path(home_override.clone())?;
    if !db_path.exists() {
        println!("{} Memory database not found, run memory-index first.", "⚠".yellow());
        return Ok(());
    }
    
    let conn = Connection::open(&db_path)?;
    let search_pattern = format!("%{}%", query);

    let mut stmt = conn.prepare(
        "SELECT filename, title, content FROM knowledge_notes 
         WHERE title LIKE ?1 OR content LIKE ?1 
         ORDER BY filename ASC",
    )?;

    let rows = stmt.query_map(params![search_pattern], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    println!("{} for '{}'", "Memory Search Results".cyan().bold(), query.yellow());
    println!("========================================");
    let mut matches = 0;

    for row in rows {
        let (filename, title, content) = row?;
        matches += 1;
        println!("• {} [{}]", title.green().bold(), filename.dimmed());

        for line in content.lines() {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                let trimmed = line.trim();
                println!("    {}", trimmed);
            }
        }
    }

    if matches == 0 {
        println!("{}", "No matching notes found.".yellow());
    }

    println!("========================================");
    println!("Total matched documents: {}", matches);

    Ok(())
}
