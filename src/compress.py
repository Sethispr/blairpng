# blair/blairBlender/compress.py
import concurrent.futures
import configparser
import glob
import os
import time
from pathlib import Path
from tqdm import tqdm


# Try to load oxipng and makes sure its already installed.
# Do pip install -r requirements.txt or pip install oxipng
# Can add a subprocess here to download requirements automatically.
try:
    import oxipng

    OXIPNG_AVAILABLE = True
except ImportError:
    OXIPNG_AVAILABLE = False

"""
All stats for the file size before and after.
Fields:
- `before`: Original file size in bytes
- `after`: Optimized file size in bytes
"""


class Stats:
    def __init__(self, before: int = 0, after: int = 0):
        self.before = before
        self.after = after

    def reduction(self) -> int:
        return self.before - self.after

    def reduction_pct(self) -> float:
        if self.before == 0:
            return 0.0
        return (self.reduction() / self.before) * 100.0


"""
Configs loaded from blair.toml (or .blair.config).
For libdeflater it supports 1-12.
Zopfli supports 1-255 (1-10 is more than enough).
"""


class Config:
    def __init__(self):
        # All suggested defaults here, if no blair.toml is found we will use these options.
        self.level = 6
        self.strip_metadata = True
        self.optimize_alpha = True
        self.fast_eval = False
        self.max_workers = "auto"
        self.deflater = "libdeflate"
        self.deflate_level = 12
        self.filters = ["none", "sub", "up", "average", "paeth", "minsum", "bigrams"]
        self.show_tqdm = True
        self.verbose = False


def load_config(config_path: str = "blair.toml") -> Config:
    """Load configs from blair.toml or return the defaults if missing."""
    cfg = Config()
    path = Path(config_path)

    if path.exists():
        parser = configparser.ConfigParser()
        parser.read(path)

        if "compression" in parser:
            c = parser["compression"]
            cfg.level = c.getint("level", cfg.level)
            cfg.strip_metadata = c.getboolean("strip_metadata", cfg.strip_metadata)
            cfg.optimize_alpha = c.getboolean("optimize_alpha", cfg.optimize_alpha)
            cfg.fast_eval = c.getboolean("fast_eval", cfg.fast_eval)
            cfg.max_workers = c.get("max_workers", cfg.max_workers)

        if "advanced" in parser:
            a = parser["advanced"]
            cfg.deflater = a.get("deflate_method", cfg.deflater)
            cfg.deflate_level = a.getint("deflate_level", cfg.deflate_level)
            filters_str = a.get("custom_filters", "")
            if filters_str:
                cfg.filters = [f.strip().lower() for f in filters_str.split(",")]

        print(f"ℹ️  Loaded config from {config_path}")
    return cfg


def parse_filter(name: str):
    """Parse string into oxipng RowFilter."""
    mapping = {
        "none": oxipng.RowFilter.NoOp,
        "sub": oxipng.RowFilter.Sub,
        "up": oxipng.RowFilter.Up,
        "average": oxipng.RowFilter.Average,
        "paeth": oxipng.RowFilter.Paeth,
        "minsum": oxipng.RowFilter.MinSum,
        "bigrams": oxipng.RowFilter.Bigrams,
    }
    return mapping.get(name.lower(), oxipng.RowFilter.NoOp)


def optimize_single_png(png_path: str, cfg: Config) -> Stats:
    """Optimize a single PNG file. Logic mirrors the Rust core crate."""
    original_size = os.path.getsize(png_path)

    try:
        filters = [parse_filter(f) for f in cfg.filters]

        # Build all Deflaters (the libdeflater is faster, but zopfli is better in longer iterations but super slow).
        if cfg.deflater.lower() == "zopfli":
            deflate = oxipng.Deflaters.zopfli(cfg.deflate_level)
        else:
            deflate = oxipng.Deflaters.libdeflater(cfg.deflate_level)

        oxipng.optimize(
            png_path,
            level=cfg.level,
            filter=filters,
            deflate=deflate,
            optimize_alpha=cfg.optimize_alpha,
            strip=(
                oxipng.StripChunks.safe()
                if cfg.strip_metadata
                else oxipng.StripChunks.none()
            ),
            fast_evaluation=cfg.fast_eval,
            fix_errors=True,
        )

        new_size = os.path.getsize(png_path)
        if cfg.verbose:
            print(f"✅ Optimized: {os.path.basename(png_path)}")

        return Stats(original_size, new_size)
    except Exception as e:
        if cfg.verbose:
            print(f"❌ Failed {png_path}: {e}")
        return Stats(original_size, original_size)


def run_batch(directory: str):
    """Main execution loop. Uses ThreadPoolExecutor to mimic Rayon/Parallel iterators."""
    cfg = load_config()

    # Find all .png img files in the directory.
    files = glob.glob(os.path.join(directory, "*.png"))
    if not files:
        print(f"⚠️  No .png files found in {directory}")
        return
    if not OXIPNG_AVAILABLE:
        print("❌ CRITICAL: oxipng not installed. Run 'pip install oxipng'.")
        return

    # Auto threading setups and workers based on your cpu cores.
    cpu_count = os.cpu_count() or 1
    workers = cpu_count if cfg.max_workers == "auto" else int(cfg.max_workers)

    print(f"🔍 Found {len(files)} PNGs")
    print(f"⚙️  Optimizing with {workers} workers (Level {cfg.level})...")

    start_time = time.time()
    total_stats = Stats()

    results = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=workers) as executor:
        future_to_file = {
            executor.submit(optimize_single_png, f, cfg): f for f in files
        }

        # Use tqdm for progress if it is enabled.
        if cfg.show_tqdm:
            with tqdm(
                total=len(files), desc="🖼️  Compressing", unit="file", ncols=100
            ) as pbar:
                for future in concurrent.futures.as_completed(future_to_file):
                    stat = future.result()
                    total_stats.before += stat.before
                    total_stats.after += stat.after
                    savings_mb = (total_stats.reduction()) / (1024 * 1024)
                    pbar.set_postfix_str(f"💾 {savings_mb:.2f}MB saved")
                    pbar.update(1)
        else:
            for future in concurrent.futures.as_completed(future_to_file):
                stat = future.result()
                total_stats.before += stat.before
                total_stats.after += stat.after

    # Summary of all collected stats here.
    duration = time.time() - start_time
    print("\n" + "=" * 50)
    print(f"🎉 OPTIMIZATION COMPLETED in {duration:.1f}s")
    print(f"📊 Files: {len(files)}")
    print(
        f"💾 Total Savings: {total_stats.reduction_pct():.1f}% ({total_stats.reduction()/(1024*1024):.2f} MB)"
    )
    print(
        f"📦 Final Size: {total_stats.after/(1024*1024):.2f} MB (was {total_stats.before/(1024*1024):.2f} MB)"
    )
    print(f"⚡ Speed: {len(files)/duration:.1f} files/sec")
    print("=" * 50)


if __name__ == "__main__":
    # Directory is hardcoded and also not using pathlib for now.
    # If you plan to use this standalone python script, change this directory.
    TARGET_DIR = r"D:\blair\blairBlender\cards"
    run_batch(TARGET_DIR)
