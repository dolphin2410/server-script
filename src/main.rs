pub mod config;
pub mod web;
pub mod util;
pub mod backup;

use std::path::Path;
use std::process::{Command, Stdio};
use tokio::fs;
use tokio::runtime::Builder;
use crate::util::{java_util, logger, runner_util};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Loads the config
    let configuration = config::load_config().await?;

    let jarfile = "server.jar";
    let thread = Builder::new_current_thread().enable_all().build().unwrap();

    // Download the jar
    thread.spawn(async move {
        web::download_server(&configuration, jarfile).await.unwrap();
        "Returned"
    }).await?;

    let configuration = config::load_config().await?;


    let executable = java_util::find_executable();

    let args = runner_util::default_args(jarfile, &configuration);

    loop {
        fs::create_dir(Path::new("plugins")).await?;

        // Download plugins
        for plugin in configuration.plugins.to_vec() {
            let file_name = plugin.split("/").last().unwrap();
            thread.spawn(web::download(plugin.clone(),format!("plugins/{}", file_name))).await??;
        }

        // Execute the program
        Command::new(&executable)
            .args(&args)
            .stdout(Stdio::inherit())
            .spawn()
            .unwrap().wait().unwrap();


        if configuration.backup {
            logger::log("Starting Backup...");
            backup::backup().await?;
        }

        if !&configuration.restart {
            break;
        }

        logger::log("Restarting...");
    }

    logger::log("Exiting...");

    Ok(())
}