use std::io::Write;

use atros::{
    args::CliArgs, cache::Cache, executor::Executor, get_active_system, get_step_file_paths,
    initialize::initialize, system_pacman::PackageManager,
};

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    match args {
        CliArgs::Init => {
            initialize()?;
        }
        CliArgs::Run => {
            let mut cache = Cache::get()?;

            let system = get_active_system()?;
            let package_manager = PackageManager::get_by_system(&system);
            let executor = Executor::try_get()?;

            let step_file_paths = get_step_file_paths()?;

            for path in step_file_paths {
                let file_cache = cache.files.entry(path.clone()).or_default();

                executor
                    .parse(&path)?
                    .run(&system, &package_manager, file_cache)?;
                std::io::stdout().write_all(b"\n")?;
            }

            cache.save()?;
        }
    }

    Ok(())
}
