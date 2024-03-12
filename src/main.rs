mod args;

use anyhow::{Context, Result};
use args::{Commands, SniptipArgs};
use clap::Parser;

const SNIPS_PATH: &str = ".sniptip";

fn main() -> Result<()> {
    let cli = SniptipArgs::parse();

    let base_path = match std::env::var("SNIPS_BASE") {
        Ok(val) => val,
        Err(_) => std::env::var("HOME")?,
    };
    let sniptips_dir = format!("{}/{}", base_path, SNIPS_PATH);

    match &cli.command {
        Commands::Init => sniptip::init(&sniptips_dir)?,
        Commands::Add(args) => {
            let path = format!("{}/{}", sniptips_dir, &args.name);
            sniptip::check_init(&sniptips_dir);
            sniptip::add_snip(&path, &args.snip)
                .with_context(|| format!("Unable to add sniptip: {}", args.name))?;
        }
        Commands::AddClip { name } => {
            let path = format!("{}/{}", sniptips_dir, name);
            sniptip::check_init(&sniptips_dir);
            sniptip::add_snip_from_clipboard(&path)
                .with_context(|| format!("Unable to add sniptip from clipboard: {}", name))?;
        }
        Commands::Query { query } => {
            sniptip::check_init(&sniptips_dir);
            sniptip::query_snip(query, &sniptips_dir).with_context(|| {
                format!("Could not read sniptips store at path: {}", sniptips_dir)
            })?;
        }
        Commands::Show { name } => {
            sniptip::check_init(&sniptips_dir);
            let path = format!("{}/{}", sniptips_dir, name);
            sniptip::show_snip(&path).with_context(|| {
                format!("Unable to access sniptip <{}> at path: {}", name, path)
            })?;
        }
        Commands::List => {
            sniptip::check_init(&sniptips_dir);
            sniptip::list_snips(&sniptips_dir).with_context(|| {
                format!("Could not read sniptips store at path: {}", sniptips_dir)
            })?;
        }
        Commands::Delete { name } => {
            sniptip::check_init(&sniptips_dir);
            let path = format!("{}/{}", sniptips_dir, name);
            sniptip::delete_snip(&path)?;
        }
    }

    Ok(())
}
