use std::fs;

use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Photo<'a> {
    #[serde(rename = "albumId")]
    pub album_id: u32,
    pub id: u32,
    pub title: &'a str,
    pub url: &'a str,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: &'a str,
}

pub async fn download_file_by_url(url: &str, dir: &str) -> Result<()> {
    let filename = url
        .split('/')
        .last()
        .map(|name| format!("{name}.png"))
        .unwrap();

    let client = reqwest::Client::new();
    let contents = client
        .get(url)
        .send()
        .await
        .context("sending url failed")?
        .bytes()
        .await
        .context("extrating text failed")?;
    let path = format!("{}/{}", dir, filename);
    fs::write(path, contents).context("write file error")?;

    Ok(())
}
