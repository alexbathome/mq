use clap::Parser;
use std::path::PathBuf;

/// Command line interface for the mq application
#[derive(Parser, Debug)]
#[command(author, version, about = "JQ query for GitOps Kubernetes Manifest repositories", long_about = None)]
pub struct Cli {
    /// Path to the repository containing Kubernetes manifests
    pub repo: PathBuf,

    /// JQ filter to apply to the manifests
    pub filter: String,
}
