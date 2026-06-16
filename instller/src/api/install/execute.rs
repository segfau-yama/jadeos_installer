use super::types::{InstallPhase, InstallPlan, InstallationReport};

pub fn run_install_plan(plan: &InstallPlan) -> InstallationReport {
    let mut log = vec![
        "Scaffold mode: install commands are planned but not executed yet.".to_string(),
        format!("Target disk: {}", plan.target_disk),
    ];

    for command in &plan.commands {
        log.push(format!(
            "[{}] {}",
            command.phase.label(),
            command.render_command()
        ));
    }

    InstallationReport {
        final_phase: InstallPhase::Finish,
        current_command: None,
        log,
    }
}
