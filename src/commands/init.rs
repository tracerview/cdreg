use crate::register::{Register, REGISTER_FILE};
use anyhow::{bail, Result};
use std::collections::HashMap;
use std::path::Path;

pub fn run(zone: String) -> Result<()> {
    if zone.contains(' ') || !zone.contains('.') {
        bail!("--zone must be a valid domain (e.g. example.com): got {:?}", zone);
    }

    if Path::new(REGISTER_FILE).exists() {
        bail!("composite domain register already exists — run `cdreg reset` if you want to reinitialise");
    }

    let register = Register {
        zone,
        parts: HashMap::new(),
    };

    crate::register::save(&register)?;
    println!("Created composite domain register");
    Ok(())
}
