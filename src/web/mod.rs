use std::cmp::min;
use crate::config::Configuration;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use reqwest::Client;
use futures::StreamExt;
use crate::util::progress_bar::ProgressBar;

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

    let mut downloaded: i32 = 0;

    let mut bar = ProgressBar::new(max_size as f32);

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file"))).expect("Error");
        file.write(&chunk)
            .or(Err(format!("Error while writing to file"))).expect("Error");
        let new = min(downloaded + chunk.len() as i32, max_size);
        downloaded = new;
        let _ = &bar.set(downloaded as f32);
        let _ = &bar.print();
    }

    println!();
}