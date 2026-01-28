use comfy_qa::models::Checklist;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_checklist_parsing() {
    let checklist_content = r#"# ComfyUI QA Test - test-project

- [x] Pack One (20)
- [ ] Pack Two (15)
"#;

    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("test-project");
    fs::create_dir(&project_dir).unwrap();
    let checklist_path = project_dir.join("checklist.md");
    fs::write(&checklist_path, checklist_content).unwrap();

    let checklist = Checklist::from_file(&checklist_path).unwrap();

    assert_eq!(checklist.project_name, "test-project");
    assert_eq!(checklist.packs.len(), 2);
    assert_eq!(checklist.packs[0].name, "Pack One");
    assert_eq!(checklist.packs[0].node_count, 20);
    assert!(checklist.packs[0].tested);
    assert_eq!(checklist.packs[1].name, "Pack Two");
    assert_eq!(checklist.packs[1].node_count, 15);
    assert!(!checklist.packs[1].tested);
}

#[test]
fn test_checklist_tested_status() {
    let checklist_content = r#"# ComfyUI QA Test - test-project

- [x] TestedPack (10)
- [ ] UntestedPack (5)
"#;

    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("test-project");
    fs::create_dir(&project_dir).unwrap();
    let checklist_path = project_dir.join("checklist.md");
    fs::write(&checklist_path, checklist_content).unwrap();

    let checklist = Checklist::from_file(&checklist_path).unwrap();

    assert_eq!(checklist.packs.len(), 2);
    assert!(checklist.packs[0].tested);
    assert!(!checklist.packs[1].tested);
}

#[test]
fn test_checklist_empty_project() {
    let checklist_content = r#"# ComfyUI QA Test - empty-project
"#;

    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("empty-project");
    fs::create_dir(&project_dir).unwrap();
    let checklist_path = project_dir.join("checklist.md");
    fs::write(&checklist_path, checklist_content).unwrap();

    let checklist = Checklist::from_file(&checklist_path).unwrap();

    assert_eq!(checklist.project_name, "empty-project");
    assert_eq!(checklist.packs.len(), 0);
}

#[test]
fn test_checklist_with_special_characters() {
    let checklist_content = r#"# ComfyUI QA Test - test-project

- [x] Pack-With-Dashes (5)
- [ ] Pack_With_Underscores (10)
"#;

    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("test-project");
    fs::create_dir(&project_dir).unwrap();
    let checklist_path = project_dir.join("checklist.md");
    fs::write(&checklist_path, checklist_content).unwrap();

    let checklist = Checklist::from_file(&checklist_path).unwrap();

    assert_eq!(checklist.packs[0].name, "Pack-With-Dashes");
    assert_eq!(checklist.packs[0].node_count, 5);
    assert_eq!(checklist.packs[1].name, "Pack_With_Underscores");
    assert_eq!(checklist.packs[1].node_count, 10);
}
