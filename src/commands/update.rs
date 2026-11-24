use crate::register;
use crate::route;
use anyhow::{bail, Result};
use std::io::{self, Write};

fn confirm(prompt: &str) -> Result<bool> {
    print!("{} [y/N] ", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase() == "y")
}

pub fn run(name: String, repo: Option<String>, routes: Option<Vec<String>>) -> Result<()> {
    if repo.is_none() && routes.is_none() {
        bail!("at least one of --repo or --route must be provided");
    }

    if let Some(ref r) = repo {
        let parts: Vec<&str> = r.splitn(2, '/').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            bail!("repo must be in org/repo format: got {:?}", r);
        }
    }

    if let Some(ref rs) = routes {
        if rs.is_empty() {
            bail!("at least one route pattern is required");
        }
        for r in rs {
            route::validate(r)?;
        }
    }

    let mut reg = register::load()?;

    let (current_repo, current_routes, worker, zone_name) = {
        let part = reg.parts.get(&name).ok_or_else(|| {
            anyhow::anyhow!("part {:?} is not registered — run `cdreg add` first", name)
        })?;
        (
            part.repo.clone(),
            part.routes.clone(),
            part.worker.clone(),
            crate::terraform::infer_zone_name(&reg.zone),
        )
    };

    if let Some(ref new_repo) = repo {
        if current_repo != "TBC" {
            if !confirm(&format!("repo is currently {:?}. Update to {:?}?", current_repo, new_repo))? {
                println!("Aborted.");
                return Ok(());
            }
        }
    }

    if let Some(ref new_routes) = routes {
        let current = current_routes.join(", ");
        let new = new_routes.join(", ");
        if !confirm(&format!("routes are currently [{}]. Update to [{}]?", current, new))? {
            println!("Aborted.");
            return Ok(());
        }
    }

    let new_routes_snapshot = routes.as_ref().map(|rs| rs.clone());

    let part = reg.parts.get_mut(&name).unwrap();
    if let Some(r) = repo { part.repo = r; }
    if let Some(r) = routes { part.routes = r; }

    register::save(&reg)?;
    println!("Updated {:?}", name);

    if let Some(ref new_routes) = new_routes_snapshot {
        crate::terraform::update_workers_routes(&name, &zone_name, new_routes, &worker)?;
    }

    Ok(())
}
