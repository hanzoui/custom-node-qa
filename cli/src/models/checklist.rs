use super::NodePack;
use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Checklist {
    pub file_path: PathBuf,
    pub project_name: String,
    pub packs: Vec<NodePack>,
}

#[derive(Debug, Clone)]
pub struct DetailedChecklist {
    pub packs: HashMap<String, Vec<String>>, // pack_name -> vec of node names
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

impl DetailedChecklist {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read checklist: {}", path.display()))?;

        let mut packs: HashMap<String, Vec<String>> = HashMap::new();
        let mut current_pack: Option<String> = None;

        // Regex to match pack headers: ## PackName
        let pack_header_re = Regex::new(r"^##\s+(.+)$")?;
        // Regex to match node items: - [ ] NodeName or - [x] NodeName
        let node_re = Regex::new(r"^-\s+\[[x ]\]\s+(.+)$")?;

        for line in contents.lines() {
            if let Some(caps) = pack_header_re.captures(line) {
                let pack_name = caps[1].trim().to_string();
                current_pack = Some(pack_name.clone());
                packs.entry(pack_name).or_insert_with(Vec::new);
            } else if let Some(caps) = node_re.captures(line) {
                if let Some(pack_name) = &current_pack {
                    let node_name = caps[1].trim().to_string();
                    if let Some(nodes) = packs.get_mut(pack_name) {
                        nodes.push(node_name);
                    }
                }
            }
        }

        Ok(DetailedChecklist { packs })
    }

    pub fn get_nodes(&self, pack_name: &str) -> Option<&Vec<String>> {
        self.packs.get(pack_name)
    }
}
