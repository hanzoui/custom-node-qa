use crate::models::{Checklist, Metadata};
use colored::Colorize;

pub struct ReportGenerator;

impl ReportGenerator {
    pub fn generate_text(checklist: &Checklist, metadata: Option<&Metadata>) -> String {
        let mut lines = Vec::new();

        lines.push(format!("📊 Project: {}", checklist.project_name.bold()));
        lines.push(String::new());

        if let Some(meta) = metadata {
            if let Some(stats) = &meta.stats {
                lines.push(format!("✅ Tested: {}/{}", stats.tested_packs, stats.total_packs));
                lines.push(format!("📈 Progress: {:.1}%", stats.completion_percent));
                lines.push(format!("🔢 Total Nodes: {}", stats.total_nodes));
                lines.push(String::new());
            }

            lines.push(format!("🌐 Environment: {}", meta.environment.url));
            if let Some(version) = &meta.environment.comfyui_version {
                lines.push(format!("   ComfyUI: {}", version));
            }
            lines.push(String::new());
        }

        let tested = checklist.packs.iter().filter(|p| p.tested).count();
        let total = checklist.packs.len();

        lines.push(format!("Packs: {}/{} tested", tested, total));
        lines.push(String::new());

        // Group by tested status
        let tested_packs: Vec<_> = checklist.packs.iter().filter(|p| p.tested).collect();
        let untested_packs: Vec<_> = checklist.packs.iter().filter(|p| !p.tested).collect();

        if !tested_packs.is_empty() {
            lines.push("✅ Tested:".green().to_string());
            for pack in tested_packs {
                lines.push(format!("   • {} ({})", pack.name, pack.node_count));
            }
            lines.push(String::new());
        }

        if !untested_packs.is_empty() {
            lines.push("⏳ Untested:".yellow().to_string());
            for pack in untested_packs {
                lines.push(format!("   • {} ({})", pack.name, pack.node_count));
            }
        }

        lines.join("\n")
    }

    pub fn generate_html(checklist: &Checklist, metadata: Option<&Metadata>) -> String {
        let tested = checklist.packs.iter().filter(|p| p.tested).count();
        let total = checklist.packs.len();
        let percent = if total > 0 {
            (tested as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>QA Report - {}</title>
    <style>
        body {{ font-family: system-ui, -apple-system, sans-serif; padding: 2rem; max-width: 900px; margin: 0 auto; }}
        h1 {{ color: #333; }}
        .stats {{ display: flex; gap: 2rem; margin: 2rem 0; }}
        .stat {{ background: #f5f5f5; padding: 1rem; border-radius: 8px; }}
        .stat-value {{ font-size: 2rem; font-weight: bold; color: #0066cc; }}
        .stat-label {{ color: #666; font-size: 0.9rem; }}
        .progress {{ background: #e0e0e0; height: 24px; border-radius: 12px; overflow: hidden; }}
        .progress-bar {{ background: #4caf50; height: 100%; transition: width 0.3s; }}
        table {{ width: 100%; border-collapse: collapse; margin-top: 2rem; }}
        th, td {{ padding: 0.75rem; text-align: left; border-bottom: 1px solid #ddd; }}
        th {{ background: #f5f5f5; font-weight: 600; }}
        .tested {{ color: #4caf50; }}
        .untested {{ color: #ff9800; }}
    </style>
</head>
<body>
    <h1>QA Report: {}</h1>
    <div class="stats">
        <div class="stat">
            <div class="stat-value">{}/{}</div>
            <div class="stat-label">Packs Tested</div>
        </div>
        <div class="stat">
            <div class="stat-value">{:.1}%</div>
            <div class="stat-label">Progress</div>
        </div>
    </div>
    <div class="progress">
        <div class="progress-bar" style="width: {:.1}%"></div>
    </div>
    <table>
        <thead>
            <tr>
                <th>Pack Name</th>
                <th>Node Count</th>
                <th>Status</th>
            </tr>
        </thead>
        <tbody>
            {}
        </tbody>
    </table>
</body>
</html>"#,
            checklist.project_name,
            checklist.project_name,
            tested,
            total,
            percent,
            percent,
            checklist
                .packs
                .iter()
                .map(|pack| {
                    let status_class = if pack.tested { "tested" } else { "untested" };
                    let status_text = if pack.tested { "✅ Tested" } else { "⏳ Untested" };
                    format!(
                        r#"<tr><td>{}</td><td>{}</td><td class="{}">{}</td></tr>"#,
                        pack.name, pack.node_count, status_class, status_text
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
