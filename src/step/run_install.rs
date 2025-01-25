use std::process::Command;

use crate::{cache::CachedTask, system_pacman::PackageManager};

use super::{
    tools::{is_cmd_passes, println_std},
    InstallTask, PackageToInstall, System,
};

pub fn run(
    task: &InstallTask,
    system: &System,
    package_manager: &PackageManager,
    task_cache: &mut CachedTask,
) -> anyhow::Result<()> {
    if task.packages.is_empty() {
        println_std("Warning: Package list is empty, continuing")?;
        return Ok(());
    }

    let packages = task
        .packages
        .iter()
        .map(|package| match package {
            PackageToInstall::Primal(p) => p.to_string() + " ",
            PackageToInstall::WithSettings(sys_package) => {
                if sys_package.system.is_none() || sys_package.system.as_ref() == Some(system) {
                    sys_package.list.join(" ") + " "
                } else {
                    "".to_string()
                }
            }
        })
        .collect::<String>();

    let cmd_str = format!(
        "{} {} {}",
        package_manager.install_cmd,
        if let Some(params) = &task.params {
            params.join(" ")
        } else {
            "".to_string()
        },
        packages
    );

    let skip = task_cache.should_skip(&cmd_str)?;

    if skip {
        return Ok(());
    }

    task_cache.command = cmd_str.clone();

    println_std("Installing given package(s)")?;

    let output = Command::new("sh").arg("-c").arg(cmd_str).output()?;

    match is_cmd_passes(&output) {
        Ok(success) => {
            if success {
                task_cache.failed = false;
                println_std("Successfully executed!")?;
            };
        }
        Err(err) => {
            task_cache.failed = true;
            return Err(err);
        }
    }

    Ok(())
}
