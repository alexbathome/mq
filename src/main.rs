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
    let objects: Vec<Value> = crate::repository::read_json_objects(&cli.repo)?;
    let filtered = crate::filter::run_query(objects, &cli.filter)?;
    for v in filtered {
        let vv: Value = Val::into(v);
        println!("{}", serde_json::to_string_pretty(&vv)?);
    }
    Ok(())
}
