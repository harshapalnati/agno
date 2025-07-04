use crate::tool::tool_traits::Tool;
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

pub struct SearchTool;

impl SearchTool {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Deserialize)]
struct DuckDuckGoResponse {
    #[serde(rename = "AbstractText")]
    abstract_text: String,
    #[serde(rename = "RelatedTopics")]
    related_topics: Vec<RelatedTopic>,
}

#[derive(Debug, Deserialize)]
struct RelatedTopic {
    #[serde(rename = "Text")]
    text: Option<String>,
}

#[async_trait]
impl Tool for SearchTool {
    fn name(&self) -> &'static str {
        "search"
    }

    async fn call(&self, input: &str) -> String {
        let query = input.trim();
        let url = format!(
            "https://api.duckduckgo.com/?q={}&format=json&no_redirect=1&no_html=1",
            urlencoding::encode(query)
        );

        let client = Client::new();
        let response = match client.get(&url).send().await {
            Ok(resp) => resp,
            Err(err) => {
                return format!("❌ Failed to send request: {}", err);
            }
        };

        let parsed: Result<DuckDuckGoResponse, _> = response.json().await;
        match parsed {
            Ok(data) => {
                if !data.abstract_text.is_empty() {
                    format!("🔎 {}", data.abstract_text)
                } else if let Some(related) = data
                    .related_topics
                    .into_iter()
                    .find_map(|topic| topic.text)
                {
                    format!("🔎 Related: {}", related)
                } else {
                    "🤷 No relevant result found.".to_string()
                }
            }
            Err(err) => format!("❌ Failed to parse JSON: {}", err),
        }
    }
}
