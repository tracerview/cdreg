use anyhow::{bail, Context, Result};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

const TERRAFORM_DIR: &str = "terraform";
const WORKERS_TF: &str = "terraform/workers.tf";
const ROUTE_TYPE: &str = "cloudflare_workers_route";

pub fn infer_zone_name(zone: &str) -> String {
    let parts: Vec<&str> = zone.split('.').collect();
    let raw = if parts.len() >= 2 { parts[parts.len() - 2] } else { zone };
    raw.replace('-', "_")
}

pub fn add_workers_route(name: &str, zone_name: &str, pattern: &str, worker: &str) -> Result<()> {
    if !Path::new(TERRAFORM_DIR).exists() {
        bail!(
            "terraform/ directory not found — follow the usage guide at https://github.com/tracerview/composite-domain before running this command"
        );
    }

    let marker = format!("resource \"{ROUTE_TYPE}\" \"{name}\"");

    if Path::new(WORKERS_TF).exists() {
        let content = fs::read_to_string(WORKERS_TF)
            .with_context(|| format!("failed to read {WORKERS_TF}"))?;
        if content.contains(&marker) {
            bail!(
                "conflict in terraform/workers.tf: {marker} already exists — resolve manually"
            );
        }
    }

    let block = format!(
        "\nresource \"{ROUTE_TYPE}\" \"{name}\" {{\n  zone_id     = cloudflare_zone.{zone_name}.id\n  pattern     = \"{pattern}\"\n  script_name = \"{worker}\"\n}}\n"
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(WORKERS_TF)
        .with_context(|| format!("failed to open {WORKERS_TF}"))?;
    write!(file, "{block}")?;
    println!("Added workers route for {:?} to terraform/workers.tf", name);
    Ok(())
}

pub fn remove_workers_route(name: &str) -> Result<()> {
    if !Path::new(WORKERS_TF).exists() {
        return Ok(());
    }

    let content = fs::read_to_string(WORKERS_TF)
        .with_context(|| format!("failed to read {WORKERS_TF}"))?;

    let marker = format!("resource \"{ROUTE_TYPE}\" \"{name}\"");
    if !content.contains(&marker) {
        return Ok(());
    }

    let new_content = remove_block(&content, name)
        .ok_or_else(|| anyhow::anyhow!(
            "failed to locate resource block for {:?} in terraform/workers.tf", name
        ))?;

    fs::write(WORKERS_TF, new_content)
        .with_context(|| format!("failed to write {WORKERS_TF}"))?;
    println!("Removed workers route for {:?} from terraform/workers.tf", name);
    Ok(())
}

pub fn update_workers_route_pattern(name: &str, new_pattern: &str) -> Result<()> {
    if !Path::new(WORKERS_TF).exists() {
        return Ok(());
    }

    let content = fs::read_to_string(WORKERS_TF)
        .with_context(|| format!("failed to read {WORKERS_TF}"))?;

    let marker = format!("resource \"{ROUTE_TYPE}\" \"{name}\"");
    if !content.contains(&marker) {
        return Ok(());
    }

    let new_content = replace_pattern_in_block(&content, name, new_pattern)
        .ok_or_else(|| anyhow::anyhow!(
            "failed to update pattern for {:?} in terraform/workers.tf", name
        ))?;

    fs::write(WORKERS_TF, new_content)
        .with_context(|| format!("failed to write {WORKERS_TF}"))?;
    println!("Updated workers route pattern for {:?} in terraform/workers.tf", name);
    Ok(())
}

fn replace_pattern_in_block(content: &str, name: &str, new_pattern: &str) -> Option<String> {
    let marker = format!("resource \"{ROUTE_TYPE}\" \"{name}\"");
    let block_start = content.find(&marker)?;

    // Find the closing brace of the block
    let mut depth: i32 = 0;
    let mut block_end = content.len();
    for (i, ch) in content[block_start..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    block_end = block_start + i + 1;
                    break;
                }
            }
            _ => {}
        }
    }

    let block = &content[block_start..block_end];

    // Find and replace the quoted value on the pattern line
    let pattern_prefix = "pattern     = \"";
    let rel_start = block.find(pattern_prefix)?;
    let after_prefix = &block[rel_start + pattern_prefix.len()..];
    let rel_end = after_prefix.find('"')?;

    let abs_prefix_end = block_start + rel_start + pattern_prefix.len();
    let abs_value_end = abs_prefix_end + rel_end;

    Some(format!(
        "{}{}{}",
        &content[..abs_prefix_end],
        new_pattern,
        &content[abs_value_end..]
    ))
}

fn remove_block(content: &str, name: &str) -> Option<String> {
    let marker = format!("resource \"{ROUTE_TYPE}\" \"{name}\"");
    let start = content.find(&marker)?;

    // Start of the line containing the marker
    let line_start = content[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);

    // Include the preceding blank line if there is one
    let block_start = if line_start > 1 {
        let prev_nl = content[..line_start - 1].rfind('\n').map(|i| i + 1).unwrap_or(0);
        if content[prev_nl..line_start - 1].trim().is_empty() {
            prev_nl
        } else {
            line_start
        }
    } else {
        line_start
    };

    // Find the closing brace (tracking depth)
    let mut depth: i32 = 0;
    let mut block_end = content.len();
    for (i, ch) in content[start..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    let pos = start + i;
                    block_end = content[pos..].find('\n')
                        .map(|j| pos + j + 1)
                        .unwrap_or(content.len());
                    break;
                }
            }
            _ => {}
        }
    }

    Some(format!("{}{}", &content[..block_start], &content[block_end..]))
}
