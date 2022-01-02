use std::cmp::min;
use std::path::Path;
use futures::FutureExt;
use futures::future::BoxFuture;
use reqwest::{Client, Response};
use tokio::fs::{File, self};
use tokio::io::AsyncWriteExt;
use futures::stream::StreamExt;
use crate::config::Configuration;
use crate::util::progress_bar::ProgressBar;

pub fn request(url: &str) -> BoxFuture<'_, (Response, i32)> {
    async move {
        let response = Client::new()
        .get(url)
        .send()
        .await
        .unwrap();
    
    if let Some(size) = response.content_length() {
        (response, size as i32)
    } else {
        request(url).await
     }
    }.boxed()
}

/// Downloads the jar from the configuration URL
pub async fn download_server(config: &Configuration, target: &str) -> Result<(), std::io::Error> {
    let server = &config.server;

    let (response, max_size) = request(server).await;

    let mut stream = response.bytes_stream();

    let temp_dir = Path::new(".temp");
    if !temp_dir.exists() {
        fs::create_dir(temp_dir).await?;
    }

    let target_file = Path::new(target);

    let path_buf = temp_dir.join(target_file.file_name().unwrap());

    let temp_target_file = Path::new(path_buf.as_os_str());

    let mut file = File::create(temp_target_file).await?;

    let mut downloaded = 0;

    let mut bar = ProgressBar::new(max_size);

    while let Some(item) = stream.next().await {
        let chunk = item.unwrap();
        let _ = file.write(&chunk).await?;
        downloaded = min(downloaded + chunk.len() as i32, max_size);
        bar.set(downloaded);
        bar.print();
    }

    let _ = fs::copy(temp_target_file, target_file).await.unwrap();

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