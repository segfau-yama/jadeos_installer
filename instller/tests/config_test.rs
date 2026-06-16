use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use jade_installer::api::config::{load_installer_config, save_installer_config};
use jade_installer::gui::state::InstallerConfig;

#[test]
fn installer_config_round_trip_works() {
    let path = unique_temp_path("installer-config-round-trip.toml");
    let config = InstallerConfig {
        schema_version: 1,
        hostname: "jadeos".to_string(),
        username: "jade".to_string(),
        target_disk: "/dev/nvme0n1".to_string(),
        disk_erase_confirmed: false,
    };

    save_installer_config(&path, &config).expect("config should save");
    let loaded = load_installer_config(&path).expect("config should load");

    assert_eq!(loaded, config);

    let _ = fs::remove_file(path);
}

#[test]
fn password_is_not_persisted_to_installer_toml() {
    let path = unique_temp_path("installer-config-no-password.toml");
    let config = InstallerConfig {
        schema_version: 1,
        hostname: "jadeos".to_string(),
        username: "jade".to_string(),
        target_disk: "/dev/nvme0n1".to_string(),
        disk_erase_confirmed: false,
    };

    save_installer_config(&path, &config).expect("config should save");
    let contents = fs::read_to_string(&path).expect("config file should exist");

    assert!(!contents.to_ascii_lowercase().contains("password"));

    let _ = fs::remove_file(path);
}

fn unique_temp_path(filename: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should move forward")
        .as_nanos();

    std::env::temp_dir().join(format!("{nanos}-{filename}"))
}
