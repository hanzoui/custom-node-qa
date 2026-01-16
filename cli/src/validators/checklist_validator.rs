use crate::models::{Checklist, Workflow};
use crate::validators::{ValidationError, ValidationResult};
use std::collections::{HashMap, HashSet};

pub struct ChecklistValidator;

impl ChecklistValidator {
    pub fn validate(
        checklist: &Checklist,
        workflows: &HashMap<String, Workflow>,
    ) -> Vec<ValidationResult> {
        let mut results = Vec::new();

        // Check for duplicates
        results.extend(Self::check_duplicates(checklist));

        // Check for missing workflows
        results.extend(Self::check_missing_workflows(checklist, workflows));

        // Check node counts
        results.extend(Self::check_node_counts(checklist, workflows));

        results
    }

    fn check_duplicates(checklist: &Checklist) -> Vec<ValidationResult> {
        let mut results = Vec::new();
        let mut seen = HashSet::new();

        for pack in &checklist.packs {
            if !seen.insert(&pack.name) {
                results.push(ValidationResult::error(
                    ValidationError::DuplicatePack(pack.name.clone()).to_string(),
                    Some(pack.name.clone()),
                ));
            }
        }

        results
    }

    fn check_missing_workflows(
        checklist: &Checklist,
        workflows: &HashMap<String, Workflow>,
    ) -> Vec<ValidationResult> {
        let mut results = Vec::new();

        for pack in &checklist.packs {
            if pack.tested && !workflows.contains_key(&pack.name) {
                results.push(ValidationResult::error(
                    ValidationError::MissingWorkflow(pack.name.clone()).to_string(),
                    Some(pack.name.clone()),
                ));
            }
        }

        results
    }

    fn check_node_counts(
        checklist: &Checklist,
        workflows: &HashMap<String, Workflow>,
    ) -> Vec<ValidationResult> {
        let mut results = Vec::new();

        for pack in &checklist.packs {
            if let Some(workflow) = workflows.get(&pack.name) {
                if pack.node_count != workflow.node_count {
                    results.push(ValidationResult::error(
                        ValidationError::CountMismatch {
                            pack: pack.name.clone(),
                            checklist_count: pack.node_count,
                            workflow_count: workflow.node_count,
                        }
                        .to_string(),
                        Some(pack.name.clone()),
                    ));
                }
            }
        }

        results
    }
}
