use serde::{Deserialize, Serialize};

use super::System;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageWithSettings {
    pub system: Option<System>,
    pub list: Vec<String>,
    pub params: Option<Vec<Param>>,
}

impl From<InstallTask> for PackageWithSettings {
    fn from(value: InstallTask) -> Self {
        let mut list = vec![];
        let mut params = value.params.unwrap_or_default();

        value.packages.iter().for_each(|p| match p {
            PackagesToInstall::Primal(package) => list.push(package.to_owned()),
            PackagesToInstall::WithSettings(package) => {
                list.extend(package.list.to_owned());
                if let Some(p_params) = &package.params {
                    params.extend(p_params.to_owned());
                }
            }
        });

        PackageWithSettings {
            system: value.system,
            list,
            params: if !params.is_empty() {
                Some(params)
            } else {
                None
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstallTask {
    pub system: Option<System>,
    pub packages: Vec<PackagesToInstall>,
    pub params: Option<Vec<Param>>,
}

#[derive(Debug)]
pub struct RefinedInstallTask {
    pub packages: Vec<String>,
    pub params: Vec<String>,
}

impl InstallTask {
    pub fn refine_tasks(&self, system: &System) -> Vec<RefinedInstallTask> {
        if let Some(ref pkg_system) = self.system {
            if pkg_system != system {
                return vec![];
            }
        }

        let mut refined_tasks: Vec<RefinedInstallTask> = vec![RefinedInstallTask {
            packages: vec![],
            params: {
                if let Some(ref pkg_params) = self.params {
                    let mut new_params = vec![];

                    pkg_params.iter().for_each(|param| match param {
                        Param::Primal(current_param) => new_params.push(current_param.to_owned()),
                        Param::WithSystem(current_param) => {
                            new_params.extend(current_param.list.to_owned());
                        }
                    });

                    new_params
                } else {
                    vec![]
                }
            },
        }];

        for p in &self.packages {
            match p {
                PackagesToInstall::Primal(pkg) => refined_tasks[0].packages.push(pkg.to_owned()),
                PackagesToInstall::WithSettings(current_pkg) => {
                    if let Some(ref pkg_system) = current_pkg.system {
                        if pkg_system != system {
                            continue;
                        }
                    }

                    if let Some(ref pkg_params) = current_pkg.params {
                        let mut new_params = refined_tasks[0].params.clone();

                        pkg_params.iter().for_each(|param| match param {
                            Param::Primal(current_param) => {
                                new_params.push(current_param.to_owned())
                            }
                            Param::WithSystem(current_param) => {
                                new_params.extend(current_param.list.to_owned());
                            }
                        });

                        refined_tasks.push(RefinedInstallTask {
                            packages: current_pkg.list.to_owned(),
                            params: new_params,
                        });
                    } else {
                        refined_tasks[0]
                            .packages
                            .extend(current_pkg.list.to_owned());
                    }
                }
            }
        }

        refined_tasks
    }
}
