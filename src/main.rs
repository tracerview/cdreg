mod cli;
mod commands;
mod register;
mod route;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init { zone } => commands::init::run(zone),
        Command::Add { name, worker, routes } => commands::add::run(name, worker, routes),
        Command::SetRepo { name, repo, force } => commands::set_repo::run(name, repo, force),
        Command::SetRoutes { name, routes } => commands::set_routes::run(name, routes),
        Command::Remove { name, yes } => commands::remove::run(name, yes),
        Command::Validate => commands::validate::run(),
        Command::Show => commands::show::run(),
        Command::Reset => commands::reset::run(),
    }
}
