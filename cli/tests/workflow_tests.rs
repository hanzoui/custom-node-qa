use comfy_qa::models::Workflow;
use serde_json::json;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_workflow_parsing() {
    let workflow_json = json!({
        "id": "test-workflow",
        "nodes": [
            {"id": "1", "type": "NodeA"},
            {"id": "2", "type": "NodeB"},
            {"id": "3", "type": "NodeA"}
        ]
    });

    let temp_dir = TempDir::new().unwrap();
    let workflow_path = temp_dir.path().join("all-nodes-test-pack.json");
    fs::write(&workflow_path, workflow_json.to_string()).unwrap();

    let workflow = Workflow::from_file(&workflow_path).unwrap();

    assert_eq!(workflow.pack_name, "test-pack");
    assert_eq!(workflow.node_count, 3);
}

#[test]
fn test_workflow_unique_node_types() {
    let workflow_json = json!({
        "id": "test-workflow",
        "nodes": [
            {"id": "1", "type": "NodeA"},
            {"id": "2", "type": "NodeB"},
            {"id": "3", "type": "NodeA"},
            {"id": "4", "type": "NodeC"}
        ]
    });

    let temp_dir = TempDir::new().unwrap();
    let workflow_path = temp_dir.path().join("all-nodes-test-pack.json");
    fs::write(&workflow_path, workflow_json.to_string()).unwrap();

    let workflow = Workflow::from_file(&workflow_path).unwrap();
    let unique_types = workflow.get_unique_node_types();

    assert_eq!(unique_types.len(), 3);
    assert!(unique_types.contains(&"NodeA".to_string()));
    assert!(unique_types.contains(&"NodeB".to_string()));
    assert!(unique_types.contains(&"NodeC".to_string()));
}

#[test]
fn test_load_all_workflows() {
    let temp_dir = TempDir::new().unwrap();
    let workflows_dir = temp_dir.path().join("workflows");
    fs::create_dir(&workflows_dir).unwrap();

    let workflow1_json = json!({
        "id": "workflow1",
        "nodes": [
            {"id": "1", "type": "NodeA"}
        ]
    });

    let workflow2_json = json!({
        "id": "workflow2",
        "nodes": [
            {"id": "1", "type": "NodeB"},
            {"id": "2", "type": "NodeC"}
        ]
    });

    fs::write(
        workflows_dir.join("all-nodes-pack1.json"),
        workflow1_json.to_string(),
    )
    .unwrap();
    fs::write(
        workflows_dir.join("all-nodes-pack2.json"),
        workflow2_json.to_string(),
    )
    .unwrap();

    let workflows = Workflow::load_all(&workflows_dir).unwrap();

    assert_eq!(workflows.len(), 2);
    assert!(workflows.contains_key("pack1"));
    assert!(workflows.contains_key("pack2"));
    assert_eq!(workflows["pack1"].node_count, 1);
    assert_eq!(workflows["pack2"].node_count, 2);
}

#[test]
fn test_empty_workflow() {
    let workflow_json = json!({
        "id": "empty-workflow",
        "nodes": []
    });

    let temp_dir = TempDir::new().unwrap();
    let workflow_path = temp_dir.path().join("all-nodes-empty-pack.json");
    fs::write(&workflow_path, workflow_json.to_string()).unwrap();

    let workflow = Workflow::from_file(&workflow_path).unwrap();

    assert_eq!(workflow.pack_name, "empty-pack");
    assert_eq!(workflow.node_count, 0);
    assert_eq!(workflow.get_unique_node_types().len(), 0);
}
