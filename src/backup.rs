use std::path::Path;
use chrono::Local;
use flate2::write::GzEncoder;
use flate2::Compression;
use termcolor::Color;
use tokio::fs;
use tokio::fs::File;
use crate::util::logger;

/// Backup the current directory. Excludes the following folders - `.backup` and `cache`
/// The backup file format will be `.backup/YYYYMMDD-HHmmSS.tar.gz`.
/// The backup process isn't stopped even if an element has an issue with compressing. 
/// It skips the one with the error, but the error message is still printed
pub async fn backup() -> Result<(), std::io::Error> {
    let backup_dir_path = Path::new(".backup"); // Check if backup folder exists
    if !backup_dir_path.exists() {
        fs::create_dir(backup_dir_path).await?;
    }

    let date = Local::now().format("%Y%m%d-%H%M%S");
    let compressed_backup_file = File::create(format!(".backup/{}.tar.gz", date)).await?;   // Backup file

    let encoder = GzEncoder::new(compressed_backup_file.into_std().await, Compression::default());
    let mut tar_encoder = tar::Builder::new(encoder);

    while let Some(entry) = fs::read_dir(".").await?.next_entry().await? {  // Iterate backup directory
        let file = entry.file_name();   // File
        let file_name = file.to_str().unwrap(); // Name of file

        if file_name == ".backup" || file_name == "cache" { // Exclude these files
            continue;
        }

        if let Err(_) = tar_encoder.append_path(Path::new(file_name)) { // Error handling
            logger::color_println(format!("Failed to add directory: '{}' to the backup archive...", file_name), Some(Color::Red), None);
        }
    }

    tar_encoder.finish().expect("Error occurred while finishing backup");

    logger::color_println("Backup Complete!", None, None);

    Ok(())
}