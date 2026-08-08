use anyhow::Result;
use colored::*;

use crate::branch_manager;

pub fn run_agent_workflow(branch_type: &str, description: &str, files: &[String]) -> Result<()> {
    println!(
        "{}",
        "Autonomous Repository Agent (Rust Engine)".cyan().bold()
    );
    println!("==================================================");

    let branch = branch_manager::create_feature_branch(branch_type, description)?;
    let commit_msg = format!("{}: {}", branch_type, description);

    if !files.is_empty() {
        branch_manager::safe_commit(&commit_msg, files)?;
        branch_manager::safe_push()?;
        println!(
            "{} Workflow completed successfully on branch '{}'",
            "✅".green(),
            branch
        );
    } else {
        println!("{} No files specified for staging.", "⚠".yellow());
    }

    println!("==================================================");
    Ok(())
}
