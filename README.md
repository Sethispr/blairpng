<img align="center" src="https://cdn.jsdelivr.net/gh/Sethispr/blair-top.gg/assets/blairshowcasenowgoplayblairorelse.webp" alt="Blair cards showcase banner"></img>

<div align=center>Blair's card lossless compressor cli built in idiomatic Rust, fast multithreaded with compile time ownership guarantees.</div>

## Installing

### From Cargo

```bash
cargo install blairpng (soon)
```

### Build from source (make sure to use --release)

```bash
git clone https://github.com/sethispr/blairpng.git
cd blairpng
cargo build --release
cargo run --release
```

## Usage

Run blairpng on a directory containing .png files:

```bash
blairpng [OPTIONS (optional)] [DIRECTORY PATH]
```

If no directory is put, it uses the current directory.

### Some examples

- Optimize all png files in the current folder with default settings:

  ```bash
  blairpng
  ```

- Highest compression without deflating stuff etc (level 6 is default too and recommended)

  ```bash
  blairpng -l 6
  ```

- Verbose output to see savings for each png

  ```bash
  blairpng --verbose
  ```

- Use a custom config file

  ```bash
  blairpng --config wow.toml /path/to/cards
  ```

- Generate an example config file

  ```bash
  blairpng --init
  ```

- Help menu for these options below

  ```bash
  blairpng --help
  ```

### Options

- `directory`: Directory with `.png` files (default: current directory)
- `-l, --level <0-6>`: Optimization preset level (higher = better compression but slower, default: 6)
- `-j, --threads <N>`: Number of threads (default: all available cores)
- `--quiet`: Hide the progress bar
- `--verbose`: Print detailed logs for every png
- `--config <path>`: Path to custom `blair.toml` config
- `--init`: Generate an example `blair.toml` in the current directory

## Configs (blair.toml)

blairpng is already filled with good defaults (you can just run `blairpng [card folder path]`, but you can change them with the optional `blair.toml` in the working directory or set it via `--config [path]`.

Example config (`blairpng --init` will generate you this)

```toml
# Blair Example Config
# Best lossless compression but fastest possible settings by default
level = 6
strip_metadata = true
optimize_alpha = true
fast_eval = false
filters = ["none", "sub", "up", "average", "paeth", "minsum", "bigrams"]
deflater = "libdeflater"
deflate_level = 12
```

Options:

- `level`: Default comoression level (higher = more compressed but slower) (0-6)
- `strip_metadata`: Remove useless metadata (true = safe strip)
- `optimize_alpha`: Optimize the transparent pixels for better compression
- `fast_eval`: Faster but less thorough filter evaluation
- `filters`: List of row filters to try (strings matching oxipng's RowFilter)
- `deflater`: `"libdeflater"` (fast) or `"zopfli"` (better compression but much slower)
- `deflate_level`: Compression effort (1-12 for libdeflater, 1-255 for zopfli)

Uses [`oxipng`](https://github.com/oxipng/oxipng), [`zopfli`](https://github.com/google/zopfli) and [`libdeflate`](https://github.com/ebiggers/libdeflate)
