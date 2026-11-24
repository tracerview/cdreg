use anyhow::{bail, Context, Result};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

const TERRAFORM_DIR: &str = "terraform";
const WORKERS_TF: &str = "terraform/workers.tf";
const ROUTE_TYPE: &str = "cloudflare_workers_route";
const KNOWN_FIELDS: &[&str] = &["zone_id", "pattern", "script_name"];

pub fn infer_zone_name(zone: &str) -> String {
    let parts: Vec<&str> = zone.split('.').collect();
    let raw = if parts.len() >= 2 { parts[parts.len() - 2] } else { zone };
    raw.replace('-', "_")
}

pub fn add_workers_routes(name: &str, zone_name: &str, routes: &[String], worker: &str) -> Result<()> {
    if !Path::new(TERRAFORM_DIR).exists() {
        bail!(
            "terraform/ directory not found — follow the usage guide at https://github.com/tracerview/composite-domain before running this command"
        );
    }

    if Path::new(WORKERS_TF).exists() {
        let content = fs::read_to_string(WORKERS_TF)
            .with_context(|| format!("failed to read {WORKERS_TF}"))?;
        let existing = find_part_resource_names(&content, name);
        if !existing.is_empty() {
            bail!(
                "conflict in terraform/workers.tf: resource(s) for {:?} already exist — resolve manually", name
            );
        }
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(WORKERS_TF)
        .with_context(|| format!("failed to open {WORKERS_TF}"))?;

    for (i, route) in routes.iter().enumerate() {
        write!(file, "{}", route_block(&format!("{name}_{i}"), zone_name, route, worker))?;
    }

    println!("Added {} workers route(s) for {:?} to terraform/workers.tf", routes.len(), name);
    Ok(())
}

pub fn remove_workers_routes(name: &str) -> Result<()> {
    if !Path::new(WORKERS_TF).exists() {
        return Ok(());
    }

    let content = fs::read_to_string(WORKERS_TF)
        .with_context(|| format!("failed to read {WORKERS_TF}"))?;

    let existing = find_part_resource_names(&content, name);
    if existing.is_empty() {
        return Ok(());
    }

    let mut current = content;
    for rname in &existing {
        if let Some(updated) = remove_block(&current, rname) {
            current = updated;
        }
    }

    fs::write(WORKERS_TF, &current)
        .with_context(|| format!("failed to write {WORKERS_TF}"))?;
    println!("Removed {} workers route(s) for {:?} from terraform/workers.tf", existing.len(), name);
    Ok(())
}

pub fn update_workers_routes(name: &str, zone_name: &str, routes: &[String], worker: &str) -> Result<()> {
    if !Path::new(WORKERS_TF).exists() {
        return Ok(());
    }

    let content = fs::read_to_string(WORKERS_TF)
        .with_context(|| format!("failed to read {WORKERS_TF}"))?;

    let existing = find_part_resource_names(&content, name);
    if existing.is_empty() {
        return Ok(());
    }

    for rname in &existing {
        check_block_for_drift(&content, rname)?;
    }

    let mut current = content;
    for rname in &existing {
        if let Some(updated) = remove_block(&current, rname) {
            current = updated;
        }
    }

    fs::write(WORKERS_TF, &current)
        .with_context(|| format!("failed to write {WORKERS_TF}"))?;

    let mut file = OpenOptions::new()
        .append(true)
        .open(WORKERS_TF)
        .with_context(|| format!("failed to open {WORKERS_TF}"))?;

    for (i, route) in routes.iter().enumerate() {
        write!(file, "{}", route_block(&format!("{name}_{i}"), zone_name, route, worker))?;
    }

    println!("Updated {} workers route(s) for {:?} in terraform/workers.tf", routes.len(), name);
    Ok(())
}

fn route_block(resource_name: &str, zone_name: &str, pattern: &str, worker: &str) -> String {
    format!(
        "\nresource \"{ROUTE_TYPE}\" \"{resource_name}\" {{\n  zone_id     = cloudflare_zone.{zone_name}.id\n  pattern     = \"{pattern}\"\n  script_name = \"{worker}\"\n}}\n"
    )
}

fn find_part_resource_names(content: &str, part_name: &str) -> Vec<String> {
    let mut names = Vec::new();

    // Legacy non-indexed name
    if content.contains(&format!("resource \"{ROUTE_TYPE}\" \"{part_name}\"")) {
        names.push(part_name.to_string());
    }

    // Indexed names: name_0, name_1, ...
    let mut i = 0;
    loop {
        let indexed = format!("{part_name}_{i}");
        if content.contains(&format!("resource \"{ROUTE_TYPE}\" \"{indexed}\"")) {
            names.push(indexed);
            i += 1;
        } else {
            break;
        }
    }

    names
}

fn check_block_for_drift(content: &str, resource_name: &str) -> Result<()> {
    let marker = format!("resource \"{ROUTE_TYPE}\" \"{resource_name}\"");
    let start = content.find(&marker)
        .ok_or_else(|| anyhow::anyhow!("block {resource_name:?} not found"))?;

    let mut depth: i32 = 0;
    let mut block_end = content.len();
    for (i, ch) in content[start..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 { block_end = start + i + 1; break; }
            }
            _ => {}
        }
    }

    for line in content[start..block_end].lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed == "{" || trimmed == "}" || trimmed.starts_with("resource ") {
            continue;
        }
        if !KNOWN_FIELDS.iter().any(|f| trimmed.starts_with(f)) {
            bail!(
                "drift detected in terraform/workers.tf: \"{resource_name}\" contains unexpected content: {trimmed:?} — resolve manually before running this command"
            );
        }
    }

    Ok(())
}

fn remove_block(content: &str, name: &str) -> Option<String> {
    let marker = format!("resource \"{ROUTE_TYPE}\" \"{name}\"");
    let start = content.find(&marker)?;

    let line_start = content[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);

    let block_start = if line_start > 0 {
        let prev_nl = content[..line_start - 1].rfind('\n').map(|i| i + 1).unwrap_or(0);
        if content[prev_nl..line_start - 1].trim().is_empty() { prev_nl } else { line_start }
    } else {
        0
    };

    let mut depth: i32 = 0;
    let mut block_end = content.len();
    for (i, ch) in content[start..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    let pos = start + i;
                    block_end = content[pos..].find('\n').map(|j| pos + j + 1).unwrap_or(content.len());
                    break;
                }
            }
            _ => {}
        }
    }

    Some(format!("{}{}", &content[..block_start], &content[block_end..]))
}
