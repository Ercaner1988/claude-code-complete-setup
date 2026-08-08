use anyhow::{Context, Result};
use colored::*;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use regex::Regex;
use rusqlite::{params, Connection};
use std::collections::{HashSet, VecDeque};
use std::fs;
use std::path::PathBuf;

use crate::installer::get_home_dir;

pub fn get_db_path(home_override: Option<String>) -> Result<PathBuf> {
    let home = get_home_dir(home_override)?;
    let claude_dir = home.join(".claude");
    fs::create_dir_all(&claude_dir)?;
    Ok(claude_dir.join("memory_index.db"))
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;
    for (x, y) in a.iter().zip(b.iter()) {
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a.sqrt() * norm_b.sqrt())
    }
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS knowledge_notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            filename TEXT UNIQUE NOT NULL,
            title TEXT,
            content TEXT NOT NULL,
            embedding BLOB
        )",
        [],
    )?;

    // FTS5 Sanal Tablosu
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS knowledge_fts USING fts5(
            filename UNINDEXED,
            title,
            content
        )",
        [],
    )?;

    // Graph Kenarları Tablosu
    conn.execute(
        "CREATE TABLE IF NOT EXISTS note_edges (
            src TEXT NOT NULL,
            dst TEXT NOT NULL,
            tur TEXT NOT NULL,
            agirlik REAL NOT NULL,
            PRIMARY KEY (src, dst, tur)
        )",
        [],
    )?;

    Ok(())
}

fn bytes_to_f32_vec(bytes: &[u8]) -> Vec<f32> {
    bytes
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
        .collect()
}

fn f32_vec_to_bytes(vec: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(vec.len() * 4);
    for &val in vec {
        bytes.extend_from_slice(&val.to_le_bytes());
    }
    bytes
}

pub fn index_memory(home_override: Option<String>) -> Result<()> {
    let home = get_home_dir(home_override.clone())?;
    let db_path = get_db_path(home_override)?;
    let knowledge_dir = home.join("claude_global_memory").join("knowledge");

    if !knowledge_dir.exists() {
        println!(
            "{} Memory directory missing, skipping index: {:?}",
            "⚠".yellow(),
            knowledge_dir
        );
        return Ok(());
    }

    let conn = Connection::open(&db_path)
        .with_context(|| format!("Failed to open SQLite database at {:?}", db_path))?;

    init_db(&conn)?;

    conn.execute("DELETE FROM knowledge_notes", [])?;
    conn.execute("DELETE FROM knowledge_fts", [])?;
    conn.execute("DELETE FROM note_edges", [])?;

    let mut files_data = Vec::new();
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

            files_data.push((filename, title, content));
        }
    }

    if files_data.is_empty() {
        println!("{}", "No markdown files found to index.".yellow());
        return Ok(());
    }

    println!(
        "{}",
        "Generating embeddings via FastEmbed (BGEMSI)...".blue()
    );
    let model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::BGESmallENV15).with_show_download_progress(true),
    )?;

    let texts: Vec<String> = files_data
        .iter()
        .map(|(_, title, content)| format!("{}\n{}", title, content))
        .collect();

    let embeddings = model.embed(texts, None)?;

    // DB'ye Ekle
    for (i, (filename, title, content)) in files_data.iter().enumerate() {
        let emb_bytes = f32_vec_to_bytes(&embeddings[i]);
        conn.execute(
            "INSERT INTO knowledge_notes (filename, title, content, embedding) VALUES (?1, ?2, ?3, ?4)",
            params![filename, title, content, emb_bytes],
        )?;

        conn.execute(
            "INSERT INTO knowledge_fts (filename, title, content) VALUES (?1, ?2, ?3)",
            params![filename, title, content],
        )?;
    }

    // Wikilink & Semantik Kenarları Oluştur
    let wikilink_re = Regex::new(r"\[\[(.*?)\]\]")?;
    for (i, (src_file, _, content)) in files_data.iter().enumerate() {
        // 1. Wikilinks
        for cap in wikilink_re.captures_iter(content) {
            let target = cap[1].trim();
            let dst_file = if target.ends_with(".md") {
                target.to_string()
            } else {
                format!("{}.md", target)
            };
            conn.execute(
                "INSERT OR REPLACE INTO note_edges (src, dst, tur, agirlik) VALUES (?1, ?2, 'wikilink', 1.0)",
                params![src_file, dst_file],
            )?;
        }

        // 2. Kosinüs Semantik Kenarlar (Threshold = 0.70)
        // ponytail: lineer kosinüs; not > ~5k olursa ANN ekle
        for j in (i + 1)..files_data.len() {
            let sim = cosine_similarity(&embeddings[i], &embeddings[j]);
            if sim >= 0.70 {
                let dst_file = &files_data[j].0;
                conn.execute(
                    "INSERT OR REPLACE INTO note_edges (src, dst, tur, agirlik) VALUES (?1, ?2, 'semantic', ?3)",
                    params![src_file, dst_file, sim],
                )?;
                conn.execute(
                    "INSERT OR REPLACE INTO note_edges (src, dst, tur, agirlik) VALUES (?1, ?2, 'semantic', ?3)",
                    params![dst_file, src_file, sim],
                )?;
            }
        }
    }

    println!(
        "{} Successfully indexed {} notes with embeddings and graph edges into SQLite ({:?})",
        "✓".green().bold(),
        files_data.len(),
        db_path
    );

    Ok(())
}

pub fn search_memory(query: &str, mode: &str, home_override: Option<String>) -> Result<()> {
    let db_path = get_db_path(home_override.clone())?;
    if !db_path.exists() {
        println!(
            "{} Memory database not found, run memory-index first.",
            "⚠".yellow()
        );
        return Ok(());
    }

    let conn = Connection::open(&db_path)?;

    match mode {
        "keyword" => search_keyword(&conn, query),
        "semantic" => search_semantic(&conn, query),
        _ => search_hybrid(&conn, query), // default hybrid
    }
}

fn search_keyword(conn: &Connection, query: &str) -> Result<()> {
    println!(
        "{} (FTS5) for '{}'",
        "Memory Keyword Search".cyan().bold(),
        query.yellow()
    );
    println!("========================================");

    let mut stmt = conn.prepare(
        "SELECT filename, title, content FROM knowledge_fts WHERE knowledge_fts MATCH ?1 ORDER BY rank",
    )?;

    let rows = stmt.query_map(params![query], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    });

    let mut matches = 0;
    if let Ok(rows) = rows {
        for row in rows {
            let (filename, title, content) = row?;
            matches += 1;
            println!("• {} [{}]", title.green().bold(), filename.dimmed());
            for line in content.lines() {
                if line.to_lowercase().contains(&query.to_lowercase()) {
                    println!("    {}", line.trim());
                }
            }
        }
    }

    println!("========================================");
    println!("Total matched documents: {}", matches);
    Ok(())
}

fn search_semantic(conn: &Connection, query: &str) -> Result<()> {
    println!(
        "{} for '{}'",
        "Memory Semantic Search".cyan().bold(),
        query.yellow()
    );
    println!("========================================");

    let model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::BGESmallENV15).with_show_download_progress(false),
    )?;

    let query_emb = model.embed(vec![query.to_string()], None)?[0].clone();

    let mut stmt =
        conn.prepare("SELECT filename, title, content, embedding FROM knowledge_notes")?;
    let mut results: Vec<(String, String, String, f32)> = Vec::new();

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, Option<Vec<u8>>>(3)?,
        ))
    })?;

    for row in rows {
        let (filename, title, content, emb_opt) = row?;
        if let Some(emb_bytes) = emb_opt {
            let emb = bytes_to_f32_vec(&emb_bytes);
            // ponytail: lineer kosinüs; not > ~5k olursa ANN ekle
            let score = cosine_similarity(&query_emb, &emb);
            results.push((filename, title, content, score));
        }
    }

    results.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));

    let mut count = 0;
    for (filename, title, _content, score) in results.iter().take(5) {
        if *score > 0.30 {
            count += 1;
            println!(
                "• {} [{}] (Score: {:.4})",
                title.green().bold(),
                filename.dimmed(),
                score
            );
        }
    }

    println!("========================================");
    println!("Total matched documents: {}", count);
    Ok(())
}

fn search_hybrid(conn: &Connection, query: &str) -> Result<()> {
    println!(
        "{} for '{}'",
        "Memory Hybrid Search (FTS5 + Semantic)".cyan().bold(),
        query.yellow()
    );
    println!("========================================");
    // Basitleştirilmiş Hibrit: Önce semantik getir, eşleşenleri sun
    search_semantic(conn, query)
}

pub fn get_related_notes(note_filename: &str, home_override: Option<String>) -> Result<()> {
    let db_path = get_db_path(home_override)?;
    if !db_path.exists() {
        println!("{} Memory database not found.", "⚠".yellow());
        return Ok(());
    }

    let conn = Connection::open(&db_path)?;

    println!(
        "{} for '{}'",
        "Graph Related Notes".cyan().bold(),
        note_filename.yellow()
    );
    println!("========================================");

    // BFS ile 1 ve 2 adımlık komşuları bul
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((note_filename.to_string(), 0));
    visited.insert(note_filename.to_string());

    let mut stmt = conn.prepare("SELECT dst, tur, agirlik FROM note_edges WHERE src = ?1")?;

    while let Some((curr, dist)) = queue.pop_front() {
        if dist >= 2 {
            continue;
        }

        let rows = stmt.query_map(params![curr], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, f64>(2)?,
            ))
        })?;

        for row in rows {
            let (dst, tur, agirlik) = row?;
            if !visited.contains(&dst) {
                visited.insert(dst.clone());
                println!(
                    "  [Hop {}] {} (Edge: {}, Weight: {:.2})",
                    dist + 1,
                    dst.green().bold(),
                    tur.dimmed(),
                    agirlik
                );
                queue.push_back((dst, dist + 1));
            }
        }
    }

    println!("========================================");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity_orthogonality() {
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![0.0, 1.0, 0.0];
        assert_eq!(cosine_similarity(&v1, &v2), 0.0);

        let v3 = vec![1.0, 2.0, 3.0];
        assert!((cosine_similarity(&v3, &v3) - 1.0).abs() < 1e-5);
    }
}
