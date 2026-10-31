# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`envy`: small Rust CLI (edition 2024). Reads a YAML file of env vars, prints shell `export NAME="value"` lines to stdout. Meant to be used as `eval "$(envy)"`. Whole program lives in `src/main.rs` (no modules; unit tests in the same file).

## Commands

```bash
cargo build                         # debug build
cargo build --release               # binary at target/release/envy
cargo run -- files/test.yaml        # run against sample config
cargo run                           # uses ~/.envyrc.yaml
cargo run -- files/envy.d           # sample drop-in dir (20-override wins)
cargo test
cargo test <name>                   # single test, e.g. cargo test later_file_overrides_earlier
cargo clippy / cargo fmt            # lint / format
```

`.vscode/launch.json` has a debug config for the `envy` binary.

## Behavior to know before editing

- Config sources (`config_sources`): optional positional arg, which may be a file or a directory. A directory contributes its `*.yaml`/`*.yml` files (no hidden files or subdirs) sorted byte-wise by name (`10-x` before `2-y`). With no arg: `~/.envyrc.yaml` (`CONF_FILE_NAME`) first, then `~/.envy.d/` (`CONF_DIR_NAME`); either may be missing but not both.
- Merge: files are read in order and later ones override earlier ones per key (arrays replace, never append). An overridden key moves to the end of the output. Per file, only the **first** YAML document is used, its root must be a mapping, and an empty file is fine. Any bad file aborts the run.
- Values: string → printed as-is; array → items joined with `:` (PATH-style). Non-string array items and non-string scalars (ints, bools, nested maps) are **silently skipped**, not errored.
- Output is plain `export NAME="value"` via the `print_value!` macro. No escaping of `"`, `$`, or backticks, so values are shell-expanded when eval'd (e.g. `$HOME` in `files/test.yaml` is intentional/expected to expand).
- Output order follows YAML mapping order.
- Errors propagate from `main` as `Box<dyn Error>` (missing file, bad YAML, no home dir).
- Version: `--version` comes from `Cargo.toml` via `#[command(version)]`. Bump it there only.
- Uses `yaml-rust` 0.4 (unmaintained crate), not `serde_yaml`.
