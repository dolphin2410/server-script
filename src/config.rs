use std::path::Path;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::fs::File;
use anyhow::Result;
use crate::cli::Cli;

/// A struct of the server-script configurations. Serde will parse the configuration file with some default fields.
#[derive(Deserialize, Serialize, Clone)]
pub struct Configuration {
    #[serde(default = "default_server")]
    /// The server url notation [using http(s) or paper_api]
    pub server: String,

    #[serde(default = "bool::default")]
    /// Debug?
    pub debug: bool,

    #[serde(default = "debug_port")]
    /// Debug Port?
    pub debug_port: i32,

    #[serde(default = "bool::default")]
    /// Backup after server close?
    pub backup: bool,

    #[serde(default = "bool::default")]
    /// Restart after server close?
    pub restart: bool,

    #[serde(default = "bool::default")]
    /// Don't update server on every run?
    pub no_update: bool,

    #[serde(default = "memory")]
    /// Memory
    pub memory: i32,

    #[serde(default = "default_plugins")]
    /// Plugins to install before run
    pub plugins: Vec<String>,

    #[serde(default = "Vec::new")]
    /// JVM args
    pub jvm_args: Vec<String>,

    #[serde(default = "bool_true")]
    /// Show ip on run?
    pub show_ip: bool
}

impl Configuration {
    /// Apply CLI configuration to the current configuration
    pub async fn apply(&mut self, cli: &Cli) {
        if let Some(cli_server) = &cli.server {
            self.server = cli_server.clone();
        }

        if let Some(cli_debug) = cli.debug {
            self.debug = cli_debug;
        }

        if let Some(cli_debug_port) = cli.debug_port {
            self.debug_port = cli_debug_port;
        }

        if let Some(cli_backup) = cli.backup {
            self.backup = cli_backup;
        }

        if let Some(cli_memory) = cli.memory {
            self.memory = cli_memory
        }

        if let Some(cli_no_update) = cli.no_update {
            self.no_update = cli_no_update;
        }
        if let Some(cli_show_ip) = cli.show_ip {
            self.show_ip = cli_show_ip;
        }

        if cli.save_config.unwrap_or(false) {
            save_config(self.clone()).await.unwrap();
        }
    }
}

/// The default server url
pub fn default_server() -> String {
    "paperapi://1.21.8".to_string()
}

/// The default memory in Gigabytes
pub fn memory() -> i32 {
    2
}

/// The default debug port
pub fn debug_port() -> i32 {
    5005
}

/// The default plugins
pub fn default_plugins() -> Vec<String> {
    Vec::<String>::new().into_iter().map(String::from).collect()
}

/// TRUE
pub fn bool_true() -> bool {
    true
}

/// Loads the `server.conf.json` file and deserializes it to the `Configuration` struct.
pub async fn load_config() -> Result<Configuration> {
    let path = Path::new("server.conf.json");

    // Create file if doesn't exists. Defaults to an empty object
    if !path.exists() {
        let _ = File::create(&path).await?;
        fs::write(&path, "{}").await?;
    }

    // Parse the configurations
    let data = serde_json::from_str::<Configuration>(
        fs::read_to_string(&path).await?
            .as_str()
    )?;

    // Pretty Print
    let data_str = serde_json::to_string_pretty::<Configuration>(&data)?;
    fs::write(path, data_str).await?;

    Ok(data)
}

/// Saves Configuration to server.conf.json
pub async fn save_config(config: Configuration) -> Result<()> {
    let path = Path::new("server.conf.json");
    if !path.exists() {
        let _ = File::create(&path).await?;
        fs::write(&path, "{}").await?;
    }

    let data_str = serde_json::to_string_pretty(&config)?;
    fs::write(path, data_str).await?;

    Ok(())
}