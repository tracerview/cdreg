# cdreg

Manages `composite-domain-register.json` — the register that maps logical composite domain parts to their Cloudflare workers and routes.

## Install

```sh
mise use "github:tracerview/composite-domain-register[exe=cdreg]"
```

Run `cdreg -h` for a full command reference, or `cdreg <command> -h` for details on a specific command.

## Typical workflow

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

## Register format

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

## Releasing a new version

```sh
cargo install cargo-release   # one-time
cargo release patch           # or minor / major
```

This bumps `Cargo.toml`, commits, tags `v<version>`, and pushes. GitHub Actions builds and publishes binaries for macOS arm64/x86_64 and Linux x86_64.
