use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use bytes::BytesMut;
use clap::Parser;
use local_ip_address::local_ip;
use requestty::Question;
use server_script::util::eula::{eula_agreed, agree_eula};
use server_script::util::logger::LogStream;
use termcolor::Color;
use tokio::fs;
use server_script::{backup, web, config, cli, util::{java_util, logger, runner_util}};

const LOCAL_SERVER_PATH: &str = "server.jar";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::System::Console::SetConsoleTitleA;
        use windows::core::PCSTR;
        unsafe { 
            SetConsoleTitleA(PCSTR("Server Script".as_ptr()));
            println!(); // todo fix this
        }
    }

    let cli = cli::Cli::parse();

    let mut configuration = config::load_config().await?;

    configuration.apply(&cli).await;

    request_eula().await?;

    let mut log_stream = LogStream::with_colors(Some(Color::Cyan), None);
    log_stream.add_header("[Logger] ".to_string());

    log_stream.logln(format!("Running server-script v{}", env!("CARGO_PKG_VERSION"))).unwrap();
    log_stream.logln("Report bugs here: https://github.com/dolphin2410/server-script").unwrap();

    let mut no_wifi = false;

    if let Ok(ip) = local_ip() {
        if configuration.show_ip {
            log_stream.logln(format!("Your machine's IP is {}", ip)).unwrap();
        }
    } else {
        configuration.no_update = true;
        no_wifi = true;
    }

    let jar_path = Path::new(LOCAL_SERVER_PATH);
    let mut jar_buf = BytesMut::with_capacity(1024 * 1024);    // a megabyte

    if !jar_path.exists() || !configuration.no_update {
        // Download the jar
        web::download_server(&configuration, &mut jar_buf).await.unwrap();
        File::create(LOCAL_SERVER_PATH)?.write_all(&jar_buf[..])?;
    }

    let executable = java_util::find_executable().expect("Java executable wasn't found.");

    let args = runner_util::default_args(LOCAL_SERVER_PATH, &configuration);

    let mut plugin_buffer = BytesMut::with_capacity(1024 * 1024); // a megabyte

    loop {
        let plugins_path = Path::new("plugins");
        if !plugins_path.exists() {
            fs::create_dir(plugins_path).await?;
        }

        // Download plugins
        if !no_wifi {
            for plugin_url in &configuration.plugins {
                let file_name = plugin_url.as_str().split('/').last().expect("Invalid Plugin URL"); // todo this isn't a good way to name plugins
                web::download(plugin_url.as_str(), &mut plugin_buffer).await?;
    
                File::create(file_name)?.write_all(&plugin_buffer[..])?;
                plugin_buffer.clear();
            }
        }

        // Execute the program
        Command::new(&executable)
            .args(&args)
            .stdout(Stdio::inherit())   // Inherit cmd interface
            .spawn()
            .unwrap().wait().unwrap();

        if configuration.backup {
            logger::color_println("Starting Backup...", None, None);
            backup::backup().await?;
        }

        if !&configuration.restart {
            break;
        }

        logger::color_println("Restarting...", None, None);
    }

    logger::color_println("Exiting...", None, None);

    Ok(())
}

async fn request_eula() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !eula_agreed().await {
        let q = Question::confirm("Will you agree to EULA?").default(true).build();
        let answer = requestty::prompt_one(q)?;
        if answer.as_bool().unwrap_or(false) {
            agree_eula().await?;
        } else {
            return Err("You must agree to eula".into());
        }
    }

    return Ok(())
}

#[cfg(test)]
mod tests {
    use server_script::util::logger;
    use termcolor::Color;

    #[test]
    fn test() {
        logger::color_print("None foreground", None, Some(Color::Black));
    }
}