use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn cmd(dir: &TempDir) -> Command {
    let mut c = Command::cargo_bin("cdreg").unwrap();
    c.current_dir(dir.path());
    c
}

fn init_with_part(dir: &TempDir) {
    cmd(dir).args(["init", "--zone", "example.com"]).assert().success();
    cmd(dir).args(["add", "landingpage", "--worker", "cp-landingpage-static", "--route", "example.com/*"])
        .assert().success();
}

fn register(dir: &TempDir) -> serde_json::Value {
    let content = std::fs::read_to_string(dir.path().join("composite-domain-register.json")).unwrap();
    serde_json::from_str(&content).unwrap()
}

#[test]
fn removes_part_with_yes_flag() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["remove", "landingpage", "--yes"]).assert().success();

    let val = register(&dir);
    assert!(val["parts"]["landingpage"].is_null());
}

#[test]
fn removes_part_when_confirmed_interactively() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["remove", "landingpage"])
        .write_stdin("y\n")
        .assert().success();

    let val = register(&dir);
    assert!(val["parts"]["landingpage"].is_null());
}

#[test]
fn aborts_when_declined_interactively() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["remove", "landingpage"])
        .write_stdin("n\n")
        .assert().success();

    let val = register(&dir);
    assert!(!val["parts"]["landingpage"].is_null());
}

#[test]
fn errors_on_part_not_found() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["remove", "nonexistent", "--yes"])
        .assert().failure().stderr(contains("not registered"));
}

#[test]
fn errors_if_register_missing() {
    let dir = TempDir::new().unwrap();
    cmd(&dir).args(["remove", "landingpage", "--yes"])
        .assert().failure().stderr(contains("init"));
}
