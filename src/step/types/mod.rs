pub use install::*;
use serde::{Deserialize, Serialize};
pub use shell::*;

mod general;
mod install;
mod shell;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum System {
    Mac,
    Arch,
    Debian,
    Fedora,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Task {
    Install(InstallTask),
    Shell(ShellTask),
}
