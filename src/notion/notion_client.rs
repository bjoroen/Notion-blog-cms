use reqwest::{header, Client, Error};
use serde_json::Value;

use super::notion_db_struct::NotionDbResponse;

pub struct NotionClient {
    client: reqwest::Client,
}

impl NotionClient {
    pub async fn new() -> Self {
        Self {
            client: Self::build_client(),
        }
    }

    fn build_client() -> Client {
        let key_sec = std::env::var("KEY_SEC").expect("KEY must be set.");
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", key_sec).parse().unwrap(),
        );
        headers.insert("Notion-Version", "2022-06-28".parse().unwrap());

        reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap()
    }

    pub async fn db_request(&self) -> Result<NotionDbResponse, Error> {
        let db_id = std::env::var("DB_ID").expect("DB ID must be set.");
        let request_url = format!(
            "https://api.notion.com/v1/databases/{db_id}/query",
            db_id = db_id
        );

        let res: NotionDbResponse = self.client.post(request_url).send().await?.json().await?;

        Ok(res)
    }

    pub async fn page_request(&self, block_id: String) -> Result<Value, Error> {
        let request_url = format!(
            "https://api.notion.com/v1/blocks/{block_id}/children",
            block_id = block_id
        );

        let res: Value = self.client.get(request_url).send().await?.json().await?;

        Ok(res)
    }
}
