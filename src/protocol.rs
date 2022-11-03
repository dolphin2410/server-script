use std::error::Error;

use crate::util::paper_api;

pub enum Protocol {
    HTTP { url: String },
    PaperAPI { version: String, build: Option<u32> }
}

impl Protocol {
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

    pub fn parse_protocol(server: &str) -> Result<Protocol, Box<dyn Error + Send + Sync>> {
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
            _ => Err("Invalid Protocol".into())
        }

    }
}