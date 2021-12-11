use std::cmp::min;
use crate::config::Configuration;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use reqwest::Client;
use futures::StreamExt;
use crate::util::progress_bar::ProgressBar;

/// Downloads the jar from the configuration URL
pub async fn download_server(config: &Configuration, target: &str) {
    let server = &config.server;

    let response = Client::new()
        .get(server)
        .send()
        .await
        .expect("Error");

    let max_size = response.content_length().unwrap() as i32;

    let mut stream = response.bytes_stream();

    let mut file = File::create(Path::new(target)).expect("Error");

    let mut downloaded = 0;

    let mut bar = ProgressBar::new(max_size);

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file"))).expect("Error");
        file.write(&chunk)
            .or(Err(format!("Error while writing to file"))).expect("Error");
        let new = min(downloaded + chunk.len() as i32, max_size);
        downloaded = new;
        let _ = &bar.set(downloaded);
        let _ = &bar.print();
    }

    // Exit Carriage Return
    bar.clear_text()
}

/// Downloads the file from the url and saves it to the target
pub async fn download(url: &str, target: &str) {
    let response = Client::new()
        .get(url)
        .send()
        .await
        .expect("Error");

    let mut file = File::create(target).expect("Error occurred while creating file");

    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file"))).expect("Error");
        file.write(&chunk)
            .or(Err(format!("Error while writing to file"))).expect("Error");
    }
}