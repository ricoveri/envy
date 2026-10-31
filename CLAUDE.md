# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`envy`: small Rust CLI (edition 2024). Reads a YAML file of env vars, prints shell `export NAME="value"` lines to stdout. Meant to be used as `eval "$(envy)"`. Whole program lives in `src/main.rs` (no modules, no tests yet).

## Commands

```bash
cargo build                         # debug build
cargo build --release               # binary at target/release/envy
cargo run -- files/test.yaml        # run against sample config
cargo run                           # uses ~/.envyrc.yaml
cargo test                          # no tests exist yet
cargo test <name>                   # run a single test once added
cargo clippy / cargo fmt            # lint / format
```

`.vscode/launch.json` has a debug config for the `envy` binary.

## Behavior to know before editing

- Config path: optional positional arg, else `~/.envyrc.yaml` (`CONF_FILE_NAME`). Only the **first** YAML document is used and its root must be a mapping.
- Values: string → printed as-is; array → items joined with `:` (PATH-style). Non-string array items and non-string scalars (ints, bools, nested maps) are **silently skipped**, not errored.
- Output is plain `export NAME="value"` via the `print_value!` macro. No escaping of `"`, `$`, or backticks, so values are shell-expanded when eval'd (e.g. `$HOME` in `files/test.yaml` is intentional/expected to expand).
- Output order follows YAML mapping order.
- Errors propagate from `main` as `Box<dyn Error>` (missing file, bad YAML, no home dir).
- Version mismatch: `Cargo.toml` says `0.1.0`, but the clap `#[command(version = "1.0.0")]` in `main.rs` is what `--version` prints.
- Uses `yaml-rust` 0.4 (unmaintained crate), not `serde_yaml`.
