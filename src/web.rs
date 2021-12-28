use std::cmp::min;
use std::path::Path;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use futures::stream::StreamExt;
use crate::config::Configuration;
use crate::util::progress_bar::ProgressBar;

/// Downloads the jar from the configuration URL
pub async fn download_server(config: &Configuration, target: &str) -> Result<(), std::io::Error> {
    let server = &config.server;

    let response = Client::new()
        .get(server)
        .send()
        .await
        .expect("Error");

    let max_size = response.content_length().unwrap() as i32;

    let mut stream = response.bytes_stream();

    let mut file = File::create(Path::new(target)).await?;

    let mut downloaded = 0;

    let mut bar = ProgressBar::new(max_size);

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file"))).expect("Error");
        let _ = file.write(&chunk).await?;
        downloaded = min(downloaded + chunk.len() as i32, max_size);
        let _ = &bar.set(downloaded);
        let _ = &bar.print();
    }

    // Exit Carriage Return
    bar.clear_text();
    Ok(())
}

/// Downloads the file from the url and saves it to the target
pub async fn download(url: String, target: String) -> Result<(), std::io::Error> {
    let response = Client::new()
        .get(&url)
        .send()
        .await
        .expect("Error");

    let mut file = File::create(&target).await?;

    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file"))).expect("Error");
        file.write(&chunk).await?;
    }

    Ok(())
}