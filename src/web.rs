use std::error::Error;
use bytes::{BytesMut, BufMut};
use follow_redirects::ClientExt;
use hyper::Client;
use hyper::body::HttpBody;
use hyper::header::CONTENT_LENGTH;
use hyper_tls::HttpsConnector;
use crate::config::Configuration;
use crate::protocol;
use crate::util::progress_bar::ProgressBar;

/// Downloads the jar from the configuration URL
pub async fn download_server(config: &Configuration, buf: &mut BytesMut) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::builder()
        .build::<_, hyper::Body>(HttpsConnector::new());

    let server = protocol::Protocol::parse_protocol(&config.server)?.generate_url().await.parse()?;

    let mut response = client.follow_redirects().get(server).await?;

    let total = response.headers()[CONTENT_LENGTH].to_str().unwrap().parse::<u64>()?;

    let mut bar = ProgressBar::new(total);

    while let Some(chunk) = response.body_mut().data().await {
        buf.put(&chunk?[..]);
        bar.set_cursor(buf.len() as u64);
        bar.print();
    }

    bar.clear_text();
    Ok(())
}

/// Downloads the file from the url and saves it to the target
pub async fn download(url: &str, target: &mut BytesMut) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::builder()
        .build::<_, hyper::Body>(HttpsConnector::new());
    
    let uri = url.parse()?;

    let mut response = client.follow_redirects().get(uri).await?;

    while let Some(chunk) = response.body_mut().data().await {
        target.put(&chunk?[..]);
    }

    Ok(())
}

/// Fetch data from the web
pub async fn fetch_bytes(url: hyper::Uri, buffer: &mut BytesMut) -> Result<(), Box<dyn Error>> {
    let client = Client::builder()
        .build::<_, hyper::Body>(HttpsConnector::new());
    
    let mut res = client.follow_redirects().get(url).await?;
    while let Some(chunk) = res.body_mut().data().await {
        buffer.put(&chunk?[..])
    }

    Ok(())
}