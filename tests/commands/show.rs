use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn cmd(dir: &TempDir) -> Command {
    let mut c = Command::cargo_bin("cdreg").unwrap();
    c.current_dir(dir.path());
    c
}

#[test]
fn shows_zone_and_all_parts() {
    let dir = TempDir::new().unwrap();
    cmd(&dir).args(["init", "--zone", "example.com"]).assert().success();
    cmd(&dir).args(["add", "partone", "--worker", "cp-partone-static", "--route", "example.com/one/*"])
        .assert().success();
    cmd(&dir).args(["add", "parttwo", "--worker", "cp-parttwo-static", "--route", "example.com/two/*"])
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
    cmd(&dir).args(["init", "--zone", "example.com"]).assert().success();
    cmd(&dir).arg("show")
        .assert().success()
        .stdout(contains("example.com"))
        .stdout(contains("No parts"));
}

#[test]
fn flags_tbc_repo_in_output() {
    let dir = TempDir::new().unwrap();
    cmd(&dir).args(["init", "--zone", "example.com"]).assert().success();
    cmd(&dir).args(["add", "landingpage", "--worker", "cp-landingpage-static", "--route", "example.com/*"])
        .assert().success();

    cmd(&dir).arg("show")
        .assert().success()
        .stdout(contains("TBC"))
        .stdout(contains("[!]"));
}

#[test]
fn errors_if_register_missing() {
    let dir = TempDir::new().unwrap();
    cmd(&dir).arg("show").assert().failure().stderr(contains("init"));
}
