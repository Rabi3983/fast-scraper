rustuse scraper::{Html, Selector};
use serde_json::json;

pub fn parse_html(content: &str) -> Html {
    Html::parse_document(content)
}

pub fn extract_text(document: &Html, selector: &str) -> Vec<String> {
    let selector = Selector::parse(selector).unwrap();
    document.select(&selector).map(|element| element.text().collect()).collect()
}

pub fn extract_html(document: &Html, selector: &str) -> Vec<String> {
    let selector = Selector::parse(selector).unwrap();
    document.select(&selector).map(|element| element.inner_html()).collect()
}