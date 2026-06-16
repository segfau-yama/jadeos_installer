use std::fs;

use jade_installer::api::disk::parse_lsblk_output;
use jade_installer::api::install::{generate_install_plan, preview_install_plan, InstallError};
use jade_installer::gui::state::InstallerConfig;

#[test]
fn empty_hostname_is_rejected() {
    let error = generate_install_plan(&config_with(|config| config.hostname = String::new()))
        .expect_err("empty hostname should fail");

    assert_eq!(error, InstallError::EmptyHostname);
}

#[test]
fn empty_username_is_rejected() {
    let error = generate_install_plan(&config_with(|config| config.username = String::new()))
        .expect_err("empty username should fail");

    assert_eq!(error, InstallError::EmptyUsername);
}

#[test]
fn empty_target_disk_is_rejected() {
    let error = generate_install_plan(&config_with(|config| config.target_disk = String::new()))
        .expect_err("empty target disk should fail");

    assert_eq!(error, InstallError::EmptyTargetDisk);
}

#[test]
fn missing_erase_confirmation_is_rejected() {
    let error = generate_install_plan(&config_with(|config| config.disk_erase_confirmed = false))
        .expect_err("missing erase confirmation should fail");

    assert_eq!(error, InstallError::DiskEraseNotConfirmed);
}

#[test]
fn plan_contains_destructive_actions() {
    let plan = generate_install_plan(&valid_config()).expect("valid config should produce a plan");

    assert!(plan.commands.iter().any(|command| command.destructive));
}

#[test]
fn plan_contains_nixos_generate_config() {
    let plan = generate_install_plan(&valid_config()).expect("valid config should produce a plan");

    assert!(plan
        .rendered_commands()
        .iter()
        .any(|command| command.contains("nixos-generate-config --root /mnt")));
}

#[test]
fn plan_contains_nixos_install_flake_command() {
    let plan = generate_install_plan(&valid_config()).expect("valid config should produce a plan");

    assert!(plan
        .rendered_commands()
        .iter()
        .any(|command| command.contains("nixos-install --flake /mnt/etc/nixos#jadeos")));
}

#[test]
fn summary_preview_can_be_created_before_erase_confirmation() {
    let preview = preview_install_plan(&config_with(|config| config.disk_erase_confirmed = false))
        .expect("summary should still be able to preview the plan");

    assert_eq!(preview.efi_partition, "/dev/nvme0n1p1");
    assert_eq!(preview.root_partition, "/dev/nvme0n1p2");
}

#[test]
fn lsblk_json_is_parsed_into_disk_list() {
    let fixture = fs::read_to_string("tests/fixtures/lsblk.json").expect("fixture should exist");
    let disks = parse_lsblk_output(&fixture).expect("fixture should parse");

    assert_eq!(disks.len(), 2);
    assert_eq!(disks[0].path, "/dev/nvme0n1");
    assert_eq!(disks[1].path, "/dev/sda");
    assert!(disks[1].mounted);
}

fn valid_config() -> InstallerConfig {
    InstallerConfig {
        schema_version: 1,
        hostname: "jadeos".to_string(),
        username: "jade".to_string(),
        target_disk: "/dev/nvme0n1".to_string(),
        disk_erase_confirmed: true,
    }
}

fn config_with(edit: impl FnOnce(&mut InstallerConfig)) -> InstallerConfig {
    let mut config = valid_config();
    edit(&mut config);
    config
}
