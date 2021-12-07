use std::path::Path;
use std::fs;
use std::fs::File;
use serde_derive::{Deserialize, Serialize};

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

    #[serde(default = "Vec::new")]
    pub plugins: Vec<String>,

    #[serde(default = "Vec::new")]
    pub jvm_args: Vec<String>,
}

// The default server url
pub fn default_server() -> String {
    String::from("https://clip.aroxu.me/download?mc_version=1.18")
}

// The default memory in Gigabytes
pub fn memory() -> i32 {
    1
}

// The default debug port
pub fn debug_port() -> i32 {
    5005
}

pub fn load_config() -> Configuration {
    let path = Path::new("config.json");

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