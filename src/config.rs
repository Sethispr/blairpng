use anyhow::{Context, Result};
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Configs loaded from blair.toml or the defaults.
/// For libdeflater it supports 1-12 any is fine (12 for max compression).
/// Its also way faster than zopfli (lv 12 libdeflater is faster than lv 12 zopfli).
/// Zopfli supports 1-255 but I dont suggest 255 it will not be that effective after 10 iterations.
/// So use around 1-10 iterations for zopfli.
///
/// Fields:
/// - `level`: Default optimization level (0-6)
/// - `strip_metadata`: Strip metadata from .png files
/// - `optimize_alpha`: Enable alpha channel optimization (the transparent pixels)
/// - `fast_eval`: Use faster but less accurate filter evaluation
/// - `filters`: List of row filters to try (e.g. "none", "sub")
/// - `deflater`: Compression backend ("libdeflater" or "zopfli")
/// - `deflate_level`: Compression level (iterations) for the deflaters (1-12, 1-255)
#[derive(Debug, Deserialize)]
pub struct Config {
    pub level: u8,
    pub strip_metadata: bool,
    pub optimize_alpha: bool,
    pub fast_eval: bool,
    pub filters: Vec<String>,
    pub deflater: String,
    pub deflate_level: u8,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            level: 6,
            strip_metadata: true,
            optimize_alpha: true,
            fast_eval: false,
            filters: ["none", "sub", "up", "average", "paeth", "minsum", "bigrams"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            deflater: "libdeflater".to_string(),
            deflate_level: 12,
        }
    }
}

/// Load configs from blair.toml or use the defaults.
pub fn load_config(path: Option<&Path>) -> Result<Config> {
    let config_path = path
        .map(|p| p.to_owned())
        .unwrap_or_else(|| PathBuf::from("blair.toml"));
    if config_path.exists() {
        let data = fs::read_to_string(&config_path)
            .with_context(|| format!("Couldn't read config file: {}", config_path.display()))?;
        toml::from_str(&data).context("Config parsing failed")
    } else {
        Ok(Default::default())
    }
}

/// Generates a suggested config file in the current directory.
pub fn gen_config() -> Result<()> {
    let path = PathBuf::from("blair.toml");
    if path.exists() {
        anyhow::bail!("blair.toml already exists in current directory");
    }
    let example = r#"# Blair Example Configs
# You can customize these options but these are set as their the best lossless and fastest settings

level = 6
strip_metadata = true
optimize_alpha = true
fast_eval = false
filters = ["none", "sub", "up", "average", "paeth", "minsum", "bigrams"]
deflater = "libdeflate"
deflate_level = 12
"#;
    fs::write(&path, example)?;
    println!("✓ Generated blair.toml filled example config");
    Ok(())
}
