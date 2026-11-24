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

#[test]
fn shows_zone_and_all_parts() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir).args(["add", "partone", "--worker", "cp-partone-static", "--routes", "example.com/one/*"])
        .assert().success();
    cmd(&dir).args(["add", "parttwo", "--worker", "cp-parttwo-static", "--routes", "example.com/two/*"])
        .assert().success();

    cmd(&dir).arg("show")
        .assert().success()
        .stdout(contains("example.com"))
        .stdout(contains("partone"))
        .stdout(contains("parttwo"))
        .stdout(contains("cp-partone-static"))
        .stdout(contains("cp-parttwo-static"));
}

#[test]
fn indicates_no_parts_when_empty() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir).arg("show")
        .assert().success()
        .stdout(contains("example.com"))
        .stdout(contains("No parts"));
}

#[test]
fn flags_tbc_repo_in_output() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir).args(["add", "landingpage", "--worker", "cp-landingpage-static", "--routes", "example.com/*"])
        .assert().success();

    cmd(&dir).arg("show")
        .assert().success()
        .stdout(contains("TBC"))
        .stdout(contains("[!]"));
}

#[test]
fn shows_part_name_column_header() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir).args(["add", "landingpage", "--worker", "cp-landingpage-static", "--routes", "example.com/*"])
        .assert().success();

    cmd(&dir).arg("show")
        .assert().success()
        .stdout(contains("PART-NAME"));
}

#[test]
fn errors_if_register_missing() {
    let dir = TempDir::new().unwrap();
    cmd(&dir).arg("show").assert().failure().stderr(contains("init"));
}
