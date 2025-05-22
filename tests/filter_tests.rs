// This file contains unit tests for the filter module, ensuring that the filter logic
// correctly applies the JQ-like queries to JSON values.

// use mq::filter::run_query;
use serde_json::Value;

// #[test]
// fn test_simple_filter() {
//     let input: Vec<Value> = vec![
//         serde_json::json!({
//             "name": "pod1",
//             "status": "Running"
//         }),
//         serde_json::json!({
//             "name": "pod2",
//             "status": "Pending"
//         }),
//     ];

//     let filter = r#"select(.status == "Running")"#;
//     let result = run_query(input, filter).unwrap();

//     assert_eq!(result.len(), 1);
//     assert_eq!(result[0]["name"], "pod1");
// }

// #[test]
// fn test_length_function() {
//     let input: Vec<Value> = vec![
//         serde_json::json!({
//             "name": "pod1",
//             "status": "Running"
//         }),
//         serde_json::json!({
//             "name": "pod2",
//             "status": "Pending"
//         }),
//     ];

//     let filter = "length";
//     let result = run_query(input, filter).unwrap();

//     assert_eq!(result.len(), 1);
//     assert_eq!(result[0], 2);
// }

// #[test]
// fn test_pipe_operations() {
//     let input: Vec<Value> = vec![
//         serde_json::json!({
//             "name": "pod1",
//             "status": "Running"
//         }),
//         serde_json::json!({
//             "name": "pod2",
//             "status": "Pending"
//         }),
//     ];

//     let filter = r#".[] | select(.status == "Running") | .name"#;
//     let result = run_query(input, filter).unwrap();

//     assert_eq!(result.len(), 1);
//     assert_eq!(result[0], "pod1");
// }

// #[test]
// fn test_extract_field() {
//     let input: Vec<Value> = vec![serde_json::json!({
//         "metadata": {
//             "name": "deployment-1",
//             "namespace": "default"
//         },
//         "kind": "Deployment",
//         "spec": {
//             "replicas": 3
//         }
//     })];

//     let filter = ".[] | .metadata.name";
//     let result = run_query(input, filter).unwrap();

//     assert_eq!(result.len(), 1);
//     assert_eq!(result[0], "deployment-1");
// }

// #[test]
// fn test_kubernetes_specific_filter() {
//     let input: Vec<Value> = vec![
//         serde_json::json!({
//             "metadata": {
//                 "name": "deployment-1",
//             },
//             "kind": "Deployment",
//         }),
//         serde_json::json!({
//             "metadata": {
//                 "name": "service-1",
//             },
//             "kind": "Service",
//         }),
//     ];

//     let filter = r#".[] | select(.kind == "Deployment") | .metadata.name"#;
//     let result = run_query(input, filter).unwrap();

//     assert_eq!(result.len(), 1);
//     assert_eq!(result[0], "deployment-1");
// }

// #[test]
// fn test_invalid_filter() {
//     let input: Vec<Value> = vec![serde_json::json!({
//         "name": "pod1"
//     })];

//     let filter = ".[] | invalid_function()";
//     let result = run_query(input, filter);

//     assert!(result.is_err());
// }
