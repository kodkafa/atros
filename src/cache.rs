use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{self, BufReader, Read, Seek, Write},
    path::Path,
};

use crate::get_home_var;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Cache {
    pub files: HashMap<String, CacheFile>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CacheFile {
    pub tasks: Vec<CachedTask>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CachedTask {
    pub failed: bool,
    pub command: String,
}

impl CachedTask {
    pub fn should_skip(&self, cmd: &str) -> anyhow::Result<bool> {
        if !self.failed && self.command == cmd {
            let mut input = String::new();

            print!("{}\n{}   {} You ran the same task before and it succeed. Still want to run it again? (y/N) ", "│".bright_black(), "│".bright_black(), "Warning:".yellow());
            io::stdout().flush()?;

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input = input.trim().to_lowercase();

            if input == "y" {
                println!(
                    "{}   Running again...\n{}",
                    "│".bright_black(),
                    "│".bright_black()
                );
                return Ok(false);
            } else {
                println!("{}   Skipping...", "│".bright_black());
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl Cache {
    pub fn get() -> anyhow::Result<Self> {
        let cache_file = get_cache_file()?;
        let mut reader = BufReader::new(cache_file);
        let content_length = reader.seek(std::io::SeekFrom::End(0))? as usize;
        reader.seek(std::io::SeekFrom::Start(0))?;

        let mut content = String::with_capacity(content_length);

        reader.read_to_string(&mut content)?;

        let cache: anyhow::Result<Cache> =
            serde_json::from_str(&content).or_else(|_| Ok(Cache::default()));

        cache
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let stringified = serde_json::to_string(self)?;

        let file_path = get_cache_path()?;
        let mut file = File::create(file_path)?;
        file.write_all(stringified.as_bytes())?;

        Ok(())
    }
}

fn get_cache_path() -> anyhow::Result<String> {
    let home = get_home_var()?;
    let system_cache = match std::env::consts::OS {
            "macos" => Ok("Library/Caches"),
            "linux" => Ok(".cache"),
            sys => Err(anyhow::anyhow!("Unsupported system is being used: {} Only Mac and Linux distributions are allowed for now", sys)),
        }?;
    let file_path = format!("{home}/{system_cache}/atros/cache.json");

    let path = Path::new(&file_path);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    Ok(file_path)
}

fn get_cache_file() -> anyhow::Result<File> {
    let file_path = get_cache_path()?;

    Ok(OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(file_path)?)
}
