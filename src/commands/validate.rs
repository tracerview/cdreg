use crate::register::{self, REGISTER_FILE};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn run() -> Result<()> {
    if !Path::new(REGISTER_FILE).exists() {
        anyhow::bail!("{} not found", REGISTER_FILE);
    }

    let raw = fs::read_to_string(REGISTER_FILE)?;

    let value: serde_json::Value = serde_json::from_str(&raw)
        .map_err(|e| anyhow::anyhow!("{} contains invalid JSON: {}", REGISTER_FILE, e))?;

    register::validate_against_schema(&value)?;

    // Parse into typed struct to check for TBC repos
    let reg: register::Register = serde_json::from_value(value)?;
    let tbc: Vec<&str> = reg
        .parts
        .iter()
        .filter(|(_, p)| p.repo == "TBC")
        .map(|(name, _)| name.as_str())
        .collect();

    for name in &tbc {
        eprintln!("warning: repo for {:?} is TBC", name);
    }

    println!("{} is valid", REGISTER_FILE);
    Ok(())
}
