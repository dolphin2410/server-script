use std::error::Error;

use bytes::{Buf, BytesMut};
use serde::Deserialize;

use crate::web::fetch_bytes;

#[derive(Deserialize)]
struct VersionInfo {
    builds: Vec<u32>
}

/// Fetch paper url with specified version and build. If there isn't any builds, it will automatically find the latest one
pub async fn fetch_paper(version: &str, build: &Option<u32>) -> Result<String, Box<dyn Error>> {
    let build = if let Some(build) = build {
        build.to_owned()
    } else {
        find_latest_build(version).await.unwrap()
    };

    Ok(format!("https://api.papermc.io/v2/projects/paper/versions/{version}/builds/{build}/downloads/paper-{version}-{build}.jar"))
}

/// Finds the latest build with the specified version
pub async fn find_latest_build(version: &str) -> Result<u32, Box<dyn Error>> {
    let builds_url = format!("https://api.papermc.io/v2/projects/paper/versions/{version}").parse::<hyper::Uri>()?;
    let mut buffer = BytesMut::with_capacity(1024);
    fetch_bytes(builds_url, &mut buffer).await?;
    let version_info = serde_json::from_reader::<_, VersionInfo>(buffer.reader())?;
    Ok(version_info.builds.last().unwrap().to_owned())
}