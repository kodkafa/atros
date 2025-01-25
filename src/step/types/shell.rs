use serde::{Deserialize, Serialize};

use super::System;

#[derive(Serialize, Deserialize, Debug)]
pub struct ShellTask {
    // #[serde(flatten)]
    // boilerplate: TaskBoilerPlate,
    pub system: Option<System>,
    pub cmd: String,
}
