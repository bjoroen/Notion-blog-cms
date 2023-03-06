use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotionDbResponse {
    pub object: String,
    pub results: Vec<ResultArray>,
    #[serde(rename = "next_cursor")]
    pub next_cursor: Value,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub page: Page,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ResultArray")]
#[serde(rename_all = "camelCase")]
pub struct ResultArray {
    pub object: String,
    pub id: String,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "last_edited_time")]
    pub last_edited_time: String,
    #[serde(rename = "created_by")]
    pub created_by: CreatedBy,
    #[serde(rename = "last_edited_by")]
    pub last_edited_by: LastEditedBy,
    pub cover: Value,
    pub icon: Value,
    pub parent: Parent,
    pub archived: bool,
    pub properties: Properties,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedBy {
    pub object: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastEditedBy {
    pub object: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "database_id")]
    pub database_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    #[serde(rename = "Title")]
    pub title: Title,
    #[serde(rename = "Tags")]
    pub tags: Tags,
    #[serde(rename = "Name")]
    pub name: Name,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "rich_text")]
    pub rich_text: Vec<RichText>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichText {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Option<Text>,
    pub annotations: Annotations,
    #[serde(rename = "plain_text")]
    pub plain_text: String,
    pub href: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub content: String,
    pub link: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "multi_select")]
    pub multi_select: Vec<MultiSelect>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiSelect {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: Vec<Title2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Text2,
    pub annotations: Annotations2,
    #[serde(rename = "plain_text")]
    pub plain_text: String,
    pub href: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text2 {
    pub content: String,
    pub link: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations2 {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {}
