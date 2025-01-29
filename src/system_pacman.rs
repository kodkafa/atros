use std::process::Command;

use crate::{
    cache::CachedTask,
    step::{RefinedInstallTask, System},
    tools::{is_cmd_passes, println_std},
};

pub struct PackageManager {
    pub cmd: &'static str,
    install_cmd: InstallCmd,
}

pub struct InstallCmd {
    requires_sudo: bool,
    env_str: &'static str,
    command: &'static str,
}

impl PackageManager {
    pub fn get_by_system(system: &System) -> Self {
        let brew = PackageManager {
            cmd: "brew",
            install_cmd: InstallCmd {
                requires_sudo: false,
                env_str: "HOMEBREW_COLOR=1",
                command: "install",
            },
        };

        let pacman = PackageManager {
            cmd: "pacman",
            install_cmd: InstallCmd {
                requires_sudo: false,
                env_str: "",
                command: "-Syu --noconfirm",
            },
        };

        let debian = PackageManager {
            cmd: "apt",
            install_cmd: InstallCmd {
                requires_sudo: true,
                env_str: "",
                command: "install -y",
            },
        };

        let fedora = PackageManager {
            cmd: "dnf",
            install_cmd: InstallCmd {
                requires_sudo: true,
                env_str: "",
                command: "install -y",
            },
        };

        match system {
            System::Mac => brew,
            System::Arch => pacman,
            System::Debian => debian,
            System::Fedora => fedora,
        }
    }

    pub fn install(
        &self,
        mut task: RefinedInstallTask,
        task_cache: Option<&mut CachedTask>,
    ) -> anyhow::Result<()> {
        let mut cmd = self.cmd;

        task.params.retain(|val| {
            if val.starts_with("$atros-") {
                match val.as_str() {
                    "$atros-use-yay" => cmd = "yay",
                    "$atros-use-apt-get" => cmd = "apt-get",
                    _ => panic!("Unknown $atros parameter found"),
                }

                return false;
            }

            true
        });

        let cmd_string = format!(
            "{} {} {} {} {} {}",
            self.install_cmd.env_str,
            if self.install_cmd.requires_sudo {
                "sudo"
            } else {
                ""
            },
            cmd,
            self.install_cmd.command,
            task.params.join(" "),
            task.packages.join(" ")
        );

        if let Some(ref task_cache) = task_cache {
            if task_cache.should_skip(&cmd_string)? {
                return Ok(());
            };
        }

        let output = Command::new("sh").arg("-c").arg(&cmd_string).output()?;

        match is_cmd_passes(&output) {
            Ok(did_succeed) => {
                if did_succeed {
                    if let Some(task_cache) = task_cache {
                        task_cache.command = cmd_string;
                        task_cache.failed = false;
                    }
                    println_std("Successfully executed!")?;
                };
            }
            Err(err) => {
                if let Some(task_cache) = task_cache {
                    task_cache.command = cmd_string;
                    task_cache.failed = true;
                }
                return Err(err);
            }
        }
        Ok(())
    }
}
