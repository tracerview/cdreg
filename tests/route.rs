use assert_cmd::Command;
use tempfile::TempDir;

fn cmd(dir: &TempDir) -> Command {
    let mut c = Command::cargo_bin("cdreg").unwrap();
    c.current_dir(dir.path());
    c
}

fn setup_terraform(dir: &TempDir) {
    let tf_dir = dir.path().join("terraform");
    std::fs::create_dir_all(&tf_dir).unwrap();
    std::fs::write(tf_dir.join("main.tf"), "# placeholder\n").unwrap();
}

fn init(dir: &TempDir) {
    setup_terraform(dir);
    cmd(dir).args(["init", "--zone", "example.com"]).write_stdin("\n").assert().success();
}

// Valid route patterns accepted by add
#[test]
fn add_accepts_valid_route_patterns() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir)
        .args(["add", "part1", "--worker", "cp-part1-static",
               "--routes", "example.com/*",
               "--routes", "*.example.com/path/*"])
        .assert().success();
}

// Infix wildcard in path is rejected
#[test]
fn add_rejects_infix_wildcard_in_path() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir)
        .args(["add", "part1", "--worker", "cp-part1-static",
               "--routes", "example.com/*.jpg"])
        .assert().failure()
        .stderr(predicates::str::contains("wildcard"));
}

// Query string in route is rejected
#[test]
fn add_rejects_query_string_in_route() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir)
        .args(["add", "part1", "--worker", "cp-part1-static",
               "--routes", "example.com/?foo=bar"])
        .assert().failure()
        .stderr(predicates::str::contains("query"));
}

// No dot in hostname is rejected
#[test]
fn add_rejects_route_without_dot_in_hostname() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir)
        .args(["add", "part1", "--worker", "cp-part1-static",
               "--routes", "localhost/*"])
        .assert().failure()
        .stderr(predicates::str::contains("dot"));
}
