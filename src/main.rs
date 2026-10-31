//! envy: Environment Variable Exporter
//!
//! Loads environment variables from YAML configuration and prints each one as a shell export
//! statement. Scalar string values are printed as-is; arrays are rendered as colon-separated
//! strings (e.g., for PATH-like variables).
//!
//! Configuration sources:
//!   - No argument: `~/.envyrc.yaml` (if present), then every `*.yaml` / `*.yml` file in
//!     `~/.envy.d/` (if present), in lexicographic file name order.
//!   - A file argument: that file only.
//!   - A directory argument: every `*.yaml` / `*.yml` file in it, in lexicographic file name order.
//!
//! When several files define the same variable, the one read last wins.

use clap::Parser; // For command-line argument parsing
use std::error::Error; // Trait for error handling in Rust
use std::fs;
use std::path::{Path, PathBuf};
use yaml_rust::YamlLoader;
use yaml_rust::yaml::Hash; // Hash type used for YAML mapping in yaml-rust

// Configuration file name in the user's home directory
const CONF_FILE_NAME: &str = ".envyrc.yaml";

// Configuration directory name in the user's home directory
const CONF_DIR_NAME: &str = ".envy.d";

#[derive(Parser)]
#[command(name = "envy")]
#[command(version)]
#[command(about = "Environment Variable Exporter", long_about = None)]
/// Command-line interface arguments for the application.
struct Cli {
    /// Path to a YAML configuration file, or to a directory of YAML files (read in
    /// lexicographic order; later files override earlier ones). Defaults to
    /// `~/.envyrc.yaml` followed by `~/.envy.d/`.
    config: Option<String>,
}

/// Loads a single YAML file and returns its top-level mapping.
///
/// An empty file yields an empty mapping. Any other non-mapping root is an error.
/// Errors are prefixed with the offending file path.
fn load_file(path: &Path) -> Result<Hash, Box<dyn Error>> {
    let data = fs::read_to_string(path).map_err(|e| format!("{}: {}", path.display(), e))?;
    let docs = YamlLoader::load_from_str(&data).map_err(|e| format!("{}: {}", path.display(), e))?;

    // only the first YAML document is used
    match docs.into_iter().next() {
        None => Ok(Hash::new()),
        Some(doc) => doc
            .into_hash()
            .ok_or_else(|| format!("invalid configuration file: {}", path.display()).into()),
    }
}

/// Lists the `*.yaml` / `*.yml` regular files in `dir`, sorted by file name (byte-wise).
///
/// Hidden files (leading `.`), subdirectories and other extensions are skipped.
fn dir_files(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| format!("{}: {}", dir.display(), e))? {
        let path = entry?.path();
        let hidden = path
            .file_name()
            .and_then(|n| n.to_str())
            .is_none_or(|n| n.starts_with('.'));
        let yaml = matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("yaml" | "yml")
        );
        if !hidden && yaml && path.is_file() {
            files.push(path);
        }
    }
    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    Ok(files)
}

/// Default sources: `<home>/.envyrc.yaml` (if it exists), then `<home>/.envy.d/*` (if it exists).
fn default_sources(home: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut sources = Vec::new();

    let rc = home.join(CONF_FILE_NAME);
    if rc.is_file() {
        sources.push(rc);
    }

    let dir = home.join(CONF_DIR_NAME);
    if dir.is_dir() {
        sources.extend(dir_files(&dir)?);
    }

    if sources.is_empty() {
        return Err(format!(
            "no config found (looked for {} and {}/)",
            home.join(CONF_FILE_NAME).display(),
            home.join(CONF_DIR_NAME).display()
        )
        .into());
    }
    Ok(sources)
}

/// Resolves the ordered list of config files to read.
fn config_sources(arg: Option<String>) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    match arg {
        Some(p) => {
            let path = PathBuf::from(p);
            if path.is_dir() {
                dir_files(&path)
            } else {
                // a missing file is reported when it is opened
                Ok(vec![path])
            }
        }
        None => {
            let home = dirs::home_dir()
                .ok_or("couldn't determine home directory from environment")?;
            default_sources(&home)
        }
    }
}

/// Loads every source in order and merges them; later files override earlier ones.
fn get_env_vars(config: Option<String>) -> Result<Hash, Box<dyn Error>> {
    let mut merged = Hash::new();
    for path in config_sources(config)? {
        for (k, v) in load_file(&path)? {
            merged.insert(k, v);
        }
    }
    Ok(merged)
}

/// Renders a shell-style export statement for a variable and its value.
fn export_line(name: &str, value: &str) -> String {
    format!("export {}=\"{}\"", name, value)
}

/// Renders each variable as an export statement.
///
/// Strings are printed as-is, arrays of strings are joined with `:`. Other value types
/// (and non-string array items) are skipped.
fn render(vars: &Hash) -> Vec<String> {
    let mut lines = Vec::new();
    for (k, v) in vars {
        let Some(name) = k.as_str() else { continue };
        if let Some(items) = v.as_vec() {
            let joined = items
                .iter()
                .filter_map(|i| i.as_str())
                .collect::<Vec<_>>()
                .join(":");
            lines.push(export_line(name, &joined));
        } else if let Some(value) = v.as_str() {
            lines.push(export_line(name, value));
        }
    }
    lines
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let env_vars = get_env_vars(cli.config)?;

    for line in render(&env_vars) {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn write(dir: &Path, name: &str, content: &str) {
        fs::write(dir.join(name), content).unwrap();
    }

    fn run_dir(dir: &Path) -> Vec<String> {
        render(&get_env_vars(Some(dir.to_str().unwrap().to_string())).unwrap())
    }

    #[test]
    fn later_file_overrides_earlier() {
        let d = tempdir().unwrap();
        write(d.path(), "10-a.yaml", "X: a\nONLY_A: 1x\n");
        write(d.path(), "20-b.yaml", "X: b\n");
        let out = run_dir(d.path());
        // an overridden key moves to the end (LinkedHashMap::insert refreshes position)
        assert_eq!(out, vec![r#"export ONLY_A="1x""#, r#"export X="b""#]);
    }

    #[test]
    fn order_is_bytewise_not_natural() {
        let d = tempdir().unwrap();
        write(d.path(), "2-a.yaml", "X: from2\n");
        write(d.path(), "10-b.yaml", "X: from10\n");
        // "10-b" sorts before "2-a", so "2-a" is read last and wins
        assert_eq!(run_dir(d.path()), vec![r#"export X="from2""#]);
    }

    #[test]
    fn dir_files_filters_entries() {
        let d = tempdir().unwrap();
        write(d.path(), "a.yaml", "A: 1\n");
        write(d.path(), "b.yml", "B: 1\n");
        write(d.path(), ".hidden.yaml", "H: 1\n");
        write(d.path(), "notes.txt", "nope\n");
        fs::create_dir(d.path().join("sub.yaml")).unwrap();

        let names: Vec<_> = dir_files(d.path())
            .unwrap()
            .iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
            .collect();
        assert_eq!(names, vec!["a.yaml", "b.yml"]);
    }

    #[test]
    fn empty_file_is_empty_mapping() {
        let d = tempdir().unwrap();
        write(d.path(), "empty.yaml", "");
        assert!(load_file(&d.path().join("empty.yaml")).unwrap().is_empty());
    }

    #[test]
    fn array_override_replaces() {
        let d = tempdir().unwrap();
        write(d.path(), "10.yaml", "P:\n  - /a\n  - /b\n");
        write(d.path(), "20.yaml", "P:\n  - /c\n");
        assert_eq!(run_dir(d.path()), vec![r#"export P="/c""#]);
    }

    #[test]
    fn arrays_join_with_colon() {
        let d = tempdir().unwrap();
        write(d.path(), "a.yaml", "P:\n  - /a\n  - /b\n");
        assert_eq!(run_dir(d.path()), vec![r#"export P="/a:/b""#]);
    }

    #[test]
    fn file_arg_returns_just_that_file() {
        let d = tempdir().unwrap();
        write(d.path(), "a.yaml", "A: 1\n");
        let p = d.path().join("a.yaml");
        let got = config_sources(Some(p.to_str().unwrap().to_string())).unwrap();
        assert_eq!(got, vec![p]);
    }

    #[test]
    fn default_sources_rc_then_dir() {
        let home = tempdir().unwrap();
        write(home.path(), CONF_FILE_NAME, "X: rc\n");
        let dir = home.path().join(CONF_DIR_NAME);
        fs::create_dir(&dir).unwrap();
        write(&dir, "10-a.yaml", "X: dir\n");

        let sources = default_sources(home.path()).unwrap();
        assert_eq!(sources, vec![home.path().join(CONF_FILE_NAME), dir.join("10-a.yaml")]);
    }

    #[test]
    fn default_sources_errors_when_nothing_found() {
        let home = tempdir().unwrap();
        let err = default_sources(home.path()).unwrap_err().to_string();
        assert!(err.contains("no config found"));
    }
}
