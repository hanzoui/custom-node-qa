use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub file_path: PathBuf,
    pub pack_name: String,
    pub node_count: usize,
    pub nodes: Vec<WorkflowNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNode {
    #[serde(rename = "type")]
    pub node_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WorkflowJson {
    nodes: Option<Vec<WorkflowNodeRaw>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WorkflowNodeRaw {
    #[serde(rename = "type")]
    node_type: String,
}

impl Workflow {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read workflow file: {}", path.display()))?;

        let workflow_json: WorkflowJson = serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse workflow JSON: {}", path.display()))?;

        let nodes = workflow_json
            .nodes
            .unwrap_or_default()
            .into_iter()
            .map(|n| WorkflowNode {
                node_type: n.node_type,
            })
            .collect::<Vec<_>>();

        let node_count = nodes.len();

        // Extract pack name from filename: all-nodes-{pack-name}.json
        let pack_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.strip_prefix("all-nodes-"))
            .unwrap_or("unknown")
            .to_string();

        Ok(Workflow {
            file_path: path.to_path_buf(),
            pack_name,
            node_count,
            nodes,
        })
    }

    pub fn load_all<P: AsRef<Path>>(workflows_dir: P) -> Result<HashMap<String, Workflow>> {
        let mut workflows = HashMap::new();

        for entry in WalkDir::new(workflows_dir.as_ref())
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file()
                && path.extension().and_then(|s| s.to_str()) == Some("json")
                && path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .map(|s| s.starts_with("all-nodes-"))
                    .unwrap_or(false)
            {
                match Workflow::from_file(path) {
                    Ok(workflow) => {
                        workflows.insert(workflow.pack_name.clone(), workflow);
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                    }
                }
            }
        }

        Ok(workflows)
    }

    pub fn get_unique_node_types(&self) -> Vec<String> {
        let mut types: Vec<String> = self.nodes.iter().map(|n| n.node_type.clone()).collect();
        types.sort();
        types.dedup();
        types
    }
}
