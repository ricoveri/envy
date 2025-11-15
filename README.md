# envy

A simple environment variable exporter that loads variables from a YAML configuration file and outputs them as shell export statements.

## Overview

`envy` reads a YAML file containing environment variable definitions and outputs them as shell-compatible export statements. This makes it easy to manage and load environment variables across different shell sessions.

## Features

- 📝 **YAML Configuration**: Define environment variables in a simple YAML file
- 🔗 **Array Support**: Automatically converts arrays to colon-separated strings (perfect for PATH-like variables)
- 🏠 **Home Directory Integration**: Reads configuration from `~/.envyrc.yaml` by default
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
envy
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

- [dirs](https://crates.io/crates/dirs) - Platform-specific standard locations
- [yaml-rust](https://crates.io/crates/yaml-rust) - YAML parser

## Roadmap

See [TODO](TODO) for planned features, including:

- CLI argument parsing with `clap`
- Custom configuration file path support

## License

[License information to be added]

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
