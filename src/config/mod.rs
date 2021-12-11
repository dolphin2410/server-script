use std::path::Path;
use std::fs;
use std::fs::File;
use serde::{Deserialize, Serialize};

/// A struct of the server-script configurations. Serde will parse the configuration file with some default fields.
#[derive(Deserialize, Serialize)]
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

    #[serde(default = "memory")]
    pub memory: i32,

    #[serde(default = "default_plugins")]
    pub plugins: Vec<String>,

    #[serde(default = "Vec::new")]
    pub jvm_args: Vec<String>,
}

/// The default server url
pub fn default_server() -> String {
    String::from("https://clip.aroxu.me/download?mc_version=1.18")
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
    vec![
        "https://github.com/monun/auto-reloader/releases/latest/download/AutoReloader.jar"
    ].into_iter().map(String::from).collect()
}

/// Loads the `server.conf.json` file and deserializes it to the `Configuration` struct.
pub fn load_config() -> Configuration {
    let path = Path::new("server.conf.json");

    let mut created = false;

    // Create file if doesn't exists. Defaults to an empty object
    if !path.exists() {
        let _ = File::create(&path);
        let _ = fs::write(&path, "{}");
        created = true
    }

    // Parse the configurations
    let data = serde_json::from_str::<Configuration>(
        fs::read_to_string(&path)
            .expect("Failed to read configuration")
            .as_str()
    )
        .expect("Failed to parse JSON");


    // If a new file was created, set the default values
    if created {
        let data_str = serde_json::to_string_pretty::<Configuration>(&data).expect("Failed to serialize configuration");
        let _ = fs::write(path, data_str);
    }

    data
}