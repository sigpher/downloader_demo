use anyhow::{Context, Ok, Result};

use std::fs;

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

pub async fn download_photos() -> Result<()> {
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
