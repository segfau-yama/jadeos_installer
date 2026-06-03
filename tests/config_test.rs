use jade_installer::{
    api::config::{load_config, save_config},
    gui::state::InstallerConfig,
};
use tempfile::tempdir;

#[test]
fn installer_toml_can_roundtrip() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("installer.toml");

    let expected = InstallerConfig {
        schema_version: 1,
        hostname: "jadeos".to_string(),
        username: "jade".to_string(),
        target_disk: "/dev/nvme0n1".to_string(),
        disk_erase_confirmed: false,
    };

    save_config(&path, &expected).expect("save config");
    let actual = load_config(&path).expect("load config");

    assert_eq!(actual, expected);
}

#[test]
fn password_is_not_saved_in_installer_toml() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("installer.toml");

    let config = InstallerConfig {
        schema_version: 1,
        hostname: "jadeos".to_string(),
        username: "jade".to_string(),
        target_disk: "/dev/sda".to_string(),
        disk_erase_confirmed: true,
    };

    save_config(&path, &config).expect("save config");
    let content = std::fs::read_to_string(path).expect("read installer.toml");

    assert!(!content.contains("password"));
}
