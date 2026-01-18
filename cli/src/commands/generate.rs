use crate::models::Workflow;
use anyhow::{Context, Result};
use console::style;
use dialoguer::{Input, Select};
use serde_json::json;
use std::fs;
use std::path::PathBuf;

pub fn run(search_query: Option<String>) -> Result<()> {
    let repo_root = find_repo_root()?;
    let workflows_dir = repo_root.join("workflows");

    // Get search query
    let query = if let Some(q) = search_query {
        q
    } else {
        Input::<String>::new()
            .with_prompt("Search for nodes (e.g., 'sampler', 'load', 'image')")
            .interact_text()?
    };

    println!();
    println!("{} Searching for nodes matching '{}'...", style("→").cyan(), query);
    println!();

    // Load all workflows and find matching nodes
    let workflows = Workflow::load_all(&workflows_dir)?;
    let mut matching_nodes = Vec::new();
    let query_lower = query.to_lowercase();

    for workflow in workflows.values() {
        for node in &workflow.nodes {
            if node.node_type.to_lowercase().contains(&query_lower) {
                if !matching_nodes.contains(&node.node_type) {
                    matching_nodes.push(node.node_type.clone());
                }
            }
        }
    }

    if matching_nodes.is_empty() {
        println!("{} No nodes found matching '{}'", style("✗").red(), query);
        return Ok(());
    }

    matching_nodes.sort();

    println!("{} Found {} matching nodes:", style("✓").green(), matching_nodes.len());
    for (i, node) in matching_nodes.iter().enumerate().take(20) {
        println!("  {}. {}", i + 1, node);
    }
    if matching_nodes.len() > 20 {
        println!("  ... and {} more", matching_nodes.len() - 20);
    }
    println!();

    // Ask user to confirm
    let options = vec![
        format!("Generate workflow with all {} nodes", matching_nodes.len()),
        "Cancel".to_string(),
    ];

    let selection = Select::new()
        .with_prompt("What would you like to do?")
        .items(&options)
        .default(0)
        .interact()?;

    if selection != 0 {
        return Ok(());
    }

    // Ask where to save
    let save_options = vec![
        "Local only (gitignored, workflows/generated-local/)",
        "Commit to repo (workflows/generated-shared/)",
    ];

    let save_selection = Select::new()
        .with_prompt("Where should this workflow be saved?")
        .items(&save_options)
        .default(0)
        .interact()?;

    let (output_dir, is_local) = if save_selection == 0 {
        (workflows_dir.join("generated-local"), true)
    } else {
        (workflows_dir.join("generated-shared"), false)
    };

    fs::create_dir_all(&output_dir)?;

    // Get filename
    let default_name = format!("search-{}.json", query.replace(" ", "-"));
    let filename = Input::<String>::new()
        .with_prompt("Filename")
        .default(default_name)
        .interact_text()?;

    let output_path = output_dir.join(&filename);

    // Generate workflow JSON
    let workflow_json = generate_workflow_json(&matching_nodes);

    // Save file
    fs::write(&output_path, serde_json::to_string_pretty(&workflow_json)?)?;

    println!();
    println!("{} Workflow generated!", style("✓").green());
    println!("  File: {}", style(output_path.display()).yellow());
    if is_local {
        println!("  {}", style("(local only, gitignored)").dim());
    }
    println!();

    // Show copy-paste instructions
    show_load_instructions(&output_path)?;

    Ok(())
}

fn generate_workflow_json(node_types: &[String]) -> serde_json::Value {
    let mut nodes = Vec::new();

    // Create nodes in a grid layout
    let cols = 4;
    let spacing_x = 400;
    let spacing_y = 300;

    for (i, node_type) in node_types.iter().enumerate() {
        let row = i / cols;
        let col = i % cols;
        let pos_x = 100 + (col * spacing_x);
        let pos_y = 100 + (row * spacing_y);

        nodes.push(json!({
            "id": i + 1,
            "type": node_type,
            "pos": [pos_x, pos_y],
            "size": [300, 100],
            "flags": {},
            "order": i,
            "mode": 0,
            "inputs": [],
            "outputs": [],
            "properties": {
                "Node name for S&R": node_type
            },
            "widgets_values": []
        }));
    }

    json!({
        "id": "00000000-0000-0000-0000-000000000000",
        "revision": 0,
        "last_node_id": node_types.len(),
        "last_link_id": 0,
        "nodes": nodes,
        "links": [],
        "groups": [],
        "config": {},
        "extra": {},
        "version": 0.4
    })
}

fn show_load_instructions(file_path: &PathBuf) -> Result<()> {
    println!("{}", style("═".repeat(70)).cyan());
    println!();
    println!("{}", style("How to Load This Workflow in ComfyUI:").bold().cyan());
    println!();
    println!("{}", style("Method 1: Copy-Paste (Easiest)").bold());
    println!("  1. Open ComfyUI in your browser");
    println!("  2. Click {} on the canvas", style("anywhere").yellow());
    println!("  3. Press {} (Windows/Linux) or {} (Mac)",
        style("Ctrl+V").yellow(),
        style("Cmd+V").yellow()
    );
    println!("  4. {} The workflow loads instantly!", style("✓").green());
    println!();
    println!("{}", style("Method 2: From File").bold());
    println!("  1. Open the file in a text editor:");
    println!("     {}", style(file_path.display()).dim());
    println!("  2. Select all (Ctrl+A) and copy (Ctrl+C)");
    println!("  3. Go to ComfyUI canvas");
    println!("  4. Press Ctrl+V");
    println!();
    println!("{}", style("Method 3: Drag & Drop").bold());
    println!("  1. Drag this file onto the ComfyUI canvas");
    println!("  2. It will load automatically");
    println!();
    println!("{}", style("═".repeat(70)).cyan());
    println!();

    println!("Press Enter when ready to return to dashboard...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(())
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
