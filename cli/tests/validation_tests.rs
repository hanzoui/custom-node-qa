use comfy_qa::models::Checklist;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_valid_checklist_parsing() {
    let checklist_content = r#"# Hanzo Studio QA Test - valid-project

- [x] Pack One (10)
- [x] Pack Two (5)
"#;

    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("valid-project");
    fs::create_dir(&project_dir).unwrap();
    let checklist_path = project_dir.join("checklist.md");
    fs::write(&checklist_path, checklist_content).unwrap();

    let result = Checklist::from_file(&checklist_path);

    assert!(result.is_ok());
    let checklist = result.unwrap();
    assert_eq!(checklist.packs.len(), 2);
}

#[test]
fn test_checklist_with_no_packs() {
    let checklist_content = r#"# Hanzo Studio QA Test - test-project

No packs listed
"#;

    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("test-project");
    fs::create_dir(&project_dir).unwrap();
    let checklist_path = project_dir.join("checklist.md");
    fs::write(&checklist_path, checklist_content).unwrap();

    let result = Checklist::from_file(&checklist_path);

    assert!(result.is_ok());
    let checklist = result.unwrap();
    assert_eq!(checklist.packs.len(), 0);
}

#[test]
fn test_empty_file() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("empty-project");
    fs::create_dir(&project_dir).unwrap();
    let checklist_path = project_dir.join("empty.md");
    fs::write(&checklist_path, "").unwrap();

    let result = Checklist::from_file(&checklist_path);

    assert!(result.is_ok());
}

#[test]
fn test_checklist_multiple_formats() {
    let checklist_content = r#"# Hanzo Studio QA Test - test-project

- [x] FirstPack (10)
- [ ] SecondPack (20)
- [x] ThirdPack (5)
"#;

    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("test-project");
    fs::create_dir(&project_dir).unwrap();
    let checklist_path = project_dir.join("checklist.md");
    fs::write(&checklist_path, checklist_content).unwrap();

    let checklist = Checklist::from_file(&checklist_path).unwrap();

    assert_eq!(checklist.packs.len(), 3);
    assert!(checklist.packs[0].tested);
    assert!(!checklist.packs[1].tested);
    assert!(checklist.packs[2].tested);
}
