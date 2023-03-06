use serde::{Deserialize, Serialize};
use serde_json::Value;

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PageData {
//     pub object: String,
//     pub results: Vec<Result>,
//     #[serde(rename = "next_cursor")]
//     pub next_cursor: Value,
//     #[serde(rename = "has_more")]
//     pub has_more: bool,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub block: Block,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Result {
//     pub object: String,
//     pub id: String,
//     pub parent: Parent,
//     #[serde(rename = "created_time")]
//     pub created_time: String,
//     #[serde(rename = "last_edited_time")]
//     pub last_edited_time: String,
//     #[serde(rename = "created_by")]
//     pub created_by: CreatedBy,
//     #[serde(rename = "last_edited_by")]
//     pub last_edited_by: LastEditedBy,
//     #[serde(rename = "has_children")]
//     pub has_children: bool,
//     pub archived: bool,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     #[serde(rename = "heading_1")]
//     pub heading_1: Option<Heading>,
//     #[serde(rename = "heading_2")]
//     pub heading_2: Option<Heading>,
//     #[serde(rename = "heading_3")]
//     pub heading_3: Option<Heading>,
//     pub paragraph: Option<Paragraph>,
//     pub code: Option<Code>,
//     pub image: Option<Image>,
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub caption: Vec<Value>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub file: File,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub url: String,
    #[serde(rename = "expiry_time")]
    pub expiry_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Heading {
    #[serde(rename = "rich_text")]
    pub rich_text: Vec<RichText>,
    #[serde(rename = "is_toggleable")]
    pub is_toggleable: bool,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Code {
    pub caption: Option<Vec<Value>>,
    #[serde(rename = "rich_text")]
    pub rich_text: Vec<RichText>,
    pub language: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "page_id")]
    pub page_id: String,
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
pub struct Heading2 {
    #[serde(rename = "rich_text")]
    pub rich_text: Vec<RichText>,
    pub color: Option<String>,
    #[serde(rename = "is_toggleable")]
    pub is_toggleable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichText {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub text: Option<Text>,
    pub annotations: Annotations,
    #[serde(rename = "plain_text")]
    pub plain_text: Option<String>,
    pub href: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub content: Option<String>,
    pub link: Option<Value>,
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
pub struct Paragraph {
    #[serde(rename = "rich_text")]
    pub rich_text: Vec<RichText>,
    pub color: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub url: Option<String>,
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
pub struct Block {}
