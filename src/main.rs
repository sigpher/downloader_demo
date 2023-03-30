use anyhow::{Context, Ok, Result};
use downloader_demo::{Photo, download_file_by_url};
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
