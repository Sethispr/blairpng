use clap::Parser;
use std::path::PathBuf;

/// Cli args for the optimizer.
///
/// Fields:
/// - `directory`: Path to the directory with the .png files (default: current directory)
/// - `level`: Optimization level (0-6, default: 6)
/// - `threads`: Number of threads to use (optional)
/// - `quiet`: Doesn't show the progress bar (optional)
/// - `verbose`: Print detailed info for each file (optional)
/// - `config`: Path to custom config file (optional)
/// - `init`: Generate example config file (optional)
#[derive(Parser)]
#[command(name = "blairpng")]
#[command(about = "Blair's card png optimizer", long_about = None)]
pub struct Args {
    #[arg(default_value = ".", help = "Directory with .png files")]
    pub directory: PathBuf,
    #[arg(
        short,
        long,
        default_value_t = 6,
        help = "Optimization level (0-6, higher = more compression, slower)"
    )]
    pub level: u8,
    #[arg(short = 'j', long, help = "Number of threads to use")]
    pub threads: Option<usize>,
    #[arg(short, long, help = "Don't show the progress bar")]
    pub quiet: bool,
    #[arg(short, long, help = "Print detailed infos for each file")]
    pub verbose: bool,
    #[arg(short, long, help = "Path to custom config file")]
    pub config: Option<PathBuf>,
    #[arg(
        long,
        help = "This generates an example config file to use, blairpng does not need one but you can use for custom opts"
    )]
    pub init: bool,
}
