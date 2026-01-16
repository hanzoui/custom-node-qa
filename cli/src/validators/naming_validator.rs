use crate::models::Checklist;
use crate::validators::ValidationResult;
use regex::Regex;

pub struct NamingValidator;

impl NamingValidator {
    pub fn validate_checklist(checklist: &Checklist) -> Vec<ValidationResult> {
        let mut results = Vec::new();

        // Validate project name (kebab-case)
        if !Self::is_valid_project_name(&checklist.project_name) {
            results.push(ValidationResult::error(
                format!(
                    "Invalid project name '{}': must be kebab-case",
                    checklist.project_name
                ),
                None,
            ));
        }

        results
    }

    pub fn is_valid_project_name(name: &str) -> bool {
        let re = Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").unwrap();
        re.is_match(name)
    }

    pub fn is_valid_workflow_filename(filename: &str) -> bool {
        filename.starts_with("all-nodes-") && filename.ends_with(".json")
    }
}
