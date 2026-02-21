use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub project_name: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: Option<DateTime<Utc>>,
    pub environment: Environment,
    pub packs: HashMap<String, PackMetadata>,
    pub stats: Option<Stats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    #[serde(rename = "type")]
    pub env_type: String,
    pub url: String,
    pub comfyui_version: Option<String>,
    pub frontend_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackMetadata {
    pub node_count: usize,
    pub tested: bool,
    pub workflow_file: Option<String>,
    pub notes: Option<String>,
    pub issues: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub total_packs: usize,
    pub tested_packs: usize,
    pub total_nodes: usize,
    pub completion_percent: f64,
}

impl Metadata {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let metadata: Metadata = serde_json::from_str(&contents)?;
        Ok(metadata)
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }

    pub fn calculate_stats(&mut self) {
        let total_packs = self.packs.len();
        let tested_packs = self.packs.values().filter(|p| p.tested).count();
        let total_nodes = self.packs.values().map(|p| p.node_count).sum();
        let completion_percent = if total_packs > 0 {
            (tested_packs as f64 / total_packs as f64) * 100.0
        } else {
            0.0
        };

        self.stats = Some(Stats {
            total_packs,
            tested_packs,
            total_nodes,
            completion_percent,
        });
        self.last_updated = Some(Utc::now());
    }
}
