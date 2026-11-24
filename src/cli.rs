use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cdreg", about = "Manage the composite domain register", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Create a new composite-domain-register.json
    Init {
        /// The domain zone (e.g. example.com)
        #[arg(long)]
        zone: String,
    },
    /// Register a new composite part
    Add {
        /// Logical name (kebab-case)
        part_name: String,
        /// Cloudflare worker name (must start with cp-)
        #[arg(long)]
        worker: String,
        /// Route pattern (repeatable)
        #[arg(long = "route", required = true)]
        routes: Vec<String>,
    },
    /// Update repo or routes for a part
    Update {
        /// Logical name of the part
        part_name: String,
        /// GitHub repo in org/repo format
        #[arg(long)]
        repo: Option<String>,
        /// Route pattern (repeatable)
        #[arg(long = "routes", num_args = 1..)]
        routes: Option<Vec<String>>,
    },
    /// Remove a composite part
    Remove {
        /// Logical name of the part
        part_name: String,
        /// Skip confirmation prompt
        #[arg(long)]
        yes: bool,
    },
    /// Validate the register against the schema
    Validate,
    /// Print a summary of the register
    Show,
}
