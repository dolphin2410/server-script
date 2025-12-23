use anyhow::Result;

use crate::util::paper_api;

/// Protocol to generate static url
pub enum Protocol {
    /// Raw Static URL
    HTTP { 
        /// The URL
        url: String
     },
    /// PaperAPI dynamic URL
    PaperAPI { 
        /// The paper version
        version: String, 
        /// Paper build, defaults to latest if set to None
        build: Option<u32>
    }
}

impl Protocol {
    /// Creates a static url based on the protocol
    pub async fn generate_url(&self) -> String {
        match &self {
            Protocol::HTTP { url } => {
                url.clone()
            },
            Protocol::PaperAPI { version, build } => {
                paper_api::fetch_paper(version, build).await.unwrap()
            }
        }
    }

    /// Check if it is HTTP or PAPER_API
    pub fn parse_protocol(server: &str) -> Result<Protocol> {
        let split = server.split("://").collect::<Vec<_>>();
        match split[0].to_lowercase().as_str() {
            "http" | "https" => Ok(Protocol::HTTP { url: split[1].to_string() }),
            "paperapi" | "paper_api" => {
                let data = split[1].split(':').collect::<Vec<_>>();
                // e.g) paper_api://1.19.2:122
                if data.len() == 2 {
                    Ok(Protocol::PaperAPI { version: data[0].to_string(), build: Some(data[1].parse::<u32>()?) })
                } else {
                    Ok(Protocol::PaperAPI { version: data[0].to_string(), build: None })
                }
            },
            _ => Err(anyhow::anyhow!("Invalid Protocol"))
        }

    }
}