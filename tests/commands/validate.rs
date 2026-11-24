use assert_cmd::Command;
use predicates::str::contains;
use std::fs;
use tempfile::TempDir;

fn cmd(dir: &TempDir) -> Command {
    let mut c = Command::cargo_bin("cdreg").unwrap();
    c.current_dir(dir.path());
    c
}

fn setup_terraform(dir: &TempDir) {
    let tf_dir = dir.path().join("terraform");
    fs::create_dir_all(&tf_dir).unwrap();
    fs::write(tf_dir.join("main.tf"), "# placeholder\n").unwrap();
}

fn init(dir: &TempDir) {
    setup_terraform(dir);
    cmd(dir).args(["init", "--zone", "example.com"]).write_stdin("\n").assert().success();
}

fn write_register(dir: &TempDir, content: &str) {
    fs::write(dir.path().join("composite-domain-register.json"), content).unwrap();
}

#[test]
fn valid_register_exits_zero() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir).args(["add", "landingpage", "--worker", "cp-landingpage-static", "--route", "example.com/*"])
        .assert().success();
    cmd(&dir).args(["update", "landingpage", "--repo=tracerview/repo"]).assert().success();
    cmd(&dir).arg("validate").assert().success().stdout(contains("valid"));
}

#[test]
fn tbc_repo_warns_but_exits_zero() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir).args(["add", "landingpage", "--worker", "cp-landingpage-static", "--route", "example.com/*"])
        .assert().success();
    cmd(&dir).arg("validate")
        .assert().success()
        .stderr(contains("TBC"));
}

#[test]
fn errors_on_missing_required_field() {
    let dir = TempDir::new().unwrap();
    write_register(&dir, r#"{"zone":"example.com","parts":{"p":{"worker":"cp-p","routes":["example.com/*"]}}}"#);
    cmd(&dir).arg("validate").assert().failure().stderr(contains("schema validation"));
}

#[test]
fn errors_on_worker_missing_cp_prefix() {
    let dir = TempDir::new().unwrap();
    write_register(&dir, r#"{"zone":"example.com","parts":{"p":{"repo":"TBC","worker":"no-prefix","routes":["example.com/*"]}}}"#);
    cmd(&dir).arg("validate").assert().failure().stderr(contains("schema validation"));
}

#[test]
fn errors_on_missing_file() {
    let dir = TempDir::new().unwrap();
    cmd(&dir).arg("validate").assert().failure().stderr(contains("not found"));
}

#[test]
fn errors_on_malformed_json() {
    let dir = TempDir::new().unwrap();
    write_register(&dir, "{ not valid json }");
    cmd(&dir).arg("validate").assert().failure().stderr(contains("invalid JSON"));
}
