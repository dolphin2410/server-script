use std::path::Path;
use chrono::Local;
use flate2::write::GzEncoder;
use flate2::Compression;
use tokio::fs;
use tokio::fs::File;
use crate::util::logger;

pub async fn backup() -> Result<(), std::io::Error> {
    let backup_dir_path = Path::new(".backup");
    if !backup_dir_path.exists() {
        fs::create_dir(backup_dir_path).await?;
    }

    let date = Local::now().format("%Y%m%d-%H%M%S");
    let tar_gz = File::create(format!(".backup/{}.tar.gz", date)).await?;
    let enc = GzEncoder::new(tar_gz.into_std().await, Compression::default());
    let mut tar = tar::Builder::new(enc);

    let mut files = fs::read_dir("./").await?;
    while let Some(entry) = files.next_entry().await? {
        let file = entry.file_name();
        let file_name = file.to_str().unwrap();
        if file_name == ".backup" || file_name == "cache" {
            continue;
        }
        tar.append_path(Path::new(file_name)).expect("Failed to add the directory to the backup archive");
    }

    tar.finish().expect("Error occurred while finishing backup");

    logger::log("Backup Complete!");

    Ok(())
}