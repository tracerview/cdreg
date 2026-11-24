use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub const REGISTER_FILE: &str = "composite-domain-register.json";

const SCHEMA: &str = include_str!("../schema.json");

#[derive(Debug, Serialize, Deserialize)]
pub struct Register {
    pub zone: String,
    pub parts: HashMap<String, Part>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    pub repo: String,
    pub routes: Vec<String>,
    pub worker: String,
}

pub fn load() -> Result<Register> {
    let path = Path::new(REGISTER_FILE);
    if !path.exists() {
        bail!(
            "{} not found — run `cdreg init --zone <DOMAIN>` first",
            REGISTER_FILE
        );
    }
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", REGISTER_FILE))?;
    let register: Register = serde_json::from_str(&raw)
        .with_context(|| format!("{} contains invalid JSON", REGISTER_FILE))?;
    Ok(register)
}

pub fn save(register: &Register) -> Result<()> {
    let value = serde_json::to_value(register)?;
    validate_against_schema(&value)?;
    let json = serde_json::to_string_pretty(register)?;
    fs::write(REGISTER_FILE, json + "\n")
        .with_context(|| format!("failed to write {}", REGISTER_FILE))?;
    Ok(())
}

pub fn validate_against_schema(value: &Value) -> Result<()> {
    let schema: Value = serde_json::from_str(SCHEMA).expect("embedded schema is valid JSON");
    let compiled = jsonschema::validator_for(&schema)
        .expect("embedded schema compiles");
    let errors: Vec<_> = compiled.iter_errors(value).collect();
    if errors.is_empty() {
        return Ok(());
    }
    let messages: Vec<String> = errors
        .iter()
        .map(|e| format!("  - {} (at {})", e, e.instance_path()))
        .collect();
    bail!("schema validation failed:\n{}", messages.join("\n"));
}
