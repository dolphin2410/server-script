use std::path::Path;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::fs::File;

use crate::cli::Cli;

/// A struct of the server-script configurations. Serde will parse the configuration file with some default fields.
#[derive(Deserialize, Serialize, Clone)]
pub struct Configuration {
    #[serde(default = "default_server")]
    pub server: String,

    #[serde(default = "bool::default")]
    pub debug: bool,

    #[serde(default = "debug_port")]
    pub debug_port: i32,

    #[serde(default = "bool::default")]
    pub backup: bool,

    #[serde(default = "bool::default")]
    pub restart: bool,

    #[serde(default = "bool::default")]
    pub no_update: bool,

    #[serde(default = "memory")]
    pub memory: i32,

    #[serde(default = "default_plugins")]
    pub plugins: Vec<String>,

    #[serde(default = "Vec::new")]
    pub jvm_args: Vec<String>,
}

impl Configuration {
    pub async fn apply(&mut self, cli: &Cli) {
        self.server = cli.server.clone();
        if cli.debug {
            self.debug = cli.debug;
        }
        self.debug_port = cli.debug_port;
        if cli.backup {
            self.backup = cli.backup;
        }
        self.memory = cli.memory;
        if cli.no_update {
            self.no_update = cli.no_update;
        }

        if cli.save_config {
            save_config(self.clone()).await.unwrap();
        }
    }
}

pub fn default_version() -> String {
    String::from("1.19.2")
}

/// The default server url
pub fn default_server() -> String {
    format!("paperapi://{}", default_version())
}

/// The default memory in Gigabytes
pub fn memory() -> i32 {
    1
}

/// The default debug port
pub fn debug_port() -> i32 {
    5005
}

/// The default plugins
pub fn default_plugins() -> Vec<String> {
    Vec::<String>::new().into_iter().map(String::from).collect()
}

/// Loads the `server.conf.json` file and deserializes it to the `Configuration` struct.
pub async fn load_config() -> Result<Configuration, std::io::Error> {
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

pub async fn save_config(config: Configuration) -> Result<(), std::io::Error> {
    let path = Path::new("server.conf.json");
    if !path.exists() {
        let _ = File::create(&path).await?;
        fs::write(&path, "{}").await?;
    }

    let data_str = serde_json::to_string_pretty(&config)?;
    fs::write(path, data_str).await?;

    Ok(())
}