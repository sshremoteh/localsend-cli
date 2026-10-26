//! `config.toml`: optional user settings. A commented template is written
//! on the first run; command-line flags and environment variables take
//! precedence.

use crate::Args;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

#[derive(Default, Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub alias: Option<String>,
    pub port: Option<u16>,
    pub destination: Option<PathBuf>,
    pub auto_accept: Option<String>,
    pub organize_files: Option<String>,
    pub multicast: Option<bool>,
    pub save_history: Option<bool>,
}

const CONFIG_TEMPLATE: &str = "\
# LocalSend CLI configuration. Command-line flags and environment variables
# (LOCALSEND_ALIAS, LOCALSEND_PORT, LOCALSEND_DESTINATION) take precedence.

# Device name shown to other devices (default: the hostname).
#alias = \"My Device\"

# Port of the HTTP server.
#port = 53317

# Directory where received files are saved (default: the system Downloads folder).
#destination = \"~/Downloads\"

# Auto-accept incoming transfers: \"none\" (ask), \"paired\" (paired devices only), \"always\"
#auto_accept = \"paired\"

# Organize received files: \"none\", \"sender\" (subfolder by sender), \"date\" (subfolder by date)
#organize_files = \"none\"

# Enable multicast discovery
#multicast = true

# Save transfer history
#save_history = true
";

/// Default port of the HTTP server.
const DEFAULT_PORT: u16 = 53317;

/// The settings actually used by the app.
#[derive(Clone, Debug)]
pub struct ResolvedConfig {
    pub alias: String,
    pub port: u16,
    pub destination: PathBuf,
    pub auto_accept: String,
    pub organize_files: String,
    pub multicast: bool,
    pub save_history: bool,
}

/// Reads `config.toml` and resolves every setting, in order of precedence:
/// command-line flag > environment variable > config file > default
///
/// The environment variables are read by clap.
pub fn load_with_fallback(dir: &Path, args: &Args) -> anyhow::Result<ResolvedConfig> {
    let config = load(dir)?;
    Ok(ResolvedConfig {
        alias: args
            .alias
            .clone()
            .or(config.alias)
            .unwrap_or_else(default_alias),
        port: args.port.or(config.port).unwrap_or(DEFAULT_PORT),
        destination: match args.destination.clone().or(config.destination) {
            Some(destination) => expand_tilde(destination),
            None => dirs::download_dir().unwrap_or_else(|| PathBuf::from(".")),
        },
        auto_accept: config.auto_accept.unwrap_or_else(|| "paired".to_string()),
        organize_files: config.organize_files.unwrap_or_else(|| "none".to_string()),
        multicast: config.multicast.unwrap_or(true),
        save_history: config.save_history.unwrap_or(true),
    })
}

/// Reads `config.toml`, writing a commented template when it is missing.
fn load(dir: &Path) -> anyhow::Result<Config> {
    let path = dir.join("config.toml");
    match std::fs::read_to_string(&path) {
        Ok(text) => toml::from_str(&text)
            .with_context(|| format!("Invalid config file: {}", path.display())),
        Err(err) if err.kind() == ErrorKind::NotFound => {
            // Best effort: a missing template is no reason to refuse to run.
            let _ = std::fs::write(&path, CONFIG_TEMPLATE);
            Ok(Config::default())
        }
        Err(err) => Err(err).context(format!("Could not read {}", path.display())),
    }
}

/// Saves the updated configuration back to `config.toml`.
pub fn save(dir: &Path, resolved: &ResolvedConfig) -> anyhow::Result<()> {
    let path = dir.join("config.toml");
    let config = Config {
        alias: Some(resolved.alias.clone()),
        port: Some(resolved.port),
        destination: Some(resolved.destination.clone()),
        auto_accept: Some(resolved.auto_accept.clone()),
        organize_files: Some(resolved.organize_files.clone()),
        multicast: Some(resolved.multicast),
        save_history: Some(resolved.save_history),
    };
    let content = toml::to_string_pretty(&config)
        .context("Failed to serialize configuration")?;
    std::fs::write(&path, content)
        .with_context(|| format!("Failed to write config to {}", path.display()))?;
    Ok(())
}

fn default_alias() -> String {
    gethostname::gethostname()
        .to_string_lossy()
        .trim_end_matches(".local")
        .to_string()
}

/// Replaces a leading `~/` with the home directory; config values are not
/// expanded by the shell.
fn expand_tilde(path: PathBuf) -> PathBuf {
    match (path.strip_prefix("~"), dirs::home_dir()) {
        (Ok(rest), Some(home)) => home.join(rest),
        _ => path,
    }
}
