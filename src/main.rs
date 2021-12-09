pub mod config;
pub mod web;
pub mod util;

use futures::executor;
use std::process::{Command, Stdio};
use crate::util::{java_util, runner_util};

#[tokio::main]
async fn main() {

    // Loads the config
    let configuration = config::load_config();

    let jarfile = "server.jar";

    // Download the jar
    executor::block_on(web::download_server(&configuration, jarfile));

    let executable = java_util::find_executable();

    let args = runner_util::default_args(jarfile, &configuration);

    // Execute the program
    Command::new(executable)
        .args(args)
        .stdout(Stdio::inherit())
        .spawn()
        .unwrap().wait().unwrap();
}