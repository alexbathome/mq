# mq - Manifest Query

A command-line tool for querying Kubernetes manifests in GitOps repositories using JQ-like filters.

## Overview

`mq` is designed to help you quickly extract and filter information from Kubernetes manifest repositories. It recursively scans a directory for YAML/JSON files containing Kubernetes manifests and allows you to apply JQ filters to find the information you need.

Key features:
- Recursively processes YAML/JSON files in a directory
- Handles multi-document YAML files 
- Uses Rayon for parallel file processing
- Applies JQ filters to the parsed manifests
- Outputs results as pretty-printed JSON

## Installation

```
cargo install --path .
```

## Usage

```
mq <REPOSITORY_PATH> <FILTER>
```

Where:
- `REPOSITORY_PATH` is the path to a directory containing Kubernetes manifest files
- `FILTER` is a JQ filter expression

## Examples

Get all resource names:
```
mq ./my-repo '.metadata.name'
```

Get all Deployments:
```
mq ./my-repo 'select(.kind=="Deployment")'
```

Get all containers in Deployments:
```
mq ./my-repo 'select(.kind=="Deployment") | .spec.template.spec.containers[]'
```

Count resources by kind:
```
mq ./my-repo 'group_by(.kind) | map({kind: .[0].kind, count: length}) | sort_by(.kind)'
```

## JQ Filter Reference

For detailed information on JQ filter syntax, see [JQ Manual](https://stedolan.github.io/jq/manual/).

Some common filter patterns:

- `.metadata.name`: Extract the name field
- `select(.kind=="Deployment")`: Filter by resource kind
- `.items[]`: Extract items from a list
- `map(...)`: Transform each item in a list
- `sort_by(...)`: Sort items by a key
- `group_by(...)`: Group items by a key
- `length`: Count items

## Development

### Building

```
cargo build
```

### Running Tests

```
cargo test
```

## License

MIT