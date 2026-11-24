use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cdreg", about = "Manage the composite domain register")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Init {
        #[arg(long)]
        zone: String,
    },
    Add {
        name: String,
        #[arg(long)]
        worker: String,
        #[arg(long = "route", required = true)]
        routes: Vec<String>,
    },
    SetRepo {
        name: String,
        repo: String,
        #[arg(long)]
        force: bool,
    },
    SetRoutes {
        name: String,
        routes: Vec<String>,
    },
    Remove {
        name: String,
        #[arg(long)]
        yes: bool,
    },
    Validate,
    Show,
}
