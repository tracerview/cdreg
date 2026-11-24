use assert_cmd::Command;
use predicates::str::contains;
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

fn init_with_part(dir: &TempDir) {
    setup_terraform(dir);
    cmd(dir).args(["init", "--zone", "example.com"]).write_stdin("\n").assert().success();
    cmd(dir).args(["add", "landingpage", "--worker", "cp-landingpage-static", "--routes", "example.com/*"])
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

#[test]
fn removes_workers_route_from_workers_tf() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["remove", "landingpage", "--yes"]).assert().success();

    let tf = std::fs::read_to_string(dir.path().join("terraform/workers.tf")).unwrap();
    assert!(!tf.contains("resource \"cloudflare_workers_route\" \"landingpage_0\""));
}

#[test]
fn remove_succeeds_if_workers_tf_missing() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    std::fs::remove_file(dir.path().join("terraform/workers.tf")).unwrap();
    cmd(&dir).args(["remove", "landingpage", "--yes"]).assert().success();
}
