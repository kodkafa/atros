use anyhow::anyhow;
use std::fs;
use step::System;

pub mod args;
pub mod cache;
pub mod executor;
pub mod initialize;
pub mod step;
pub mod system_pacman;

pub fn get_active_system() -> anyhow::Result<System> {
    match std::env::consts::OS {
        "macos" => Ok(System::Mac),
        "linux" => {
            let os_release = fs::read_to_string("/etc/os-release")?.to_lowercase();

            if os_release.contains("debian") {
                Ok(System::Debian)
            } else if os_release.contains("arch") {
                Ok(System::Arch)
            } else if os_release.contains("fedora") {
                Ok(System::Fedora)
            } else {
                Err(anyhow::anyhow!("Unsupported linux distribution is being used. Only Debian, Arch and Fedora based distributions are allowed for now"))
            }
        }
        sys => Err(anyhow::anyhow!("Unsupported system is being used: {} Only Mac and Linux distributions are allowed for now", sys)),
    }
}

pub fn get_home_var() -> anyhow::Result<String> {
    std::env::var_os("HOME")
        .ok_or(anyhow!("HOME variable is not defined"))?
        .into_string()
        .map_err(|_| anyhow!("Cannot convert HOME path into string"))
}

pub fn get_step_file_paths() -> anyhow::Result<Vec<String>> {
    let home = get_home_var()?;

    let entries: Vec<_> = fs::read_dir(format!("{}/.config/atros/steps", home))?
        .filter_map(|e| {
            let entry = e.ok()?;

            if entry.metadata().ok()?.is_dir() {
                return None;
            }

            let path = entry.path();
            let path = path.to_str()?;

            Some(path.to_owned())
        })
        .collect();

    Ok(entries)
}
