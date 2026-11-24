use crate::register::REGISTER_FILE;
use anyhow::{bail, Result};
use std::io::{self, Write};
use std::path::Path;

pub fn run() -> Result<()> {
    if !Path::new(REGISTER_FILE).exists() {
        bail!("composite domain register not found — nothing to reset");
    }

    println!("This will permanently delete the composite domain register ({}).", REGISTER_FILE);
    print!("Type 'yes' to confirm: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim() != "yes" {
        println!("Aborted.");
        return Ok(());
    }

    std::fs::remove_file(REGISTER_FILE)?;
    println!("Composite domain register deleted.");
    Ok(())
}
