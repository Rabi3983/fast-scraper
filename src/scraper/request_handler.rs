rustuse reqwest::blocking::get;
use reqwest::header::{USER_AGENT};
use std::error::Error;

pub fn fetch_html(url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .send()?;

    let body = response.text()?;
    Ok(body)
}