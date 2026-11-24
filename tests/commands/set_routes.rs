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
fn replaces_routes_with_single_pattern() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["set-routes", "landingpage", "example.com/new/*"])
        .assert().success();

    let val = register(&dir);
    assert_eq!(val["parts"]["landingpage"]["routes"], serde_json::json!(["example.com/new/*"]));
}

#[test]
fn replaces_routes_with_multiple_patterns() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["set-routes", "landingpage", "example.com/a", "example.com/a/*"])
        .assert().success();

    let val = register(&dir);
    assert_eq!(
        val["parts"]["landingpage"]["routes"],
        serde_json::json!(["example.com/a", "example.com/a/*"])
    );
}

#[test]
fn errors_on_part_not_found() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["set-routes", "nonexistent", "example.com/*"])
        .assert().failure().stderr(contains("not registered"));
}

#[test]
fn errors_if_register_missing() {
    let dir = TempDir::new().unwrap();
    cmd(&dir).args(["set-routes", "landingpage", "example.com/*"])
        .assert().failure().stderr(contains("init"));
}
