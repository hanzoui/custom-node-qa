use crate::models::{Checklist, Workflow};
use anyhow::Result;
use console::style;
use dialoguer::Select;
use std::path::PathBuf;
use std::process::Command;

pub fn run(project: String) -> Result<()> {
    let repo_root = find_repo_root()?;
    let checklists_dir = repo_root.join("checklists");
    let workflows_dir = repo_root.join("workflows");
    let project_dir = checklists_dir.join(&project);

    if !project_dir.exists() {
        println!();
        println!("{} Project '{}' not found", style("✗").red(), project);
        println!();
        println!("Run {} to create it", style("comfy-qa setup").cyan());
        return Ok(());
    }

    let checklist_path = project_dir.join("checklist.md");
    let checklist = Checklist::from_file(checklist_path)?;
    let workflows = Workflow::load_all(workflows_dir)?;

    println!();
    println!("{}", style(format!("Testing Progress: {}", project)).bold());
    println!("{}", style("─".repeat(50)).dim());
    println!();

    let total = checklist.packs.len();
    let tested = checklist.packs.iter().filter(|p| p.tested).count();
    let percent = if total > 0 {
        (tested as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    // Progress bar
    let bar_width = 30;
    let filled = (bar_width as f64 * percent / 100.0) as usize;
    let bar = format!(
        "[{}{}] {:.0}%",
        "█".repeat(filled),
        "░".repeat(bar_width - filled),
        percent
    );

    println!("  {}", style(bar).cyan());
    println!();
    println!("  Tested: {} out of {} packs", style(tested).green(), total);
    println!();

    // Show what needs testing
    let untested: Vec<_> = checklist.packs.iter().filter(|p| !p.tested).collect();

    if !untested.is_empty() {
        println!("{}", style("Packs to test:").bold());
        println!();

        let show_all = untested.len() <= 10;

        if show_all {
            for (i, pack) in untested.iter().enumerate() {
                println!("  {}. {} ({} nodes)", i + 1, pack.name, pack.node_count);
            }
        } else {
            for (i, pack) in untested.iter().take(10).enumerate() {
                println!("  {}. {} ({} nodes)", i + 1, pack.name, pack.node_count);
            }
            println!("  ... and {} more", untested.len() - 10);
            println!();

            let options = vec!["Show all packs", "Continue"];
            let selection = Select::new()
                .with_prompt("What would you like to do?")
                .items(&options)
                .default(0)
                .interact()?;

            if selection == 0 {
                println!();
                println!("{}", style("All packs to test:").bold());
                println!();
                for (i, pack) in untested.iter().enumerate() {
                    println!("  {}. {} ({} nodes)", i + 1, pack.name, pack.node_count);
                }
            }
        }

        println!();
        println!("{}", style("To test a pack:").dim());
        println!(
            "  1. In browser console: {}",
            style(format!("await QA.testPack('{}')", untested[0].name)).yellow()
        );
        println!("  2. Save the downloaded file to the 'workflows' folder");
        println!("  3. Mark it as tested in your checklist");
    } else {
        println!("{} All packs tested!", style("✓").green());
    }

    // Check for issues
    let mut issues = Vec::new();
    for pack in &checklist.packs {
        if let Some(workflow) = workflows.get(&pack.name) {
            if pack.node_count != workflow.node_count {
                issues.push(format!(
                    "{}: count mismatch (checklist: {}, workflow: {})",
                    pack.name, pack.node_count, workflow.node_count
                ));
            }
        } else if pack.tested {
            issues.push(format!("{}: workflow file missing", pack.name));
        }
    }

    if !issues.is_empty() {
        println!();
        println!("{}", style("Issues found:").yellow());
        println!();

        let show_all = issues.len() <= 5;

        if show_all {
            // Show all if 5 or fewer
            for issue in &issues {
                println!("  {} {}", style("!").yellow(), issue);
            }
        } else {
            // Show first 5 with option to see all
            for issue in issues.iter().take(5) {
                println!("  {} {}", style("!").yellow(), issue);
            }
            println!("  ... and {} more", issues.len() - 5);
            println!();

            let options = vec!["Show all issues", "Continue"];
            let selection = Select::new()
                .with_prompt("What would you like to do?")
                .items(&options)
                .default(0)
                .interact()?;

            if selection == 0 {
                println!();
                println!("{}", style("All issues:").yellow());
                println!();
                for (i, issue) in issues.iter().enumerate() {
                    println!("  {}. {} {}", i + 1, style("!").yellow(), issue);
                }
            }
        }

        // Offer actionable next steps
        println!();
        println!("{}", style("What would you like to do?").bold());

        let next_actions = vec![
            "See detailed diff (shows exact node count changes)",
            "Fix automatically (sync workflow data to checklist)",
            "Back to dashboard",
        ];

        let action = Select::new().items(&next_actions).default(0).interact()?;

        match action {
            0 => {
                // Run diff command
                println!();
                let status = Command::new(std::env::current_exe()?)
                    .arg("diff")
                    .arg(&project)
                    .status()?;

                if !status.success() {
                    println!("{}", style("Failed to run diff").red());
                }
                println!();
                pause();
            }
            1 => {
                // Run sync command
                println!();
                println!("{}", style("Running sync...").cyan());
                println!();
                let status = Command::new(std::env::current_exe()?)
                    .arg("sync")
                    .arg(&project)
                    .status()?;

                if !status.success() {
                    println!("{}", style("Failed to run sync").red());
                }
                println!();
                pause();
            }
            2 => {
                // Continue - do nothing
            }
            _ => {}
        }
    }

    println!();

    Ok(())
}

fn pause() {
    println!("Press Enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}

fn find_repo_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;

    for ancestor in current_dir.ancestors() {
        if ancestor.join("checklists").exists() {
            return Ok(ancestor.to_path_buf());
        }
    }

    anyhow::bail!("Could not find repository root")
}
