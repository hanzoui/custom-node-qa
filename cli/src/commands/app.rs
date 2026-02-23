#![allow(clippy::ptr_arg)]
#![allow(unused_assignments)]

use anyhow::Result;
use console::style;
use dialoguer::{Confirm, Input, Select};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

const BROWSER_SCRIPT: &str = include_str!("../../scripts/devtools-node-pack-tester.js");

/// Main application entry point - intelligently decides what to show
pub fn run() -> Result<()> {
    let repo_root = find_repo_root()?;
    let checklists_dir = repo_root.join("checklists");

    // Check if any projects exist
    let projects = get_existing_projects(&checklists_dir)?;

    if projects.is_empty() {
        // First-time setup
        return run_first_time_setup();
    }

    // Has projects - show dashboard menu
    run_dashboard_menu(None)
}

fn run_first_time_setup() -> Result<()> {
    clear_screen();

    println!(
        "\n{}",
        style("Welcome to Hanzo Studio Node Testing").bold().cyan()
    );
    println!("{}", style("─".repeat(50)).dim());
    println!();
    println!("Let's set up your first testing project.");
    println!();

    let start = Confirm::new()
        .with_prompt("Ready to begin?")
        .default(true)
        .interact()?;

    if !start {
        return Ok(());
    }

    // Create project
    let project_name = create_new_project()?;

    // Go to dashboard
    run_dashboard_menu(Some(project_name))
}

fn run_dashboard_menu(initial_project: Option<String>) -> Result<()> {
    let mut current_project = initial_project;

    loop {
        clear_screen();

        // Get or select project
        let project_name = match &current_project {
            Some(p) => p.clone(),
            None => match select_project()? {
                Some(p) => {
                    current_project = Some(p.clone());
                    p
                }
                None => {
                    println!("\nNo projects found.");
                    return Ok(());
                }
            },
        };

        // Show dashboard
        show_dashboard(&project_name)?;

        // Check if git is available
        let git_available = crate::git::check_git_installed() && crate::git::check_in_repo();

        // Main menu
        let mut options = vec![
            "Check testing progress",
            "Get browser script",
            "View all packs",
            "Compare with repo",
            "Run validation",
            "Sync checklists",
            "Generate workflow from search",
            "Generate API test script",
        ];

        if git_available {
            options.push("");
            options.push("🔄 Pull latest from team");
            options.push("📝 Review my changes");
            options.push("📤 Commit and share my work");
        }

        options.extend_from_slice(&[
            "",
            "📖 Help - How does this work?",
            "Switch project",
            "Cleanup projects",
            "Exit",
        ]);

        println!();
        let choice = Select::new()
            .with_prompt("What would you like to do?")
            .items(&options)
            .default(0)
            .interact()?;

        // Handle menu selection
        let selected_option = options[choice];

        match selected_option {
            "Check testing progress" => {
                crate::commands::check::run(project_name.clone())?;
                pause();
            }
            "Get browser script" => {
                show_browser_script(&project_name)?;
                pause();
            }
            "View all packs" => {
                show_pack_list(&project_name)?;
                pause();
            }
            "Compare with repo" => {
                run_diff_interactive(&project_name)?;
            }
            "Run validation" => {
                run_validate_interactive(&project_name)?;
            }
            "Sync checklists" => {
                run_sync_interactive(&project_name)?;
            }
            "Generate workflow from search" => {
                crate::commands::generate::run(None)?;
            }
            "Generate API test script" => {
                crate::commands::generate_api_test::run(Some(project_name.clone()))?;
            }
            "🔄 Pull latest from team" => {
                crate::git::git_pull()?;
                pause();
            }
            "📝 Review my changes" => {
                crate::git::git_status()?;
                pause();
            }
            "📤 Commit and share my work" => {
                crate::git::git_commit_and_push()?;
                pause();
            }
            "📖 Help - How does this work?" => {
                show_help_guide()?;
            }
            "Switch project" => {
                current_project = None;
            }
            "Cleanup projects" => {
                run_cleanup()?;
                pause();
            }
            "Exit" => break,
            _ => {}
        }
    }

    Ok(())
}

fn create_new_project() -> Result<String> {
    clear_screen();

    println!("\n{}", style("Create Testing Project").bold().cyan());
    println!("{}", style("─".repeat(50)).dim());
    println!();

    let repo_root = find_repo_root()?;
    let checklists_dir = repo_root.join("checklists");

    // Show existing if any
    let existing = get_existing_projects(&checklists_dir)?;
    if !existing.is_empty() {
        println!("Existing projects:");
        for proj in &existing {
            println!("  {}", style(proj).dim());
        }
        println!();
    }

    println!("Project name (lowercase, dashes only)");
    println!("Example: vue-nodes-cloud\n");

    let project_name: String = Input::new()
        .with_prompt("Name")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.is_empty() {
                return Err("Name cannot be empty");
            }
            if !input
                .chars()
                .all(|c| c.is_ascii_lowercase() || c == '-' || c.is_numeric())
            {
                return Err("Use only lowercase, numbers, and dashes");
            }
            Ok(())
        })
        .interact_text()?;

    let project_dir = checklists_dir.join(&project_name);

    if project_dir.exists() {
        println!("\n{} Project exists, using it", style("✓").green());
    } else {
        fs::create_dir_all(&project_dir)?;

        let metadata = format!(
            r#"{{
  "project_name": "{}",
  "created_at": "{}",
  "environment": {{
    "type": "cloud",
    "url": "https://app.hanzo.ai"
  }},
  "packs": {{}}
}}
"#,
            project_name,
            chrono::Utc::now().to_rfc3339()
        );

        fs::write(project_dir.join("metadata.json"), metadata)?;
        fs::write(
            project_dir.join("checklist.md"),
            "# Node Pack Testing Checklist\n\n",
        )?;
        fs::write(
            project_dir.join("checklist-detailed.md"),
            "# Node Pack Testing Checklist\n\n",
        )?;

        println!("\n{} Project created", style("✓").green());
    }

    pause();

    Ok(project_name)
}

fn show_dashboard(project: &str) -> Result<()> {
    println!("\n{}", style("Hanzo Studio Node Testing").bold().cyan());
    println!("{}", style("─".repeat(70)).dim());
    println!();
    println!("  Project: {}", style(project).bold());

    let repo_root = find_repo_root()?;
    let project_dir = repo_root.join("checklists").join(project);

    if let Ok(checklist) = crate::models::Checklist::from_file(project_dir.join("checklist.md")) {
        let total = checklist.packs.len();
        let tested = checklist.packs.iter().filter(|p| p.tested).count();

        if total > 0 {
            let percent = (tested as f64 / total as f64) * 100.0;
            println!(
                "  Progress: {} / {} ({:.0}%)",
                style(tested).green(),
                total,
                percent
            );

            let bar_width: usize = 40;
            let filled = (bar_width as f64 * percent / 100.0) as usize;
            let empty = bar_width.saturating_sub(filled);
            println!(
                "  [{}{}]",
                style("█".repeat(filled)).cyan(),
                style("░".repeat(empty)).dim()
            );
        } else {
            println!("  {}", style("No packs yet").dim());
        }
    }

    // Show git status if available
    if let Some(status) = crate::git::get_repo_status_summary() {
        println!("  Git: {}", style(status).dim());
    }

    println!();
    println!("{}", style("Links:").dim());
    println!(
        "  {} {}",
        style("→").dim(),
        style("github.com/hanzoui/hanzo-studio-custom-node-qa").dim()
    );

    Ok(())
}

fn show_browser_script(project: &str) -> Result<()> {
    clear_screen();

    println!("\n{}", style("Browser Script").bold().cyan());
    println!("{}", style("─".repeat(70)).dim());
    println!();

    let options = vec!["Print it here", "Save to scripts/ folder"];
    let choice = Select::new()
        .with_prompt("How would you like the script?")
        .items(&options)
        .default(0)
        .interact()?;

    if choice == 1 {
        let repo_root = find_repo_root()?;
        let script_path = repo_root
            .join("scripts")
            .join(format!("browser-script-{}.js", project));
        fs::write(&script_path, BROWSER_SCRIPT)?;
        println!();
        println!("{} Saved to:", style("✓").green());
        println!("  {}", style(script_path.display()).yellow());
        println!();
        println!("Open the file and copy all text into browser console.");
    } else {
        println!();
        println!("{}", style("─".repeat(70)).yellow());
        println!("{}", BROWSER_SCRIPT);
        println!("{}", style("─".repeat(70)).yellow());
        println!();
        println!("{}", style("Instructions:").bold());
        println!("  1. Copy all text above");
        println!("  2. Open Hanzo Studio in browser");
        println!("  3. Press F12 (Developer Tools)");
        println!("  4. Click 'Console' tab");
        println!("  5. Paste and press Enter");
        println!();
        println!("{}", style("Common commands:").bold());
        println!("  await QA.listPacks()");
        println!("  await QA.testPack('pack-name')");
        println!("  await QA.export('{}')", project);
    }

    println!();
    Ok(())
}

fn show_pack_list(project: &str) -> Result<()> {
    let repo_root = find_repo_root()?;
    let project_dir = repo_root.join("checklists").join(project);
    let workflows_dir = repo_root.join("workflows");

    loop {
        clear_screen();

        println!("\n{}", style("Node Packs").bold().cyan());
        println!("{}", style("─".repeat(70)).dim());
        println!();

        let checklist = match crate::models::Checklist::from_file(project_dir.join("checklist.md"))
        {
            Ok(c) => c,
            Err(e) => {
                println!("{} Failed to load checklist: {}", style("✗").red(), e);
                pause();
                return Ok(());
            }
        };

        let workflows = crate::models::Workflow::load_all(&workflows_dir).unwrap_or_default();

        // Categorize packs
        let mut tested_ok = Vec::new();
        let mut tested_issues = Vec::new();
        let mut untested = Vec::new();

        for pack in &checklist.packs {
            if pack.tested {
                if let Some(workflow) = workflows.get(&pack.name) {
                    if pack.node_count == workflow.node_count {
                        tested_ok.push(pack);
                    } else {
                        tested_issues.push((pack, workflow.node_count));
                    }
                } else {
                    tested_issues.push((pack, 0));
                }
            } else {
                untested.push(pack);
            }
        }

        println!("{}: {}", style("✓ Tested & OK").green(), tested_ok.len());
        println!(
            "{}: {}",
            style("⚠ Tested with Issues").yellow(),
            tested_issues.len()
        );
        println!("{}: {}", style("○ Not Tested").dim(), untested.len());
        println!();

        let options = vec![
            format!("View all tested packs ({})", tested_ok.len()),
            format!("View packs with issues ({})", tested_issues.len()),
            format!("View untested packs ({})", untested.len()),
            "Search for a pack".to_string(),
            "Back to dashboard".to_string(),
        ];

        let selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(&options)
            .default(0)
            .interact()?;

        match selection {
            0 => show_pack_category(project, "Tested & OK", &tested_ok, &workflows)?,
            1 => show_packs_with_issues(project, &tested_issues)?,
            2 => show_pack_category(project, "Not Tested", &untested, &workflows)?,
            3 => {
                println!();
                let search = dialoguer::Input::<String>::new()
                    .with_prompt("Search for pack name")
                    .interact_text()?;

                let matches: Vec<_> = checklist
                    .packs
                    .iter()
                    .filter(|p| p.name.to_lowercase().contains(&search.to_lowercase()))
                    .collect();

                if matches.is_empty() {
                    println!();
                    println!("{} No packs found matching '{}'", style("✗").red(), search);
                    pause();
                } else {
                    show_pack_category(
                        project,
                        &format!("Search: {}", search),
                        &matches,
                        &workflows,
                    )?;
                }
            }
            4 => break,
            _ => break,
        }
    }

    Ok(())
}

fn show_pack_category(
    project: &str,
    category: &str,
    packs: &[&crate::models::NodePack],
    workflows: &std::collections::HashMap<String, crate::models::Workflow>,
) -> Result<()> {
    if packs.is_empty() {
        clear_screen();
        println!("\n{}", style(format!("{} (0)", category)).bold());
        println!();
        println!("No packs in this category.");
        println!();
        pause();
        return Ok(());
    }

    loop {
        clear_screen();
        println!(
            "\n{}",
            style(format!("{} ({})", category, packs.len())).bold()
        );
        println!("{}", style("─".repeat(70)).dim());
        println!();

        let mut items: Vec<String> = packs
            .iter()
            .map(|p| {
                let status = if p.tested {
                    style("✓").green().to_string()
                } else {
                    style("○").dim().to_string()
                };
                format!("{} {} ({} nodes)", status, p.name, p.node_count)
            })
            .collect();

        items.push(style("← Back").dim().to_string());

        let selection = Select::new()
            .with_prompt("Select a pack for details")
            .items(&items)
            .default(0)
            .interact()?;

        if selection >= packs.len() {
            break;
        }

        show_pack_details(project, packs[selection], workflows)?;
    }

    Ok(())
}

fn show_packs_with_issues(
    project: &str,
    packs_with_issues: &[(&crate::models::NodePack, usize)],
) -> Result<()> {
    if packs_with_issues.is_empty() {
        clear_screen();
        println!("\n{}", style("Packs with Issues (0)").bold());
        println!();
        println!("{} No issues found!", style("✓").green());
        println!();
        pause();
        return Ok(());
    }

    loop {
        clear_screen();
        println!(
            "\n{}",
            style(format!("Packs with Issues ({})", packs_with_issues.len())).bold()
        );
        println!("{}", style("─".repeat(70)).dim());
        println!();

        let mut items: Vec<String> = packs_with_issues
            .iter()
            .map(|(pack, workflow_count)| {
                if *workflow_count == 0 {
                    format!(
                        "{} {} - missing workflow file",
                        style("!").yellow(),
                        pack.name
                    )
                } else {
                    let delta = *workflow_count as i64 - pack.node_count as i64;
                    let sign = if delta > 0 { "+" } else { "" };
                    format!(
                        "{} {} - count mismatch (checklist: {}, workflow: {}, {}{})",
                        style("!").yellow(),
                        pack.name,
                        pack.node_count,
                        workflow_count,
                        sign,
                        delta
                    )
                }
            })
            .collect();

        items.push(style("← Back").dim().to_string());

        let selection = Select::new()
            .with_prompt("Select a pack for details")
            .items(&items)
            .default(0)
            .interact()?;

        if selection >= packs_with_issues.len() {
            break;
        }

        let (pack, _) = packs_with_issues[selection];
        let workflows_dir = find_repo_root()?.join("workflows");
        let workflows = crate::models::Workflow::load_all(&workflows_dir).unwrap_or_default();
        show_pack_details(project, pack, &workflows)?;
    }

    Ok(())
}

fn show_pack_details(
    _project: &str,
    pack: &crate::models::NodePack,
    workflows: &std::collections::HashMap<String, crate::models::Workflow>,
) -> Result<()> {
    clear_screen();

    println!("\n{}", style(&pack.name).bold().cyan());
    println!("{}", style("─".repeat(70)).dim());
    println!();

    println!(
        "Status: {}",
        if pack.tested {
            style("✓ Tested").green()
        } else {
            style("○ Not Tested").yellow()
        }
    );

    println!("Checklist node count: {}", pack.node_count);

    if let Some(workflow) = workflows.get(&pack.name) {
        println!("Workflow node count: {}", workflow.node_count);

        if pack.node_count != workflow.node_count {
            let delta = workflow.node_count as i64 - pack.node_count as i64;
            let sign = if delta > 0 { "+" } else { "" };
            println!();
            println!(
                "{} Count mismatch: {}{} nodes",
                style("⚠").yellow(),
                sign,
                delta
            );
        }
    } else if pack.tested {
        println!();
        println!("{} Workflow file missing", style("⚠").yellow());
    }

    println!();
    pause();
    Ok(())
}

pub fn run_cleanup() -> Result<()> {
    clear_screen();

    println!("\n{}", style("Cleanup Empty Projects").bold().cyan());
    println!("{}", style("─".repeat(50)).dim());
    println!();

    let repo_root = find_repo_root()?;
    let checklists_dir = repo_root.join("checklists");

    let mut empty_projects = Vec::new();

    if let Ok(entries) = fs::read_dir(&checklists_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                if name == "templates" || name == "schema" {
                    continue;
                }

                let checklist_path = path.join("checklist.md");
                if let Ok(content) = fs::read_to_string(&checklist_path) {
                    let has_checked = content.contains("- [x]");
                    let metadata_path = path.join("metadata.json");
                    let has_real_metadata = metadata_path.exists()
                        && fs::read_to_string(&metadata_path)
                            .map(|c| c.contains("\"packs\"") && c.len() > 200)
                            .unwrap_or(false);

                    if !has_checked && !has_real_metadata {
                        empty_projects.push(name.to_string());
                    }
                }
            }
        }
    }

    if empty_projects.is_empty() {
        println!("No empty projects found.");
        return Ok(());
    }

    println!("Empty projects:");
    for proj in &empty_projects {
        println!("  {}", style(proj).yellow());
    }
    println!();

    let delete = Confirm::new()
        .with_prompt("Delete all empty projects?")
        .default(false)
        .interact()?;

    if delete {
        for proj in &empty_projects {
            let proj_dir = checklists_dir.join(proj);
            fs::remove_dir_all(&proj_dir)?;
            println!("{} Deleted {}", style("✓").green(), proj);
        }
        println!();
    }

    Ok(())
}

fn select_project() -> Result<Option<String>> {
    let repo_root = find_repo_root()?;
    let projects = get_existing_projects(&repo_root.join("checklists"))?;

    if projects.is_empty() {
        return Ok(None);
    }

    let selection = Select::new()
        .with_prompt("Select project")
        .items(&projects)
        .interact()?;

    Ok(Some(projects[selection].clone()))
}

fn get_existing_projects(checklists_dir: &PathBuf) -> Result<Vec<String>> {
    let mut projects = Vec::new();

    if let Ok(entries) = fs::read_dir(checklists_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    if name != "templates" && name != "schema" {
                        projects.push(name.to_string());
                    }
                }
            }
        }
    }

    projects.sort();
    Ok(projects)
}

fn find_repo_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;

    for ancestor in current_dir.ancestors() {
        if ancestor.join("checklists").exists() {
            return Ok(ancestor.to_path_buf());
        }
    }

    anyhow::bail!("Not in custom-node-qa repository")
}

fn run_diff_interactive(project: &str) -> Result<()> {
    loop {
        clear_screen();

        println!(
            "\n{}",
            style("Compare Checklist vs Workflows").bold().cyan()
        );
        println!("{}", style("─".repeat(70)).dim());
        println!();
        println!("This compares your checklist against actual workflow files to find:");
        println!("  • Count mismatches (nodes added/removed)");
        println!("  • Missing workflow files");
        println!("  • New packs not in checklist");
        println!();

        let repo_root = find_repo_root()?;
        let checklists_dir = repo_root.join("checklists");
        let workflows_dir = repo_root.join("workflows");
        let project_dir = checklists_dir.join(project);

        let checklist = match crate::models::Checklist::from_file(project_dir.join("checklist.md"))
        {
            Ok(c) => c,
            Err(e) => {
                println!("{} Failed to load checklist: {}", style("✗").red(), e);
                pause();
                return Ok(());
            }
        };

        let workflows = crate::models::Workflow::load_all(&workflows_dir).unwrap_or_default();

        // Calculate diff
        let mut matches = Vec::new();
        let mut count_mismatches = Vec::new();
        let mut missing_workflows = Vec::new();

        for pack in &checklist.packs {
            if let Some(workflow) = workflows.get(&pack.name) {
                if pack.node_count == workflow.node_count {
                    matches.push((pack.name.clone(), pack.node_count));
                } else {
                    count_mismatches.push((
                        pack.name.clone(),
                        pack.node_count,
                        workflow.node_count,
                    ));
                }
            } else if pack.tested {
                missing_workflows.push((pack.name.clone(), pack.node_count));
            }
        }

        // Display summary
        println!("{}: {}", style("✓ Matches").green(), matches.len());
        println!(
            "{}: {}",
            style("⚠ Count Mismatches").yellow(),
            count_mismatches.len()
        );
        println!(
            "{}: {}",
            style("✗ Missing Workflows").red(),
            missing_workflows.len()
        );
        println!();

        let mut options = vec![];
        if !matches.is_empty() {
            options.push(format!("View matching packs ({})", matches.len()));
        }
        if !count_mismatches.is_empty() {
            options.push(format!(
                "View count mismatches ({})",
                count_mismatches.len()
            ));
        }
        if !missing_workflows.is_empty() {
            options.push(format!(
                "View missing workflows ({})",
                missing_workflows.len()
            ));
        }
        options.push("Fix issues automatically (sync)".to_string());
        options.push("Back to dashboard".to_string());

        let selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(&options)
            .default(0)
            .interact()?;

        let mut option_index = 0;

        if !matches.is_empty() {
            if selection == option_index {
                show_diff_category("Matching Packs", &matches, None)?;
                option_index += 1;
                continue;
            }
            option_index += 1;
        }

        if !count_mismatches.is_empty() {
            if selection == option_index {
                show_count_mismatches_interactive(
                    project,
                    &count_mismatches,
                    &workflows,
                    &project_dir,
                )?;
                option_index += 1;
                continue;
            }
            option_index += 1;
        }

        if !missing_workflows.is_empty() {
            if selection == option_index {
                show_diff_category("Missing Workflow Files", &missing_workflows, None)?;
                option_index += 1;
                continue;
            }
            option_index += 1;
        }

        if selection == option_index {
            // Run sync
            run_sync_interactive(project)?;
            break;
        } else {
            // Back
            break;
        }
    }

    Ok(())
}

fn show_diff_category(
    title: &str,
    simple_items: &[(String, usize)],
    detailed_items: Option<Vec<(String, Option<String>)>>,
) -> Result<()> {
    clear_screen();
    println!("\n{}", style(title).bold());
    println!("{}", style("─".repeat(70)).dim());
    println!();

    if let Some(items) = detailed_items {
        for (name, detail) in items {
            if let Some(d) = detail {
                println!("  {} - {}", name, d);
            } else {
                println!("  {}", name);
            }
        }
    } else {
        for (name, count) in simple_items {
            println!("  {} ({} nodes)", name, count);
        }
    }

    println!();
    pause();
    Ok(())
}

fn show_count_mismatches_interactive(
    project: &str,
    count_mismatches: &[(String, usize, usize)],
    workflows: &std::collections::HashMap<String, crate::models::Workflow>,
    project_dir: &std::path::Path,
) -> Result<()> {
    let detailed_checklist_path = project_dir.join("checklist-detailed.md");
    let detailed_checklist = if detailed_checklist_path.exists() {
        Some(crate::models::DetailedChecklist::from_file(
            &detailed_checklist_path,
        )?)
    } else {
        None
    };

    loop {
        clear_screen();
        println!(
            "\n{}",
            style(format!("Count Mismatches ({})", count_mismatches.len())).bold()
        );
        println!("{}", style("─".repeat(70)).dim());
        println!();

        let mut items: Vec<String> = count_mismatches
            .iter()
            .map(|(name, checklist_count, workflow_count)| {
                let delta = *workflow_count as i64 - *checklist_count as i64;
                let sign = if delta > 0 { "+" } else { "" };
                format!(
                    "{} - checklist: {}, workflow: {} ({}{})",
                    style(name).yellow(),
                    checklist_count,
                    workflow_count,
                    sign,
                    delta
                )
            })
            .collect();

        items.push(style("← Back").dim().to_string());

        let selection = Select::new()
            .with_prompt("Select a pack to see detailed node diff")
            .items(&items)
            .default(0)
            .interact()?;

        if selection >= count_mismatches.len() {
            break;
        }

        let (pack_name, checklist_count, workflow_count) = &count_mismatches[selection];

        if let Some(workflow) = workflows.get(pack_name) {
            if let Some(ref detailed) = detailed_checklist {
                let node_diff =
                    crate::commands::diff::calculate_node_diff(pack_name, workflow, detailed);
                show_node_diff_details(&node_diff, *checklist_count, *workflow_count)?;
            } else {
                clear_screen();
                println!("\n{}", style(pack_name).bold());
                println!("{}", style("─".repeat(70)).dim());
                println!();
                println!("{}", style("Cannot show detailed diff:").yellow());
                println!(
                    "  checklist-detailed.md not found for project '{}'",
                    project
                );
                println!();
                println!(
                    "Count mismatch: {} → {} nodes",
                    checklist_count, workflow_count
                );
                println!();
                pause();
            }
        }
    }

    Ok(())
}

fn show_node_diff_details(
    node_diff: &crate::commands::diff::NodeDiff,
    checklist_count: usize,
    workflow_count: usize,
) -> Result<()> {
    clear_screen();
    println!("\n{}", style(&node_diff.pack_name).bold().cyan());
    println!("{}", style("─".repeat(70)).dim());
    println!();

    let delta = workflow_count as i64 - checklist_count as i64;
    let sign = if delta > 0 { "+" } else { "" };

    println!(
        "Count: {} → {} ({}{} nodes)",
        checklist_count, workflow_count, sign, delta
    );
    println!();

    if !node_diff.missing_from_checklist.is_empty() {
        println!(
            "{} {} nodes in workflow but NOT in checklist:",
            style("⚠").yellow(),
            style(node_diff.missing_from_checklist.len()).bold()
        );
        for (i, node) in node_diff.missing_from_checklist.iter().enumerate() {
            println!("  {}. {} {}", i + 1, style("+").green(), node);
        }
        println!();
    }

    if !node_diff.extra_in_checklist.is_empty() {
        println!(
            "{} {} nodes in checklist but NOT in workflow:",
            style("⚠").yellow(),
            style(node_diff.extra_in_checklist.len()).bold()
        );
        for (i, node) in node_diff.extra_in_checklist.iter().enumerate() {
            println!("  {}. {} {}", i + 1, style("-").red(), node);
        }
        println!();
    }

    if node_diff.missing_from_checklist.is_empty() && node_diff.extra_in_checklist.is_empty() {
        println!("{} No specific node differences found", style("ℹ").cyan());
        println!("This might be a duplicate node or parsing issue.");
        println!();
    }

    pause();
    Ok(())
}

fn run_validate_interactive(project: &str) -> Result<()> {
    loop {
        clear_screen();

        println!("\n{}", style("Validation Report").bold().cyan());
        println!("{}", style("─".repeat(70)).dim());
        println!();
        println!("Checking for:");
        println!("  • Schema issues");
        println!("  • Format errors");
        println!("  • Data consistency");
        println!();

        let repo_root = find_repo_root()?;
        let checklists_dir = repo_root.join("checklists");
        let workflows_dir = repo_root.join("workflows");
        let project_dir = checklists_dir.join(project);

        let checklist_path = project_dir.join("checklist.md");
        let metadata_path = project_dir.join("metadata.json");

        let checklist = match crate::models::Checklist::from_file(&checklist_path) {
            Ok(c) => c,
            Err(e) => {
                println!("{} Failed to load checklist: {}", style("✗").red(), e);
                pause();
                return Ok(());
            }
        };

        let metadata = if metadata_path.exists() {
            crate::models::Metadata::from_file(&metadata_path).ok()
        } else {
            None
        };

        let workflows = crate::models::Workflow::load_all(&workflows_dir).unwrap_or_default();

        let results = crate::validators::Validator::validate_project(
            &checklist,
            &workflows,
            metadata.as_ref(),
        );

        let errors: Vec<_> = results
            .iter()
            .filter(|r| r.severity == crate::validators::Severity::Error)
            .collect();
        let warnings: Vec<_> = results
            .iter()
            .filter(|r| r.severity == crate::validators::Severity::Warning)
            .collect();

        if errors.is_empty() && warnings.is_empty() {
            println!("{} All validation checks passed!", style("✓").green());
            println!();
            pause();
            break;
        }

        println!("{}: {}", style("Errors").red(), errors.len());
        println!("{}: {}", style("Warnings").yellow(), warnings.len());
        println!();

        let mut options = vec![];
        if !errors.is_empty() {
            options.push(format!("View errors ({})", errors.len()));
        }
        if !warnings.is_empty() {
            options.push(format!("View warnings ({})", warnings.len()));
        }
        options.push("View all issues".to_string());
        options.push("Back to dashboard".to_string());

        let selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(&options)
            .default(0)
            .interact()?;

        let mut option_index = 0;

        if !errors.is_empty() {
            if selection == option_index {
                show_validation_results("Errors", &errors)?;
                option_index += 1;
                continue;
            }
            option_index += 1;
        }

        if !warnings.is_empty() {
            if selection == option_index {
                show_validation_results("Warnings", &warnings)?;
                option_index += 1;
                continue;
            }
            option_index += 1;
        }

        if selection == option_index {
            let all_results: Vec<_> = results.iter().collect();
            show_validation_results("All Issues", &all_results)?;
            continue;
        } else {
            break;
        }
    }

    Ok(())
}

fn show_validation_results(
    title: &str,
    results: &[&crate::validators::ValidationResult],
) -> Result<()> {
    clear_screen();
    println!("\n{}", style(title).bold());
    println!("{}", style("─".repeat(70)).dim());
    println!();

    for (i, result) in results.iter().enumerate() {
        let severity_icon = match result.severity {
            crate::validators::Severity::Error => style("✗").red(),
            crate::validators::Severity::Warning => style("⚠").yellow(),
        };
        println!("{}. {} {}", i + 1, severity_icon, result.message);
        if let Some(pack) = &result.pack {
            println!("   Pack: {}", style(pack).dim());
        }
    }

    println!();
    pause();
    Ok(())
}

fn show_help_guide() -> Result<()> {
    loop {
        clear_screen();

        println!("\n{}", style("How Does This Work?").bold().cyan());
        println!("{}", style("─".repeat(70)).dim());
        println!();

        let topics = vec![
            "📋 Overview - What is this tool?",
            "🔄 The Complete Workflow",
            "📁 Understanding the Files",
            "🎯 What Each Command Does",
            "🌿 Git Basics (pull, commit, push)",
            "💡 Common Scenarios",
            "🔧 Troubleshooting",
            "← Back to dashboard",
        ];

        let selection = Select::new()
            .with_prompt("Choose a topic")
            .items(&topics)
            .default(0)
            .interact()?;

        match selection {
            0 => show_help_overview()?,
            1 => show_help_workflow()?,
            2 => show_help_files()?,
            3 => show_help_commands()?,
            4 => show_help_git_basics()?,
            5 => show_help_scenarios()?,
            6 => show_help_troubleshooting()?,
            7 => break,
            _ => break,
        }
    }

    Ok(())
}

fn show_help_overview() -> Result<()> {
    clear_screen();
    println!("\n{}", style("What is this tool?").bold());
    println!("{}", style("─".repeat(70)).dim());
    println!();
    println!("This tool helps QA teams track which Hanzo Studio custom node packs have");
    println!(
        "been tested. You work in a {} stored on GitHub,",
        style("shared repository").yellow()
    );
    println!("where multiple testers collaborate and share results.");
    println!();
    println!("{}", style("The Team Workflow:").bold());
    println!("  1. Clone the shared GitHub repo");
    println!("  2. Pull latest changes (see what teammates tested)");
    println!("  3. Test node packs in Hanzo Studio browser");
    println!("  4. Save workflow files and mark as tested");
    println!("  5. Commit and push to share with team");
    println!("  6. Others pull your updates");
    println!();
    println!("{}", style("Key Concepts:").bold());
    println!(
        "  • {} = Team collaboration through GitHub",
        style("Git").yellow()
    );
    println!(
        "  • {} = what exists in Hanzo Studio",
        style("Workflow files").yellow()
    );
    println!(
        "  • {} = what's been tested",
        style("Checklist files").yellow()
    );
    println!(
        "  • This tool {} and helps collaboration",
        style("compares").yellow()
    );
    println!();
    pause();
    Ok(())
}

fn show_help_workflow() -> Result<()> {
    clear_screen();
    println!("\n{}", style("The Complete Workflow").bold());
    println!("{}", style("─".repeat(70)).dim());
    println!();
    println!("{}", style("Step 0: Start your day (EVERY TIME)").bold());
    println!("  • Open terminal in repo folder");
    println!("  • Run: git pull origin main");
    println!("  • This gets your teammates' latest work!");
    println!();
    println!("{}", style("Step 1: Get the browser script").bold());
    println!("  • Dashboard → 'Get browser script' → Copy");
    println!();
    println!("{}", style("Step 2: Open Hanzo Studio in browser").bold());
    println!("  • Open your Hanzo Studio instance");
    println!("  • Press F12 (Developer Tools)");
    println!("  • Click 'Console' tab");
    println!("  • Paste the script and press Enter");
    println!();
    println!("{}", style("Step 3: Test a pack").bold());
    println!("  • Run: await QA.testPack('pack-name')");
    println!("  • Browser downloads a JSON file");
    println!("  • Save it to the workflows/ folder");
    println!();
    println!("{}", style("Step 4: Mark as tested").bold());
    println!("  • Open checklists/your-project/checklist.md");
    println!("  • Find the pack and change [ ] to [x]");
    println!();
    println!("{}", style("Step 5: Share with team (git)").bold());
    println!("  • git add .");
    println!("  • git commit -m 'Tested pack-name: X nodes verified'");
    println!("  • git push origin main");
    println!("  • Now your teammates can see your work!");
    println!();
    pause();
    Ok(())
}

fn show_help_files() -> Result<()> {
    clear_screen();
    println!("\n{}", style("Understanding the Files").bold());
    println!("{}", style("─".repeat(70)).dim());
    println!();
    println!("{}", style("workflows/*.json").yellow().bold());
    println!("  • Generated by browser script");
    println!("  • Contains actual node definitions from Hanzo Studio");
    println!(
        "  • This is the {} - what really exists",
        style("source of truth").green()
    );
    println!();
    println!(
        "{}",
        style("checklists/your-project/checklist.md")
            .yellow()
            .bold()
    );
    println!("  • Manually edited by you");
    println!("  • Lists all packs with [ ] or [x]");
    println!("  • Shows node counts for each pack");
    println!();
    println!(
        "{}",
        style("checklists/your-project/checklist-detailed.md")
            .yellow()
            .bold()
    );
    println!("  • Detailed version with individual node names");
    println!("  • Also manually edited");
    println!();
    println!("{}", style("How they relate:").bold());
    println!("  Workflow files → Tool compares → Checklist files");
    println!("  If counts don't match = something is wrong!");
    println!();
    pause();
    Ok(())
}

fn show_help_commands() -> Result<()> {
    clear_screen();
    println!("\n{}", style("What Each Command Does").bold());
    println!("{}", style("─".repeat(70)).dim());
    println!();
    println!("{}", style("Check testing progress").cyan());
    println!("  Shows how many packs tested, what's left, and any mismatches");
    println!();
    println!("{}", style("Get browser script").cyan());
    println!("  Shows JavaScript to paste into browser DevTools console");
    println!();
    println!("{}", style("View all packs").cyan());
    println!("  Browse all packs, filter by status, see details");
    println!();
    println!("{}", style("Compare with repo (Diff)").cyan());
    println!("  Compares checklist vs workflow files, shows mismatches");
    println!("  Example: Checklist says 130 nodes, workflow has 129");
    println!();
    println!("{}", style("Run validation").cyan());
    println!("  Checks for format errors, schema issues, consistency problems");
    println!();
    println!("{}", style("Sync checklists").cyan());
    println!(
        "  {} Regenerates entire checklist from workflow files",
        style("WARNING:").red()
    );
    println!("  Useful when workflow files are correct but checklist is wrong");
    println!("  {} You'll lose your [x] marks!", style("DANGER:").red());
    println!();
    pause();
    Ok(())
}

fn show_help_git_basics() -> Result<()> {
    clear_screen();
    println!("\n{}", style("Git Basics for Team Collaboration").bold());
    println!("{}", style("─".repeat(70)).dim());
    println!();
    println!("{}", style("What is Git?").bold());
    println!("  Git is like 'track changes' for files. It lets multiple people work");
    println!("  on the same project without overwriting each other's work.");
    println!();
    println!("{}", style("Essential Commands:").bold());
    println!();
    println!("  {}", style("git pull origin main").cyan());
    println!("    Get everyone else's latest changes");
    println!(
        "    {} Run this every time before you start testing!",
        style("IMPORTANT:").yellow()
    );
    println!();
    println!("  {}", style("git status").cyan());
    println!("    See what files you changed");
    println!();
    println!("  {}", style("git add .").cyan());
    println!("    Mark all your changes to be saved");
    println!();
    println!("  {}", style("git commit -m \"Tested pack-name\"").cyan());
    println!("    Save a snapshot with a description");
    println!();
    println!("  {}", style("git push origin main").cyan());
    println!("    Upload to GitHub for your team to see");
    println!();
    println!("{}", style("Typical Session:").bold());
    println!("  1. git pull origin main          (get latest)");
    println!("  2. [do your testing work]");
    println!("  3. git add .");
    println!("  4. git commit -m \"Tested 3 packs\"");
    println!("  5. git push origin main          (share with team)");
    println!();
    println!("{}", style("Platform-Specific:").bold());
    println!("  Windows: Use Git Bash or PowerShell");
    println!("  Mac: Use Terminal app");
    println!("  Linux: Use any terminal (bash, zsh, etc.)");
    println!();
    pause();
    Ok(())
}

fn show_help_scenarios() -> Result<()> {
    clear_screen();
    println!("\n{}", style("Common Scenarios").bold());
    println!("{}", style("─".repeat(70)).dim());
    println!();
    println!("{}", style("Scenario 1: Your first day").bold());
    println!("  1. git clone <repo-url>");
    println!("  2. cd hanzo-studio-custom-node-qa");
    println!("  3. Run tool, select/create project");
    println!("  4. Start testing!");
    println!();
    println!("{}", style("Scenario 2: Starting your day").bold());
    println!("  1. git pull origin main  (get teammates' updates!)");
    println!("  2. Check dashboard - see what's new");
    println!("  3. Pick untested packs");
    println!("  4. Test, commit, push, repeat");
    println!();
    println!("{}", style("Scenario 3: Someone tested same pack").bold());
    println!("  1. You: test pack, commit, push");
    println!("  2. Git: 'rejected, pull first'");
    println!("  3. git pull origin main");
    println!("  4. Fix conflict (keep both [x] marks)");
    println!("  5. git push origin main");
    println!();
    println!("{}", style("Scenario 4: Pack was updated").bold());
    println!("  1. Re-test: QA.testPack('pack-name')");
    println!("  2. Save new workflow file");
    println!("  3. Update checklist (manually or sync)");
    println!("  4. Commit, push, explain in message");
    println!();
    pause();
    Ok(())
}

fn show_help_troubleshooting() -> Result<()> {
    clear_screen();
    println!("\n{}", style("Troubleshooting").bold());
    println!("{}", style("─".repeat(70)).dim());
    println!();
    println!(
        "{}",
        style("'Count mismatch' - What does this mean?").bold()
    );
    println!("  Your checklist says X nodes, workflow file says Y nodes.");
    println!("  → Either workflow is outdated (re-test) OR checklist is wrong (edit/sync)");
    println!();
    println!("{}", style("'Workflow file missing'").bold());
    println!("  You marked [x] as tested but there's no JSON file.");
    println!("  → Did you save the workflow file? Run await QA.testPack() again");
    println!();
    println!("{}", style("'I synced and lost my [x] marks!'").bold());
    println!("  Sync overwrites the entire checklist.");
    println!("  → Always preview (dry run) first!");
    println!("  → Use git to recover: git checkout HEAD~1 checklists/");
    println!();
    println!("{}", style("'Tool won't run'").bold());
    println!("  Make sure you're in the repo directory with checklists/ folder.");
    println!();
    println!("{}", style("'Browser script not working'").bold());
    println!("  • Make sure Hanzo Studio is fully loaded");
    println!("  • Check browser console for error messages");
    println!("  • Try refreshing and pasting script again");
    println!();
    pause();
    Ok(())
}

fn run_sync_interactive(project: &str) -> Result<()> {
    clear_screen();

    println!(
        "\n{}",
        style("Sync: Regenerate Checklists from Workflow Files")
            .bold()
            .cyan()
    );
    println!("{}", style("─".repeat(70)).dim());
    println!();
    println!("{}", style("What this does:").bold());
    println!("  • Reads all workflow/*.json files (source of truth)");
    println!("  • Regenerates checklist.md from scratch");
    println!("  • Updates node counts to match workflow files");
    println!();
    println!("{}", style("⚠ Important:").yellow().bold());
    println!("  • Your [x] checked marks will be LOST");
    println!("  • Any manual edits to checklist.md will be OVERWRITTEN");
    println!("  • Workflow JSON files are not modified (they're the source)");
    println!();
    println!("{}", style("When to use this:").dim());
    println!("  ✓ Workflow files are up-to-date and correct");
    println!("  ✓ Checklist has wrong node counts");
    println!("  ✓ You want to start fresh");
    println!();
    println!("{}", style("When NOT to use this:").dim());
    println!("  ✗ You've manually marked packs as tested [x]");
    println!("  ✗ Checklist is correct, workflows are outdated");
    println!();

    let options = vec!["Preview changes (dry run)", "Apply sync now", "Cancel"];

    let selection = Select::new()
        .with_prompt("What would you like to do?")
        .items(&options)
        .default(0)
        .interact()?;

    match selection {
        0 => {
            // Dry run
            println!();
            crate::commands::sync::run(project.to_string(), true)?;
            println!();
            pause();
        }
        1 => {
            // Apply
            println!();
            crate::commands::sync::run(project.to_string(), false)?;
            println!();
            println!("{} Sync complete!", style("✓").green());
            println!();
            pause();
        }
        _ => {
            // Cancel
        }
    }

    Ok(())
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}

fn pause() {
    println!();
    println!("Press Enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
