use std::error::Error;

use bytes::Buf;
use serde::Deserialize;

use crate::web::fetch_bytes;

#[derive(Deserialize)]
struct VersionInfo {
    builds: Vec<u32>
}

pub async fn fetch_paper(version: &str, build: &Option<u32>) -> Result<String, Box<dyn Error>> {
    let build = if let Some(build) = build {
        build.to_owned()
    } else {
        find_build(version).await.unwrap()
    };

    Ok(format!("https://api.papermc.io/v2/projects/paper/versions/{version}/builds/{build}/downloads/paper-{version}-{build}.jar"))
}

pub async fn find_build(version: &str) -> Result<u32, Box<dyn Error>> {
    let builds_url = format!("https://api.papermc.io/v2/projects/paper/versions/{version}").parse::<hyper::Uri>()?;
    let buf = fetch_bytes(builds_url).await?;
    let version_info = serde_json::from_reader::<_, VersionInfo>(buf.reader())?;
    Ok(version_info.builds.last().unwrap().to_owned())
}