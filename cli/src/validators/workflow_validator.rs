use crate::models::Workflow;
use crate::validators::ValidationResult;
use std::collections::HashMap;

pub struct WorkflowValidator;

impl WorkflowValidator {
    pub fn validate(workflows: &HashMap<String, Workflow>) -> Vec<ValidationResult> {
        let mut results = Vec::new();

        for workflow in workflows.values() {
            // Check if workflow has nodes
            if workflow.node_count == 0 {
                results.push(ValidationResult::warning(
                    "Workflow file has 0 nodes".to_string(),
                    Some(workflow.pack_name.clone()),
                ));
            }
        }

        results
    }
}
