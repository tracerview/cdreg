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
        Command::Update { name, repo, routes } => commands::update::run(name, repo, routes),
        Command::Remove { name, yes } => commands::remove::run(name, yes),
        Command::Validate => commands::validate::run(),
        Command::Show => commands::show::run(),
    }
}
