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
fn sets_repo_on_tbc_part_without_prompt() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["update", "landingpage", "--repo=tracerview/repo-landingpage"])
        .assert().success();

    let val = register(&dir);
    assert_eq!(val["parts"]["landingpage"]["repo"], "tracerview/repo-landingpage");
}

#[test]
fn prompts_when_repo_already_set_and_confirms() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["update", "landingpage", "--repo=tracerview/repo-landingpage"])
        .assert().success();

    cmd(&dir).args(["update", "landingpage", "--repo=tracerview/new-repo"])
        .write_stdin("y\n")
        .assert().success();

    let val = register(&dir);
    assert_eq!(val["parts"]["landingpage"]["repo"], "tracerview/new-repo");
}

#[test]
fn prompts_when_repo_already_set_and_aborts() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["update", "landingpage", "--repo=tracerview/repo-landingpage"])
        .assert().success();

    cmd(&dir).args(["update", "landingpage", "--repo=tracerview/new-repo"])
        .write_stdin("n\n")
        .assert().success().stdout(contains("Aborted"));

    let val = register(&dir);
    assert_eq!(val["parts"]["landingpage"]["repo"], "tracerview/repo-landingpage");
}

#[test]
fn updates_routes_with_confirmation() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["update", "landingpage", "--routes=example.com/new/*"])
        .write_stdin("y\n")
        .assert().success();

    let val = register(&dir);
    assert_eq!(val["parts"]["landingpage"]["routes"], serde_json::json!(["example.com/new/*"]));
}

#[test]
fn aborts_routes_update_when_declined() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["update", "landingpage", "--routes=example.com/new/*"])
        .write_stdin("n\n")
        .assert().success().stdout(contains("Aborted"));

    let val = register(&dir);
    assert_eq!(val["parts"]["landingpage"]["routes"], serde_json::json!(["example.com/*"]));
}

#[test]
fn updates_multiple_routes() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["update", "landingpage", "--routes=example.com/a", "--routes=example.com/a/*"])
        .write_stdin("y\n")
        .assert().success();

    let val = register(&dir);
    assert_eq!(
        val["parts"]["landingpage"]["routes"],
        serde_json::json!(["example.com/a", "example.com/a/*"])
    );
}

#[test]
fn errors_on_bad_repo_format() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["update", "landingpage", "--repo=not-a-repo"])
        .assert().failure().stderr(contains("org/repo"));
}

#[test]
fn errors_on_part_not_found() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["update", "nonexistent", "--repo=tracerview/repo"])
        .assert().failure().stderr(contains("not registered"));
}

#[test]
fn errors_if_no_flags_provided() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["update", "landingpage"])
        .assert().failure().stderr(contains("--repo or --routes"));
}

#[test]
fn errors_if_register_missing() {
    let dir = TempDir::new().unwrap();
    cmd(&dir).args(["update", "landingpage", "--repo=tracerview/repo"])
        .assert().failure().stderr(contains("init"));
}

#[test]
fn updates_workers_tf_pattern_on_route_change() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    cmd(&dir).args(["update", "landingpage", "--routes=example.com/new/*"])
        .write_stdin("y\n")
        .assert().success();

    let tf = std::fs::read_to_string(dir.path().join("terraform/workers.tf")).unwrap();
    assert!(tf.contains("pattern     = \"example.com/new/*\""));
    assert!(!tf.contains("pattern     = \"example.com/*\""));
}

#[test]
fn workers_tf_unchanged_on_repo_only_update() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    let before = std::fs::read_to_string(dir.path().join("terraform/workers.tf")).unwrap();

    cmd(&dir).args(["update", "landingpage", "--repo=tracerview/repo-landingpage"])
        .assert().success();

    let after = std::fs::read_to_string(dir.path().join("terraform/workers.tf")).unwrap();
    assert_eq!(before, after);
}

#[test]
fn update_route_no_op_if_workers_tf_missing() {
    let dir = TempDir::new().unwrap();
    init_with_part(&dir);
    std::fs::remove_file(dir.path().join("terraform/workers.tf")).unwrap();
    cmd(&dir).args(["update", "landingpage", "--routes=example.com/new/*"])
        .write_stdin("y\n")
        .assert().success();
}
