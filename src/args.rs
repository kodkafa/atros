use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub enum CliArgs {
    /// Initialize your first atros configuration on ~/.config/atros/
    Init,
    /// Run defined steps
    Run,
    // Run(RunArgs),
    // /// Create a new step from scratch
    // NewStep,
    // /// Upgrade Atros to next version if there is any
    // Upgrade,
    // /// Clear the cache. It's good for re-executing every step cleanly
    // Clear,
}

// #[derive(Debug, Args)]
// // #[command(version, about, long_about = None)]
// pub struct RunArgs {
//     #[arg(long)]
//     /// To test different step folders, give this parameter a folder path
//     pub steps_path: Option<String>,
// }
