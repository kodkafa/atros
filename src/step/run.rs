use std::io::{stdout, Write};

use colored::Colorize;

use crate::{
    cache::{CacheFile, CachedTask},
    step::{Step, Task},
    system_pacman::PackageManager,
    tools::println_std,
};

use super::{run_install, run_shell, System};

pub fn run(
    step: &Step,
    system: &System,
    package_manager: &PackageManager,
    file_cache: &mut CacheFile,
) -> anyhow::Result<()> {
    stdout().write_all(format!("{} Step: {}\n", "┌──".bright_black(), step.title).as_bytes())?;

    if let Some(desc) = &step.description {
        println_std(format!("Description: {desc}"))?;
    }

    for (idx, task) in step.tasks.iter().enumerate() {
        println_std(format!("\n{} Task No: {}", "├─>".bright_black(), idx + 1))?;

        let task_system = match task {
            Task::Install(task) => &task.system,
            Task::Shell(task) => &task.system,
        };

        if let Some(sys) = task_system {
            if system != sys {
                println_std(format!(
                    "Warning: This task is defined for {sys:#?}, Skipping..."
                ))?;
                continue;
            }
        }

        let task_cache = match file_cache.tasks.get_mut(idx) {
            Some(task) => task,
            None => {
                file_cache.tasks.push(CachedTask {
                    failed: true,
                    command: "".to_string(),
                });

                file_cache
                    .tasks
                    .get_mut(idx)
                    .ok_or(anyhow::anyhow!("An error occured while getting task cache"))?
            }
        };

        match task {
            Task::Install(task) => {
                run_install::run(task, system, package_manager, task_cache)?;
            }
            Task::Shell(task) => {
                run_shell::run(task, task_cache)?;
            }
        }
    }

    Ok(())
}
