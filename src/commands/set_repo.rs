use crate::register;
use anyhow::{bail, Result};

pub fn run(name: String, repo: String, force: bool) -> Result<()> {
    let parts: Vec<&str> = repo.splitn(2, '/').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        bail!("repo must be in org/repo format: got {:?}", repo);
    }

    let mut reg = register::load()?;

    let part = reg.parts.get_mut(&name).ok_or_else(|| {
        anyhow::anyhow!("part {:?} is not registered — run `cdreg add` first", name)
    })?;

    if part.repo != "TBC" && !force {
        bail!(
            "repo for {:?} is already set to {:?} — use --force to overwrite",
            name,
            part.repo
        );
    }

    part.repo = repo.clone();
    register::save(&reg)?;
    println!("Set repo for {:?} to {:?}", name, repo);
    Ok(())
}
