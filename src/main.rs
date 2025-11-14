rustuse std::fs;
use std::io::{self, Write};
use scraper::{Html, Selector};

mod scraper;
mod config;
mod utils;

fn main() -> io::Result<()> {
    // Load configuration settings (e.g., user-agent, retry logic, etc.)
    let config = config::load_config("src/config/settings.example.json").expect("Failed to load config");

    // Example of scraping a URL
    let url = "https://www.scrapethissite.com/pages/forms/";

    // Fetch and parse HTML content
    let body = scraper::request_handler::fetch_html(url).expect("Failed to fetch HTML");
    let document = Html::parse_document(&body);

    // Extract data using the selector engine
    let extracted_data = scraper::selector_engine::extract_data(&document, &config);

    // Output the extracted data to a JSON file
    let output_path = "data/sample_output.json";
    fs::write(output_path, extracted_data.to_string())?;
    Ok(())
}