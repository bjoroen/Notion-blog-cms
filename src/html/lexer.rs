use serde_json::Value;

use crate::notion::page_data_struct::{Code, Heading, Image, Paragraph};

use super::token::TokenType;

pub struct Lexer<'a> {
    pub tokens: Vec<TokenType>,
    json_source: &'a Value,
    pub title: String,
}

impl Lexer<'_> {
    pub fn new(json_source: &Value, title: String) -> Lexer<'_> {
        Lexer {
            title,
            json_source,
            tokens: Vec::new(),
        }
    }

    pub fn collect_tokens(&mut self) {
        Self::get_data_from_json(&self.json_source, &mut self.tokens)
    }

    pub fn get_data_from_json(v: &Value, arr: &mut Vec<TokenType>) {
        match Some(v) {
            Some(v) => match v {
                _ if v.is_array() => {
                    let val_as_array = v.as_array().unwrap();
                    for item in val_as_array {
                        Self::get_data_from_json(item, arr)
                    }
                }
                _ if v.is_object() => {
                    for (key, value) in v.as_object().unwrap().iter() {
                        match value {
                            _ if value.is_array() | value.is_object() => {
                                Self::get_data_from_json(value, arr)
                            }
                            &_ => {}
                        }
                        match key {
                            _ if key == "paragraph" => {
                                let p = serde_json::from_value::<Paragraph>(value.to_owned());
                                match p {
                                    Ok(v) => {
                                        if v.rich_text.len() > 0 {
                                            arr.push(TokenType::Paragraph {
                                                literal: v.rich_text[0]
                                                    .plain_text
                                                    .to_owned()
                                                    .unwrap(),
                                                start_tag: "<p>".to_string(),
                                                end_tag: "</p>".to_string(),
                                            })
                                        } else {
                                            // Reponse from Notion has Empty Paragraphs if there is
                                            // a new line in the notion document
                                            arr.push(TokenType::Paragraph {
                                                literal: String::from("\n"),
                                                start_tag: "<p>".to_string(),
                                                end_tag: "</p>".to_string(),
                                            })
                                        }
                                    }
                                    Err(e) => panic!("{}", e),
                                }
                            }
                            _ if key == "heading_1" => {
                                let h1: Heading = serde_json::from_value(value.to_owned()).unwrap();
                                let h1_content = &h1.rich_text[0].plain_text;
                                match h1_content {
                                    Some(v) => arr.push(TokenType::HeadingOne {
                                        literal: v.to_string(),
                                        start_tag: "<h1>".to_string(),
                                        end_tag: "</h1>".to_string(),
                                    }),
                                    None => {}
                                }
                            }
                            _ if key == "heading_2" => {
                                let h2: Heading = serde_json::from_value(value.to_owned()).unwrap();
                                let h2_content = &h2.rich_text[0].plain_text;
                                match h2_content {
                                    Some(v) => arr.push(TokenType::HeadingTwo {
                                        literal: v.to_string(),
                                        start_tag: "<h2>".to_string(),
                                        end_tag: "</h2>".to_string(),
                                    }),
                                    None => {}
                                }
                            }
                            _ if key == "heading_3" => {
                                let h3: Heading = serde_json::from_value(value.to_owned()).unwrap();
                                let h3_content = &h3.rich_text[0].plain_text;
                                match h3_content {
                                    Some(v) => arr.push(TokenType::HeadingThree {
                                        literal: v.to_string(),
                                        start_tag: "<h3>".to_string(),
                                        end_tag: "</h3>".to_string(),
                                    }),
                                    None => {}
                                }
                            }

                            _ if key == "image" => {
                                let image: Image =
                                    serde_json::from_value(value.to_owned()).unwrap();
                                let link_to_image = image.file.url;
                                // let caption = match &image.caption {
                                //     Some(caption) => &caption[0].plain_text.unwrap(),
                                //     None => {
                                //         panic!("Panic MF")
                                //     }
                                // };

                                arr.push(TokenType::Image {
                                    literal: link_to_image,
                                    start_tag: "<img>".to_string(),
                                    end_tag: "</img>".to_string(),
                                })
                            }

                            _ if key == "code" => {
                                if let Ok(code) = serde_json::from_value::<Code>(value.to_owned()) {
                                    let content_of_code = &code.rich_text[0].plain_text.as_ref();
                                    let languge = &code.language.to_string();
                                    arr.push(TokenType::Code {
                                        literal: content_of_code.unwrap().to_owned(),
                                        language: languge.to_string(),
                                        start_tag: "<pre><code class=\"".to_string(),
                                        end_tag: "</code></pre>".to_string(),
                                    })
                                }
                            }
                            &_ => {}
                        }
                    }
                }
                &_ => {}
            },
            None => return,
        }
    }
}
