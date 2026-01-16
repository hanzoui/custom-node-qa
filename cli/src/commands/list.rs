use anyhow::Result;
use colored::Colorize;
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
        println!("💡 Create one with: {}", "comfy-qa new <project-name>".cyan());
        return Ok(());
    }

    projects.sort();

    println!("{} ({}):\n", "📁 QA Projects".bold(), projects.len());

    for project in &projects {
        println!("  • {}", project);
    }

    println!("\n💡 View project status: {}", "comfy-qa status <project>".cyan());

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
