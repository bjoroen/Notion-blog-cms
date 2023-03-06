use reqwest::Error;
use serde_json::Value;

use super::notion_client::NotionClient;

pub async fn get_notion_page_data(page_id: String) -> Result<Value, Error> {
    let notion_client = NotionClient::new().await;
    let page_request_result: Value = notion_client
        .page_request(page_id)
        .await
        .expect("Wanted PageData Json got: {}");

    Ok(page_request_result)
}
