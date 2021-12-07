pub mod config;
pub mod web;
pub mod util;

use std::io::{BufRead, BufReader};
use futures::executor;
use std::process::{Command, Stdio};
use crate::util::{java_util, runner_util};

#[tokio::main]
async fn main() {
    let configuration = config::load_config();

    let jarfile = "server.jar";

    executor::block_on(web::download_server(&configuration, jarfile));

    let executable = java_util::find_executable();

    let args = runner_util::default_args(jarfile, &configuration);

    let mut result = Command::new(executable)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdout = result.stdout.as_mut().unwrap();
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            if let Ok(data) = line {
                println!("{}", data)
            }
        }
    }

    result.wait().unwrap();
}