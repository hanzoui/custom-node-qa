use crate::models::Checklist;
use anyhow::Result;
use colored::Colorize;
use comfy_table::{presets::UTF8_FULL, Attribute, Cell, Color as TableColor, Table};
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn run() -> Result<()> {
    let repo_root = find_repo_root()?;
    let checklists_dir = repo_root.join("checklists");

    let mut projects = Vec::new();

    for entry in WalkDir::new(&checklists_dir)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_dir()
            && path != checklists_dir
            && !path.ends_with("templates")
            && !path.ends_with("schema")
        {
            if let Some(project_name) = path.file_name().and_then(|s| s.to_str()) {
                projects.push(project_name.to_string());
            }
        }
    }

    if projects.is_empty() {
        println!("No projects found in checklists/");
        println!(
            "💡 Create one with: {}",
            "comfy-qa new <project-name>".cyan()
        );
        return Ok(());
    }

    projects.sort();

    println!("\n{} ({})\n", "📁 QA Projects".bold(), projects.len());

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec![
        Cell::new("Project")
            .add_attribute(Attribute::Bold)
            .fg(TableColor::Cyan),
        Cell::new("Packs")
            .add_attribute(Attribute::Bold)
            .fg(TableColor::Cyan),
        Cell::new("Status")
            .add_attribute(Attribute::Bold)
            .fg(TableColor::Cyan),
    ]);

    for project in &projects {
        let project_dir = checklists_dir.join(project);
        let checklist_path = project_dir.join("checklist.md");

        let (pack_count, status) = match Checklist::from_file(&checklist_path) {
            Ok(checklist) => {
                let tested = checklist.packs.iter().filter(|p| p.tested).count();
                let total = checklist.packs.len();
                let status = if tested == total && total > 0 {
                    format!("✓ Complete ({}/{})", tested, total)
                } else if tested > 0 {
                    format!("⏳ In Progress ({}/{})", tested, total)
                } else {
                    format!("○ Not Started (0/{})", total)
                };
                (total, status)
            }
            Err(_) => (0, "⚠ Error".to_string()),
        };

        let status_cell = if status.contains("Complete") {
            Cell::new(&status).fg(TableColor::Green)
        } else if status.contains("Progress") {
            Cell::new(&status).fg(TableColor::Yellow)
        } else if status.contains("Error") {
            Cell::new(&status).fg(TableColor::Red)
        } else {
            Cell::new(&status).fg(TableColor::DarkGrey)
        };

        table.add_row(vec![
            Cell::new(project),
            Cell::new(pack_count.to_string()),
            status_cell,
        ]);
    }

    println!("{}", table);
    println!("\n💡 View details: {}", "comfy-qa status <project>".cyan());
    println!("💡 Start testing: {}\n", "comfy-qa".cyan());

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
