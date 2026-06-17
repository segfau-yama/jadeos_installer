#[cfg(not(all(feature = "web", not(feature = "desktop"))))]
mod execute;
mod nix_templates;
mod plan;
mod types;
mod validate;

#[cfg(all(feature = "web", not(feature = "desktop")))]
pub use crate::web_api::install::run_install_plan;
#[cfg(not(all(feature = "web", not(feature = "desktop"))))]
pub use execute::{run_install_plan, run_install_plan_with_progress};
pub use plan::{generate_install_plan, preview_install_plan};
pub use types::{InstallCommand, InstallError, InstallPhase, InstallPlan, InstallationReport};
