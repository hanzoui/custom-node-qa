use crate::models::{Environment, Export, Metadata, PackMetadata};
use anyhow::{Context, Result};
use chrono::Utc;
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn run(export_file: String, project: String) -> Result<()> {
    let repo_root = find_repo_root()?;
    let checklists_dir = repo_root.join("checklists");

    // Load export
    let export = Export::from_file(&export_file)
        .with_context(|| format!("Failed to load export file: {}", export_file))?;

    println!(
        "📦 Importing {} packs from {}",
        export.packs.len(),
        export_file.bold()
    );

    // Create project directory
    let project_dir = checklists_dir.join(&project);
    fs::create_dir_all(&project_dir)?;

    // Generate metadata.json
    let mut packs_metadata = HashMap::new();

    for pack in &export.packs {
        packs_metadata.insert(
            pack.name.clone(),
            PackMetadata {
                node_count: pack.node_count,
                tested: false,
                workflow_file: Some(format!("workflows/all-nodes-{}.json", pack.name)),
                notes: None,
                issues: None,
            },
        );
    }

    let metadata = Metadata {
        project_name: project.clone(),
        created_at: Utc::now(),
        last_updated: None,
        environment: Environment {
            env_type: "unknown".to_string(),
            url: export.environment.url.clone(),
            hanzo_studio_version: export.environment.hanzo_studio_version.clone(),
            frontend_version: None,
        },
        packs: packs_metadata,
        stats: None,
    };

    let metadata_path = project_dir.join("metadata.json");
    metadata.to_file(metadata_path)?;

    // Generate checklists
    let mut checklist_lines = vec!["# Node Pack QA Checklist".to_string(), String::new()];

    let mut sorted_packs = export.packs.clone();
    sorted_packs.sort_by(|a, b| a.name.cmp(&b.name));

    for pack in &sorted_packs {
        checklist_lines.push(format!("- [ ] {} ({})", pack.name, pack.node_count));
    }

    let checklist_md = checklist_lines.join("\n");
    fs::write(project_dir.join("checklist.md"), checklist_md)?;

    // Generate detailed checklist
    let mut detailed_lines = vec!["# Node Pack QA Checklist".to_string(), String::new()];

    for pack in &sorted_packs {
        detailed_lines.push(format!("## {}", pack.name));
        detailed_lines.push(String::new());

        for node in &pack.nodes {
            let name = node.display_name.as_ref().unwrap_or(&node.name);
            let deprecated = if node.deprecated.unwrap_or(false) {
                " ~~DEPRECATED~~"
            } else {
                ""
            };
            detailed_lines.push(format!("- [ ] {}{}", name, deprecated));
        }

        detailed_lines.push(String::new());
    }

    let checklist_detailed_md = detailed_lines.join("\n");
    fs::write(
        project_dir.join("checklist-detailed.md"),
        checklist_detailed_md,
    )?;

    println!("✅ Created project: {}", project.bold());
    println!("   📄 {}", project_dir.join("metadata.json").display());
    println!("   📄 {}", project_dir.join("checklist.md").display());
    println!(
        "   📄 {}",
        project_dir.join("checklist-detailed.md").display()
    );
    println!("\n💡 Next steps:");
    println!("   1. Test node packs in browser using QA.testPack()");
    println!("   2. Save workflow files to workflows/");
    println!("   3. Mark tested packs in checklist.md");
    println!("   4. Run: comfy-qa validate {}", project);

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
