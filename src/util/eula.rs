use std::path::Path;

use tokio::fs;
use anyhow::Result;

/// Check if eula is agreed
pub async fn eula_agreed() -> bool {
    let path = Path::new("eula.txt");
    if !path.exists() {
        return false;
    }

    let Ok(buf) = fs::read_to_string(path).await else {
        return false;
    };

    for line in buf.lines() {
        let no_whitespace: String = line.chars().filter(|c| !c.is_whitespace()).collect();
        if no_whitespace.contains("eula=true") {
            return true;
        }
    }

    return false;
}

/// Agree to eula
pub async fn agree_eula() -> Result<()> {
    let path = Path::new("eula.txt");
    fs::write(path, b"eula=true").await?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::{agree_eula, eula_agreed};

    #[tokio::test]
    async fn test_agree_eula() {
        println!("{}", eula_agreed().await);
        agree_eula().await.unwrap();
        println!("{}", eula_agreed().await);
    }
}