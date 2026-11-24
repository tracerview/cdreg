# cdreg

CLI helper for maintaining the composite domain register (`composite-domain-register.json`).

## Key Decisions

**Language: Rust**
- Must produce a single static binary with no runtime dependencies
- Distributed via mise using the `ubi` backend (`mise use ubi:tracerview/cdreg`)
- GitHub Actions builds multi-platform binaries on release (macOS arm64/x86_64, Linux x86_64)

**Distribution: own repo**
- `cdreg` is a shared tool across all composite-domain instances, not tied to any one domain
- Will live at `tracerview/cdreg` with its own release pipeline
- This directory is the implementation; the repo will be created from it

**Schema: embedded**
- `schema.json` is embedded in the binary at compile time
- No external schema file required at runtime

**JSON output**
- Pretty-printed with 2-space indentation
- Keys written in natural insertion order (not sorted)

**`set-repo` overwrite guard**
- If `repo` is already set to a non-`TBC` value, exit non-zero with an error
- Re-run with `--force` to overwrite

## Repository Structure

```
/
├── README.md
├── Cargo.toml
├── Cargo.lock
├── .gitignore          # includes dist/
├── .github/
│   └── workflows/
│       └── release.yml
├── src/
│   ├── main.rs
│   ├── cli.rs          # clap command definitions
│   ├── register.rs     # data model, read/write, schema validation
│   ├── route.rs        # Cloudflare route pattern validation
│   └── commands/
│       ├── mod.rs
│       ├── init.rs
│       ├── add.rs
│       ├── set_repo.rs
│       ├── set_routes.rs
│       ├── remove.rs
│       ├── validate.rs
│       └── show.rs
├── tests/              # integration tests mirror src/ layout
│   ├── commands/
│   │   ├── init.rs
│   │   ├── add.rs
│   │   ├── set_repo.rs
│   │   ├── set_routes.rs
│   │   ├── remove.rs
│   │   ├── validate.rs
│   │   └── show.rs
│   └── route.rs
├── schema.json         # embedded at compile time via include_str!
└── dist/               # gitignored — local build output only
```

## Build & Release Pipeline

Modelled on `ctracey/cfops`.

**Trigger:** push of a `v*` tag (e.g. `v0.1.0`)

**Matrix targets:**
- `x86_64-unknown-linux-gnu` (ubuntu-latest)
- `aarch64-apple-darwin` (macos-latest)
- `x86_64-apple-darwin` (macos-latest)

**Steps per target:**
1. `actions/checkout@v4`
2. `dtolnay/rust-toolchain@stable` with the matrix target
3. `cargo build --release --target <target>`
4. Stage binary as `cdreg-<target>`
5. `actions/upload-artifact@v4`

**Release job** (runs after all builds):
- `actions/download-artifact@v4`
- `softprops/action-gh-release@v2` publishes all `cdreg-*` binaries to the GitHub Release

Artifacts are published to GitHub Releases only — `dist/` is gitignored and never committed.

## Versioning

**Tool:** `cargo-release`

**Flow:**
1. `cargo release <patch|minor|major>` — bumps version in `Cargo.toml`, commits, tags `v<version>`, pushes
2. Tag push triggers the GitHub Actions release workflow
3. Version in `Cargo.toml` is always the source of truth

## Route Pattern Validation

Cloudflare Workers route patterns follow these rules
(ref: https://developers.cloudflare.com/workers/configuration/routing/routes/):

- Must contain a hostname with at least one dot (e.g. `example.com`)
- Wildcards (`*`) may only appear at the **start of the hostname** (`*example.com`, `*.example.com`) or the **end of the path** (`example.com/*`, `example.com/path/*`)
- No infix wildcards — `example.com/*.jpg` is invalid
- No query parameters — `?` is not permitted
- No fragments — `#` is not permitted
- No spaces

`cdreg` validates all route patterns on `add` and `set-routes`.

## Usage Scenarios

### 1. Init register
Create a new `composite-domain-register.json` for a domain.
```
cdreg init --zone example.com
```

### 2. Add a composite part
Register a new composite domain part entry under `parts`.
```
cdreg add <LOGICAL-NAME> --worker cp-<LOGICAL-NAME>-<SUFFIX> --route "example.com/*"
```

### 3. Set part repo
Update a part's `repo` field from `TBC` to the actual GitHub repo once created.
```
cdreg set-repo <LOGICAL-NAME> org/repo
```

### 4. Update routes
Modify the route patterns for an existing composite part.
```
cdreg set-routes <LOGICAL-NAME> "example.com/path" "example.com/path/*"
```

### 5. Remove a part
Remove a composite part entry from the register.
```
cdreg remove <LOGICAL-NAME>
```

### 6. Validate
Validate `composite-domain-register.json` against the schema.
```
cdreg validate
```

### 7. Show
Print a summary of the current register state.
```
cdreg show
```

## Register Schema

See [`schema.json`](./schema.json) — embedded in the binary at compile time.
