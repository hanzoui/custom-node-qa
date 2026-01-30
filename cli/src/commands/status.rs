use crate::generators::ReportGenerator;
use crate::models::{Checklist, Metadata};
use anyhow::{Context, Result};
use colored::Colorize;
use comfy_table::{presets::UTF8_FULL, Attribute, Cell, Color as TableColor, Table};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn run(project: Option<String>, format: String, _all: bool) -> Result<()> {
    let repo_root = find_repo_root()?;
    let checklists_dir = repo_root.join("checklists");

    if let Some(project_name) = project {
        show_single_project(&checklists_dir, &project_name, &format)
    } else {
        show_all_projects(&checklists_dir, &format)
    }
}

fn show_single_project(checklists_dir: &Path, project_name: &str, format: &str) -> Result<()> {
    let project_dir = checklists_dir.join(project_name);

    if !project_dir.exists() {
        anyhow::bail!("Project '{}' not found", project_name);
    }

    let checklist_path = project_dir.join("checklist.md");
    let metadata_path = project_dir.join("metadata.json");

    let checklist = Checklist::from_file(checklist_path)
        .with_context(|| format!("Failed to load checklist for '{}'", project_name))?;

    let metadata = if metadata_path.exists() {
        Some(Metadata::from_file(&metadata_path)?)
    } else {
        None
    };

    match format {
        "json" => {
            let json_output = generate_json_status(&checklist, metadata.as_ref());
            println!("{}", serde_json::to_string_pretty(&json_output)?);
        }
        "html" => {
            let html = ReportGenerator::generate_html(&checklist, metadata.as_ref());
            let output_path = project_dir.join("report.html");
            fs::write(&output_path, html)?;
            println!("✅ Generated HTML report: {}", output_path.display());
        }
        _ => {
            let report = ReportGenerator::generate_text(&checklist, metadata.as_ref());
            println!("{}", report);
        }
    }

    Ok(())
}

fn show_all_projects(checklists_dir: &PathBuf, format: &str) -> Result<()> {
    let mut projects = Vec::new();

    for entry in WalkDir::new(checklists_dir)
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
        return Ok(());
    }

    if format == "json" {
        let mut all_statuses = Vec::new();
        for project_name in &projects {
            let project_dir = checklists_dir.join(project_name);
            let checklist_path = project_dir.join("checklist.md");
            let metadata_path = project_dir.join("metadata.json");

            let checklist = Checklist::from_file(&checklist_path).ok();
            let metadata = if metadata_path.exists() {
                Metadata::from_file(&metadata_path).ok()
            } else {
                None
            };

            if let Some(checklist) = checklist {
                all_statuses.push(generate_json_status(&checklist, metadata.as_ref()));
            }
        }

        println!("{}", serde_json::to_string_pretty(&all_statuses)?);
    } else {
        println!("\n{}\n", "📊 All Projects".bold());

        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.set_header(vec![
            Cell::new("Project")
                .add_attribute(Attribute::Bold)
                .fg(TableColor::Cyan),
            Cell::new("Tested")
                .add_attribute(Attribute::Bold)
                .fg(TableColor::Cyan),
            Cell::new("Total")
                .add_attribute(Attribute::Bold)
                .fg(TableColor::Cyan),
            Cell::new("Progress")
                .add_attribute(Attribute::Bold)
                .fg(TableColor::Cyan),
            Cell::new("ComfyUI Version")
                .add_attribute(Attribute::Bold)
                .fg(TableColor::Cyan),
        ]);

        for project_name in &projects {
            let project_dir = checklists_dir.join(project_name);
            let checklist_path = project_dir.join("checklist.md");
            let metadata_path = project_dir.join("metadata.json");

            let checklist = match Checklist::from_file(&checklist_path) {
                Ok(c) => c,
                Err(_) => {
                    table.add_row(vec![
                        Cell::new(project_name).fg(TableColor::Yellow),
                        Cell::new("Error").fg(TableColor::Red),
                        Cell::new("-"),
                        Cell::new("-"),
                        Cell::new("-"),
                    ]);
                    continue;
                }
            };

            let metadata = if metadata_path.exists() {
                Metadata::from_file(&metadata_path).ok()
            } else {
                None
            };

            let tested = checklist.packs.iter().filter(|p| p.tested).count();
            let total = checklist.packs.len();
            let percent = if total > 0 {
                (tested as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            let progress_bar = create_mini_progress_bar(percent);
            let progress_cell = if percent == 100.0 {
                Cell::new(&progress_bar).fg(TableColor::Green)
            } else if percent >= 50.0 {
                Cell::new(&progress_bar).fg(TableColor::Yellow)
            } else {
                Cell::new(&progress_bar).fg(TableColor::Red)
            };

            let version_str = metadata
                .as_ref()
                .and_then(|m| m.environment.comfyui_version.as_deref())
                .unwrap_or("-");

            table.add_row(vec![
                Cell::new(project_name),
                Cell::new(tested.to_string()).fg(if tested == total {
                    TableColor::Green
                } else {
                    TableColor::White
                }),
                Cell::new(total.to_string()),
                progress_cell,
                Cell::new(version_str).fg(TableColor::DarkGrey),
            ]);
        }

        println!("{}", table);
        println!(
            "\n💡 Run {} to see details\n",
            "comfy-qa status <project>".cyan()
        );
    }

    Ok(())
}

fn generate_json_status(checklist: &Checklist, metadata: Option<&Metadata>) -> serde_json::Value {
    let tested = checklist.packs.iter().filter(|p| p.tested).count();
    let total = checklist.packs.len();
    let percent = if total > 0 {
        (tested as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    serde_json::json!({
        "project": checklist.project_name,
        "stats": {
            "total_packs": total,
            "tested_packs": tested,
            "completion_percent": percent,
        },
        "environment": metadata.map(|m| serde_json::json!({
            "type": m.environment.env_type,
            "url": m.environment.url,
            "comfyui_version": m.environment.comfyui_version,
        })),
        "packs": checklist.packs.iter().map(|p| serde_json::json!({
            "name": p.name,
            "node_count": p.node_count,
            "tested": p.tested,
        })).collect::<Vec<_>>()
    })
}

fn create_mini_progress_bar(percent: f64) -> String {
    let bar_width = 15;
    let filled = (bar_width as f64 * percent / 100.0) as usize;
    format!(
        "[{}{}] {:.0}%",
        "█".repeat(filled),
        "░".repeat(bar_width - filled),
        percent
    )
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
