use colored::Colorize;

use crate::get_home_var;
use std::{
    fs,
    io::{self, Write},
    path::Path,
    process::Command,
};

pub fn initialize() -> anyhow::Result<()> {
    let home = get_home_var()?;
    let path_str = format!("{home}/.config/atros");

    let folder_path = Path::new(&path_str);

    let is_exists = folder_path.exists() && folder_path.is_dir();
    if is_exists {
        let mut input = String::new();

        print!(
            "{} .config/atros/ folder already exists. Would you like to rewrite it? {}, and this can't be undone. (y/N) ",
            "Warning:".yellow(),
            "This will delete all of your previous tasks".red()
        );
        io::stdout().flush()?;

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().to_lowercase();

        if input != "y" {
            println!("Exiting...");
            return Ok(());
        } else {
            fs::remove_dir_all(folder_path)?;
        }
    }
    println!("Initializing...");

    let git_clone = Command::new("git")
        .arg("clone")
        .arg("--recurse-submodules")
        .arg("https://github.com/kodkafa/atros-config-template")
        .arg(path_str)
        .output()?;

    if !git_clone.status.success() {
        println!("Cloning Atros configuration template with git failed. Is git installed on your system?");
        println!("Git output:\n{}", String::from_utf8(git_clone.stderr)?);
        return Err(anyhow::anyhow!("Git failed to clone"));
    } else {
        println!("Successfully initialized!");
    }

    Ok(())
}
