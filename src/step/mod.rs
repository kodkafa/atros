use serde::{Deserialize, Serialize};
pub use types::*;

use crate::{cache::CacheFile, system_pacman::PackageManager};

mod run;
mod run_install;
mod run_shell;
mod tools;
mod types;

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
    pub title: String,
    pub description: Option<String>,
    pub tasks: Vec<Task>,
}

impl Step {
    pub fn run(
        &self,
        system: &System,
        package_manager: &PackageManager,
        file_cache: &mut CacheFile,
    ) -> anyhow::Result<()> {
        run::run(self, system, package_manager, file_cache)?;
        Ok(())
    }
}
