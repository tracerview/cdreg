# cdreg — Requirements

EARS-style requirements for the `cdreg` CLI tool.

---

## Conventions

- **REG** refers to `composite-domain-register.json` in the current working directory
- **LOGICAL-NAME** is a kebab-case identifier representing the part's purpose (e.g. `landingpage`, `topic-x`)
- **WORKER-NAME** follows the pattern `cp-<LOGICAL-NAME>-<SUFFIX>` (e.g. `cp-landingpage-static`)
- Requirements use EARS syntax: WHEN / WHILE / IF / WHERE / ubiquitous

---

## 1. General Behaviour

**REQ-G-01**
The system shall locate REG in the current working directory.

**REQ-G-02**
WHEN a write operation completes successfully, the system shall validate the resulting REG against the schema before saving.

**REQ-G-03**
IF schema validation fails after a write, THEN the system shall abort the write, report the validation errors, and leave the existing REG unchanged.

**REQ-G-04**
The system shall write REG as pretty-printed JSON with 2-space indentation.

**REQ-G-05**
WHEN any command succeeds, the system shall exit with code 0.

**REQ-G-06**
WHEN any command fails, the system shall exit with a non-zero code and write a human-readable error to stderr.

---

## 2. `cdreg init`

Initialises a new REG for a domain.

```
cdreg init --zone <DOMAIN>
```

**REQ-INIT-01**
WHEN `init` is run with a valid `--zone` value, the system shall create REG with `zone` set to the provided value and `parts` set to an empty object.

**REQ-INIT-02**
IF REG already exists in the current directory, THEN the system shall abort and report that the register already exists.

**REQ-INIT-03**
IF `--zone` is not provided, THEN the system shall exit with an error indicating the flag is required.

**REQ-INIT-04**
IF the `--zone` value does not resemble a valid domain (e.g. contains spaces, no dot), THEN the system shall exit with a validation error.

### Acceptance Criteria

- `cdreg init --zone example.com` → creates valid REG, `zone` = `"example.com"`, `parts` = `{}`
- Running `init` a second time in the same directory → error, no file overwrite
- `cdreg init` (no zone) → error on missing flag
- `cdreg init --zone "not a domain"` → validation error

---

## 3. `cdreg add`

Registers a new composite domain part.

```
cdreg add <LOGICAL-NAME> --worker <WORKER-NAME> --route <PATTERN> [--route <PATTERN> ...]
```

**REQ-ADD-01**
WHEN `add` is run with a valid LOGICAL-NAME, worker name, and at least one route, the system shall append the new part to `parts` in REG with `repo` set to `"TBC"`.

**REQ-ADD-02**
IF REG does not exist, THEN the system shall abort and instruct the user to run `cdreg init` first.

**REQ-ADD-03**
IF a part with the given LOGICAL-NAME already exists in REG, THEN the system shall abort and report the conflict.

**REQ-ADD-04**
IF the worker name does not match the pattern `^cp-`, THEN the system shall abort with a validation error describing the required prefix.

**REQ-ADD-05**
IF no `--route` flag is provided, THEN the system shall abort with an error indicating at least one route is required.

**REQ-ADD-06**
IF LOGICAL-NAME contains characters other than lowercase letters, digits, and hyphens, THEN the system shall abort with a validation error.

### Acceptance Criteria

- `cdreg add landingpage --worker cp-landingpage-static --route "example.com/*"` → part added with `repo: "TBC"`, `routes: ["example.com/*"]`, `worker: "cp-landingpage-static"`
- Running the same command again → conflict error, REG unchanged
- `--worker landingpage-static` (missing `cp-` prefix) → validation error
- No `--route` provided → error
- `--worker cp-landingpage-static --route "a" --route "b"` → `routes: ["a", "b"]`

---

## 4. `cdreg set-repo`

Assigns the GitHub repo to a registered composite part.

```
cdreg set-repo <LOGICAL-NAME> <org/repo>
```

**REQ-REPO-01**
WHEN `set-repo` is run with a valid LOGICAL-NAME and repo string, the system shall update the `repo` field for that part.

**REQ-REPO-02**
IF REG does not exist, THEN the system shall abort and instruct the user to run `cdreg init` first.

**REQ-REPO-03**
IF the LOGICAL-NAME does not exist in `parts`, THEN the system shall abort and report that the part is not registered.

**REQ-REPO-04**
IF the repo argument does not match the pattern `<org>/<repo>` (one slash, non-empty on both sides), THEN the system shall abort with a format error.

**REQ-REPO-05**
IF the part's `repo` is already set to a value other than `"TBC"`, THEN the system shall warn the user and require `--force` to overwrite.

### Acceptance Criteria

- `cdreg set-repo landingpage tracerview/repo-landingpage` → `repo` updated
- Part not in register → error
- `cdreg set-repo landingpage not-a-repo` → format error
- `cdreg set-repo landingpage tracerview/repo-landingpage` when repo already set → warning + prompt for `--force`
- `cdreg set-repo landingpage tracerview/new-repo --force` when repo already set → overwrites

---

## 5. `cdreg set-routes`

Replaces the route patterns for a registered composite part.

```
cdreg set-routes <LOGICAL-NAME> <PATTERN> [<PATTERN> ...]
```

**REQ-ROUTES-01**
WHEN `set-routes` is run with a valid LOGICAL-NAME and at least one pattern, the system shall replace the `routes` array for that part with the provided patterns.

**REQ-ROUTES-02**
IF REG does not exist, THEN the system shall abort and instruct the user to run `cdreg init` first.

**REQ-ROUTES-03**
IF the LOGICAL-NAME does not exist in `parts`, THEN the system shall abort and report that the part is not registered.

**REQ-ROUTES-04**
IF no route patterns are provided, THEN the system shall abort with an error indicating at least one route is required.

### Acceptance Criteria

- `cdreg set-routes landingpage "example.com/*"` → `routes` replaced with `["example.com/*"]`
- `cdreg set-routes landingpage "example.com/a" "example.com/a/*"` → `routes` has two entries
- Part not in register → error
- No patterns provided → error

---

## 6. `cdreg remove`

Removes a composite part from the register.

```
cdreg remove <LOGICAL-NAME>
```

**REQ-RM-01**
WHEN `remove` is run and the LOGICAL-NAME exists in `parts`, the system shall remove that entry from REG.

**REQ-RM-02**
IF REG does not exist, THEN the system shall abort and instruct the user to run `cdreg init` first.

**REQ-RM-03**
IF the LOGICAL-NAME does not exist in `parts`, THEN the system shall abort and report that the part is not registered.

**REQ-RM-04**
WHEN `remove` is run interactively, the system shall prompt the user for confirmation before deleting.

**REQ-RM-05**
WHERE a `--yes` flag is provided, the system shall skip the confirmation prompt.

### Acceptance Criteria

- `cdreg remove landingpage` (interactive) → confirmation prompt → confirmed → entry removed
- `cdreg remove landingpage --yes` → removed without prompt
- `cdreg remove landingpage` → declined at prompt → REG unchanged
- Part not in register → error, REG unchanged

---

## 7. `cdreg validate`

Validates REG against the schema.

```
cdreg validate
```

**REQ-VAL-01**
WHEN `validate` is run and REG is valid, the system shall report success and exit 0.

**REQ-VAL-02**
IF REG does not exist, THEN the system shall report the file is missing and exit non-zero.

**REQ-VAL-03**
IF REG fails schema validation, THEN the system shall report each validation error with a human-readable description and exit non-zero.

**REQ-VAL-04**
IF REG is not valid JSON, THEN the system shall report a parse error and exit non-zero.

**REQ-VAL-05**
WHEN `validate` is run and one or more parts have `repo` set to `"TBC"`, the system shall print a warning for each such part (but still exit 0 if otherwise valid).

### Acceptance Criteria

- Valid REG → success message, exit 0
- REG with `repo: "TBC"` → warning printed, exit 0
- REG missing required field → schema error reported, exit non-zero
- REG with worker name missing `cp-` prefix → schema error reported, exit non-zero
- File missing → missing file error, exit non-zero
- Malformed JSON → parse error, exit non-zero

---

## 8. `cdreg show`

Prints a human-readable summary of the current register state.

```
cdreg show
```

**REQ-SHOW-01**
WHEN `show` is run and REG exists, the system shall print the zone and a table of parts showing LOGICAL-NAME, worker, repo, and routes.

**REQ-SHOW-02**
IF REG does not exist, THEN the system shall report the file is missing and exit non-zero.

**REQ-SHOW-03**
WHEN a part has `repo` set to `"TBC"`, the system shall visually distinguish it in the output (e.g. flag or colour).

### Acceptance Criteria

- Valid REG with two parts → zone printed, both parts listed with all fields
- Empty `parts: {}` → zone printed, indication that no parts are registered
- File missing → error, exit non-zero
- Part with `repo: "TBC"` → visually flagged in output
