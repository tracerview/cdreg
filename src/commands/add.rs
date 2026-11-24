use crate::register::{self, Part};
use crate::route;
use anyhow::{bail, Result};

pub fn run(name: String, worker: String, routes: Vec<String>) -> Result<()> {
    if !name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        bail!("LOGICAL-NAME must contain only lowercase letters, digits, and hyphens: got {:?}", name);
    }

    if !worker.starts_with("cp-") {
        bail!("worker name must be prefixed with 'cp-': got {:?}", worker);
    }

    if routes.is_empty() {
        bail!("at least one --route is required");
    }

    for r in &routes {
        route::validate(r)?;
    }

    let mut reg = register::load()?;

    if reg.parts.contains_key(&name) {
        bail!("part {:?} already exists in the register", name);
    }

    reg.parts.insert(name.clone(), Part {
        repo: "TBC".to_string(),
        worker,
        routes,
    });

    register::save(&reg)?;
    println!("Added part {:?}", name);
    Ok(())
}
