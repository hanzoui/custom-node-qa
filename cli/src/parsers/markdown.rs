use regex::Regex;

pub struct MarkdownParser;

impl MarkdownParser {
    pub fn extract_checkbox_items(content: &str) -> Vec<(bool, String, Option<usize>)> {
        let re = Regex::new(r"^- \[([ x])\] (.+?)(?: \((\d+)\))?$").unwrap();
        let mut items = Vec::new();

        for line in content.lines() {
            if let Some(caps) = re.captures(line) {
                let checked = &caps[1] == "x";
                let name = caps[2].trim().to_string();
                let count = caps.get(3).and_then(|m| m.as_str().parse::<usize>().ok());

                items.push((checked, name, count));
            }
        }

        items
    }
}
