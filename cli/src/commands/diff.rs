use crate::models::{Checklist, DetailedChecklist, Workflow};
use anyhow::{Context, Result};
use colored::Colorize;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub fn run(project: String, json: bool) -> Result<()> {
    let repo_root = find_repo_root()?;
    let checklists_dir = repo_root.join("checklists");
    let workflows_dir = repo_root.join("workflows");

    let project_dir = checklists_dir.join(&project);
    if !project_dir.exists() {
        anyhow::bail!("Project '{}' not found", project);
    }

    let checklist_path = project_dir.join("checklist.md");
    let checklist = Checklist::from_file(&checklist_path)
        .with_context(|| format!("Failed to load checklist for '{}'", project))?;

    let workflows = Workflow::load_all(&workflows_dir)?;

    let diff = calculate_diff(&checklist, &workflows);

    if json {
        print_json_diff(&project, &diff);
    } else {
        print_text_diff(&project, &diff);
    }

    Ok(())
}

struct DiffResult {
    matches: Vec<(String, usize)>,
    count_mismatches: Vec<(String, usize, usize)>,
    missing_workflows: Vec<(String, usize)>,
    new_packs: Vec<(String, usize)>,
    untested: Vec<(String, usize)>,
}

#[derive(Debug, Clone)]
pub struct NodeDiff {
    pub pack_name: String,
    pub missing_from_checklist: Vec<String>,  // In workflow but not in checklist
    pub extra_in_checklist: Vec<String>,      // In checklist but not in workflow
}

pub fn calculate_node_diff(
    pack_name: &str,
    workflow: &Workflow,
    detailed_checklist: &DetailedChecklist,
) -> NodeDiff {
    let workflow_nodes = workflow.get_unique_node_types();
    let checklist_nodes = detailed_checklist
        .get_nodes(pack_name)
        .cloned()
        .unwrap_or_default();

    let workflow_set: HashSet<_> = workflow_nodes.iter().collect();
    let checklist_set: HashSet<_> = checklist_nodes.iter().collect();

    let missing_from_checklist: Vec<String> = workflow_nodes
        .iter()
        .filter(|n| !checklist_set.contains(n))
        .cloned()
        .collect();

    let extra_in_checklist: Vec<String> = checklist_nodes
        .iter()
        .filter(|n| !workflow_set.contains(n))
        .cloned()
        .collect();

    NodeDiff {
        pack_name: pack_name.to_string(),
        missing_from_checklist,
        extra_in_checklist,
    }
}

fn calculate_diff(checklist: &Checklist, workflows: &HashMap<String, Workflow>) -> DiffResult {
    let mut result = DiffResult {
        matches: Vec::new(),
        count_mismatches: Vec::new(),
        missing_workflows: Vec::new(),
        new_packs: Vec::new(),
        untested: Vec::new(),
    };

    let checklist_packs: HashMap<_, _> = checklist
        .packs
        .iter()
        .map(|p| (p.name.clone(), p))
        .collect();

    let workflow_pack_names: HashSet<_> = workflows.keys().cloned().collect();

    // Check packs in checklist
    for pack in &checklist.packs {
        if let Some(workflow) = workflows.get(&pack.name) {
            if pack.node_count == workflow.node_count {
                if pack.tested {
                    result.matches.push((pack.name.clone(), pack.node_count));
                } else {
                    result
                        .untested
                        .push((pack.name.clone(), pack.node_count));
                }
            } else {
                result.count_mismatches.push((
                    pack.name.clone(),
                    pack.node_count,
                    workflow.node_count,
                ));
            }
        } else if pack.tested {
            result
                .missing_workflows
                .push((pack.name.clone(), pack.node_count));
        }
    }

    // Check for new packs in workflows
    for workflow_name in workflow_pack_names {
        if !checklist_packs.contains_key(&workflow_name) {
            if let Some(workflow) = workflows.get(&workflow_name) {
                result
                    .new_packs
                    .push((workflow.pack_name.clone(), workflow.node_count));
            }
        }
    }

    result
}

fn print_text_diff(project: &str, diff: &DiffResult) {
    println!("📊 Comparing {} checklist vs workflow files...\n", project.bold());

    if !diff.matches.is_empty() {
        println!("{} ({}):", "✅ Matches".green(), diff.matches.len());
        for (pack, count) in &diff.matches {
            println!("   ✓ {} ({} nodes)", pack, count);
        }
        println!();
    }

    if !diff.count_mismatches.is_empty() {
        println!(
            "{} ({}):",
            "⚠️  Count Mismatches".yellow(),
            diff.count_mismatches.len()
        );
        for (pack, checklist_count, workflow_count) in &diff.count_mismatches {
            let delta = *workflow_count as i64 - *checklist_count as i64;
            let sign = if delta > 0 { "+" } else { "" };
            println!(
                "   • {}: checklist has {}, workflow has {} ({}{})",
                pack, checklist_count, workflow_count, sign, delta
            );
        }
        println!();
    }

    if !diff.missing_workflows.is_empty() {
        println!(
            "{} ({}):",
            "❌ Missing Workflows".red(),
            diff.missing_workflows.len()
        );
        for (pack, count) in &diff.missing_workflows {
            println!("   • {} ({} nodes)", pack, count);
        }
        println!();
    }

    if !diff.new_packs.is_empty() {
        println!(
            "{} ({}):",
            "🆕 New Packs Not in Checklist".cyan(),
            diff.new_packs.len()
        );
        for (pack, count) in &diff.new_packs {
            println!("   • {} ({} nodes)", pack, count);
        }
        println!();
    }

    if !diff.untested.is_empty() {
        println!("{} ({}):", "⏳ Untested".yellow(), diff.untested.len());
        for (pack, count) in &diff.untested {
            println!("   • {} ({} nodes)", pack, count);
        }
        println!();
    }

    // Summary
    let total_packs = diff.matches.len() + diff.count_mismatches.len() + diff.untested.len();
    let completion = if total_packs > 0 {
        (diff.matches.len() as f64 / total_packs as f64) * 100.0
    } else {
        0.0
    };

    println!("📈 Summary:");
    println!("   Completion: {:.0}% ({}/{})", completion, diff.matches.len(), total_packs);
    println!("   Drift: {} count mismatches, {} new packs", diff.count_mismatches.len(), diff.new_packs.len());

    if !diff.count_mismatches.is_empty() || !diff.new_packs.is_empty() {
        println!("\n💡 Run {} to update", format!("comfy-qa sync {}", project).cyan());
    }
}

fn print_json_diff(project: &str, diff: &DiffResult) {
    let json_output = json!({
        "project": project,
        "summary": {
            "matches": diff.matches.len(),
            "count_mismatches": diff.count_mismatches.len(),
            "missing_workflows": diff.missing_workflows.len(),
            "new_packs": diff.new_packs.len(),
            "untested": diff.untested.len(),
        },
        "details": {
            "matches": diff.matches.iter().map(|(name, count)| {
                json!({"pack": name, "node_count": count})
            }).collect::<Vec<_>>(),
            "count_mismatches": diff.count_mismatches.iter().map(|(name, checklist, workflow)| {
                json!({
                    "pack": name,
                    "checklist_count": checklist,
                    "workflow_count": workflow,
                    "delta": *workflow as i64 - *checklist as i64
                })
            }).collect::<Vec<_>>(),
            "missing_workflows": diff.missing_workflows.iter().map(|(name, count)| {
                json!({"pack": name, "node_count": count})
            }).collect::<Vec<_>>(),
            "new_packs": diff.new_packs.iter().map(|(name, count)| {
                json!({"pack": name, "node_count": count})
            }).collect::<Vec<_>>(),
            "untested": diff.untested.iter().map(|(name, count)| {
                json!({"pack": name, "node_count": count})
            }).collect::<Vec<_>>(),
        }
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

    anyhow::bail!("Could not find repository root")
}
