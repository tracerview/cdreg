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
fn sets_repo_on_tbc_part() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["set-repo", "landingpage", "tracerview/repo-landingpage"])
        .assert().success();

    let val = register(&dir);
    assert_eq!(val["parts"]["landingpage"]["repo"], "tracerview/repo-landingpage");
}

#[test]
fn errors_on_part_not_found() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["set-repo", "nonexistent", "tracerview/repo"])
        .assert().failure().stderr(contains("not registered"));
}

#[test]
fn errors_on_bad_repo_format() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["set-repo", "landingpage", "not-a-repo"])
        .assert().failure().stderr(contains("org/repo"));
    cmd(&dir).args(["set-repo", "landingpage", "/repo"])
        .assert().failure().stderr(contains("org/repo"));
    cmd(&dir).args(["set-repo", "landingpage", "org/"])
        .assert().failure().stderr(contains("org/repo"));
}

#[test]
fn errors_when_repo_already_set_without_force() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["set-repo", "landingpage", "tracerview/repo-landingpage"])
        .assert().success();
    cmd(&dir).args(["set-repo", "landingpage", "tracerview/new-repo"])
        .assert().failure().stderr(contains("--force"));
}

#[test]
fn force_overwrites_existing_repo() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["set-repo", "landingpage", "tracerview/repo-landingpage"])
        .assert().success();
    cmd(&dir).args(["set-repo", "landingpage", "tracerview/new-repo", "--force"])
        .assert().success();

    let val = register(&dir);
    assert_eq!(val["parts"]["landingpage"]["repo"], "tracerview/new-repo");
}

#[test]
fn errors_if_register_missing() {
    let dir = TempDir::new().unwrap();
    cmd(&dir).args(["set-repo", "landingpage", "tracerview/repo"])
        .assert().failure().stderr(contains("init"));
}
