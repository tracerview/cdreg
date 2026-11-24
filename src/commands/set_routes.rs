use crate::register;
use crate::route;
use anyhow::{bail, Result};

pub fn run(name: String, routes: Vec<String>) -> Result<()> {
    if routes.is_empty() {
        bail!("at least one route pattern is required");
    }

    for r in &routes {
        route::validate(r)?;
    }

    let mut reg = register::load()?;

    let part = reg.parts.get_mut(&name).ok_or_else(|| {
        anyhow::anyhow!("part {:?} is not registered — run `cdreg add` first", name)
    })?;

    part.routes = routes.clone();
    register::save(&reg)?;
    println!("Updated routes for {:?}", name);
    Ok(())
}
