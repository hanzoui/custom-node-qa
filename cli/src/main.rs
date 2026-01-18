use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod generators;
mod models;
mod parsers;
mod validators;

#[derive(Parser)]
#[command(name = "comfy-qa")]
#[command(about = "ComfyUI Custom Node QA Tool", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Delete empty/unused projects
    Cleanup,

    /// Check your testing progress
    Check {
        /// Project name
        project: String,
    },
    /// Validate checklist format, schema, and consistency
    Validate {
        /// Project name to validate (validates all if not specified)
        project: Option<String>,

        /// Output in JSON format
        #[arg(long)]
        json: bool,

        /// Auto-fix issues where possible
        #[arg(long)]
        fix: bool,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Compare checklist state vs workflow files
    Diff {
        /// Project name to diff
        project: String,

        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },

    /// Regenerate checklists from workflow files
    Sync {
        /// Project name to sync
        project: String,

        /// Show what would be changed without modifying files
        #[arg(long)]
        dry_run: bool,
    },

    /// Generate workflow from node search query
    Generate {
        /// Search query for node names (e.g., "sampler", "load")
        query: Option<String>,
    },

    /// Import browser export JSON and create/update project
    Import {
        /// Path to export JSON file
        export_file: String,

        /// Project name
        project: String,
    },

    /// Create new QA project from template
    New {
        /// Project name (kebab-case)
        project_name: String,
    },

    /// Show completion stats and generate report
    Status {
        /// Project name (shows all if not specified)
        project: Option<String>,

        /// Output format (text, json, html)
        #[arg(long, default_value = "text")]
        format: String,

        /// Show all projects
        #[arg(long)]
        all: bool,
    },

    /// List all QA projects
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // If no command specified, run the main app (auto-setup or menu)
    let Some(command) = cli.command else {
        return commands::app::run();
    };

    match command {
        Commands::Cleanup => commands::app::run_cleanup(),

        Commands::Check { project } => commands::check::run(project),
        Commands::Validate {
            project,
            json,
            fix,
            verbose,
        } => commands::validate::run(project, json, fix, verbose),

        Commands::Diff { project, json } => commands::diff::run(project, json),

        Commands::Sync { project, dry_run } => commands::sync::run(project, dry_run),

        Commands::Generate { query } => commands::generate::run(query),

        Commands::Import {
            export_file,
            project,
        } => commands::import::run(export_file, project),

        Commands::New { project_name } => commands::new_project::run(project_name),

        Commands::Status {
            project,
            format,
            all,
        } => commands::status::run(project, format, all),

        Commands::List => commands::list::run(),
    }
}

