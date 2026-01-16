use crate::validators::NamingValidator;
use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

pub fn run(project_name: String) -> Result<()> {
    if !NamingValidator::is_valid_project_name(&project_name) {
        anyhow::bail!("Invalid project name '{}': must be kebab-case (e.g., vue-node-cloud)", project_name);
    }

    let repo_root = find_repo_root()?;
    let checklists_dir = repo_root.join("checklists");
    let project_dir = checklists_dir.join(&project_name);

    if project_dir.exists() {
        anyhow::bail!("Project '{}' already exists", project_name);
    }

    // Create project directory
    fs::create_dir_all(&project_dir)?;

    // Copy templates
    let template_dir = checklists_dir.join("templates");
    if !template_dir.exists() {
        anyhow::bail!("Templates directory not found at {}", template_dir.display());
    }

    let checklist_template = template_dir.join("checklist.md");
    let checklist_detailed_template = template_dir.join("checklist-detailed.md");

    if checklist_template.exists() {
        fs::copy(&checklist_template, project_dir.join("checklist.md"))?;
    } else {
        // Create empty checklist
        fs::write(
            project_dir.join("checklist.md"),
            "# Node Pack QA Checklist\n\n",
        )?;
    }

    if checklist_detailed_template.exists() {
        fs::copy(
            &checklist_detailed_template,
            project_dir.join("checklist-detailed.md"),
        )?;
    } else {
        fs::write(
            project_dir.join("checklist-detailed.md"),
            "# Node Pack QA Checklist\n\n",
        )?;
    }

    // Create placeholder metadata.json
    let metadata_json = format!(
        r#"{{
  "project_name": "{}",
  "created_at": "{}",
  "environment": {{
    "type": "cloud",
    "url": "https://app.comfy.org"
  }},
  "packs": {{}}
}}
"#,
        project_name,
        chrono::Utc::now().to_rfc3339()
    );

    fs::write(project_dir.join("metadata.json"), metadata_json)?;

    println!("✅ Created new project: {}", project_name.bold());
    println!("   📁 {}", project_dir.display());
    println!("   📄 checklist.md");
    println!("   📄 checklist-detailed.md");
    println!("   📄 metadata.json");
    println!("\n💡 Next steps:");
    println!("   1. Open ComfyUI in browser");
    println!("   2. Run: QA.export('{}')", project_name);
    println!("   3. Run: comfy-qa import {}-export.json {}", project_name, project_name);

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
