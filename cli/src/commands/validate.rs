use crate::models::{Checklist, Metadata, Workflow};
use crate::validators::{Severity, Validator};
use anyhow::{Context, Result};
use colored::Colorize;
use serde_json::json;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn run(project: Option<String>, json: bool, fix: bool, verbose: bool) -> Result<()> {
    let repo_root = find_repo_root()?;
    let checklists_dir = repo_root.join("checklists");
    let workflows_dir = repo_root.join("workflows");

    if let Some(project_name) = project {
        validate_single_project(&checklists_dir, &workflows_dir, &project_name, json, fix, verbose)
    } else {
        validate_all_projects(&checklists_dir, &workflows_dir, json, fix, verbose)
    }
}

fn validate_single_project(
    checklists_dir: &PathBuf,
    workflows_dir: &PathBuf,
    project_name: &str,
    json: bool,
    fix: bool,
    verbose: bool,
) -> Result<()> {
    let project_dir = checklists_dir.join(project_name);

    if !project_dir.exists() {
        anyhow::bail!("Project '{}' not found", project_name);
    }

    let checklist_path = project_dir.join("checklist.md");
    let metadata_path = project_dir.join("metadata.json");

    let checklist = Checklist::from_file(&checklist_path)
        .with_context(|| format!("Failed to load checklist for '{}'", project_name))?;

    let metadata = if metadata_path.exists() {
        Some(Metadata::from_file(&metadata_path)?)
    } else {
        None
    };

    let workflows = Workflow::load_all(workflows_dir)?;

    let results = Validator::validate_project(&checklist, &workflows, metadata.as_ref());

    if json {
        print_json_results(project_name, &results);
    } else {
        print_text_results(project_name, &results, verbose);
    }

    let has_errors = results.iter().any(|r| r.severity == Severity::Error);

    if fix && has_errors {
        println!("\n{}", "🔧 Auto-fix not yet implemented".yellow());
    }

    if has_errors {
        anyhow::bail!("Validation failed with errors");
    }

    Ok(())
}

fn validate_all_projects(
    checklists_dir: &PathBuf,
    workflows_dir: &PathBuf,
    json: bool,
    _fix: bool,
    verbose: bool,
) -> Result<()> {
    let mut all_results = Vec::new();
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

    let workflows = Workflow::load_all(workflows_dir)?;

    for project_name in &projects {
        let project_dir = checklists_dir.join(project_name);
        let checklist_path = project_dir.join("checklist.md");
        let metadata_path = project_dir.join("metadata.json");

        let checklist = match Checklist::from_file(&checklist_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("⚠️  Skipping {}: {}", project_name, e);
                continue;
            }
        };

        let metadata = if metadata_path.exists() {
            Metadata::from_file(&metadata_path).ok()
        } else {
            None
        };

        let results = Validator::validate_project(&checklist, &workflows, metadata.as_ref());
        all_results.push((project_name.clone(), results));
    }

    if json {
        let json_output = json!({
            "projects": all_results.iter().map(|(name, results)| {
                json!({
                    "name": name,
                    "errors": results.iter().filter(|r| r.severity == Severity::Error).count(),
                    "warnings": results.iter().filter(|r| r.severity == Severity::Warning).count(),
                    "results": results.iter().map(|r| {
                        json!({
                            "severity": match r.severity {
                                Severity::Error => "error",
                                Severity::Warning => "warning",
                            },
                            "message": r.message,
                            "pack": r.pack,
                        })
                    }).collect::<Vec<_>>()
                })
            }).collect::<Vec<_>>()
        });
        println!("{}", serde_json::to_string_pretty(&json_output)?);
    } else {
        for (project_name, results) in &all_results {
            print_text_results(project_name, results, verbose);
            println!();
        }

        let total_errors: usize = all_results
            .iter()
            .map(|(_, r)| r.iter().filter(|res| res.severity == Severity::Error).count())
            .sum();

        let total_warnings: usize = all_results
            .iter()
            .map(|(_, r)| {
                r.iter()
                    .filter(|res| res.severity == Severity::Warning)
                    .count()
            })
            .sum();

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!(
            "📊 Summary: {} projects validated",
            all_results.len().to_string().bold()
        );
        println!("   Errors: {}", total_errors.to_string().red());
        println!("   Warnings: {}", total_warnings.to_string().yellow());
    }

    let has_errors = all_results
        .iter()
        .any(|(_, results)| results.iter().any(|r| r.severity == Severity::Error));

    if has_errors {
        anyhow::bail!("Validation failed with errors");
    }

    Ok(())
}

fn print_text_results(project_name: &str, results: &[crate::validators::ValidationResult], verbose: bool) {
    println!("📋 Validating: {}", project_name.bold());

    let errors: Vec<_> = results
        .iter()
        .filter(|r| r.severity == Severity::Error)
        .collect();
    let warnings: Vec<_> = results
        .iter()
        .filter(|r| r.severity == Severity::Warning)
        .collect();

    if errors.is_empty() && warnings.is_empty() {
        println!("   {}", "✅ All checks passed".green());
        return;
    }

    if !errors.is_empty() {
        println!("   {} errors:", errors.len().to_string().red());
        for result in errors {
            if verbose {
                result.print();
            } else {
                println!("      {}", result.message);
            }
        }
    }

    if !warnings.is_empty() {
        println!("   {} warnings:", warnings.len().to_string().yellow());
        for result in warnings {
            if verbose {
                result.print();
            } else {
                println!("      {}", result.message);
            }
        }
    }
}

fn print_json_results(project_name: &str, results: &[crate::validators::ValidationResult]) {
    let json_output = json!({
        "project": project_name,
        "errors": results.iter().filter(|r| r.severity == Severity::Error).count(),
        "warnings": results.iter().filter(|r| r.severity == Severity::Warning).count(),
        "results": results.iter().map(|r| {
            json!({
                "severity": match r.severity {
                    Severity::Error => "error",
                    Severity::Warning => "warning",
                },
                "message": r.message,
                "pack": r.pack,
            })
        }).collect::<Vec<_>>()
    });

    println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
}

fn find_repo_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;

    for ancestor in current_dir.ancestors() {
        if ancestor.join(".git").exists() || ancestor.join("checklists").exists() {
            return Ok(ancestor.to_path_buf());
        }
    }

    anyhow::bail!("Could not find repository root (looking for .git or checklists/ directory)")
}
