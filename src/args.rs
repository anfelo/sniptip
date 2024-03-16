use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct SniptipArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Inits sniptip tool
    Init,

    /// Adds a sniptip
    Add(AddArgs),

    /// Queries for a sniptip
    Query { query: String },

    /// Shows a sniptip
    Show { name: String },

    /// Lists all sniptips
    List,

    /// Deletes a sniptip
    Delete { name: String },
}

#[derive(Args, Debug)]
pub struct AddArgs {
    pub name: String,
    pub snip: String,
}
