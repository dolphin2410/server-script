use std::error::Error;

use bytes::{Buf, BytesMut};
use serde::Deserialize;

use crate::web::fetch_bytes;

#[derive(Deserialize)]
struct BuildServerDefault {
    url: String
}

#[derive(Deserialize)]
struct BuildDownloads {
    #[serde(rename="server:default")]
    server_default: BuildServerDefault
}

#[derive(Deserialize)]
struct BuildInfo {
    id: u32,
    downloads: BuildDownloads
}

/// Fetch paper url with specified version and build. If there isn't any builds, it will automatically find the latest one
pub async fn fetch_paper(version: &str, build: &Option<u32>) -> Result<String, Box<dyn Error>> {
    let builds_list = get_builds_list(version).await?;

    let build_index = if let Some(build) = build {
        builds_list.iter().position(|x| &x.id == build).ok_or("Build not Found")?
    } else {
        0
    };

    Ok(builds_list[build_index].downloads.server_default.url.clone())
}

/// Fetches the list of builds of the given version
async fn get_builds_list(version: &str) -> Result<Vec<BuildInfo>, Box<dyn Error>> {
    let builds_url = format!("https://fill.papermc.io/v3/projects/paper/versions/{version}/builds").parse::<hyper::Uri>()?;
    let mut buffer = BytesMut::with_capacity(1024);
    fetch_bytes(builds_url, &mut buffer).await?;

    let version_info = serde_json::from_reader::<_, Vec<BuildInfo>>(buffer.reader())?;
    Ok(version_info)
}