use serde::{Deserialize, Serialize};

use super::System;

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageWithSettings {
    pub system: Option<System>,
    pub list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PackageToInstall {
    Primal(String),
    WithSettings(PackageWithSettings),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstallTask {
    pub system: Option<System>,
    pub packages: Vec<PackageToInstall>,
    pub params: Option<Vec<String>>,
}
