<p align="center">
  <img src="assets/banner.png" alt="RisingVice">
</p>

<h1 align="center">RisingVice</h1>

<p align="center">
  <img src="https://img.shields.io/badge/platform-Windows-0078D6?logo=windows&logoColor=white" alt="Platform">
  <img src="https://img.shields.io/badge/console-Xbox%20Series%20X-107C10?logo=xbox&logoColor=white" alt="Console">
  <img src="https://img.shields.io/badge/tested%20on-RTX%203070%208%20GB-76B900?logo=nvidia&logoColor=white" alt="Tested on">
  <img src="https://img.shields.io/badge/Language-Rust-DEA584?logo=rust&logoColor=white" alt="Language: Rust">
  <img src="https://img.shields.io/badge/status-playable%20%C2%B7%204--7%20fps%20%C2%B7%20peak%2012-brightgreen" alt="Status: playable, 4-7 fps, peak 12">
  <img src="https://img.shields.io/badge/target-30%20fps%20at%202560x1440-blue" alt="Target">
  <img src="https://img.shields.io/badge/progress-in%20development-orange" alt="Progress">
  <img src="https://img.shields.io/badge/license-GPL--3.0--or--later-blue" alt="License">
</p>

An Xbox Series emulator for Windows, extended to run on NVIDIA hardware.

It maps a title's own executable, binds its imports and runs it as an Xbox Series
console on the host GPU.

> [!IMPORTANT]
> **RisingVice ships no game data.** You supply your own copy of the title; nothing
> from the game lives in this repository, and the tools refuse to write inside it.
>
> **Current state:** runs and renders frames at a stable **3.9 to 6.8 fps**, with a
> **12.4 fps peak** on one run.
> **Target:** 30 fps at 2560x1440.

---

## About

RisingVice runs Xbox Series titles natively on Windows. It does not emulate the
console hardware instruction by instruction. Instead it maps the title's own
executable into memory, resolves its imports against platform modules written in
Rust, and forwards the graphics work to the host GPU through Direct3D 12.

The design has three layers:

- **Loader.** Reads the title's PE images, maps them, applies relocations and binds
  every import. If anything is missing, it says so before the title runs.
- **HLE.** Hand written versions of the system modules the title calls: kernel,
  file access, storage, networking, audio, input, and the graphics front end.
- **GPU.** Tuple and command stream translation onto the host D3D12 device,
  including memory management, resource layouts and descriptor heaps.

Everything is built from files the user supplies. No game data is stored or
distributed with this project.

## Status

| Setting | Value |
|---|---|
| Console profile | Xbox Series X |
| Machine | Ryzen 7 7700X, RTX 3070 (8 GB), 32 GB DDR5, Windows 11 |
| Runs | Yes: binds, submits, presents frames |
| Frame rate | 3.9 to 6.8 fps steady, 12.4 fps peak |
| Target | 30 fps at 2560x1440 |
| Ray-traced lighting | Not implemented |

## Frame rate

Measured on the RTX 3070. Full notes in [`docs/risingvice-notes.md`](docs/risingvice-notes.md).

| Configuration | fps | Notes |
|---|---|---:|
| Default | 3.5 to 5.3 | Right after a fresh build. Gains come from the settings below |
| `RISSING_CARD_BUFFERS_MIN_MB=1` | **3.9 to 6.8**, peak 12.4 | Best steady result. Use this |
| `+ RISSING_ASYNC_PRESENT=1` | 2.5 to 10.6 | Spiky: higher peaks, lower floor |
| `RISSING_RELAY=2` | 30 | Menus only; never reaches the world |
| `RISSING_DISPLAY_MODE=1920x1080` | 3.1 to 4.1 | No gain. The cost is not fill rate |

A world frame takes 200 to 280 ms:

- 90 to 290 ms copying back into title memory (25 to 96 MB)
- 70 to 195 ms on the GPU, often idle
- 40 to 200 ms preparing uploads and buffer copies

Memory sits at about 6.9 GB of the 7 GB usable.

## Progress

### Fixed: integer render targets

`integer_format()` and `blend_for()` in `crates/Rissing-gpu/src/pipeline/host.rs`
disable blending where D3D12 refuses it. Mesh pipeline refusals went from **17 to 1**,
and textured geometry now draws.

### Ruled out

1. CPU copy-back scan: about 1 ms per frame.
2. Scattered copy-back ranges: already merged.
3. Compute pipelines with empty bytecode: none, 2 KB to 337 KB.
4. Compute pipelines missing samplers: same counts as working ones.
5. Lower resolution: no change at 1080p.
6. `RISSING_RELAY=2` for world frames: menus only.

### Still open

| Issue | Detail |
|---|---|
| 7 compute pipelines refused | `E_INVALIDARG`, valid bytecode, no message from D3D12 |
| Ray-traced lighting | Not implemented. These are the black areas |
| 8 GB VRAM | Exhausted before 2560x1440 becomes possible |

## Build and run

```sh
cargo build --release
cp rissing.example.toml rissing.toml   # set game_dir, must be outside this repo

# bind only, no window
./target/release/Rissing.exe run '<Title>.exe'

# run
RISSING_CARD_BUFFERS_MIN_MB=1 ./target/release/Rissing.exe run '<Title>.exe' --execute
```

Toolchain is pinned in `rust-toolchain.toml`.

### Requirements

| Requirement | Minimum |
|---|---|
| OS | Windows 10 or 11, 64 bit |
| GPU | NVIDIA, Direct3D 12 capable. 8 GB VRAM supported, the target needs more |
| CPU | Any x86-64 with SSE4.2 |
| RAM | 16 GB tested at the low end, 32 GB recommended |
| Rust | 1.88 or newer, pinned by `rust-toolchain.toml` |
| Game data | Not included. You must obtain your own copy of the title |

### Command line

The `Rissing` binary has these commands:

| Command | What it does |
|---|---|
| `inspect` | Summarise one PE image: headers, sections, imports, TLS, build id |
| `census` | Aggregate the imports of every image under the given paths |
| `lib-exports` | List the `dll!export` pairs an import library describes |
| `shader-info` | Find shader containers and report their layout |
| `translate` | Run every shader through the translator and report what fails |
| `run` | Prepare, map and bind a title executable. With `--execute`, run it |
| `config` | Show the resolved configuration and check it |

### Settings

| Variable | Default | Effect |
|---|---|---|
| `RISSING_GAME_DIR` | none | Where the title lives. Required |
| `RISSING_CACHE_DIR` | `%LOCALAPPDATA%\Rissing\cache` | Derived data |
| `RISSING_DATA_DIR` | `%LOCALAPPDATA%\Rissing\data` | Saved data from the title |
| `RISSING_CARD_BUFFERS_MIN_MB` | unset | `1` is the best steady frame rate found |
| `RISSING_ASYNC_PRESENT` | unset | `1` raises peaks and lowers the floor |
| `RISSING_DISPLAY_MODE` | title default | `1920x1080` to override resolution |
| `RISSING_LOG` | none | Log filter, for example `Rissing_gpu=trace,info` |

## Layout

| Path | Contents |
|---|---|
| `crates/Rissing-core` | Config, paths, asset policy, logging |
| `crates/Rissing-pe` | PE/COFF parsing |
| `crates/Rissing-loader` | Title mapping and import binding |
| `crates/Rissing-hle` | Platform modules the title calls |
| `crates/Rissing-gpu` | D3D12 streams and GPU memory |
| `crates/Rissing-shader` | Shader translation |
| `crates/Rissing-formats` | Texture and audio formats |
| `crates/Rissing-pipeline` | Draw pipelines |
| `crates/Rissing-game` | Title-side helpers |
| `apps/Rissing` | CLI front end |
| `docs/` | Architecture, asset policy, measurements |

See [`docs/architecture.md`](docs/architecture.md) and [`docs/asset-policy.md`](docs/asset-policy.md).

---

## Credits

This project uses code from these repositories:

- AMD addrlib (as shipped in Mesa), for texture layouts
- FFmpeg, for WMA Pro audio decoding
