<p align="center">A simple, fast, lossless PNG optimizer for Blair</p>

<img align="center" src="https://cdn.jsdelivr.net/gh/Sethispr/blair-top.gg/assets/blairshowcasenowgoplayblairorelse.webp" alt="Blair cards showcase"></img>

---

## Blair Studio v0.5.0 (Website)

Cardmakers can also use [Blair Studio](https://blairstudio.pages.dev/) to automatically compress PNGs when submitting cards and all directly in your browser.

Blair Studio uses the same oxipng optimization (level 6) when you frame and submit cards to our Discord #content channel, no rust installation is needed.

## Installation (if using the cli)

You can get the pre built binary `.exe` from [GitHub Releases](https://github.com/sethispr/blairpng/releases) and run it directly, no Rust installation required.

### From Cargo

```bash
cargo install blairpng
```

### Build from source (Windows only)

[Install Rust](https://rustup.rs/) and [Git](https://git-scm.com/install/windows) first, then:

```bash
git clone https://github.com/sethispr/blairpng.git
cd blairpng
cargo build --release
cargo install --path .
```

---

## Usage

Optimize PNG's in a directory:

```bash
blairpng [OPTIONS] [DIRECTORY]
```

If no directory is specified, blairpng uses the current directory.

### Examples

**Basic usage** - optimize current folder with defaults:
```bash
blairpng
```

**Optimize specific folder:**
```bash
blairpng ./cards
```

**Verbose output** - see per-file savings:
```bash
blairpng --verbose
```

**Custom config:**
```bash
blairpng --config settings.toml ./cards
```

**Generate example config:**
```bash
blairpng --init
```

**See all options:**
```bash
blairpng --help
```

---

## Options

- `[DIRECTORY]` - Path to PNG files (default: current directory)
- `-l, --level <0-6>` - Compression level, higher = better but slower (default: 6)
- `-j, --threads <N>` - Number of threads (default: all cores)
- `-q, --quiet` - Hide progress bar
- `-v, --verbose` - Show compression stats per file
- `-c, --config <PATH>` - Custom config file path
- `--init` - Generate `blair.toml` example config

---

## Configuration

blairpng works great with defaults, but you can customize settings via `blair.toml`.

Generate example config:
```bash
blairpng --init
```

Example `blair.toml`:
```toml
level = 6                  # Compression level (0-6)
strip_metadata = true      # Remove unnecessary metadata
optimize_alpha = true      # Optimize transparent pixels
fast_eval = false          # Faster but less thorough

# Row filters to try (more = slower but better compression)
filters = ["none", "sub", "up", "average", "paeth", "minsum", "bigrams"]

# Compression backend
deflater = "libdeflate"    # "libdeflate" (fast) or "zopfli" (slower, better)
deflate_level = 12         # 1-12 for libdeflate, 1-255 for zopfli
```

### Config Options

| Option | Description | Default |
|--------|-------------|---------|
| `level` | Optimization preset (0-6) | `6` |
| `strip_metadata` | Remove non-essential chunks | `true` |
| `optimize_alpha` | Optimize transparency | `true` |
| `fast_eval` | Faster filter evaluation | `false` |
| `filters` | Row filters to test | See above |
| `deflater` | Compression backend | `"libdeflate"` |
| `deflate_level` | Compression effort | `12` |

---

## Performance

Typical results on Blair cards (725x1040 resolution):
- **Before:** ~1.03 MB average
- **After:** ~718 KB average
- **Savings:** ~20-30% reduction
- **Speed:** Compress ~100 cards in under 3 minutes (single cpu core, compress level=6)

Uses [oxipng](https://github.com/shssoichiro/oxipng), [libdeflate](https://github.com/ebiggers/libdeflate), and [zopfli](https://github.com/google/zopfli).

---
