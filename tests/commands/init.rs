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

fn setup_terraform_with_apex(dir: &TempDir, zone_name: &str) {
    let tf_dir = dir.path().join("terraform");
    fs::create_dir_all(&tf_dir).unwrap();
    let content = format!(
        "resource \"cloudflare_record\" \"apex\" {{\n  zone_id = cloudflare_zone.{zone_name}.id\n  name    = \"@\"\n  type    = \"A\"\n  content = \"192.0.2.1\"\n  proxied = true\n}}\n"
    );
    fs::write(tf_dir.join("main.tf"), content).unwrap();
}

#[test]
fn creates_register_with_zone_and_empty_parts() {
    let dir = TempDir::new().unwrap();
    setup_terraform(&dir);
    cmd(&dir).args(["init", "--zone", "example.com"]).write_stdin("\n").assert().success();

    let content = fs::read_to_string(dir.path().join("composite-domain-register.json")).unwrap();
    let val: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(val["zone"], "example.com");
    assert_eq!(val["parts"], serde_json::json!({}));
}

#[test]
fn errors_if_register_already_exists() {
    let dir = TempDir::new().unwrap();
    setup_terraform(&dir);
    cmd(&dir).args(["init", "--zone", "example.com"]).write_stdin("\n").assert().success();
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

#[test]
fn errors_if_terraform_missing() {
    let dir = TempDir::new().unwrap();
    cmd(&dir).args(["init", "--zone", "example.com"])
        .assert().failure()
        .stderr(contains("terraform/main.tf not found"))
        .stderr(contains("github.com/tracerview/composite-domain"));
}

#[test]
fn adds_apex_record_to_main_tf() {
    let dir = TempDir::new().unwrap();
    setup_terraform(&dir);
    cmd(&dir).args(["init", "--zone", "example.com"])
        .write_stdin("\n")
        .assert().success()
        .stdout(contains("Terraform zone name"))
        .stdout(contains("[example]"))
        .stdout(contains("Added apex A record"));

    let tf = fs::read_to_string(dir.path().join("terraform/main.tf")).unwrap();
    assert!(tf.contains("resource \"cloudflare_record\" \"apex\""));
    assert!(tf.contains("cloudflare_zone.example.id"));
}

#[test]
fn accepts_custom_zone_name_from_prompt() {
    let dir = TempDir::new().unwrap();
    setup_terraform(&dir);
    cmd(&dir).args(["init", "--zone", "example.com"])
        .write_stdin("my_zone\n")
        .assert().success();

    let tf = fs::read_to_string(dir.path().join("terraform/main.tf")).unwrap();
    assert!(tf.contains("cloudflare_zone.my_zone.id"));
    assert!(!tf.contains("cloudflare_zone.example.id"));
}

#[test]
fn infers_zone_name_from_subdomain() {
    let dir = TempDir::new().unwrap();
    setup_terraform(&dir);
    cmd(&dir).args(["init", "--zone", "sub.example.com"])
        .write_stdin("\n")
        .assert().success()
        .stdout(contains("[example]"));

    let tf = fs::read_to_string(dir.path().join("terraform/main.tf")).unwrap();
    assert!(tf.contains("cloudflare_zone.example.id"));
}

#[test]
fn ticks_when_apex_already_configured() {
    let dir = TempDir::new().unwrap();
    setup_terraform_with_apex(&dir, "example");
    cmd(&dir).args(["init", "--zone", "example.com"])
        .assert().success()
        .stdout(contains("✓"))
        .stdout(contains("already configured"));
}

#[test]
fn errors_on_apex_zone_conflict() {
    let dir = TempDir::new().unwrap();
    setup_terraform_with_apex(&dir, "other_zone");
    cmd(&dir).args(["init", "--zone", "example.com"])
        .assert().failure()
        .stderr(contains("conflict"))
        .stderr(contains("other_zone"));
}
