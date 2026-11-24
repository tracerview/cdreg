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

fn register_exists(dir: &TempDir) -> bool {
    dir.path().join("composite-domain-register.json").exists()
}

#[test]
fn deletes_register_when_confirmed_with_yes() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir).arg("reset").write_stdin("yes\n").assert().success();
    assert!(!register_exists(&dir));
}

#[test]
fn aborts_when_input_is_y_not_yes() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir).arg("reset").write_stdin("y\n").assert().success().stdout(contains("Aborted"));
    assert!(register_exists(&dir));
}

#[test]
fn aborts_on_empty_input() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir).arg("reset").write_stdin("\n").assert().success().stdout(contains("Aborted"));
    assert!(register_exists(&dir));
}

#[test]
fn aborts_on_no_input() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir).arg("reset").write_stdin("no\n").assert().success().stdout(contains("Aborted"));
    assert!(register_exists(&dir));
}

#[test]
fn errors_if_register_missing() {
    let dir = TempDir::new().unwrap();
    cmd(&dir).arg("reset").assert().failure().stderr(contains("not found"));
}

#[test]
fn init_succeeds_after_reset() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir).arg("reset").write_stdin("yes\n").assert().success();
    cmd(&dir).args(["init", "--zone", "example.com"]).assert().success();
    assert!(register_exists(&dir));
}

#[test]
fn confirmation_prompt_names_the_file() {
    let dir = TempDir::new().unwrap();
    init(&dir);
    cmd(&dir).arg("reset")
        .write_stdin("no\n")
        .assert().success()
        .stdout(contains("composite-domain-register.json"));
}
