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

fn init(dir: &TempDir) {
    setup_terraform(dir);
    cmd(dir).args(["init", "--zone", "example.com"]).write_stdin("\n").assert().success();
}

fn register(dir: &TempDir) -> serde_json::Value {
    let content = std::fs::read_to_string(dir.path().join("composite-domain-register.json")).unwrap();
    serde_json::from_str(&content).unwrap()
}

#[test]
fn adds_part_with_tbc_repo_and_correct_fields() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir)
        .args(["add", "landingpage", "--worker", "cp-landingpage-static", "--route", "example.com/*"])
        .assert().success();

    let val = register(&dir);
    let part = &val["parts"]["landingpage"];
    assert_eq!(part["repo"], "TBC");
    assert_eq!(part["worker"], "cp-landingpage-static");
    assert_eq!(part["routes"], serde_json::json!(["example.com/*"]));
}

#[test]
fn errors_on_duplicate_name() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir).args(["add", "landingpage", "--worker", "cp-landingpage-static", "--route", "example.com/*"])
        .assert().success();
    cmd(&dir).args(["add", "landingpage", "--worker", "cp-landingpage-static", "--route", "example.com/*"])
        .assert().failure().stderr(contains("already exists"));
}

#[test]
fn errors_on_missing_cp_prefix() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir).args(["add", "landingpage", "--worker", "landingpage-static", "--route", "example.com/*"])
        .assert().failure().stderr(contains("cp-"));
}

#[test]
fn errors_on_no_routes() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir).args(["add", "landingpage", "--worker", "cp-landingpage-static"])
        .assert().failure();
}

#[test]
fn accepts_multiple_routes() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir)
        .args(["add", "landingpage", "--worker", "cp-landingpage-static",
               "--route", "example.com/a", "--route", "example.com/a/*"])
        .assert().success();

    let val = register(&dir);
    let routes = &val["parts"]["landingpage"]["routes"];
    assert_eq!(routes, &serde_json::json!(["example.com/a", "example.com/a/*"]));
}

#[test]
fn errors_on_invalid_logical_name() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir).args(["add", "Landing_Page", "--worker", "cp-lp-static", "--route", "example.com/*"])
        .assert().failure().stderr(contains("lowercase"));
}

#[test]
fn errors_if_register_missing() {
    let dir = TempDir::new().unwrap();
    cmd(&dir).args(["add", "landingpage", "--worker", "cp-landingpage-static", "--route", "example.com/*"])
        .assert().failure().stderr(contains("init"));
}
