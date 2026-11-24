# Composite Domain - Register CLI tool (cdreg)

Manages `composite-domain-register.json` — the register that maps logical composite domain parts to their Cloudflare workers and routes.

## Contents

- [1. Install](#1-install)
- [2. Typical workflow](#2-typical-workflow)
- [3. Register format](#3-register-format)
- [4. Releasing new versions of cdreg](#4-releasing-a-new-version)

## 1. Install

```sh
mise use "github:tracerview/cdreg"
```

Run `cdreg -h` for a full command reference, or `cdreg <command> -h` for details on a specific command.

## 2. Typical workflow

**Start a new domain register:**

```sh
cd my-domain-repo
cdreg init --zone example.com
```

**Register a composite part as you create it:**

```sh
cdreg add landingpage --worker cp-landingpage-static --route "example.com/*"
```

The part is added immediately with `repo: TBC`. Once the GitHub repo exists, assign it:

```sh
cdreg set-repo landingpage gh-org/repo-landingpage
```

**Check the current state at any time:**

```sh
cdreg show
```

Parts with an unassigned repo are flagged with `[!]`.

**Update routes when they change:**

```sh
cdreg set-routes landingpage "example.com/" "example.com/*"
```

**Remove a part that's been retired:**

```sh
cdreg remove landingpage
```

**Validate before committing:**

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
