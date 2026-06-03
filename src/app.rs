use crate::gui::{
    pages::{disk, install, summary, user, welcome},
    state::{InstallerState, InstallerStep},
};

pub fn render_app() -> String {
    let state = InstallerState::default();

    match state.step {
        InstallerStep::Welcome => welcome::render(),
        InstallerStep::User => user::render(),
        InstallerStep::Disk => disk::render(),
        InstallerStep::Summary => summary::render(&state.config),
        InstallerStep::Install => install::render(),
    }
}
