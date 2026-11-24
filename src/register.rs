use anyhow::Result;
use std::collections::HashMap;

pub const REGISTER_FILE: &str = "composite-domain-register.json";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Register {
    pub zone: String,
    pub parts: HashMap<String, Part>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Part {
    pub repo: String,
    pub routes: Vec<String>,
    pub worker: String,
}

pub fn load() -> Result<Register> {
    todo!()
}

pub fn save(_register: &Register) -> Result<()> {
    todo!()
}
