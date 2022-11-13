use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about = "Server-Script", author = "dolphin2410", version)]
/// Server Script CLI
pub struct Cli {
    #[clap(long)]
    /// the server url notation [using http(s) or paper_api]
    pub server: Option<String>,

    #[clap(long)]
    /// Debug?
    pub debug: Option<bool>,

    #[clap(long)]
    /// Debug Port
    pub debug_port: Option<i32>,

    #[clap(long)]
    /// Backup after server closes?
    pub backup: Option<bool>,

    #[clap(long)]
    /// Restart automatically after server close?
    pub restart: Option<bool>,

    #[clap(long)]
    /// Memory [GB]
    pub memory: Option<i32>,

    #[clap(long)]
    /// Don't update paper server on every run
    pub no_update: Option<bool>,

    #[clap(long)]
    /// Save CLI config to server.conf.json?
    pub save_config: Option<bool>,

    #[clap(long)]
    /// Show IP on run?
    pub show_ip: Option<bool>
}