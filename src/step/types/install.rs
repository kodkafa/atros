use serde::{Deserialize, Serialize};

use super::System;

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageWithSettings {
    pub system: Option<System>,
    pub list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PackagesToInstall {
    Primal(String),
    WithSettings(PackageWithSettings),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsWithSystem {
    pub system: Option<System>,
    pub list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Param {
    Primal(String),
    WithSettings(ParamsWithSystem),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstallTask {
    pub system: Option<System>,
    pub packages: Vec<PackagesToInstall>,
    pub params: Option<Vec<Param>>,
}
