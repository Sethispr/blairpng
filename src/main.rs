mod cli;
mod config;
mod core;

use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::time::Instant;

use cli::Args;
use config::load_config;
use core::{build_options, find_pngs, optimize_png, print_banner, Stats};

/// Main entry point here.
fn main() -> Result<()> {
    let args = Args::parse();
    print_banner();

    if args.init {
        config::gen_config()?;
        return Ok(());
    }

    let config = load_config(args.config.as_deref())?;
    if !args.directory.exists() {
        anyhow::bail!("Directory does not exist: {}", args.directory.display());
    }
    let files = find_pngs(&args.directory)?;
    if files.is_empty() {
        println!("No .png files found in {}", args.directory.display());
        return Ok(());
    }
    let num_threads = args
        .threads
        .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, |n| n.get()));
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .ok();
    let opts = build_options(&config, Some(args.level));
    let progress: Option<ProgressBar> = if !args.quiet {
        let bar = ProgressBar::new(files.len() as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{bar:40}] {pos}/{len} {msg}")?
                .progress_chars("█▓░"),
        );
        Some(bar)
    } else {
        None
    };
    let start = Instant::now();
    let results: Vec<Stats> = files
        .par_iter()
        .map(|path| {
            let stats = optimize_png(path, &opts, args.verbose).unwrap_or_default();
            if let Some(ref pb) = progress {
                pb.inc(1);
            }
            stats
        })
        .collect();
    if let Some(pb) = progress {
        pb.finish_and_clear();
    }
    let total_before: u64 = results.iter().map(|s| s.before).sum();
    let total_after: u64 = results.iter().map(|s| s.after).sum();
    let total = Stats {
        before: total_before,
        after: total_after,
    };
    println!(
        "\n✓ Optimized {} files in {:.1}s",
        files.len(),
        start.elapsed().as_secs_f64()
    );
    println!(
        " Saved: {:.1}% ({} bytes)",
        total.reduction_pct(),
        total.reduction()
    );
    println!(" Final size: {} bytes (was {})", total.after, total.before);
    Ok(())
}
