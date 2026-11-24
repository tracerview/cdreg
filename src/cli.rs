use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cdreg", about = "Manage the composite domain register")]
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
        name: String,
        /// Cloudflare worker name (must start with cp-)
        #[arg(long)]
        worker: String,
        /// Route pattern (repeatable)
        #[arg(long = "route", required = true)]
        routes: Vec<String>,
    },
    /// Assign the GitHub repo to a part
    SetRepo {
        /// Logical name of the part
        name: String,
        /// GitHub repo in org/repo format
        repo: String,
        /// Overwrite if repo is already set
        #[arg(long)]
        force: bool,
    },
    /// Replace route patterns for a part
    SetRoutes {
        /// Logical name of the part
        name: String,
        /// One or more route patterns
        #[arg(required = true)]
        routes: Vec<String>,
    },
    /// Remove a composite part
    Remove {
        /// Logical name of the part
        name: String,
        /// Skip confirmation prompt
        #[arg(long)]
        yes: bool,
    },
    /// Validate the register against the schema
    Validate,
    /// Print a summary of the register
    Show,
    /// Delete the composite domain register
    Reset,
}
