use crate::register;
use anyhow::{bail, Result};
use std::io::{self, Write};

pub fn run(name: String, yes: bool) -> Result<()> {
    let mut reg = register::load()?;

    if !reg.parts.contains_key(&name) {
        bail!("part {:?} is not registered", name);
    }

    if !yes {
        print!("Remove part {:?}? [y/N] ", name);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            println!("Aborted.");
            return Ok(());
        }
    }

    reg.parts.remove(&name);
    register::save(&reg)?;
    println!("Removed part {:?}", name);

    crate::terraform::remove_workers_route(&name)?;

    Ok(())
}
