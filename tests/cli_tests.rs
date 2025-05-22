// Integration tests for the CLI application
use assert_cmd::Command;
use assert_fs::TempDir;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("mq").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "JQ query for GitOps Kubernetes Manifest repositories",
        ));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("mq").unwrap();
    cmd.arg("--version").assert().success();
}

#[test]
fn test_cli_with_valid_arguments() {
    // Create a temporary test directory with a sample manifest
    let temp = TempDir::new().unwrap();
    let yaml_file = temp.child("deployment.yaml");
    yaml_file
        .write_str(
            r#"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx-deployment
spec:
  replicas: 3
"#,
        )
        .unwrap();

    // Run the command with a simple filter
    let mut cmd = Command::cargo_bin("mq").unwrap();
    cmd.arg(temp.path())
        .arg(".metadata.name")
        .assert()
        .success()
        .stdout(predicate::str::contains("nginx-deployment"));
}

#[test]
fn test_cli_with_filtering_by_kind() {
    // Create a temporary test directory with multiple manifests
    let temp = TempDir::new().unwrap();

    // Add a deployment
    let deployment = temp.child("deployment.yaml");
    deployment
        .write_str(
            r#"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx-deployment
"#,
        )
        .unwrap();

    // Add a service
    let service = temp.child("service.yaml");
    service
        .write_str(
            r#"
apiVersion: v1
kind: Service
metadata:
  name: nginx-service
"#,
        )
        .unwrap();

    // Run the command with a filter for Deployments only
    let mut cmd = Command::cargo_bin("mq").unwrap();
    cmd.arg(temp.path())
        .arg("select(.kind == \"Deployment\")")
        .assert()
        .success()
        .stdout(predicate::str::contains("nginx-deployment"))
        .stdout(predicate::str::contains("Deployment"))
        .stdout(predicate::str::contains("Service").not());
}

#[test]
fn test_cli_with_invalid_path() {
    let mut cmd = Command::cargo_bin("mq").unwrap();
    cmd.arg("path/that/does/not/exist")
        .arg(".kind")
        .assert()
        .failure();
}

#[test]
fn test_cli_with_invalid_filter() {
    // Create a temporary test directory with a sample manifest
    let temp = TempDir::new().unwrap();
    let yaml_file = temp.child("deployment.yaml");
    yaml_file
        .write_str(
            r#"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx-deployment
"#,
        )
        .unwrap();

    // Run with an invalid filter
    let mut cmd = Command::cargo_bin("mq").unwrap();
    cmd.arg(temp.path())
        .arg(".invalid | function()")
        .assert()
        .failure();
}
