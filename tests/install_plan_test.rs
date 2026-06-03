use jade_installer::{
    api::install::{InstallValidationError, build_install_plan, validate},
    gui::state::InstallerConfig,
};

fn base_config() -> InstallerConfig {
    InstallerConfig {
        schema_version: 1,
        hostname: "jadeos".to_string(),
        username: "jade".to_string(),
        target_disk: "/dev/nvme0n1".to_string(),
        disk_erase_confirmed: true,
    }
}

#[test]
fn empty_hostname_is_rejected() {
    let mut config = base_config();
    config.hostname.clear();

    assert_eq!(
        validate(&config),
        Err(InstallValidationError::EmptyHostname)
    );
}

#[test]
fn empty_username_is_rejected() {
    let mut config = base_config();
    config.username.clear();

    assert_eq!(
        validate(&config),
        Err(InstallValidationError::EmptyUsername)
    );
}

#[test]
fn empty_target_disk_is_rejected() {
    let mut config = base_config();
    config.target_disk.clear();

    assert_eq!(
        validate(&config),
        Err(InstallValidationError::EmptyTargetDisk)
    );
}

#[test]
fn install_plan_requires_disk_erase_confirmation() {
    let mut config = base_config();
    config.disk_erase_confirmed = false;

    assert_eq!(
        build_install_plan(&config),
        Err(InstallValidationError::DiskEraseNotConfirmed)
    );
}

#[test]
fn install_plan_includes_destructive_actions() {
    let plan = build_install_plan(&base_config()).expect("plan should be created");
    assert!(plan.commands.iter().any(|command| command.destructive));
}

#[test]
fn install_plan_includes_nixos_generate_config() {
    let plan = build_install_plan(&base_config()).expect("plan should be created");
    assert!(plan.commands.iter().any(|command| {
        command.command == "nixos-generate-config" && command.args == ["--root", "/mnt"]
    }));
}

#[test]
fn install_plan_includes_fixed_flake_target() {
    let plan = build_install_plan(&base_config()).expect("plan should be created");
    assert!(plan.commands.iter().any(|command| {
        command.command == "nixos-install" && command.args == ["--flake", "/mnt/etc/nixos#jadeos"]
    }));
}
