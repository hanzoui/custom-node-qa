use super::NodePack;
use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Checklist {
    pub file_path: PathBuf,
    pub project_name: String,
    pub packs: Vec<NodePack>,
}

impl Checklist {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read checklist: {}", path.display()))?;

        let project_name = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let packs = Self::parse_markdown(&contents)?;

        Ok(Checklist {
            file_path: path.to_path_buf(),
            project_name,
            packs,
        })
    }

    fn parse_markdown(contents: &str) -> Result<Vec<NodePack>> {
        let re = Regex::new(r"^- \[([ x])\] (.+?) \((\d+)\)")?;
        let mut packs = Vec::new();

        for line in contents.lines() {
            if let Some(caps) = re.captures(line) {
                let tested = &caps[1] == "x";
                let name = caps[2].trim().to_string();
                let node_count = caps[3].parse::<usize>()?;

                packs.push(NodePack::new(name, node_count, tested));
            }
        }

        Ok(packs)
    }

    pub fn to_markdown(&self) -> String {
        let mut lines = vec!["# Node Pack QA Checklist".to_string(), String::new()];

        for pack in &self.packs {
            let checkbox = if pack.tested { "x" } else { " " };
            lines.push(format!(
                "- [{}] {} ({})",
                checkbox, pack.name, pack.node_count
            ));
        }

        lines.join("\n")
    }

    pub fn write(&self) -> Result<()> {
        let contents = self.to_markdown();
        fs::write(&self.file_path, contents)?;
        Ok(())
    }
}
