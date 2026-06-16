use dioxus::prelude::{Signal, WritableExt};

use crate::api::disk::list_disks;
use crate::api::install::{generate_install_plan, preview_install_plan, run_install_plan};
use crate::gui::state::{InstallerState, InstallerStep};
use crate::gui::validation::{
    disk_validation_errors, summary_validation_errors, user_validation_errors,
};

pub fn preview_plan(state: &InstallerState) -> Option<crate::api::install::InstallPlan> {
    preview_install_plan(&state.config).ok()
}

pub fn clear_error(mut state: Signal<InstallerState>) {
    state.write().ui.error_message = None;
}

pub fn navigate_to(mut state: Signal<InstallerState>, step: InstallerStep) {
    let mut draft = state.write();
    draft.ui.step = step;
    draft.ui.error_message = None;
}

pub fn refresh_disks(mut state: Signal<InstallerState>) {
    match list_disks() {
        Ok(disks) => {
            let mut draft = state.write();
            draft.ui.available_disks = disks;
            draft.ui.error_message = None;
        }
        Err(error) => state.write().ui.error_message = Some(error.to_string()),
    }
}

pub fn continue_from_user(mut state: Signal<InstallerState>) {
    let errors = {
        let snapshot = state();
        user_validation_errors(&snapshot.config, &snapshot.user)
    };

    let mut draft = state.write();
    if errors.is_empty() {
        draft.ui.step = InstallerStep::Disk;
        draft.ui.error_message = None;
    } else {
        draft.ui.error_message = Some(errors.join(" "));
    }
}

pub fn continue_from_disk(mut state: Signal<InstallerState>) {
    let errors = {
        let snapshot = state();
        disk_validation_errors(&snapshot.config)
    };

    let mut draft = state.write();
    if errors.is_empty() {
        draft.ui.step = InstallerStep::Summary;
        draft.ui.error_message = None;
    } else {
        draft.ui.error_message = Some(errors.join(" "));
    }
}

pub fn start_install(mut state: Signal<InstallerState>) {
    let summary_errors = {
        let snapshot = state();
        summary_validation_errors(&snapshot.config, &snapshot.user)
    };

    if !summary_errors.is_empty() {
        state.write().ui.error_message = Some(summary_errors.join(" "));
        return;
    }

    let config = state().config.clone();
    match generate_install_plan(&config) {
        Ok(plan) => {
            let report = run_install_plan(&plan);
            let mut draft = state.write();
            draft.runtime.install_plan = Some(plan);
            draft.runtime.install_phase = report.final_phase;
            draft.runtime.current_command = report.current_command;
            draft.runtime.install_log = report.log;
            draft.ui.error_message = None;
            draft.ui.step = InstallerStep::Install;
        }
        Err(error) => state.write().ui.error_message = Some(error.to_string()),
    }
}
