# Composite Domain - Register CLI tool (cdreg)

CLI tool for use with [composite-domain](https://github.com/tracerview/composite-domain).

Manages `composite-domain-register.json` — the register that maps logical composite domain parts to their Cloudflare workers and routes.

## Contents

- [1. Install](#1-install)
- [2. Typical workflow](#2-typical-workflow)
- [3. Register format](#3-register-format)
- [4. Releasing new versions of cdreg](#4-releasing-a-new-version)

## 1. Install

### 1.1 Prerequisites: mise

[mise](https://mise.jdx.dev/) is required to install cdreg.

| Tool | Purpose | Install |
|------|---------|---------|
| [mise](https://mise.jdx.dev/) | Per-project tool management | `brew install mise` |

Add to `~/.zshrc` (once per machine):

```zsh
eval "$(mise activate zsh)"
```

### 1.2 Install cdreg

```sh
mise use "github:tracerview/cdreg"
```

Run `cdreg -h` for a full command reference, or `cdreg <command> -h` for details on a specific command.

## 2. Typical workflow

**2.1. Start a new domain register:**

```sh
cd my-domain-repo
cdreg init --zone example.com
```

This automatically adds an Apex A record to `terraform/main.tf`. When routing via wrangler (cfops default), this is created automatically — since this composite method manages routing via a central Terraform config, `init` adds it explicitly:

```hcl
resource "cloudflare_record" "apex" {
  zone_id = cloudflare_zone.<ZONE_NAME>.id
  name    = "@"
  type    = "A"
  content = "192.0.2.1"  # dummy IP — Cloudflare proxies it, never actually hit
  proxied = true
}
```

Verify with:

```sh
dig <DOMAIN> A
```

**2.2. Register a composite part as you create it:**

```sh
cdreg add landingpage --worker cp-landingpage-static --route "example.com/*"
```
The part is added immediately with `repo: TBC`. Once the GitHub repo exists, assign it:

```sh
cdreg update landingpage --repo gh-org/repo-landingpage
```

This automatically adds a `cloudflare_workers_route` resource to `terraform/workers.tf` for each route — for example:

```hcl
resource "cloudflare_workers_route" "landingpage_0" {
  zone_id     = cloudflare_zone.acme.id
  pattern     = "acme.com"
  script_name = "cp-landingpage-static"
}

resource "cloudflare_workers_route" "landingpage_1" {
  zone_id     = cloudflare_zone.acme.id
  pattern     = "acme.com/*"
  script_name = "cp-landingpage-static"
}
```

**2.3. Check the current state at any time:**

```sh
cdreg show
```

Parts with an unassigned repo are flagged with `[!]`.

**2.4. Update routes when they change:**

```sh
cdreg update landingpage --route "example.com/" --route "example.com/*"
```

**2.5. Remove a part that's been retired:**

```sh
cdreg remove landingpage
```

Pass `--yes` to skip the confirmation prompt.

**2.6. Delete the register entirely:**

```sh
cdreg reset
```

**2.7. Validate before committing:**

```sh
cdreg validate
```

Exits non-zero on schema errors. Warns (but still exits 0) if any repos remain `TBC`.

## 3. Register format

`composite-domain-register.json` lives in the repo root alongside this tool's config. See [`schema.json`](./schema.json) for the full schema.

```json
{
  "zone": "example.com",
  "parts": {
    "landingpage": {
      "worker": "cp-landingpage-static",
      "repo": "gh-org/repo-landingpage",
      "routes": ["example.com/*"]
    }
  }
}
```

Worker names must be prefixed `cp-`. Routes follow [Cloudflare route pattern syntax](https://developers.cloudflare.com/workers/configuration/routing/routes/) — wildcards only at the start of the hostname or end of the path, no query strings or fragments.

## 4. Releasing new versions of cdreg

```sh
cargo install cargo-release   # one-time
cargo release patch           # or minor / major
```

This bumps `Cargo.toml`, commits, tags `v<version>`, and pushes. GitHub Actions builds and publishes binaries for macOS arm64/x86_64 and Linux x86_64.
