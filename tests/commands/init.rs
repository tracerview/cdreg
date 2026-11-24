use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn cmd(dir: &TempDir) -> Command {
    let mut c = Command::cargo_bin("cdreg").unwrap();
    c.current_dir(dir.path());
    c
}

#[test]
fn creates_register_with_zone_and_empty_parts() {
    let dir = TempDir::new().unwrap();
    cmd(&dir).args(["init", "--zone", "example.com"]).assert().success();

    let content = std::fs::read_to_string(dir.path().join("composite-domain-register.json")).unwrap();
    let val: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(val["zone"], "example.com");
    assert_eq!(val["parts"], serde_json::json!({}));
}

#[test]
fn errors_if_register_already_exists() {
    let dir = TempDir::new().unwrap();
    cmd(&dir).args(["init", "--zone", "example.com"]).assert().success();
    cmd(&dir).args(["init", "--zone", "example.com"])
        .assert().failure().stderr(contains("already exists")).stderr(contains("cdreg reset"));
}

#[test]
fn errors_on_missing_zone_flag() {
    let dir = TempDir::new().unwrap();
    cmd(&dir).arg("init").assert().failure();
}

#[test]
fn errors_on_invalid_zone() {
    let dir = TempDir::new().unwrap();
    cmd(&dir).args(["init", "--zone", "not a domain"])
        .assert().failure().stderr(contains("valid domain"));
    cmd(&dir).args(["init", "--zone", "nodot"])
        .assert().failure().stderr(contains("valid domain"));
}
