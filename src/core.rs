use crate::config::Config;
use anyhow::Result;
use indexmap::IndexSet;
use oxipng::{Deflaters, InFile, Options, OutFile, RowFilter, StripChunks};
use std::{
    fs,
    num::NonZeroU8,
    path::{Path, PathBuf},
};

/// Stats for the file size before and after.
///
/// Fields:
/// - `before`: Original file size in bytes
/// - `after`: Optimized file size in bytes
#[derive(Copy, Clone, Default)]
pub struct Stats {
    pub before: u64,
    pub after: u64,
}

impl Stats {
    pub fn reduction(&self) -> i64 {
        self.before as i64 - self.after as i64
    }
    pub fn reduction_pct(&self) -> f64 {
        if self.before == 0 {
            return 0.0;
        }
        (self.reduction() as f64 / self.before as f64) * 100.0
    }
}

/// Parse string into a RowFilter enum value.
pub fn parse_filter(name: &str) -> Option<RowFilter> {
    let lower = name.to_lowercase();
    match lower.as_str() {
        "none" => Some(RowFilter::None),
        "sub" => Some(RowFilter::Sub),
        "up" => Some(RowFilter::Up),
        "average" => Some(RowFilter::Average),
        "paeth" => Some(RowFilter::Paeth),
        "minsum" => Some(RowFilter::MinSum),
        "entropy" => Some(RowFilter::Entropy),
        "bigrams" => Some(RowFilter::Bigrams),
        "bigent" => Some(RowFilter::BigEnt),
        "brute" => Some(RowFilter::Brute),
        _ => {
            eprintln!("Warning: unknown filter '{}', skipping", name);
            None
        }
    }
}

/// Build opts here.
pub fn build_options(config: &Config, level_override: Option<u8>) -> Options {
    let effective_level = level_override.unwrap_or(config.level);
    let mut opts = Options::from_preset(effective_level);
    if config.strip_metadata {
        opts.strip = StripChunks::Safe;
    } else {
        opts.strip = StripChunks::None;
    }
    opts.optimize_alpha = config.optimize_alpha;
    opts.fast_evaluation = config.fast_eval;
    let mut filter_set = IndexSet::new();
    for name in &config.filters {
        if let Some(filter) = parse_filter(name) {
            filter_set.insert(filter);
        }
    }
    opts.filter = filter_set;
    if config.deflater == "zopfli" {
        let iters = match NonZeroU8::new(config.deflate_level) {
            Some(v) => v,
            None => NonZeroU8::new(1).unwrap(),
        };
        opts.deflate = Deflaters::Zopfli { iterations: iters };
    } else {
        /// So technically any options or putting libdeflate etc also just chooses libdeflater.
        opts.deflate = Deflaters::Libdeflater {
            compression: config.deflate_level,
        };
    }
    opts
}

/// Find the .png files in the directory.
pub fn find_pngs(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut pngs = vec![];
    let dir_iter = fs::read_dir(dir)?;
    for res in dir_iter {
        let ent = res?;
        let p = ent.path();
        let ext = p.extension();
        if let Some(e) = ext {
            if e.to_ascii_lowercase() == "png" {
                pngs.push(p);
            }
        }
    }
    Ok(pngs)
}

/// Optimize the .png file and return its stats.
pub fn optimize_png(path: &Path, opts: &Options, verbose: bool) -> Result<Stats> {
    let orig_size = fs::metadata(path)?.len();
    let opt_res = oxipng::optimize(
        &InFile::Path(path.to_path_buf()),
        &OutFile::Path(path.to_path_buf()),
        opts,
    );
    let new_size = if let Ok(()) = opt_res {
        fs::metadata(path)?.len()
    } else {
        let err = opt_res.unwrap_err();
        eprintln!("Error optimizing {}: {}", path.display(), err);
        orig_size
    };
    let stats = Stats {
        before: orig_size,
        after: new_size,
    };
    if verbose && orig_size != new_size {
        let fname = path.file_name().map_or("", |os| os.to_str().unwrap_or(""));
        println!(
            " {} optimized to {:+.1}% ({:+} bytes)",
            fname,
            stats.reduction_pct(),
            stats.reduction()
        );
    }
    Ok(stats)
}

pub fn print_banner() {
    println!(
        r#"
 _______     .---.        ____    .-./`) .-------.     
\  ____  \   | ,_|      .'  __ `. \ .-.')|  _ _   \    
| |    \ | ,-./  )     /   '  \  \/ `-' \| ( ' )  |    
| |____/ / \  '_ '`   |___|  /  | `-'`"'|(_ o _) /    
|   _ _ '.  > (_)  )      _.-`   | .---. | (_,_).' __  
|  ( ' )  \(  .  .-'   .'   _    | |   | |  |\ \  |  | 
| (_;_}_) | `-'`-'|___ |  _( )_  | |   | |  | \ `'   / 
|  (_,_)  /  |        \\ (_ o _) / |   | |  |  \    /  
/_______.'   `--------` '.(_,_).'  '---' ''-'   `'-'   
"#
    );
}
