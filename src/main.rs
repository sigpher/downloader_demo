use std::fs;

use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://jsonplaceholder.typicode.com/albums/1/photos";

    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .send()
        .await
        .context("sending url failed")?
        .text()
        .await
        .context("extrating text failed")?;
    let photos: Vec<Photo> = serde_json::from_str(&resp)?;

    for photo in photos {
        download_file_by_url(photo.url, "download").await?;
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Photo<'a> {
    #[serde(rename = "albumId")]
    album_id: u32,
    id: u32,
    title: &'a str,
    url: &'a str,
    #[serde(rename = "thumbnailUrl")]
    thumbnail_url: &'a str,
}

async fn download_file_by_url(url: &str, dir: &str) -> Result<()> {
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
