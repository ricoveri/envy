# envy

A simple environment variable exporter that loads variables from a YAML configuration file and outputs them as shell export statements.

## Overview

`envy` reads a YAML file containing environment variable definitions and outputs them as shell-compatible export statements. This makes it easy to manage and load environment variables across different shell sessions.

## Features

- 📝 **YAML Configuration**: Define environment variables in a simple YAML file
- 🔗 **Array Support**: Automatically converts arrays to colon-separated strings (perfect for PATH-like variables)
- 🏠 **Home Directory Integration**: Reads configuration from `~/.envyrc.yaml` by default
- 🗂️ **Drop-in Directory**: Split config across files in `~/.envy.d/`, read in alphanumeric order, with later files overriding earlier ones
- 📂 **Custom Config Path**: Specify a custom configuration file or directory via command-line argument
- 🚀 **Shell Integration Ready**: Outputs standard `export` statements for easy sourcing

## Installation

### From Source

```bash
git clone https://github.com/ricoveri/envy.git
cd envy
cargo build --release
```

The binary will be available at `target/release/envy`.

## Usage

1. Create a `.envyrc.yaml` file in your home directory:

```yaml
# Simple string values
MY_VAR: "hello"
DATABASE_URL: "postgres://localhost:5432/mydb"

# Array values (converted to colon-separated strings)
PATH:
  - "/usr/local/bin"
  - "/usr/bin"
  - "/bin"

CUSTOM_PATH:
  - "/opt/myapp/bin"
  - "/opt/tools/bin"
```

2. Run `envy` to generate export statements:

```bash
# Use default config file (~/.envyrc.yaml)
envy

# Or specify a custom config file
envy path/to/config.yaml
```

Output:

```bash
export MY_VAR="hello"
export DATABASE_URL="postgres://localhost:5432/mydb"
export PATH="/usr/local/bin:/usr/bin:/bin"
export CUSTOM_PATH="/opt/myapp/bin:/opt/tools/bin"
```

3. Source the output in your shell:

**Bash/Zsh:**

```bash
eval "$(envy)"
```

## Configuration File Format

The configuration file uses YAML format and supports two types of values:

### String Values

```yaml
VARIABLE_NAME: "value"
```

### Array Values (for PATH-like variables)

```yaml
VARIABLE_NAME:
  - "value1"
  - "value2"
  - "value3"
```

Arrays are automatically converted to colon-separated strings (e.g., `value1:value2:value3`).

## Multiple Config Files

With no argument, envy reads `~/.envyrc.yaml` (if present), then every `*.yaml` / `*.yml` file in `~/.envy.d/` (if present). At least one of them must exist.

- Files in a directory are read in plain byte-wise order of their names, so `10-x.yaml` comes **before** `2-y.yaml`. Use zero-padded prefixes (`00-`, `10-`, `20-`) to control order.
- If a variable is defined in more than one file, the file read **last** wins. Arrays replace earlier values; they are not appended.
- Hidden files, subdirectories and files with other extensions are ignored.
- An empty file is allowed. A file that can't be read or parsed aborts the run with an error naming it.

```
~/.envy.d/
├── 10-base.yaml       # LOG_LEVEL: "info"
└── 20-local.yaml      # LOG_LEVEL: "debug"   <- wins
```

## Command-Line Usage

```bash
envy [CONFIG]
```

### Arguments

- `CONFIG` (optional): Path to a YAML configuration file, or to a directory of YAML files (read as described above). If not provided, defaults to `~/.envyrc.yaml` followed by `~/.envy.d/`

### Options

- `--help`: Display help information
- `--version`: Display version information

### Examples

```bash
# Use default config
envy

# Use a directory of config files
envy ~/.envy.d

# Use custom config file
envy ./my-env.yaml
envy /etc/myapp/env.yaml

# Get help
envy --help

# Check version
envy --version
```

## Integration with Shell

Add the following to your shell's configuration file:

**Bash (`~/.bashrc` or `~/.bash_profile`):**

```bash
if command -v envy &> /dev/null; then
    eval "$(envy)"
fi
```

**Zsh (`~/.zshrc`):**

```zsh
if command -v envy &> /dev/null; then
    eval "$(envy)"
fi
```

## Development

### Prerequisites

- Rust 2024 edition or later
- Cargo

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

### Testing

```bash
cargo test
```

## Dependencies

- [clap](https://crates.io/crates/clap) - Command-line argument parser
- [dirs](https://crates.io/crates/dirs) - Platform-specific standard locations
- [yaml-rust](https://crates.io/crates/yaml-rust) - YAML parser

## License

**Copyright (c) 2025 Alejandro Ricoveri**

This software is provided 'as-is', without any express or implied
warranty. In no event will the authors be held liable for any damages
arising from the use of this software.

Permission is granted to anyone to use this software for any purpose,
including commercial applications, and to alter it and redistribute it
freely, subject to the following restrictions:

1. The origin of this software must not be misrepresented; you must not
   claim that you wrote the original software. If you use this software
   in a product, an acknowledgment in the product documentation would be
   appreciated but is not required.
2. Altered source versions must be plainly marked as such, and must not be
   misrepresented as being the original software.
3. This notice may not be removed or altered from any source distribution.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
