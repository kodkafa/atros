use std::process::Command;

use crate::{cache::CachedTask, system_pacman::PackageManager};

use super::{
    tools::{is_cmd_passes, println_std},
    InstallTask,
};

pub fn run(
    task: &InstallTask,
    package_manager: &PackageManager,
    task_cache: &mut CachedTask,
) -> anyhow::Result<()> {
    if task.packages.is_empty() {
        println_std("Warning: Package list is empty, continuing")?;
        return Ok(());
    }

    let cmd_str = format!(
        "{} {} {}",
        package_manager.install_cmd,
        if let Some(params) = &task.params {
            params.join(" ")
        } else {
            "".to_string()
        },
        task.packages.join(" ")
    );

    let skip = task_cache.should_skip(&cmd_str)?;

    if skip {
        return Ok(());
    }

    task_cache.command = cmd_str.clone();

    println_std(format!("Installing {} package(s)", task.packages.len(),))?;

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
