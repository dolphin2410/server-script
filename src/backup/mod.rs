use std::fs;
use std::fs::File;
use std::path::Path;
use chrono::Utc;
use flate2::write::GzEncoder;
use flate2::Compression;
use crate::util::logger;

pub fn backup() {
    let _ = fs::create_dir(".backup");
    let date = Utc::now().format("%Y%m%d-%H%M%S");
    let tar_gz = File::create(format!(".backup/{}.tar.gz", date)).expect("Failed to create tarball");
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    let files = fs::read_dir("./").unwrap();
    for entry in files {
        let file = entry.unwrap().file_name();
        let file_name = (&file).to_str().unwrap();
        if file_name == ".backup" || file_name == "cache" {
            continue;
        }
        tar.append_path(Path::new(file_name)).expect("Failed to add the directory to the backup archive");
    }

    tar.finish().expect("Error occurred while finishing backup");

    logger::log("Backup Complete!")
}