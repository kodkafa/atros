use std::process::Command;

use crate::{cache::CachedTask, system_pacman::PackageManager};

use super::{
    tools::{is_cmd_passes, println_std},
    InstallTask, PackagesToInstall, Param, System,
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

    let mut should_skip_recursion: Option<anyhow::Result<()>> = None;

    let packages = task
        .packages
        .iter()
        .map(|package| match package {
            PackagesToInstall::Primal(p) => p.to_string() + " ",
            PackagesToInstall::WithSettings(package_with_sett) => {
                if let Some(params_package) = &package_with_sett.params {
                    let mut new_package = package_with_sett.to_owned();
                    new_package.params = None;

                    let mut recursed_task = InstallTask {
                        system: package_with_sett.system.to_owned(),
                        packages: vec![PackagesToInstall::WithSettings(new_package)],
                        params: task.params.to_owned(),
                    };

                    recursed_task
                        .params
                        .get_or_insert(Vec::new())
                        .extend(params_package.to_owned());

                    let res = run(&recursed_task, system, package_manager, task_cache);
                    should_skip_recursion = Some(res);
                }

                if package_with_sett.system.is_none()
                    || package_with_sett.system.as_ref() == Some(system)
                {
                    package_with_sett.list.join(" ") + " "
                } else {
                    "".to_string()
                }
            }
        })
        .collect::<String>();

    if let Some(recursion_res) = should_skip_recursion {
        return recursion_res;
    }

    let cmd_string = format!(
        "{} {} {}",
        package_manager.install_cmd,
        if let Some(params) = &task.params {
            params
                .iter()
                .map(|param| match param {
                    Param::Primal(p) => p.to_string() + " ",
                    Param::WithSystem(sys_param) => {
                        if sys_param.system.is_none() || sys_param.system.as_ref() == Some(system) {
                            sys_param.list.join(" ") + " "
                        } else {
                            "".to_string()
                        }
                    }
                })
                .collect()
        } else {
            "".to_string()
        },
        packages
    );

    let skip = task_cache.should_skip(&cmd_string)?;

    if skip {
        return Ok(());
    }

    task_cache.command = cmd_string.clone();

    println_std("Installing given package(s)")?;

    let output = Command::new("sh").arg("-c").arg(cmd_string).output()?;

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
