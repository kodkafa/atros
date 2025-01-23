use serde::{Deserialize, Serialize};

use crate::{cache::CacheFile, system_pacman::PackageManager};

mod run;
mod run_install;
mod run_shell;
mod tools;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
#[derive(PartialEq)]
pub enum System {
    Mac,
    Arch,
    Debian,
    Fedora,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskBoilerPlate {
    pub system: Option<System>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstallTask {
    // #[serde(flatten)]
    // boilerplate: TaskBoilerPlate,
    pub system: Option<System>,
    pub packages: Vec<String>,
    pub params: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShellTask {
    // #[serde(flatten)]
    // boilerplate: TaskBoilerPlate,
    pub system: Option<System>,
    pub cmd: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Task {
    Install(InstallTask),
    Shell(ShellTask),
}

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
