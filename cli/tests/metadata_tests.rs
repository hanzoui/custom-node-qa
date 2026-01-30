use comfy_qa::models::Metadata;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_metadata_parsing() {
    let metadata_json = r#"{
        "project_name": "test-project",
        "created_at": "2024-01-01T00:00:00Z",
        "last_updated": "2024-01-02T00:00:00Z",
        "environment": {
            "type": "cloud",
            "url": "https://example.com",
            "comfyui_version": "v1.0.0"
        },
        "packs": {},
        "stats": {
            "total_packs": 0,
            "tested_packs": 0,
            "total_nodes": 0,
            "completion_percent": 0.0
        }
    }"#;

    let temp_dir = TempDir::new().unwrap();
    let metadata_path = temp_dir.path().join("metadata.json");
    fs::write(&metadata_path, metadata_json).unwrap();

    let metadata = Metadata::from_file(&metadata_path).unwrap();

    assert_eq!(metadata.project_name, "test-project");
    assert_eq!(metadata.environment.env_type, "cloud");
    assert_eq!(metadata.environment.url, "https://example.com");
    assert_eq!(
        metadata.environment.comfyui_version,
        Some("v1.0.0".to_string())
    );
}

#[test]
fn test_metadata_with_optional_fields() {
    let metadata_json = r#"{
        "project_name": "minimal-project",
        "created_at": "2024-01-01T00:00:00Z",
        "last_updated": null,
        "environment": {
            "type": "local",
            "url": "http://localhost:8188"
        },
        "packs": {},
        "stats": null
    }"#;

    let temp_dir = TempDir::new().unwrap();
    let metadata_path = temp_dir.path().join("metadata.json");
    fs::write(&metadata_path, metadata_json).unwrap();

    let metadata = Metadata::from_file(&metadata_path).unwrap();

    assert_eq!(metadata.project_name, "minimal-project");
    assert_eq!(metadata.environment.comfyui_version, None);
    assert!(metadata.stats.is_none());
}

#[test]
fn test_metadata_serialization() {
    let metadata_json = r#"{
        "project_name": "test-project",
        "created_at": "2024-01-01T00:00:00Z",
        "last_updated": "2024-01-02T00:00:00Z",
        "environment": {
            "type": "local",
            "url": "http://localhost:8188",
            "comfyui_version": "v1.2.3"
        },
        "packs": {},
        "stats": {
            "total_packs": 0,
            "tested_packs": 0,
            "total_nodes": 0,
            "completion_percent": 0.0
        }
    }"#;

    let temp_dir = TempDir::new().unwrap();
    let metadata_path = temp_dir.path().join("metadata.json");
    fs::write(&metadata_path, metadata_json).unwrap();

    let metadata = Metadata::from_file(&metadata_path).unwrap();

    let output_path = temp_dir.path().join("output.json");
    metadata.to_file(&output_path).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("test-project"));
    assert!(content.contains("v1.2.3"));
}
