use crate::generators::ChecklistGenerator;
use crate::models::Workflow;
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

pub fn run(project: String, dry_run: bool) -> Result<()> {
    let repo_root = find_repo_root()?;
    let checklists_dir = repo_root.join("checklists");
    let workflows_dir = repo_root.join("workflows");

    let project_dir = checklists_dir.join(&project);
    if !project_dir.exists() {
        anyhow::bail!("Project '{}' not found", project);
    }

    let workflows = Workflow::load_all(&workflows_dir)?;

    if workflows.is_empty() {
        anyhow::bail!("No workflow files found in workflows/");
    }

    // Generate new checklists
    let checklist_md = ChecklistGenerator::generate_from_workflows(&workflows);
    let checklist_detailed_md = ChecklistGenerator::generate_detailed_from_workflows(&workflows);

    let checklist_path = project_dir.join("checklist.md");
    let checklist_detailed_path = project_dir.join("checklist-detailed.md");

    if dry_run {
        println!("🔍 {} - would update:\n", "Dry run".yellow());
        println!("📄 {}", checklist_path.display());
        println!("📄 {}\n", checklist_detailed_path.display());
        println!("Preview of checklist.md:\n{}", "─".repeat(60));
        println!("{}", checklist_md.lines().take(20).collect::<Vec<_>>().join("\n"));
        if checklist_md.lines().count() > 20 {
            println!("... ({} more lines)", checklist_md.lines().count() - 20);
        }
        println!("{}\n", "─".repeat(60));
        println!("💡 Run without {} to apply changes", "--dry-run".cyan());
    } else {
        fs::write(&checklist_path, checklist_md)
            .with_context(|| format!("Failed to write {}", checklist_path.display()))?;

        fs::write(&checklist_detailed_path, checklist_detailed_md)
            .with_context(|| format!("Failed to write {}", checklist_detailed_path.display()))?;

        println!("✅ Synced {} from {} workflow files", project.bold(), workflows.len());
        println!("   📄 Updated: {}", checklist_path.display());
        println!("   📄 Updated: {}", checklist_detailed_path.display());
    }

    Ok(())
}

fn find_repo_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;

    for ancestor in current_dir.ancestors() {
        if ancestor.join(".git").exists() || ancestor.join("checklists").exists() {
            return Ok(ancestor.to_path_buf());
        }
    }

    anyhow::bail!("Could not find repository root")
}
