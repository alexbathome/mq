use jaq_json::Val;
use serde_json::Value;

// Export our modules so they can be used by tests
pub mod cli;
pub mod filter;
pub mod repository;

use anyhow::Result;
use clap::Parser;

// Import our CLI module instead of defining it inline
use cli::Cli;

fn main() -> Result<()> {
    // Parse command-line arguments using clap
    let cli = Cli::parse();

    // Run the application with the parsed CLI arguments
    run(cli)?;

    Ok(())
}

/// Run the CLI application logic: read objects, apply filter, and print results
pub fn run(cli: crate::cli::Cli) -> Result<()> {
    let queried: Vec<Value> =
        crate::repository::read_json_objects_with_query(&cli.repo, &cli.filter)?;
    for v in queried {
        println!("{}", serde_json::to_string_pretty(&v)?);
    }
    Ok(())
}
