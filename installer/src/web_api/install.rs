use crate::api::install::{InstallPhase, InstallPlan, InstallationReport};
use crate::gui::state::InstallerConfig;

pub fn run_install_plan(
    config: &InstallerConfig,
    _password: &str,
    plan: &InstallPlan,
) -> InstallationReport {
    let mut log = vec![
        "Web demo mode is enabled.".to_string(),
        format!(
            "Simulating install for {} on {}",
            config.hostname.trim(),
            plan.target_disk
        ),
        "No disk changes, mounts, or sudo commands will be executed in the browser build."
            .to_string(),
    ];

    for command in &plan.commands {
        log.push(format!(
            "[{}] {}",
            command.phase.label(),
            command.description
        ));
        log.push(format!("$ {}", command.render_command()));
    }

    log.push(format!(
        "Simulated host configuration would be generated for user {}.",
        config.username.trim()
    ));
    log.push("Web demo completed successfully.".to_string());

    InstallationReport {
        final_phase: InstallPhase::Finish,
        current_command: None,
        log,
        error_message: None,
    }
}
