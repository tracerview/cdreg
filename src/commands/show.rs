use crate::register;
use anyhow::Result;

pub fn run() -> Result<()> {
    let reg = register::load()?;

    println!("Zone: {}\n", reg.zone);

    if reg.parts.is_empty() {
        println!("No parts registered.");
        return Ok(());
    }

    // Column widths
    let name_w = reg.parts.keys().map(|k| k.len()).max().unwrap_or(4).max(4);
    let worker_w = reg.parts.values().map(|p| p.worker.len()).max().unwrap_or(6).max(6);
    let repo_w = reg.parts.values().map(|p| p.repo.len()).max().unwrap_or(4).max(4);

    println!(
        "{:<name_w$}  {:<worker_w$}  {:<repo_w$}  routes",
        "NAME", "WORKER", "REPO",
        name_w = name_w,
        worker_w = worker_w,
        repo_w = repo_w,
    );
    println!("{}", "-".repeat(name_w + worker_w + repo_w + 20));

    let mut names: Vec<&String> = reg.parts.keys().collect();
    names.sort();

    for name in names {
        let part = &reg.parts[name];
        let repo_display = if part.repo == "TBC" {
            format!("{} [!]", part.repo)
        } else {
            part.repo.clone()
        };
        println!(
            "{:<name_w$}  {:<worker_w$}  {:<repo_w$}  {}",
            name,
            part.worker,
            repo_display,
            part.routes.join(", "),
            name_w = name_w,
            worker_w = worker_w,
            repo_w = repo_w,
        );
    }

    Ok(())
}
