mod args;

use anyhow::{Context, Result};
use args::{Commands, SniptipArgs};
use clap::Parser;

const SNIPS_PATH: &str = ".sniptip";

fn main() -> Result<()> {
    let cli = SniptipArgs::parse();

    // TODO(anfelo): Specify a different path on init
    // where the sniptips will be stored
    let home_path = std::env::var("HOME")?;
    let sniptips_dir = format!("{}/./{}", home_path, SNIPS_PATH);

    match &cli.command {
        Commands::Init => sniptip::init(&sniptips_dir)?,
        Commands::Add(args) => {
            let path = format!("{}/{}", sniptips_dir, &args.name);
            sniptip::add_snip(&path, &args.snip)
                .with_context(|| format!("Unable to add sniptip: {}", args.name))?;
        }
        Commands::Query { query } => {
            sniptip::query_snip(query, &sniptips_dir).with_context(|| {
                format!("Could not read sniptips store at path: {}", sniptips_dir)
            })?;
        }
        Commands::Show { name } => {
            let path = format!("{}/{}", sniptips_dir, name);
            sniptip::show_snip(&path).with_context(|| {
                format!("Unable to access sniptip <{}> at path: {}", name, path)
            })?;
        }
        Commands::Delete { name } => {
            let path = format!("{}/{}", sniptips_dir, name);
            sniptip::delete_snip(&path)?;
        }
    }

    Ok(())
}
