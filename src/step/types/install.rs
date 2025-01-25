use serde::{Deserialize, Serialize};

use super::System;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageWithSettings {
    pub system: Option<System>,
    pub list: Vec<String>,
    pub params: Option<Vec<Param>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PackagesToInstall {
    Primal(String),
    WithSettings(PackageWithSettings),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParamsWithSystem {
    pub system: Option<System>,
    pub list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Param {
    Primal(String),
    WithSystem(ParamsWithSystem),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstallTask {
    pub system: Option<System>,
    pub packages: Vec<PackagesToInstall>,
    pub params: Option<Vec<Param>>,
}
