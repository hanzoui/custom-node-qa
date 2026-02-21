mod checklist_validator;
mod naming_validator;
mod workflow_validator;

pub use checklist_validator::ChecklistValidator;
pub use naming_validator::NamingValidator;
pub use workflow_validator::WorkflowValidator;

use crate::models::{Checklist, Metadata, Workflow};
use colored::Colorize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Count mismatch for pack '{pack}': checklist has {checklist_count}, workflow has {workflow_count}")]
    CountMismatch {
        pack: String,
        checklist_count: usize,
        workflow_count: usize,
    },

    #[error("Missing workflow file for pack '{0}'")]
    MissingWorkflow(String),

    #[error("Duplicate pack name '{0}'")]
    DuplicatePack(String),

    #[error("Invalid markdown format: {0}")]
    InvalidMarkdown(String),

    #[error("Invalid naming convention: {0}")]
    InvalidNaming(String),

    #[error("Metadata validation failed: {0}")]
    MetadataValidation(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug)]
pub struct ValidationResult {
    pub severity: Severity,
    pub message: String,
    pub pack: Option<String>,
}

impl ValidationResult {
    pub fn error(message: String, pack: Option<String>) -> Self {
        Self {
            severity: Severity::Error,
            message,
            pack,
        }
    }

    pub fn warning(message: String, pack: Option<String>) -> Self {
        Self {
            severity: Severity::Warning,
            message,
            pack,
        }
    }

    pub fn print(&self) {
        let prefix = match self.severity {
            Severity::Error => "❌ ERROR".red(),
            Severity::Warning => "⚠️  WARN".yellow(),
        };

        if let Some(pack) = &self.pack {
            println!("{}: [{}] {}", prefix, pack, self.message);
        } else {
            println!("{}: {}", prefix, self.message);
        }
    }
}

pub struct Validator;

impl Validator {
    pub fn validate_project(
        checklist: &Checklist,
        workflows: &HashMap<String, Workflow>,
        metadata: Option<&Metadata>,
    ) -> Vec<ValidationResult> {
        let mut results = Vec::new();

        // Run all validators
        results.extend(ChecklistValidator::validate(checklist, workflows));
        results.extend(WorkflowValidator::validate(workflows));
        results.extend(NamingValidator::validate_checklist(checklist));

        if let Some(metadata) = metadata {
            results.extend(Self::validate_metadata(metadata, checklist, workflows));
        } else {
            results.push(ValidationResult::warning(
                "metadata.json not found".to_string(),
                None,
            ));
        }

        results
    }

    fn validate_metadata(
        metadata: &Metadata,
        checklist: &Checklist,
        workflows: &HashMap<String, Workflow>,
    ) -> Vec<ValidationResult> {
        let mut results = Vec::new();

        // Check metadata packs match checklist
        for pack in &checklist.packs {
            if !metadata.packs.contains_key(&pack.name) {
                results.push(ValidationResult::warning(
                    "Pack in checklist but not in metadata.json".to_string(),
                    Some(pack.name.clone()),
                ));
            }
        }

        // Check metadata packs have correct counts
        for (pack_name, pack_meta) in &metadata.packs {
            if let Some(workflow) = workflows.get(pack_name) {
                if pack_meta.node_count != workflow.node_count {
                    results.push(ValidationResult::error(
                        format!(
                            "metadata.json has count {}, workflow has {}",
                            pack_meta.node_count, workflow.node_count
                        ),
                        Some(pack_name.clone()),
                    ));
                }
            }
        }

        results
    }
}
