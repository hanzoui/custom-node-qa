use crate::models::{NodePack, Workflow};
use std::collections::HashMap;

pub struct ChecklistGenerator;

impl ChecklistGenerator {
    pub fn generate_from_workflows(workflows: &HashMap<String, Workflow>) -> String {
        let mut packs: Vec<_> = workflows
            .values()
            .map(|w| NodePack::new(w.pack_name.clone(), w.node_count, false))
            .collect();

        packs.sort_by(|a, b| a.name.cmp(&b.name));

        let mut lines = vec!["# Node Pack QA Checklist".to_string(), String::new()];

        for pack in packs {
            lines.push(format!("- [ ] {} ({})", pack.name, pack.node_count));
        }

        lines.join("\n")
    }

    pub fn generate_detailed_from_workflows(workflows: &HashMap<String, Workflow>) -> String {
        let mut packs: Vec<_> = workflows.values().collect();
        packs.sort_by(|a, b| a.pack_name.cmp(&b.pack_name));

        let mut lines = vec!["# Node Pack QA Checklist".to_string(), String::new()];

        for workflow in packs {
            lines.push(format!("## {}", workflow.pack_name));
            lines.push(String::new());

            for node in &workflow.nodes {
                lines.push(format!("- [ ] {}", node.node_type));
            }

            lines.push(String::new());
        }

        lines.join("\n")
    }
}
