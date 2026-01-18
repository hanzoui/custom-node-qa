use anyhow::{Context, Result};
use console::style;
use dialoguer::{Input, Select};
use std::process::{Command, Stdio};

pub fn check_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn check_in_repo() -> bool {
    Command::new("git")
        .args(&["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn git_pull() -> Result<()> {
    println!();
    println!("{} Pulling latest changes from team...", style("→").cyan());
    println!();

    let output = Command::new("git")
        .args(&["pull", "origin", "main"])
        .output()
        .context("Failed to run git pull")?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
        println!();
        println!("{} Pull complete!", style("✓").green());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{} Pull failed:", style("✗").red());
        println!("{}", stderr);
        println!();
        println!("{}", style("Common causes:").yellow());
        println!("  • You have uncommitted changes (commit them first)");
        println!("  • Merge conflicts (need manual resolution)");
        println!("  • Not on 'main' branch");
    }

    Ok(())
}

pub fn git_status() -> Result<()> {
    println!();
    println!("{} Current git status:", style("→").cyan());
    println!();

    let output = Command::new("git")
        .args(&["status", "--short"])
        .output()
        .context("Failed to run git status")?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    if stdout.trim().is_empty() {
        println!("{} No changes to commit", style("✓").green());
    } else {
        println!("{}", stdout);
        println!();

        // Show diff stats
        let diff_output = Command::new("git")
            .args(&["diff", "--stat"])
            .output()
            .context("Failed to run git diff")?;

        let diff_stdout = String::from_utf8_lossy(&diff_output.stdout);
        if !diff_stdout.trim().is_empty() {
            println!("Changes:");
            println!("{}", diff_stdout);
        }
    }

    Ok(())
}

pub fn git_commit_and_push() -> Result<()> {
    // First check status
    let status_output = Command::new("git")
        .args(&["status", "--short"])
        .output()
        .context("Failed to check git status")?;

    let status = String::from_utf8_lossy(&status_output.stdout);

    if status.trim().is_empty() {
        println!();
        println!("{} No changes to commit", style("ℹ").cyan());
        return Ok(());
    }

    println!();
    println!("{}", style("Your changes:").bold());
    println!("{}", status);

    // Ask for commit message
    println!();
    let commit_msg = Input::<String>::new()
        .with_prompt("Commit message (describe what you tested)")
        .interact_text()?;

    // Add all changes
    println!();
    println!("{} Staging changes...", style("→").cyan());

    let add_output = Command::new("git")
        .args(&["add", "."])
        .output()
        .context("Failed to stage changes")?;

    if !add_output.status.success() {
        println!("{} Failed to stage changes", style("✗").red());
        return Ok(());
    }

    // Commit
    println!("{} Creating commit...", style("→").cyan());

    let commit_output = Command::new("git")
        .args(&["commit", "-m", &commit_msg])
        .output()
        .context("Failed to commit")?;

    if !commit_output.status.success() {
        println!("{} Commit failed:", style("✗").red());
        println!("{}", String::from_utf8_lossy(&commit_output.stderr));
        return Ok(());
    }

    println!("{} Commit created!", style("✓").green());
    println!();

    // Ask if should push
    let push_options = vec!["Push now (share with team)", "Don't push yet"];

    let push_choice = Select::new()
        .with_prompt("Push to GitHub?")
        .items(&push_options)
        .default(0)
        .interact()?;

    if push_choice == 0 {
        println!();
        println!("{} Pushing to GitHub...", style("→").cyan());

        let push_output = Command::new("git")
            .args(&["push", "origin", "main"])
            .output()
            .context("Failed to push")?;

        if push_output.status.success() {
            println!("{} Push complete! Your work is now shared with the team.", style("✓").green());
        } else {
            let stderr = String::from_utf8_lossy(&push_output.stderr);
            println!("{} Push failed:", style("✗").red());
            println!("{}", stderr);
            println!();
            println!("{}", style("Common causes:").yellow());
            println!("  • Remote has new changes (run 'Pull latest from team' first)");
            println!("  • Not on 'main' branch");
            println!("  • No permission to push");
            println!();
            println!("Your commit is saved locally. Fix the issue and try pushing again.");
        }
    } else {
        println!();
        println!("Commit saved locally. Push when ready with: git push origin main");
    }

    Ok(())
}

pub fn get_repo_status_summary() -> Option<String> {
    // Check if behind
    let fetch_output = Command::new("git")
        .args(&["fetch", "origin", "main"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if fetch_output.is_err() {
        return None;
    }

    let behind_output = Command::new("git")
        .args(&["rev-list", "--count", "HEAD..origin/main"])
        .output()
        .ok()?;

    let behind_str = String::from_utf8_lossy(&behind_output.stdout);
    let behind: usize = behind_str.trim().parse().unwrap_or(0);

    // Check for uncommitted changes
    let status_output = Command::new("git")
        .args(&["status", "--short"])
        .output()
        .ok()?;

    let status = String::from_utf8_lossy(&status_output.stdout);
    let has_changes = !status.trim().is_empty();

    let mut parts = Vec::new();
    if behind > 0 {
        parts.push(format!("{} commits behind", behind));
    }
    if has_changes {
        parts.push("uncommitted changes".to_string());
    }

    if parts.is_empty() {
        Some("Up to date".to_string())
    } else {
        Some(parts.join(", "))
    }
}
