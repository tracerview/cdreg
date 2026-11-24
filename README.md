# cdreg

CLI for maintaining `composite-domain-register.json` — the register that maps logical composite domain parts to Cloudflare workers and routes.

## Install

```sh
mise use ubi:tracerview/cdreg
```

Requires [mise](https://mise.jdx.dev/) with the `ubi` backend.

## Commands

### `cdreg init --zone <DOMAIN>`

Create a new register in the current directory.

```sh
cdreg init --zone example.com
```

### `cdreg add <NAME> --worker <WORKER> --route <PATTERN> [--route <PATTERN>...]`

Register a new composite part. `repo` is set to `TBC` until assigned.

```sh
cdreg add landingpage --worker cp-landingpage-static --route "example.com/*"
```

### `cdreg set-repo <NAME> <org/repo>`

Assign the GitHub repo to a part. Use `--force` to overwrite an existing value.

```sh
cdreg set-repo landingpage tracerview/repo-landingpage
cdreg set-repo landingpage tracerview/repo-landingpage --force
```

### `cdreg set-routes <NAME> <PATTERN> [<PATTERN>...]`

Replace the route patterns for a part.

```sh
cdreg set-routes landingpage "example.com/" "example.com/*"
```

### `cdreg remove <NAME>`

Remove a part. Prompts for confirmation unless `--yes` is passed.

```sh
cdreg remove landingpage
cdreg remove landingpage --yes
```

### `cdreg validate`

Validate the register against the schema. Warns on `TBC` repos but exits 0 if otherwise valid.

```sh
cdreg validate
```

### `cdreg show`

Print a summary of the current register state.

```sh
cdreg show
```

## Register format

The register is `composite-domain-register.json` in the current directory. Schema: [`schema.json`](./schema.json).

```json
{
  "zone": "example.com",
  "parts": {
    "landingpage": {
      "worker": "cp-landingpage-static",
      "repo": "tracerview/repo-landingpage",
      "routes": ["example.com/*"]
    }
  }
}
```

Workers must be prefixed `cp-`. Routes must be valid Cloudflare route patterns (no infix wildcards, no `?` or `#`, hostname must contain a dot).

## Contributing

### Release

Releases are cut by pushing a version tag. The GitHub Actions workflow builds binaries for macOS arm64/x86_64 and Linux x86_64 and publishes them to GitHub Releases.

```sh
cargo install cargo-release   # one-time
cargo release patch           # or minor / major
```

This bumps the version in `Cargo.toml`, commits, tags `v<version>`, and pushes — triggering the release workflow.
