// // This file contains unit tests for the repository module's ability to parse YAML/JSON files

// use assert_fs::TempDir;
// use assert_fs::prelude::*;
// // use mq::repository::read_json_objects;
// use serde_json::Value;
// use std::path::Path;

// #[test]
// fn test_read_yaml_file() {
//     let temp = TempDir::new().unwrap();

//     // Create a test YAML file
//     let yaml_file = temp.child("test.yaml");
//     yaml_file
//         .write_str(
//             r#"
// apiVersion: v1
// kind: Pod
// metadata:
//   name: nginx
//   namespace: default
// spec:
//   containers:
//   - name: nginx
//     image: nginx:latest
// "#,
//         )
//         .unwrap();

//     // Read the objects
//     let objects = read_json_objects(temp.path()).unwrap();

//     // Verify the content
//     assert_eq!(objects.len(), 1);
//     assert_eq!(objects[0]["kind"], "Pod");
//     assert_eq!(objects[0]["metadata"]["name"], "nginx");
// }

// #[test]
// fn test_read_json_file() {
//     let temp = TempDir::new().unwrap();

//     // Create a test JSON file
//     let json_file = temp.child("test.json");
//     json_file
//         .write_str(
//             r#"
// {
//   "apiVersion": "v1",
//   "kind": "Service",
//   "metadata": {
//     "name": "nginx-service"
//   },
//   "spec": {
//     "selector": {
//       "app": "nginx"
//     },
//     "ports": [
//       {
//         "port": 80,
//         "targetPort": 80
//       }
//     ]
//   }
// }
// "#,
//         )
//         .unwrap();

//     // Read the objects
//     let objects = read_json_objects(temp.path()).unwrap();

//     // Verify the content
//     assert_eq!(objects.len(), 1);
//     assert_eq!(objects[0]["kind"], "Service");
//     assert_eq!(objects[0]["metadata"]["name"], "nginx-service");
// }

// #[test]
// fn test_multiple_yaml_documents() {
//     let temp = TempDir::new().unwrap();

//     // Create a YAML file with multiple documents
//     let yaml_file = temp.child("multi.yaml");
//     yaml_file
//         .write_str(
//             r#"
// apiVersion: v1
// kind: Pod
// metadata:
//   name: pod1
// ---
// apiVersion: v1
// kind: Pod
// metadata:
//   name: pod2
// "#,
//         )
//         .unwrap();

//     // Read the objects
//     let objects = read_json_objects(temp.path()).unwrap();

//     // Verify we got two objects
//     assert_eq!(objects.len(), 2);

//     // Check if we have both pod names
//     let names: Vec<_> = objects
//         .iter()
//         .map(|obj| obj["metadata"]["name"].as_str().unwrap())
//         .collect();

//     assert!(names.contains(&"pod1"));
//     assert!(names.contains(&"pod2"));
// }

// #[test]
// fn test_invalid_files_are_ignored() {
//     let temp = TempDir::new().unwrap();

//     // Create an invalid YAML file
//     let invalid_file = temp.child("invalid.yaml");
//     invalid_file
//         .write_str(
//             r#"
// this: is: not: valid: yaml
// "#,
//         )
//         .unwrap();

//     // Create a valid YAML file
//     let valid_file = temp.child("valid.yaml");
//     valid_file
//         .write_str(
//             r#"
// apiVersion: v1
// kind: ConfigMap
// metadata:
//   name: my-config
// "#,
//         )
//         .unwrap();

//     // Read the objects
//     let objects = read_json_objects(temp.path()).unwrap();

//     // Verify we got only the valid object
//     assert_eq!(objects.len(), 1);
//     assert_eq!(objects[0]["kind"], "ConfigMap");
// }

// #[test]
// fn test_recursively_finds_files() {
//     let temp = TempDir::new().unwrap();

//     // Create a nested directory structure
//     let subdir = temp.child("subdir");
//     subdir.create_dir_all().unwrap();

//     // Create files in root and subdirectory
//     let root_file = temp.child("root.yaml");
//     root_file
//         .write_str(
//             r#"
// apiVersion: v1
// kind: Service
// metadata:
//   name: root-service
// "#,
//         )
//         .unwrap();

//     let sub_file = subdir.child("nested.yaml");
//     sub_file
//         .write_str(
//             r#"
// apiVersion: v1
// kind: Service
// metadata:
//   name: nested-service
// "#,
//         )
//         .unwrap();

//     // Read the objects
//     let objects = read_json_objects(temp.path()).unwrap();

//     // Verify we got both objects
//     assert_eq!(objects.len(), 2);

//     // Check if we have both service names
//     let names: Vec<_> = objects
//         .iter()
//         .map(|obj| obj["metadata"]["name"].as_str().unwrap())
//         .collect();

//     assert!(names.contains(&"root-service"));
//     assert!(names.contains(&"nested-service"));
// }
