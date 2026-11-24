use crate::register::{Register, REGISTER_FILE};
use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

const TERRAFORM_MAIN: &str = "terraform/main.tf";
const APEX_MARKER: &str = "resource \"cloudflare_record\" \"apex\"";

fn infer_zone_name(zone: &str) -> String {
    let parts: Vec<&str> = zone.split('.').collect();
    let raw = if parts.len() >= 2 { parts[parts.len() - 2] } else { zone };
    raw.replace('-', "_")
}

fn prompt_with_default(label: &str, default: &str) -> Result<String> {
    print!("{} [{}]: ", label, default);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();
    Ok(if trimmed.is_empty() { default.to_string() } else { trimmed.to_string() })
}

fn find_apex_zone_name(content: &str) -> Option<String> {
    let pos = content.find(APEX_MARKER)?;
    let block_end = content[pos..].find('}').map(|i| pos + i)?;
    let block = &content[pos..block_end];
    for line in block.lines() {
        let line = line.trim();
        if line.starts_with("zone_id") {
            if let Some(start) = line.find("cloudflare_zone.") {
                let rest = &line[start + "cloudflare_zone.".len()..];
                if let Some(end) = rest.find(".id") {
                    return Some(rest[..end].to_string());
                }
            }
        }
    }
    None
}

fn apex_record_block(zone_name: &str) -> String {
    format!(
        r#"
resource "cloudflare_record" "apex" {{
  zone_id = cloudflare_zone.{zone_name}.id
  name    = "@"
  type    = "A"
  content = "192.0.2.1"  # dummy IP — Cloudflare proxies it, never actually hit
  proxied = true
}}
"#
    )
}

pub fn run(zone: String) -> Result<()> {
    if zone.contains(' ') || !zone.contains('.') {
        bail!("--zone must be a valid domain (e.g. example.com): got {:?}", zone);
    }

    if Path::new(REGISTER_FILE).exists() {
        bail!("composite domain register already exists — run `cdreg reset` if you want to reinitialise");
    }

    if !Path::new(TERRAFORM_MAIN).exists() {
        bail!(
            "terraform/main.tf not found — follow the usage guide at https://github.com/tracerview/composite-domain before running init"
        );
    }

    let tf_content = fs::read_to_string(TERRAFORM_MAIN)
        .with_context(|| format!("failed to read {TERRAFORM_MAIN}"))?;

    if tf_content.contains(APEX_MARKER) {
        let inferred = infer_zone_name(&zone);
        match find_apex_zone_name(&tf_content) {
            Some(ref z) if *z == inferred => {
                println!("✓ apex A record already configured in terraform/main.tf");
            }
            Some(ref z) => {
                bail!(
                    "conflict in terraform/main.tf: cloudflare_record.apex already references zone \"{z}\" — resolve manually before running init"
                );
            }
            None => {
                bail!(
                    "conflict in terraform/main.tf: cloudflare_record.apex exists but its zone could not be determined — resolve manually before running init"
                );
            }
        }
    } else {
        let inferred = infer_zone_name(&zone);
        let zone_name = prompt_with_default("Terraform zone name", &inferred)?;
        let block = apex_record_block(&zone_name);
        let mut file = OpenOptions::new()
            .append(true)
            .open(TERRAFORM_MAIN)
            .with_context(|| format!("failed to open {TERRAFORM_MAIN}"))?;
        write!(file, "{block}")?;
        println!("Added apex A record to terraform/main.tf");
    }

    let register = Register {
        zone,
        parts: HashMap::new(),
    };

    crate::register::save(&register)?;
    println!("Created composite domain register");
    Ok(())
}
