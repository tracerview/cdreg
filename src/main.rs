mod cli;
mod commands;
mod register;
mod route;
mod terraform;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init { zone } => commands::init::run(zone),
        Command::Add { part_name, worker, routes } => commands::add::run(part_name, worker, routes),
        Command::Update { part_name, repo, routes } => commands::update::run(part_name, repo, routes),
        Command::Remove { part_name, yes } => commands::remove::run(part_name, yes),
        Command::Validate => commands::validate::run(),
        Command::Show => commands::show::run(),
        Command::Reset => commands::reset::run(),
    }
}
