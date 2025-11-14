rustuse scraper::{Html, Selector};
use crate::config::Config;
use crate::scraper::html_parser::{extract_text, extract_html};

pub fn extract_data(document: &Html, config: &Config) -> serde_json::Value {
    let mut result = vec![];

    for rule in &config.extract_rules {
        let extracted_values = match rule.extract_type.as_str() {
            "text" => extract_text(document, &rule.selector),
            "html" => extract_html(document, &rule.selector),
            _ => vec![],
        };

        let entry = json!({
            "id": rule.id,
            "url": config.url,
            "data": extracted_values,
        });

        result.push(entry);
    }

    json!(result)
}