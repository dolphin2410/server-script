use std::path::Path;
use std::process::{Command, Stdio};
use termcolor::Color;
use tokio::fs;
use server_script::{backup, web, config, cli, util::{java_util, logger, runner_util}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    if cfg!(target_os = "windows") {
        use std::ptr;
        use winapi::um::wincon::GetConsoleWindow;
        use winapi::um::winuser::SetWindowTextA;
        use std::ffi::CString;

        let window = unsafe { GetConsoleWindow() };
        if window != ptr::null_mut() {
            unsafe {
                let cstr = CString::new("Server Script").unwrap();
                SetWindowTextA(window, cstr.as_ptr());
                
            }
        }
    }

    print!("[Logger] ");
    logger::log("Running server-script v2.0.0", Some(Color::Cyan), None);
    print!("[Logger] ");
    logger::log("Report bugs here: https://github.com/dolphin2410/server-script", Some(Color::Cyan), None);

    let cli = cli::parse();

    // Loads the config
    let mut configuration = config::load_config().await?;

    configuration.apply(&cli);

    let jarfile = "server.jar";

    let jar_path = Path::new(jarfile);

    if !jar_path.exists() || !configuration.no_update {
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
            logger::log("Starting Backup...", None, None);
            backup::backup().await?;
        }

        if !&configuration.restart {
            break;
        }

        logger::log("Restarting...", None, None);
    }

    logger::log("Exiting...", None, None);

    Ok(())
}