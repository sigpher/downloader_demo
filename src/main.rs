use anyhow::{Ok, Result};
use scraper::{Html, Selector};
// use downloader_demo::download_photos;

#[tokio::main]
async fn main() -> Result<()> {
    get_movies().await?;
    Ok(())
}

pub async fn get_movies() -> Result<()> {
    let url = "http://down.foodmate.net/special/standard/42.html";
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    let doc = Html::parse_fragment(&body);
    // body > div:nth-child(7) > div.fl_rb > div > div.bz_list > ul > li:nth-child(1) > div.bz_listl > ul > a > b
    let selector = Selector::parse("div.bz_listl > ul > a > b").unwrap();
    for el in doc.select(&selector) {
        println!("title: {}", el.inner_html());
    }

    Ok(())
}
