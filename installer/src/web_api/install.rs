use crate::api::install::{InstallPhase, InstallPlan, InstallationReport};
use crate::gui::state::InstallerConfig;

pub fn run_install_plan(
    config: &InstallerConfig,
    _password: &str,
    plan: &InstallPlan,
) -> InstallationReport {
    let log = vec![
        "Web demo mode is enabled.".to_string(),
        format!(
            "Simulating install for {} on {}",
            plan.hostname, plan.target_disk
        ),
        format!(
            "Would create EFI {} and root {} on the selected disk.",
            plan.efi_partition, plan.root_partition
        ),
        format!(
            "Would clone {} into /mnt/etc/nixos before generating host files.",
            plan.repository_url
        ),
        format!(
            "Would generate host {} and user {} configuration, then run nixos-install.",
            config.hostname.trim(),
            config.username.trim()
        ),
        "No disk changes, mounts, or sudo commands will be executed in the browser build."
            .to_string(),
    ];

    InstallationReport {
        final_phase: InstallPhase::Finish,
        current_command: None,
        log: {
            let mut log = log;
            log.push("Web demo completed successfully.".to_string());
            log
        },
        error_message: None,
    }
}
