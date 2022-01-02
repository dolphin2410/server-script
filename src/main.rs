use std::path::Path;
use std::process::{Command, Stdio};
use tokio::fs;
use server_script::{backup, web, config, cli, util::{java_util, logger, runner_util}};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    logger::log("Running server-script v1.0.0");
    logger::log("Report bugs here: https://github.com/dolphin2410/server-script");

    let cli = cli::parse();

    // Loads the config
    let mut configuration = config::load_config().await?;

    configuration.apply(&cli);

    let jarfile = "server.jar";

    let jar_path = Path::new(jarfile);

    if !jar_path.exists() || !cli.no_update {
        // Download the jar
        web::download_server(&configuration, jarfile).await.unwrap();
    }

    let executable = java_util::find_executable();

    let args = runner_util::default_args(jarfile, &configuration);

    loop {
        let plugins_path = Path::new("plugins");
        if !plugins_path.exists() {
            fs::create_dir(plugins_path).await?;
        }

        // Download plugins
        for plugin in configuration.plugins.to_vec() {
            let file_name = plugin.split("/").last().unwrap();
            web::download(plugin.clone(),format!("plugins/{}", file_name)).await?;
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