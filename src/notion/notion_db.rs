use crate::notion::notion_client::NotionClient;

use super::notion_db_struct::{NotionDbResponse, Properties, ResultArray};
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NotionPageMetaData {
    pub created_time: String,
    pub id: String,
    pub last_edited_time: String,
    pub title: String,
    pub status: Option<String>,
}

impl NotionPageMetaData {
    pub fn new(db_response: ResultArray) -> Self {
        let cloned_array = db_response.clone();
        Self {
            created_time: cloned_array.created_time,
            last_edited_time: cloned_array.last_edited_time,
            id: cloned_array.id,
            status: Self::get_status(&db_response),
            // TODO: Make it so it can handle a page without a title, and set a default title.
            title: cloned_array.properties.title.rich_text[0]
                .plain_text
                .clone(),
        }
    }

    fn get_status(db_response: &ResultArray) -> Option<String> {
        if db_response.properties.tags.multi_select.len() > 0 {
            return Some(
                db_response.properties.tags.multi_select[0]
                    .color
                    .to_string(),
            );
        } else {
            None
        }
    }
}

pub async fn get_notion_db() -> Result<Vec<NotionPageMetaData>, Error> {
    let notion_client = NotionClient::new().await;
    let res: NotionDbResponse = match notion_client.db_request().await {
        Ok(response) => response,
        Err(response) => panic!("Error in request: {}", response),
    };

    let mut page_meta_data_array: Vec<NotionPageMetaData> = Vec::new();

    for element in res.results {
        let page_meta_data = NotionPageMetaData::new(element);
        if page_meta_data.status == Some("green".to_string()) {
            page_meta_data_array.push(page_meta_data)
        }
    }

    Ok(page_meta_data_array)
}
