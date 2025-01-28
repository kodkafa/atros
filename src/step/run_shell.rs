use std::process::Command;

use colored::Colorize;

use crate::{
    cache::CachedTask,
    tools::{is_cmd_passes, prettify_output, println_no_space_std, println_std, smallify_command},
};

use super::types::ShellTask;

pub fn run(task: &ShellTask, task_cache: &mut CachedTask) -> anyhow::Result<()> {
    println_std("Task Type: shell")?;
    println_std(format!("Command: `{}`", smallify_command(&task.cmd)))?;
    println_std("Running...")?;

    let skip = task_cache.should_skip(&task.cmd)?;

    if skip {
        return Ok(());
    }

    task_cache.command = task.cmd.clone();
    let output = Command::new("sh").arg("-c").arg(&task.cmd).output()?;

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

    println_no_space_std(format!(
        "{}
{}
{}   Output:
{}
{}",
        "_____________________________".bright_black(),
        "│".bright_black(),
        "│".bright_black(),
        prettify_output(&String::from_utf8(output.stdout)?),
        "│_____________________________".bright_black(),
    ))?;

    Ok(())
}
