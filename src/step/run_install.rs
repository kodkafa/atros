use crate::{cache::CachedTask, system_pacman::PackageManager, tools::println_std};

use super::{InstallTask, System};

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
    let refined_tasks = task.refine_tasks(system);

    for (idx, task) in refined_tasks.into_iter().enumerate() {
        package_manager.install(task, if idx == 0 { Some(task_cache) } else { None })?;
    }

    Ok(())
}
