use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    pub version: String,
    pub exported_at: DateTime<Utc>,
    pub project_name: String,
    pub environment: ExportEnvironment,
    pub packs: Vec<ExportPack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportEnvironment {
    pub url: String,
    pub user_agent: Option<String>,
    pub comfyui_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPack {
    pub name: String,
    pub node_count: usize,
    pub nodes: Vec<ExportNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportNode {
    pub name: String,
    pub display_name: Option<String>,
    pub deprecated: Option<bool>,
    pub category: Option<String>,
}

impl Export {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let export: Export = serde_json::from_str(&contents)?;
        Ok(export)
    }
}
