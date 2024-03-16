use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{fs, process::Command};

#[test]
fn unrecognized_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("sniptip")?;
    let wrong_subcommand = "foobar";

    cmd.arg(wrong_subcommand);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "unrecognized subcommand '{}'",
            wrong_subcommand
        )));

    Ok(())
}

#[test]
fn tool_initialization() -> Result<(), Box<dyn std::error::Error>> {
    let _ = fs::remove_dir_all("/tmp/.sniptip");

    let mut cmd1 = Command::cargo_bin("sniptip")?;

    // Run a command without initializing the tool
    cmd1.env("SNIPS_BASE", "/tmp").arg("list");
    cmd1.assert().failure().stderr(predicate::str::contains(
        "Sniptips not initialized. Run `sniptip init` to initialize.",
    ));

    let mut cmd2 = Command::cargo_bin("sniptip")?;

    // Initialize the tool
    cmd2.env("SNIPS_BASE", "/tmp").arg("init");
    cmd2.assert()
        .success()
        .stdout(predicate::str::contains("Initialized sniptip"));

    // Run the command again to check if it's already initialized
    cmd2.assert()
        .success()
        .stdout(predicate::str::contains("Already initialized!"));

    let _ = fs::remove_dir_all("/tmp/.sniptip");

    Ok(())
}

#[test]
fn add_sniptip() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir("/tmp/.sniptip").ok();

    let mut cmd = Command::cargo_bin("sniptip")?;

    // Run the add command without needed arguments
    cmd.env("SNIPS_BASE", "/tmp").arg("add");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("arguments were not provided"));

    // Add a sniptip
    cmd.args(&["sniptip_name", "<h1>Wow</h1>"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Sniptip saved!"));

    fs::remove_dir_all("/tmp/.sniptip").ok();

    Ok(())
}

#[test]
fn list_sniptip() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir("/tmp/.sniptip").ok();

    let mut cmd = Command::cargo_bin("sniptip")?;

    // Run the list command without sniptips stored
    cmd.env("SNIPS_BASE", "/tmp").arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No sniptips found!"));

    // Add a sniptip
    let mut cmd_add_1 = Command::cargo_bin("sniptip")?;
    cmd_add_1
        .env("SNIPS_BASE", "/tmp")
        .args(&["add", "sniptip_name", "<h1>Wow</h1>"]);
    cmd_add_1
        .assert()
        .success()
        .stdout(predicate::str::contains("Sniptip saved!"));

    let mut cmd_add_2 = Command::cargo_bin("sniptip")?;
    cmd_add_2
        .env("SNIPS_BASE", "/tmp")
        .args(&["add", "sniptip_2_name", "<h1>Wow 2</h1>"]);
    cmd_add_2
        .assert()
        .success()
        .stdout(predicate::str::contains("Sniptip saved!"));

    // Run the list command with sniptips stored
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("sniptip_2_name\nsniptip_name"));

    fs::remove_dir_all("/tmp/.sniptip").ok();

    Ok(())
}

#[test]
fn query_sniptip() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir("/tmp/.sniptip").ok();

    let mut cmd = Command::cargo_bin("sniptip")?;

    // Run the query command without arguments
    cmd.env("SNIPS_BASE", "/tmp").arg("query");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("arguments were not provided"));

    // Add a sniptip
    let mut cmd_add = Command::cargo_bin("sniptip")?;
    cmd_add
        .env("SNIPS_BASE", "/tmp")
        .args(&["add", "sniptip_name", "<h1>Wow</h1>"]);
    cmd_add
        .assert()
        .success()
        .stdout(predicate::str::contains("Sniptip saved!"));

    // Run the list command with sniptips stored
    cmd.args(&["sniptip_name"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("sniptip_name"));

    fs::remove_dir_all("/tmp/.sniptip").ok();

    Ok(())
}

#[test]
fn show_sniptip() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir("/tmp/.sniptip").ok();

    let mut cmd = Command::cargo_bin("sniptip")?;

    // Run the show command without arguments
    cmd.env("SNIPS_BASE", "/tmp").arg("show");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("arguments were not provided"));

    // Add a sniptip
    let mut cmd_add = Command::cargo_bin("sniptip")?;
    cmd_add
        .env("SNIPS_BASE", "/tmp")
        .args(&["add", "sniptip_name", "<h1>Wow</h1>"]);
    cmd_add
        .assert()
        .success()
        .stdout(predicate::str::contains("Sniptip saved!"));

    // Run the show command to get the stored sniptip
    cmd.args(&["sniptip_name"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("<h1>Wow</h1>"));

    fs::remove_dir_all("/tmp/.sniptip").ok();

    Ok(())
}

#[test]
fn delete_sniptip() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir("/tmp/.sniptip").ok();

    let mut cmd = Command::cargo_bin("sniptip")?;

    // Run the delete command without arguments
    cmd.env("SNIPS_BASE", "/tmp").arg("delete");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("arguments were not provided"));

    // Add a sniptip
    let mut cmd_add = Command::cargo_bin("sniptip")?;
    cmd_add
        .env("SNIPS_BASE", "/tmp")
        .args(&["add", "sniptip_name", "<h1>Wow</h1>"]);
    cmd_add
        .assert()
        .success()
        .stdout(predicate::str::contains("Sniptip saved!"));

    // Run the delete command
    cmd.args(&["sniptip_name"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Sniptip deleted!"));

    // Try to show the deleted sniptip
    let mut cmd_query = Command::cargo_bin("sniptip")?;
    cmd_query
        .env("SNIPS_BASE", "/tmp")
        .args(&["show", "sniptip_name"]);
    cmd_query
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Unable to access sniptip <sniptip_name> at path: /tmp/.sniptip/sniptip_name",
        ));

    fs::remove_dir_all("/tmp/.sniptip").ok();

    Ok(())
}
