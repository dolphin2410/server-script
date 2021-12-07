use std::path::Path;
use std::fs;
use std::fs::File;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    #[serde(default = "default_server")]
    server: String,
    #[serde(default = "f")]
    debug: bool,
    #[serde(default = "debug_port")]
    debug_port: i32,
    #[serde(default = "f")]
    backup: bool,
    #[serde(default = "f")]
    restart: bool,
    #[serde(default = "memory")]
    memory: i32,
    #[serde(default = "Vec::new")]
    plugins: Vec<String>,
    #[serde(default = "Vec::new")]
    jvm_args: Vec<String>,
}

pub fn default_server() -> String {
    String::from("https://clip.aroxu.me/download?mc_version=1.18")
}

pub fn memory() -> i32 {
    1
}

pub fn debug_port() -> i32 {
    5005
}

pub fn f() -> bool {
    false
}

pub fn fetch() -> Configuration {
    let path = Path::new("config.json");

    if !path.exists() {
        File::create(&path);
        fs::write(&path, "{}");
    }

    let data = serde_json::from_str::<Configuration>(
        fs::read_to_string(&path)
            .expect("Failed to read configuration")
            .as_str()
    )
        .expect("Failed to parse JSON");

    println!("{}", data.server);

    data
}
