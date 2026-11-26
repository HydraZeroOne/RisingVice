# Renaming: `victory` → `Rissing`

The project was renamed from **victory** to **Rissing** on 2026-11-26.
Everything that referred to the old name now uses the new one. The Rust
sources themselves are unchanged: same files, same modules, same logic.

## Crates (folder and package name)

| Before                   | Now                      |
|--------------------------|--------------------------|
| `crates/victory-core`    | `crates/Rissing-core`    |
| `crates/victory-formats` | `crates/Rissing-formats` |
| `crates/victory-game`    | `crates/Rissing-game`    |
| `crates/victory-gpu`     | `crates/Rissing-gpu`     |
| `crates/victory-hle`     | `crates/Rissing-hle`     |
| `crates/victory-loader`  | `crates/Rissing-loader`  |
| `crates/victory-pe`      | `crates/Rissing-pe`      |
| `crates/victory-pipeline`| `crates/Rissing-pipeline`|
| `crates/victory-shader`  | `crates/Rissing-shader`  |

The package `name = "…"` in each `Cargo.toml` changed with the folder. The
workspace `[workspace.dependencies]` paths and keys changed with them.

The crate identifiers inside Rust code also changed, because a hyphen
cannot appear in an identifier:

| Before           | Now              |
|------------------|------------------|
| `victory_core`   | `Rissing_core`   |
| `victory_gpu`    | `Rissing_gpu`    |
| … etc.           | `Rissing_…`      |

## Environment variables

All `VICTORY_*` variables are now `RISSING_*`:

| Before                 | Now                  |
|------------------------|----------------------|
| `VICTORY_CONFIG`       | `RISSING_CONFIG`     |
| `VICTORY_GAME_DIR`     | `RISSING_GAME_DIR`   |
| `VICTORY_CACHE_DIR`    | `RISSING_CACHE_DIR`  |
| `VICTORY_DATA_DIR`     | `RISSING_DATA_DIR`   |
| `VICTORY_TEST_TITLES`  | `RISSING_TEST_TITLES`|
| `VICTORY_LOG`          | `RISSING_LOG`        |
| `VICTORY_BCPACK_TABLES`| `RISSING_BCPACK_TABLES` |
| `VICTORY_BCPACK_CASES` | `RISSING_BCPACK_CASES`  |
| `VICTORY_XMA_STREAMS`  | `RISSING_XMA_STREAMS`   |
| `VICTORY_XMA_REFERENCE`| `RISSING_XMA_REFERENCE` |

## Files and directories

| Before                  | Now                  |
|-------------------------|----------------------|
| `victory.example.toml`  | `rissing.example.toml` |
| `./victory.toml` (default config file) | `./rissing.toml` |
| `.victory-root` (repo marker) | `.Rissing-root`  |
| `%LOCALAPPDATA%/victory/` (default cache/data root) | `%LOCALAPPDATA%/rissing/` |

## What did not change

- The Rust source files keep their names (`config.rs`, `texture.rs`,
  `bcpack.rs`, `xma.rs`, …) and their module paths.
- The repository folder is still `D:\RisingVice`.
- The C/C++ port was abandoned and its files removed; the project is Rust only.

## Verification

```
cargo check --workspace
```

passed after the rename (warnings only). A backup of the tree from before
the rename is at `D:\RisingVice.backup-2026-11-26`.
